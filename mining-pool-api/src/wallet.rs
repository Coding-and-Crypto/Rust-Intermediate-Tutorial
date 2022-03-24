use {
    diesel::{
        ExpressionMethods, 
        Insertable, 
        Queryable, 
        QueryDsl, 
        RunQueryDsl
    },
    diesel::result::Error,
    serde::{
        Deserialize, 
        Serialize
    },
    uuid::Uuid,
    super::schema::wallets,
    crate::DBPooledConnection,
    crate::miner::*,
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


pub fn get_workers_online(_wallet_dao: &WalletDAO, conn: &DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;
    match miners.filter(address.eq(_wallet_dao.address)).load::<MinerDAO>(conn) {
        Ok(result) => result
                        .into_iter()
                        .map(|m| m.to_miner(_wallet_dao.club_name.clone()))
                        .collect::<Vec<Miner>>(),
        Err(_) => vec![],
    }
}


pub fn fetch_all_wallets(conn: &DBPooledConnection) -> Vec<Wallet> {
    use crate::schema::wallets::dsl::*;
    match wallets.load::<WalletDAO>(conn) {
        Ok(result) => {
            result.into_iter().map(|w| {
                let workers_online = get_workers_online(&w, conn);
                w.to_wallet(workers_online)
            }).collect::<Vec<Wallet>>()
        },
        Err(_) => vec![],
    }
}


pub fn fetch_wallet_by_id(_address: Uuid, conn: &DBPooledConnection) -> Option<Wallet> {
    use crate::schema::wallets::dsl::*;
    match wallets.filter(address.eq(_address)).load::<WalletDAO>(conn) {
        Ok(result) => {
            match result.first() {
                Some(matched_wallet) => {
                    let workers_online = get_workers_online(&matched_wallet, conn);
                    Some(matched_wallet.to_wallet(workers_online))
                },
                _ => None,
            }
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