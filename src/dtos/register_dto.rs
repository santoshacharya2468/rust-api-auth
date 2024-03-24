use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct  RegisterDto{
    pub name:String,
    pub email:String,
    pub password:String
}