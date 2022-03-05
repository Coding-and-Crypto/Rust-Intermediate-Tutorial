use {
    actix_web::HttpResponse,
    actix_web::web::Json,
    chrono::{DateTime, Utc},
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    crate::util::*,
};


#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub wallet_address: String,
    pub nickname: String,
    pub hash_rate: i32, // MH/s
    pub shares_mined: i32,
    pub date_joined: DateTime<Utc>,
}

impl Miner {
    pub fn new(wallet_address: String, nickname: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            wallet_address,
            nickname,
            hash_rate: rand::thread_rng().gen_range(20..100),
            shares_mined: rand::thread_rng().gen_range(1..50),
            date_joined: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinerRequest {
    wallet_address: String,
    nickname: String,
}


// List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    let miners: Vec<Miner> = vec![]; // Empty for now
    ResponseType::Ok(miners).get_response()
}

// Get a miner
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    let miner: Option<Miner> = None; // Empty for now
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found".to_string())
        ).get_response(),
    }
}

// List all Miners for Wallet
#[get("/wallets/{id}/miners")]
pub async fn list_wallet_miners() -> HttpResponse {
    let miners: Vec<Miner> = vec![]; // Empty for now
    ResponseType::Ok(miners).get_response()
}

// Create new Miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(miner_request: Json<MinerRequest>) -> HttpResponse {
    let miner = Miner::new(
        miner_request.wallet_address.to_string(),
        miner_request.nickname.to_string(),
    ); // Fails on wallet address for now
    ResponseType::Created(miner).get_response()
}
