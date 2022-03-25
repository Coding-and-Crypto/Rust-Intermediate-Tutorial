use {
    diesel::{
        ExpressionMethods, 
        Insertable, 
        Queryable, 
        QueryDsl,
        RunQueryDsl
    },
    diesel::result::Error,
    rand::Rng,
    serde::{
        Deserialize, 
        Serialize
    },
    uuid::Uuid,
    super::schema::miners,
    crate::DBPooledConnection,
    crate::wallet::*,
};


// --------------- JSON Payload (REST)


#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32, // MH/s
    pub shares_mined: i32,
}

impl Miner {
    pub fn to_miner_dao(&self) -> MinerDAO {
        MinerDAO {
            id: Uuid::parse_str(self.id.as_str()).unwrap(),
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}


// --------------- POST Request Body for new Miner


#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    nickname: String,
}


// --------------- DAO Object (DB Table Records)


#[derive(Queryable, Insertable)]
#[table_name = "miners"]
pub struct MinerDAO {
    pub id: Uuid,
    pub address: Uuid,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl MinerDAO {
    pub fn to_miner(&self, club_name: String) -> Miner {
        Miner {
            id: self.id.to_string(),
            address: self.address.to_string(),
            club_name,
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}


// --------------- Service Methods


pub fn fetch_all_miners(conn: &DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;
    match wallets
        .inner_join(miners)
        .load::<(WalletDAO, MinerDAO)>(conn) {
        Ok(result) => {
            result.into_iter().map(|(w, m)| {
                m.to_miner(w.club_name)
            }).collect::<Vec<Miner>>()
        },
        Err(_) => vec![],
    }
}


pub fn fetch_miner_by_id(_id: Uuid, conn: &DBPooledConnection) -> Option<Miner> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;
    match wallets
        .inner_join(miners)
        .filter(id.eq(_id))
        .load::<(WalletDAO, MinerDAO)>(conn) {
        Ok(result) => {
            match result.first() {
                Some((w, m)) => {
                    Some(m.to_miner(w.club_name.clone()))
                },
                _ => None,
            }
        },
        Err(_) => None,
    }
}


pub fn create_new_miner(new_miner_request: NewMinerRequest,
                        _address: Uuid,
                        conn: &DBPooledConnection) -> Result<Miner, Error> {
    use crate::schema::miners::dsl::*;
    let new_miner_dao = MinerDAO {
        id: Uuid::new_v4(),
        address: _address,
        nickname: new_miner_request.nickname,
        hash_rate: rand::thread_rng().gen_range(20..100), // MH/s
        shares_mined: rand::thread_rng().gen_range(1..40),
    };
    match diesel::insert_into(miners).values(&new_miner_dao).execute(conn) {
        Ok(_) => match fetch_miner_by_id(new_miner_dao.id, conn) {
            Some(result) => Ok(result),
            None => Err(Error::NotFound)
        },
        Err(e) => Err(e),
    }
}