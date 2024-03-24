use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct User{
    pub id:i64,
    pub email:String,
    pub password:String,
    pub name:Option<String>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
   pub sub: String,
   pub auth_user:User,
   pub exp: usize,
}
