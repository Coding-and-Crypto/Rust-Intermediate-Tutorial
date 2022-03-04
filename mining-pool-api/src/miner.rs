use actix_web::web::Path;
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Response;


pub type Miners = Response<Miner>;


#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Miner {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

// List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {

    let miners = Miners { results: vec![] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(miners)
}

// Get a miner
#[get("/miners/{id}")]
pub async fn get_miner(path: Path<(String,)>) -> HttpResponse {
    let found_miner: Option<Miner> = None;
    match found_miner {
        Some(miner) => HttpResponse::Ok()
            .content_type("application/json")
            .json(miner),
        None => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
    }
}

// List all Miners for Wallet
#[get("/wallets/{id}/miners")]
pub async fn list_wallet_miners(path: Path<(String,)>) -> HttpResponse {
    // TODO find miners by wallet ID and return them
    let miners = Miners { results: vec![] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(miners)
}

// Create new Miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(path: Path<(String,)>) -> HttpResponse {
    // TODO add one miner to a wallet
    let miner = Miner::new();

    HttpResponse::Created()
        .content_type("application/json")
        .json(miner)
}
