use {
    actix_web::HttpResponse,
    actix_web::web::Json,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    crate::util::*,
};


#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub workers_online: i32,  // From our JOIN
    pub shares_mined: i32, // From our JOIN
    pub date_joined: DateTime<Utc>,
}

impl Wallet {
    pub fn new(club_name: String) -> Self {
        Self {
            address: Uuid::new_v4().to_string(),
            club_name,
            workers_online: 0,
            shares_mined: 0,
            date_joined: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletRequest {
    club_name: String,
}



// List all Wallets
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    let wallets: Vec<Wallet> = vec![]; // Empty for now
    ResponseType::Ok(wallets).get_response()
}

// Get a wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    let wallet: Option<Wallet> = None; // Empty for now
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet/Club not found".to_string())
        ).get_response(),
    }
}

// Create New Wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<WalletRequest>) -> HttpResponse {
    let wallet = Wallet::new(
        wallet_request.club_name.to_string()
    );
    ResponseType::Created(wallet).get_response()
}