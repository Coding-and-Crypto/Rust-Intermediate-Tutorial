use {
    diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    super::schema::miners,

    crate::DBPooledConnection,
    crate::wallet::fetch_wallet_by_id,
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


pub fn get_club_name(_address: Uuid, conn: &DBPooledConnection) -> String {
    match fetch_wallet_by_id(_address, &conn) {
        Some(matched_wallet) => matched_wallet.club_name,
        None => "Club name not found.".to_string(),
    }
}

pub fn fetch_all_miners(conn: &DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;
    match miners.load::<MinerDAO>(conn) {
        Ok(result) => {
            result.into_iter().map(|m| {
                let club_name = get_club_name(m.address, conn);
                m.to_miner(club_name)
            }).collect::<Vec<Miner>>()
        },
        Err(_) => vec![],
    }
}

pub fn fetch_miner_by_id(_id: Uuid, conn: &DBPooledConnection) -> Option<Miner> {
    use crate::schema::miners::dsl::*;
    match miners.filter(id.eq(_id)).load::<MinerDAO>(conn) {
        Ok(result) => {
            match result.first() {
                Some(matched_miner) => {
                    let club_name = get_club_name(matched_miner.address, conn);
                    Some(matched_miner.to_miner(club_name))
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
    let club_name = get_club_name(_address, &conn);
    let new_miner = Miner {
        id: Uuid::new_v4().to_string(),
        address: _address.to_string(),
        club_name: club_name.to_string(),
        nickname: new_miner_request.nickname,
        hash_rate: rand::thread_rng().gen_range(20..100), // MH/s
        shares_mined: rand::thread_rng().gen_range(1..40),
    };
    let new_miner_dao = new_miner.to_miner_dao();
    match diesel::insert_into(miners).values(&new_miner_dao).execute(conn) {
        Ok(_) => Ok(new_miner_dao.to_miner(club_name.to_string())),
        Err(e) => Err(e),
    }
}
