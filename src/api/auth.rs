/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-10 15:45:52
 * @LastEditTime: 2024-05-20 19:10:22
 * @FilePath: \RustPanel\src\api\auth.rs
 */
use crate::{
    common::fun::sha1_salt,
    errors::common::CommonError,
    models::{structure::rp_users::SlimUser, user::find_user_by_id},
    service::{
        db::DBPool,
        global::{CONF, USER_PASSWORD_KEY},
    },
};
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type AuthUser = SlimUser;

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<AuthUser, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        match req.headers().get("Authorization"){
            Some(token) => {
                if let Ok(claims) = decode_jwt(token.to_str().unwrap()) {
                    // decoding tokens and obtaining user information
                    if let Ok(user) = serde_json::from_str::<SlimUser>(&claims.sub) {
                        if let Ok(db_user) = find_user_by_id(
                            user.id,
                            &req.app_data::<web::Data<DBPool>>()
                                .expect("DBPool not found")
                                .clone(),
                        )
                        .map_err(|_| CommonError::InternalServerError)
                        {
                            if db_user.unwrap().password != user.password {
                                return ready(Err(CommonError::Unauthorized(
                                    String::from("password error"),
                                )
                                .into()));
                            }
                        }
                        return ready(Ok(user));
                    }
                }
            }
            None => {
                return ready(Err(
                    CommonError::Unauthorized(String::from("token is None")).into()
                ));
            }
        }
        //verify decode_jwt

        ready(Err(
            CommonError::Unauthorized(String::from("token error")).into()
        ))
    }
}

#[derive(Debug)]
pub enum AuthError {
    HashError,
    VerificationError,
}

// impl From<argon2::Error> for AuthError {
//     fn from(_: argon2::Error) -> Self {
//         AuthError::HashError
//     }
// }

// pub fn generate_password(password: String) -> String {
//     sha1_salt(password, String::from(USER_PASSWORD_KEY))
// }

// pub fn hash_password(password: &[u8], salt: &[u8]) -> String {
//     argon2::hash_encoded(password, salt, &Config::default()).unwrap()
// }
// pub fn verify_password(hash: &str, password: &[u8]) -> Result<bool, AuthError> {
//     Ok(argon2::verify_encoded(hash, password)?)
// }

pub fn verify_password_sha1(hash: &str,salt: &str, password: &str) -> Result<bool, AuthError> {
    Ok(hash == sha1_salt(String::from(password), String::from(salt)))
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(subject: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // 生成 JWT
    Ok(encode(
        &Header::new(Algorithm::HS256),
        &Claims {
            sub: String::from(subject),
            exp: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + (CONF.app.session_ttl * 60 * 60) as u64) as usize,
        },
        &EncodingKey::from_secret(USER_PASSWORD_KEY.as_ref()),
    )?)
}

fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(USER_PASSWORD_KEY.as_ref());

    // 手动创建 Validation 实例，并设置所需的字段
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.algorithms = vec![Algorithm::HS256];

    let token_data = decode::<Claims>(token, &key, &validation)?;


    Ok(token_data.claims)
}
