use poem::{Route, get, post};

use crate::controller::auth::{login, profile_data, signup};


pub fn auth_route()-> Route{
    let route = Route::new()
        .at("/signup", post(signup))
        .at("/login", post(login))
        .at("/me",get(profile_data));
    route
} 