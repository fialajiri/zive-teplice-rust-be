use rocket_db_pools::Database;

pub mod errors;
pub mod models;
pub mod repositories;
pub mod rocket_routes;
mod schema;
pub mod utils;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rocket_routes::events::get_event,
                rocket_routes::events::create_event,
                rocket_routes::events::delete_event,
                rocket_routes::events::update_event,
                rocket_routes::events::get_event_with_program,
                rocket_routes::programs::get_programs_for_event,
                rocket_routes::programs::create_program,
                rocket_routes::programs::delete_program,
                rocket_routes::programs::update_program,
                rocket_routes::news::get_news,
                rocket_routes::news::get_all_news,
                rocket_routes::news::create_news,
                rocket_routes::news::delete_news,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .attach(rocket_routes::Cors)       
        .launch()
        .await;
}
