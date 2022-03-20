#[macro_use]
extern crate simple_error;

use actix_web::{web, App, HttpServer};
use std::env;
mod api;
mod storage_manager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let bucket_name =
    env::var("GIMME_BUCKET").expect("Please set $GIMME_BUCKET environnement variable");
  let bucket_url =
    env::var("GIMME_BUCKET_URL").expect("Please set $GIMME_BUCKET_URL environnement variable");
  let bucket_region = env::var("GIMME_BUCKET_REGION")
    .expect("Please set $GIMME_BUCKET_REGION environnement variable");
  let access_key =
    env::var("GIMME_ACCESS_KEY").expect("Please set $GIMME_ACCESS_KEY environnement variable");
  let secret_key =
    env::var("GIMME_SECRET_KEY").expect("Please set $GIMME_SECRET_KEY environnement variable");

  let data = web::Data::new(api::AppStore {
    s_manager: storage_manager::StorageManager::new(
      &bucket_name[..],
      &access_key[..],
      &secret_key[..],
      bucket_region,
      bucket_url,
    ),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .service(api::root)
      .service(api::gimme)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
