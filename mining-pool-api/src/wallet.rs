use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::miner::Miner;
use crate::Response;


pub type Wallets = Response<Wallet>;


#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub id: String,
    pub joined: DateTime<Utc>,
    pub club: String,
    pub shares: Vec<Miner>,
}

impl Wallet {
    pub fn new(club: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            joined: Utc::now(),
            club,
            shares: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletRequest {
    pub club: Option<String>,
}

impl WalletRequest {
    pub fn to_wallet(&self) -> Option<Wallet> {
        match &self.club {
            Some(club) => Some(Wallet::new(club.to_string())),
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