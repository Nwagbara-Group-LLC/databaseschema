use chrono::{DateTime, Utc};
use diesel::pg::{sql_types::Timestamptz, Pg};
use diesel::prelude::*;
use diesel::sql_types::VarChar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::securities;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = securities)]
pub struct NewSecurity {
    pub symbol: String,
}

impl NewSecurity {
    pub fn new(symbol: &String) -> NewSecurity {
        NewSecurity { symbol: symbol.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, QueryableByName, AsChangeset)]
#[diesel(table_name = securities)]
#[diesel(check_for_backend(Pg))]
pub struct Security {
    #[diesel(sql_type = Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub security_id: Uuid,
    #[diesel(sql_type = VarChar)]
    pub symbol: String,
}