use poem::{Route, post};

use crate::controller::auth::{login, signup};


pub fn auth_route()-> Route{
    let route = Route::new()
        .at("/signup", post(signup))
        .at("/login", post(login));
    route
} 