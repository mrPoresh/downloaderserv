//#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Loads the .env file
    dotenv::dotenv().ok();
    // Initializes the global logger with an env logger
    env_logger::init();

    info!("***** Wotan is starting now ---> GoodLuck! *****");

    // Get App configuration from env
    let cfg = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    // let pool ...

    let adress = (cfg.host.clone(), cfg.port.clone());

    // Build Server
    let server = HttpServer::new(move || {

        App::new()
          //.data(pool)
            // Clone confuguration
            .data(cfg.clone())
            // Error logging
            .wrap(Logger::default())
            // Auth
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cfg.auth_secret_key.clone().as_bytes())
                .name("auth")
                .path("/")
                .domain(&cfg.host.clone())
                // Time from creation that cookie remains valid
                .max_age_time(time::Duration::hours(i64::from(cfg.auth_duration_in_hour)))
              //.secure()
            ))

    })
        .bind(adress)
        .unwrap()
        .run();

    server.await

}