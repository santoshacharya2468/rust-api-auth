use crate::{models::user::User, AppDb};
pub struct  UserService{
    pub db:AppDb
}
impl UserService{
    pub fn new(db:AppDb)->Self{
        Self{
            db
        }
    }
}
impl  UserService {
    pub async fn get_all(&self)->Result<Vec<User>, String>{
        let result = sqlx::query_as!(User,"select * from users").fetch_all(&self.db.pool).await;
        match result {
            Ok(user)=>Ok(user),
            Err(e)=>Err(e.to_string())
        }
    }
    pub async fn get_by_id(&self, uid:i64)->Result<User, String>{
        let result = sqlx::query_as!(User,"select * from users where id=$1",uid).fetch_one(&self.db.pool).await;
        match result {
            Ok(user)=>Ok(user),
            Err(e)=>Err(e.to_string())
        }
    }
}