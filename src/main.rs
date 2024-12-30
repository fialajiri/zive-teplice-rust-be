use rocket_db_pools::Database;

pub mod rocket_routes;
pub mod models;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()       
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await;
}
