use {
    diesel::{
        ExpressionMethods, 
        Insertable, 
        Queryable, 
        RunQueryDsl
    },
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    serde::{
        Deserialize, 
        Serialize
    },
    uuid::Uuid,
    super::schema::wallets,
    crate::DBPooledConnection,
    crate::miner::{Miner, MinerDAO},
};



// --------------- JSON Payload (REST)


#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

impl Wallet {
    pub fn to_wallet_dao(&self) -> WalletDAO {
        WalletDAO {
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            club_name: self.club_name.to_string(),
        }
    }
}


// --------------- POST Request Body for new Miner


#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}



// --------------- DAO Object (DB Table Records)


#[derive(Queryable, Insertable)]
#[table_name = "wallets"]
pub struct WalletDAO {
    pub address: Uuid,
    pub club_name: String,
}

impl WalletDAO {
    pub fn to_wallet(&self, workers_online: Vec<Miner>) -> Wallet {
        Wallet {
            address: self.address.to_string(),
            club_name: self.club_name.to_string(),
            total_hash_rate: workers_online.iter().map(|w| w.hash_rate).sum(),
            total_shares_mined: workers_online.iter().map(|w| w.shares_mined).sum(),
            total_workers_online: workers_online.len() as i32,
            workers_online,
        }
    }
}


// --------------- Service Methods


pub fn fetch_all_wallets(conn: &DBPooledConnection) -> Vec<Wallet> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;
    let all_wallets = match wallets.load::<WalletDAO>(conn) {
        Ok(result) => result,
        Err(_) => vec![],
    };
    let all_miners = match miners.load::<MinerDAO>(conn) {
        Ok(result) => result,
        Err(_) => vec![],
    };
    all_wallets.into_iter().map(|w| {
        let mut workers_online = vec![];
        for m in all_miners.iter() {
            if m.address.eq(&w.address) {
                workers_online.push(
                    m.to_miner(w.club_name.clone())
                );
            };
        };
        w.to_wallet(workers_online)
    }).collect::<Vec<Wallet>>()
}


pub fn fetch_wallet_by_id(_address: Uuid, conn: &DBPooledConnection) -> Option<Wallet> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;
    match wallets
        .filter(crate::schema::wallets::address.eq(_address))
        .load::<WalletDAO>(conn) {
        Ok(result) => match result.first() {
            Some(matched_wallet_dao) => {
                match miners
                    .filter(crate::schema::miners::address.eq(_address))
                    .load::<MinerDAO>(conn) {
                        Ok(miner_result) => Some(matched_wallet_dao.to_wallet(
                            miner_result.into_iter().map(|m| {
                                m.to_miner(matched_wallet_dao.club_name.clone())
                        }).collect::<Vec<Miner>>())),
                        Err(_) => Some(matched_wallet_dao.to_wallet(vec![])),
                    }
            },
            _ => return None,
        },
        Err(_) => None,
    }
}

pub fn create_new_wallet(new_wallet_request: NewWalletRequest, conn: &DBPooledConnection) -> Result<Wallet, Error> {
    use crate::schema::wallets::dsl::*;
    let new_wallet_dao = WalletDAO {
        address: Uuid::new_v4(),
        club_name: new_wallet_request.club_name,
    };
    match diesel::insert_into(wallets).values(&new_wallet_dao).execute(conn) {
        Ok(_) => Ok(new_wallet_dao.to_wallet(vec![])),
        Err(e) => Err(e),
    }
}