#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use {
    actix_web::{middleware, App, HttpServer},
    actix_web::web::Data,
    diesel::r2d2::ConnectionManager,
    diesel::PgConnection,
    r2d2::{Pool, PooledConnection},
    std::{env, io},
};


mod miner;
mod miner_controller;
mod schema;
mod util;
mod wallet;
mod wallet_controller;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_to_pool(pool: Data<DBPool>) -> DBPooledConnection {
    pool.get().expect("Failed to reach DB connection pool.")
}



#[actix_rt::main]
async fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not detected.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to initialize DB connection pool.");

    HttpServer::new(move|| {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(wallet_controller::list_wallets)
            .service(wallet_controller::get_wallet)
            .service(wallet_controller::create_wallet)
            .service(miner_controller::list_miners)
            .service(miner_controller::get_miner)
            .service(miner_controller::create_miner)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}