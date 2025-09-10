#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => DatabaseError::NotFound("Record not found".to_string()),
            diesel::result::Error::DatabaseError(kind, info) => {
                DatabaseError::DatabaseError(format!("{:?}: {:?}", kind, info))
            }
            _ => DatabaseError::DatabaseError(error.to_string()),
        }
    }
}

impl From<diesel_async::pooled_connection::deadpool::PoolError> for DatabaseError {
    fn from(error: diesel_async::pooled_connection::deadpool::PoolError) -> Self {
        DatabaseError::ConnectionError(error.to_string())
    }
}
