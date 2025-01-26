use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::news::{NewNews, News, UpdateNews};
use crate::schema::news;

pub struct NewsRepository;

impl NewsRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<News> {
        news::table.find(id).get_result(c).await
    }

    pub async fn all(c: &mut AsyncPgConnection) -> QueryResult<Vec<News>> {
        news::table.load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_news: NewNews) -> QueryResult<News> {
        diesel::insert_into(news::table)
            .values(new_news)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, news: UpdateNews) -> QueryResult<News> {
        diesel::update(news::table.find(id))
            .set(&news)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(news::table.find(id)).execute(c).await
    }
}
