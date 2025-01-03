use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::event::*;
use crate::schema::events;

pub struct EventRepository;

impl EventRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Event> {
        events::table.find(id).get_result(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_event: NewEvent) -> QueryResult<Event> {
        diesel::insert_into(events::table)
            .values(new_event)
            .get_result(c)
            .await
    }
}
