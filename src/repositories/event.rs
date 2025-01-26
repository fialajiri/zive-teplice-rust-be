use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::event::*;
use crate::schema::events;
use crate::schema::programs;

pub struct EventRepository;

impl EventRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Event> {
        events::table.find(id).get_result(c).await
    }

    pub async fn find_event_with_program(
        c: &mut AsyncPgConnection,
        id: i32,
    ) -> QueryResult<EventWithProgram> {
        events::table
            .left_join(programs::table.on(programs::event_id.eq(events::id)))
            .filter(events::id.eq(id))
            .select((
                events::id,
                events::title,
                events::year,
                events::is_current,
                programs::title.nullable(),
                programs::text.nullable(),
                programs::image_id.nullable(),
            ))
            .first::<EventWithProgram>(c)
            .await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_event: NewEvent) -> QueryResult<Event> {
        diesel::insert_into(events::table)
            .values(new_event)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, event: Event) -> QueryResult<Event> {
        diesel::update(events::table.find(id))
            .set(event)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(events::table.find(id)).execute(c).await
    }
}
