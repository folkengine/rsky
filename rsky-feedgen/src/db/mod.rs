use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use mockall::automock;

#[automock]
pub trait Database {
    fn establish_connection(&self) -> Result<PgConnection, Box<dyn std::error::Error>>;
}

pub struct RealDatabase;

impl Database for RealDatabase {
    fn establish_connection(&self) -> Result<PgConnection, Box<dyn std::error::Error>> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").unwrap_or("".into());
        let result = PgConnection::establish(&database_url).map_err(|_| {
            eprintln!("Error connecting to {database_url:?}");
            "Internal error"
        })?;
        Ok(result)
    }
}

#[cfg(test)]
mod feedgen_db_tests {
    use diesel::connection::SimpleConnection;
    // use diesel::test_helpers::pg_connection_no_transaction;
    use super::*;

    use mockall::predicate::*;

    #[test]
    fn test_mocked_connection() {
        let mut mock_db = MockDatabase::new();
        mock_db.expect_establish_connection()
            .returning(|| Ok(PgConnection::establish("postgres://user:password@localhost/test_db").unwrap()));

        let mut conn = mock_db.establish_connection().unwrap();
        // assert_eq!(conn.batch_execute("SELECT 1").unwrap(), 0);
        conn.batch_execute("SELECT 1").unwrap()
    }

    #[cfg(test)]
    #[ctor::ctor]
    fn init() {
        env::set_var("DATABASE_URL", "postgres://user:password@localhost/test_db");
    }

    /// Code taken from
    /// [diesel::test_helpers::pg_connection_no_transaction](https://github.com/diesel-rs/diesel/blob/e659b014ef06182a95811f554ac6b7ec7909d3b0/diesel/src/test_helpers.rs#L55)
    pub fn pg_connection() -> PgConnection {
        let mut conn = pg_connection_no_transaction();
        conn.begin_test_transaction().unwrap();
        conn
    }

    pub fn pg_connection_no_transaction() -> PgConnection {
        PgConnection::establish(&pg_database_url()).unwrap()
    }

    pub fn pg_database_url() -> String {
        dotenvy::var("PG_DATABASE_URL")
            .or_else(|_| dotenvy::var("DATABASE_URL"))
            .expect("DATABASE_URL must be set in order to run tests")
    }


    #[test]
    #[ignore]
    fn test_establish_connection() {

        let _pg_connection: PgConnection = pg_connection();

        // let conn = establish_connection().unwrap();
        // assert_eq!(conn.batch_execute("SELECT 1").unwrap(), 0);
    }
}