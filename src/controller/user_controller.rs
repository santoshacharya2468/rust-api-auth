use actix_web::{get, web, HttpResponse, Responder, web::Path};
use crate::services::user_service::UserService;
#[get("/")]
pub  async fn  get_all(user_service:web::Data<UserService>)->impl Responder{
     let user=user_service.get_all().await.unwrap();
     HttpResponse::Ok().json(user)
}
#[get("/{id}")]
pub  async fn  get_by_id(id:Path<i64>,user_service:web::Data<UserService>)->impl Responder{
    let user=user_service.get_by_id(id.into_inner()).await;
    if user.is_err(){
        return  HttpResponse::Ok().json(user.unwrap_err());
    }
    HttpResponse::Ok().json(user.unwrap())
}