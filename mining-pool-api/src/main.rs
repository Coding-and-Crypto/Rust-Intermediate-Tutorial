#[macro_use]
extern crate actix_web;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};
use serde::{Deserialize, Serialize};

mod miner;
mod wallet;


#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(wallet::list_wallets)
            .service(wallet::get_wallet)
            .service(wallet::create_wallet)
            .service(miner::list_miners)
            .service(miner::get_miner)
            .service(miner::list_wallet_miners)
            .service(miner::create_miner)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}