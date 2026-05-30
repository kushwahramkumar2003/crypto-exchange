use std::{ str::FromStr, sync::{Arc, Mutex}};

use db::{Db::Db, models::user::UserAuthError};
use poem::{Error, IntoResponse, Response, handler, http::{StatusCode, response}, web::{Data, Json}};

use crate::{dto::auth_dto::{request_input::{CreateUserInput, SigninRequest}, request_output::{CreateUserOutput, ProfileOutput, SigninUserOutput}}, middlewares::auth_middleware::UserId, utils::jwt::generate_jwt};

#[handler]
pub fn signup( 
    Json(data):Json<CreateUserInput>,
    Data(db):Data<&Arc<Mutex<Db>>>
) -> Result<Json<CreateUserOutput>, Error>  {
   let mut locked_db = db.lock().unwrap();

   let user_id = match locked_db.signup(data.email, data.password, String::from_str("ADMIN")?) {
       Ok(id) => id,
        Err(UserAuthError::Conflict) => {
            return Err(Error::from_status(StatusCode::CONFLICT));
        }
        Err(UserAuthError::Db(_)) => {
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
   }; 

   let response = CreateUserOutput {
        id: user_id,
    };

    Ok(Json(response))
}

#[handler]
pub fn login(Json(
    data):Json<SigninRequest>,
    Data(db):Data<&Arc<Mutex<Db>>>
) -> Result<Response, Error>  {
    let mut locked_db = db.lock().unwrap();
    let res = match locked_db.signin(data.email, data.password) {
        Ok(user_id)=>user_id,
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };
    let user_id = match res {
        Some(id)=>id,
        None=>{return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))}
    };
    let jwt = match generate_jwt(user_id) {
        Ok(token)=>token,
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };
    let response = SigninUserOutput{
        jwt:jwt.clone()
    };
    let mut http_response = Json(response).into_response();
    http_response.headers_mut().append(
        "Set-Cookie",
        format!("token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Lax",jwt)
            .parse()
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?,
    );

    Ok(http_response)
}

#[handler]
pub fn profile_data(
    user_id:UserId,
    Data(db):Data<&Arc<Mutex<Db>>>
)-> Result<Json<ProfileOutput>, Error> {
    let mut locked_db = db.lock().unwrap();
    
    let response = locked_db.user_profile(user_id.user_id).unwrap().unwrap();
    let profile = ProfileOutput{
        id:response.id,
        email:response.email,
        role:response.role
    };
    Ok(Json(profile))
}