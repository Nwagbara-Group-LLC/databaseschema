use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::VarChar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::exchanges;

#[derive(Debug, Insertable)]
#[diesel(table_name = exchanges)]
pub struct NewExchange {
    pub exchange: String,
}

impl NewExchange {
    pub fn new(exchange: &String) -> NewExchange {
        NewExchange { exchange: exchange.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = exchanges)]
#[diesel(check_for_backend(Pg))]
pub struct Exchange {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub exchange_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub exchange: String,
}