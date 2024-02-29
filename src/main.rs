use actix_web::{web, App, HttpServer, Responder};
use dummy_data::User;
use dummy_db::get_random_user;
mod dummy_data;
mod dummy_db;
// Example Rust REST API server logic
// using CodeWhisperer suggestions to enhance productivity

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(User {
        id: 0,
        name: "App State".to_string(),
        email: "appstate@ether.com".to_string(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::get().to(get_users))
        // Additional routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Updated get_users function to accept app_state and return User
async fn get_users() -> impl Responder {
    web::Json(get_random_user()) // Return the User in JSON format
}
