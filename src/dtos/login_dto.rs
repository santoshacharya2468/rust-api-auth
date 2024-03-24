use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct  LoginDto{
    #[validate(email)]
    #[serde(default)]
    pub email:String,
    #[validate(length(min = 8,max=32))]
    #[serde(default)]
    pub password:String
}