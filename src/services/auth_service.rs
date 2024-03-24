use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use crate::{dtos::{login_dto::LoginDto, register_dto::RegisterDto}, models::user::{ User, UserClaim}, AppDb};
use bcrypt::{verify,hash};
pub struct  AuthService{
   pub  db:AppDb
}
impl AuthService{
    pub fn new(db:AppDb)->Self{
        Self{
            db
        }
    }
}
impl  AuthService {
    pub async fn login(&self, dto:LoginDto)->Result<String,String>{
        let user = sqlx::query_as!(User,"select * from users where email=$1",dto.email).fetch_one(&self.db.pool).await;
        match user {
            Ok(user)=>{
                let valida_password=verify(&dto.password,&user.password).unwrap();
                if !valida_password{
                    return Err("Invalid password".to_string());
                }

                Ok(generate_jwt(UserClaim { sub: user.id.to_string(),
            exp: 1711254569+3600*1000,
            auth_user:user }))},
            Err(e)=>Err(e.to_string())
        }
         
    }
    pub async fn register(&self, dto:RegisterDto)->Result<User, String>{
       let hashed_password=hash(&dto.password, bcrypt::DEFAULT_COST).unwrap();
       let result= sqlx::query_as!(User,"insert into users (email,password,name) values ($1,$2,$3) returning *",dto.email,hashed_password,dto.name).fetch_one(&self.db.pool).await;
        match result {
            Ok(user)=>Ok(user),
            Err(e)=>Err(e.to_string())
        }
   }
}
pub fn generate_jwt(claim:UserClaim) -> String {
    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claim, &EncodingKey::from_secret("123456789".as_ref())).unwrap();
    token
}