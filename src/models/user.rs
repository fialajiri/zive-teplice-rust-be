use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::pg::PgValue;
use diesel::serialize::ToSql;
use diesel::{deserialize::FromSqlRow, expression::AsExpression, prelude::*, sql_types::Text};
use serde::Deserialize;
use serde::Serialize;

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub description: String,
    pub role: UserRole,
    pub user_type: UserType,
    pub request: RequestStatus,
    pub event_id: Option<i32>,
    pub image_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsExpression, Debug, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum UserRole {
    Admin,
    User,
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Admin => String::from("admin"),
            UserRole::User => String::from("user"),
        }
    }
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(UserRole::Admin),
            b"user" => Ok(UserRole::User),
            _ => Ok(UserRole::User),
        }
    }
}

impl ToSql<Text, Pg> for UserRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            UserRole::Admin => out.write_all(b"admin")?,
            UserRole::User => out.write_all(b"user")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserType {
    Admin,
    User,
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum RequestStatus {
    NotSend,
    Pending,
    Approved,
    Rejected,
}
