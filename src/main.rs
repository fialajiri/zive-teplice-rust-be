// use aws_sdk_s3::config::{Credentials, Region};
// use aws_sdk_s3::Client as S3Client;
use rocket_db_pools::Database;

pub mod rocket_routes;
pub mod models;
pub mod repositories;
pub mod errors;
pub mod utils;
mod schema;

#[rocket::main]
async fn main() {

    // let shared_config = aws_sdk_s3::Config::builder()
    //     .region(Region::new("us-east-1"))
    //     .credentials_provider(Credentials::new(
    //         "AWS_ACCESS_KEY",
    //         "AWS_SECRET_KEY",
    //         None,
    //         None,
    //         "example",
    //     ))
    //     .build();
    // let s3_client = S3Client::from_conf(shared_config);

    let _ = rocket::build()    
        .mount("/",
            rocket::routes![               
                rocket_routes::events::get_event,            
                rocket_routes::events::create_event,
                rocket_routes::events::delete_event,
                rocket_routes::events::update_event,
                rocket_routes::events::get_event_with_program,
                rocket_routes::programs::get_programs_for_event,
                rocket_routes::programs::create_program,
                rocket_routes::programs::delete_program,
            ]
        )
        .attach(rocket_routes::DbConn::init())
        .attach(rocket_routes::Cors)
        // .manage(s3_client)
        .launch()
        .await;
}
