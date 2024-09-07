use actix_web::{ post, get, web, HttpResponse, HttpRequest, Responder };
use mongodb::bson::doc;
use mongodb::sync::Client;
use crate::models::{ User, UserAuth };
use crate::auth::{ create_jwt, validate_jwt };
use serde_json::json;

#[post("/register")]
async fn register(mongo: web::Data<Client>, user: web::Json<UserAuth>) -> impl Responder {
    let users = mongo.database("auth").collection::<User>("users");

    let new_user = User::new(&user.email, &user.password);
    users.insert_one(new_user).await.unwrap();

    HttpResponse::Ok().body("User registered successfully")
}

#[post("/login")]
async fn login(mongo: web::Data<Client>, login_data: web::Json<UserAuth>) -> impl Responder {
    let users = mongo.database("auth").collection::<User>("users");
    let user = users.find_one(doc! { "email": &login_data.email }).await.unwrap();
    println!("{:?}", user);
    println!("{:?}", login_data);
    if let Some(user) = user {
        if user.verify_password(&login_data.password) {
            let token = create_jwt(&login_data.email);
            return HttpResponse::Ok().json(json!({ "token": token, "email": login_data.email, "hash": user.password_hash, "hash2": login_data.email, "pass": "5d2222" }));
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}

#[get("/protected")]
async fn protected(req: HttpRequest) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        let token = auth_header.to_str().unwrap().split("Bearer ").nth(1).unwrap();

        if validate_jwt(token) {
            return HttpResponse::Ok().body("Welcome to the protected area!");
        }
    }

    HttpResponse::Unauthorized().body("Unauthorized")
}
