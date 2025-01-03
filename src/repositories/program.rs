use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::program::*;
use crate::schema::programs;

pub struct ProgramRepository;

impl ProgramRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Program> {
        programs::table.find(id).get_result(c).await
    }

    pub async fn create_program_for_event(
        c: &mut AsyncPgConnection,
        new_program: NewProgram,
    ) -> QueryResult<Program> {
        diesel::insert_into(programs::table)
            .values(new_program)
            .get_result(c)
            .await
    }

    pub async fn find_programs_for_event(
        c: &mut AsyncPgConnection,
        event_id: i32,
    ) -> QueryResult<Vec<Program>> {
        programs::table
            .filter(programs::event_id.eq(event_id))
            .load(c)
            .await
    }
}
