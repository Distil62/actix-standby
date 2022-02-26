use crate::schema::todos;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "todos"]
pub struct Todo {
    pub id: String,
    pub label: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoPayload {
    pub label: Option<String>,
    #[serde(default = "bool::default")]
    pub done: bool,
}
