use actix_web::{dev::HttpServiceFactory, web, Scope};
use controller::auth_controller;

use crate::{controller, services::auth_service::AuthService, AppDb};

pub fn auth_routes(app_db:AppDb)->impl HttpServiceFactory{
   let auth_service = AuthService::new(app_db);
   let router=Scope::new("/auth");
   router
   .service(auth_controller::login)
   .service(auth_controller::register)
   .app_data(web::Data::new(auth_service))
}