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

#[derive(AsExpression, Debug, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum UserType {
    Seller,
    Artist,
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match self {
            UserType::Seller => String::from("seller"),
            UserType::Artist => String::from("artist"),
        }
    }
}

impl FromStr for UserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seller" => Ok(UserType::Seller),
            "artist" => Ok(UserType::Artist),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for UserType {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"seller" => Ok(UserType::Seller),
            b"artist" => Ok(UserType::Artist),
            _ => Ok(UserType::Seller),
        }
    }
}

impl ToSql<Text, Pg> for UserType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            UserType::Seller => out.write_all(b"seller")?,
            UserType::Artist => out.write_all(b"artist")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(AsExpression, Debug, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum RequestStatus {
    NotSend,
    Pending,
    Confirmed,
    Rejected,
}

impl ToString for RequestStatus {
    fn to_string(&self) -> String {
        match self {
            RequestStatus::NotSend => String::from("notsend"),
            RequestStatus::Pending => String::from("pending"),
            RequestStatus::Confirmed => String::from("confirmed"),
            RequestStatus::Rejected => String::from("rejected"),
        }
    }
}

impl FromStr for RequestStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "notsend" => Ok(RequestStatus::NotSend),
            "pending" => Ok(RequestStatus::Pending),
            "confirmed" => Ok(RequestStatus::Confirmed),
            "rejected" => Ok(RequestStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for RequestStatus {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"notsend" => Ok(RequestStatus::NotSend),
            b"pending" => Ok(RequestStatus::Pending),
            b"confirmed" => Ok(RequestStatus::Confirmed),
            b"rejected" => Ok(RequestStatus::Rejected),
            _ => Ok(RequestStatus::NotSend),
        }
    }
}

impl ToSql<Text, Pg> for RequestStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            RequestStatus::NotSend => out.write_all(b"notsend")?,
            RequestStatus::Pending => out.write_all(b"pending")?,
            RequestStatus::Confirmed => out.write_all(b"confirmed")?,
            RequestStatus::Rejected => out.write_all(b"rejected")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}
