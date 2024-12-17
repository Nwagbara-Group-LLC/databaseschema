use std::sync::Arc;

use crate::{get_connection, models::{NewSecurity, Security}, CustomAsyncPgConnectionManager};
use deadpool::managed::Pool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn create_security(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, new_security: NewSecurity) -> Security {
    println!("Creating security: {:?}", new_security);
    use crate::schema::securities::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");

    diesel::insert_into(securities)
        .values(&new_security)
        .returning(Security::as_returning())
        .get_result(&mut connection)
        .await
        .expect("Error saving new security")
}

pub async fn get_securities(pool: Arc<Pool<CustomAsyncPgConnectionManager>>) -> Vec<Security> {
    println!("Getting securities");
    use crate::schema::securities::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    securities
        .load::<Security>(&mut connection)
        .await
        .expect("Error loading securities")
}

pub async fn get_securities_by_id(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, get_security: Security) -> Security {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    securities
        .find(get_security.security_id)
        .first::<Security>(&mut connection)
        .await
        .expect("Error loading security")
}

pub async fn get_security_by_symbol(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> Security {
    println!("Getting security");
    use crate::schema::securities::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .expect("Error loading security")
}

pub async fn security_exists(pool: Arc<Pool<CustomAsyncPgConnectionManager>>, sym: &String) -> bool {
    println!("Checking if security exists");
    use crate::schema::securities::dsl::*;

    let mut connection = get_connection(pool).await.expect("Error connecting to database");
    securities
        .filter(symbol.eq(sym))
        .first::<Security>(&mut connection)
        .await
        .is_ok()
}