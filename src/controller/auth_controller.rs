use actix_web::{ post, web::{self, Json}, HttpResponse, Responder};
use crate::{dtos::{login_dto::LoginDto, register_dto::RegisterDto}, services::auth_service::AuthService};
use validator::Validate;
#[post("/login")]
pub  async fn  login(dto:Json<LoginDto>,auth_service:web::Data<AuthService>)->impl Responder{
    let valid_dto=dto.0.validate();
    if valid_dto.is_err(){
       return  HttpResponse::Ok().json(valid_dto.unwrap_err());
    }
    let result=auth_service.login(dto.0).await;
     match  result {
        Ok(token)=>HttpResponse::Ok().json(token),
        Err(e)=>HttpResponse::Ok().json(e)
    }
}
#[post("/register")]
pub  async fn  register(dto:Json<RegisterDto>,auth_service:web::Data<AuthService>)->impl Responder{
    let result: Result<_, _>=auth_service.register(dto.0).await;
    match  result {
       Ok(token)=>HttpResponse::Ok().json(token),
       Err(e)=>HttpResponse::Ok().json(e)
   }
}


