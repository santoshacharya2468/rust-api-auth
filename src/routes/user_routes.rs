use actix_web::web;
use actix_web::{dev::HttpServiceFactory, Scope};
use controller::user_controller;
use crate::services::user_service::UserService;
use crate::{controller, middlewares, AppDb};
pub fn user_routes(app_db:AppDb)->impl HttpServiceFactory{
   let user_service =UserService::new(app_db.clone());
   let router=Scope::new("/users");
   router
   .service(user_controller::get_all)
   .service(user_controller::get_by_id)
   .wrap(middlewares::auth_middleware::Authorization)
   .app_data(web::Data::new(user_service))
}
