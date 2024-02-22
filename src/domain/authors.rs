use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuthor {
    name: String,
    date_of_birth: NaiveDate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthor {
    name: Option<String>,
    date_of_birth: Option<NaiveDate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    author_id: Uuid,
    name: String,
    date_of_birth: NaiveDate,
    creation_time: DateTime<Utc>,
    deletion_time: Option<DateTime<Utc>>,
}
