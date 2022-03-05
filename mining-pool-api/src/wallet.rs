use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::miner::Miner;
use crate::config::Response;


pub type Wallets = Response<Wallet>;


#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub workers_online: i32,
    pub shares_mined: Vec<Miner>,
    pub date_joined: DateTime<Utc>,
}

impl Wallet {
    pub fn new(club_name: String) -> Self {
        Self {
            address: Uuid::new_v4().to_string(),
            club_name,
            workers_online: 0,
            shares_mined: vec![],
            date_joined: Utc::now(),
        }
    }
}

// ------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletRequest {
    pub club_name: Option<String>,
}

impl WalletRequest {
    pub fn to_wallet(&self) -> Option<Wallet> {
        match &self.club_name {
            Some(club_name) => Some(Wallet::new(club_name.to_string())),
            None => None,
        }
    }
}

// List all Wallets
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {

    let wallets = Wallets { results: vec![] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(wallets)
}

// Get a wallet
#[get("/wallets/{id}")]
pub async fn get_wallet(path: Path<(String,)>) -> HttpResponse {
    let found_wallet: Option<Wallet> = None;
    match found_wallet {
        Some(wallet) => HttpResponse::Ok()
            .content_type("application/json")
            .json(wallet),
        None => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
    }
}

// Create New Wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_req: Json<WalletRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json(wallet_req.to_wallet())
}