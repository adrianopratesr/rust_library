use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuthor {
    pub name: String,
    pub date_of_birth: NaiveDate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthor {
    pub name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub author_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub creation_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
