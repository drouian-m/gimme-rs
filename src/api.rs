use actix_web::{get, web, HttpResponse, Responder};
use serde_json::Value;

use crate::storage_manager;

pub struct AppStore {
  pub s_manager: storage_manager::StorageManager,
}

#[get("/")]
pub async fn root() -> impl Responder {
  let data = r#"
  {
    "message":"Welcome to the Gimme CDN API. Here are the available API endpoints :",
    "routes":[
      {
        "url":"<host>/create-token",
        "method":"POST",
        "description":"Create access tokens"
      },
      {
        "url":"<host>/packages",
        "method":"POST",
        "description":"Add a package to the CDN"
      },
      {
        "url":"<host>/gimme",
        "method":"GET",
        "description":"Get a package from the CDN"
      }
    ]
  }"#;
  let v: Value = serde_json::from_str(data).unwrap();
  HttpResponse::Ok().json(v)
}

#[get("/gimme/{object_name:.*}")]
pub async fn gimme(
  app_store: web::Data<AppStore>,
  object_name: web::Path<String>,
) -> impl Responder {
  let result = app_store
    .s_manager
    .get_object(object_name.to_string())
    .await;
  return match result {
    Ok(data) => HttpResponse::Ok().body(data),
    Err(err) => {
      eprint!("{}", err);
      HttpResponse::InternalServerError().body("Error while getting package")
    }
  };
}
