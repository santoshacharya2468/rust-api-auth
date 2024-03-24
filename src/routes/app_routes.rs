use actix_web::{web::{self}, Scope};
use crate::AppDb;
use super::{auth_routes::auth_routes, user_routes::user_routes};
pub  fn app_routes(db:AppDb)->Scope {
   web::scope("/api")
  .service(user_routes(db.clone()))
  .service(auth_routes(db)
)
}