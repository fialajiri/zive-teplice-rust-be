

use rocket::fairing::Fairing;

use rocket::{Request, Response};

pub mod events;
pub mod programs;
pub mod news;
pub mod gallery;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);



pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Add CORS headers to responses",
            kind: rocket::fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>)  {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}