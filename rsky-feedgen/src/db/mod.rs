use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("".into());
    let result = PgConnection::establish(&database_url).map_err(|_| {
        eprintln!("Error connecting to {database_url:?}");
        "Internal error"
    })?;

    Ok(result)
}

#[cfg(test)]
mod feedgen_db_tests {
    // use diesel::test_helpers::pg_connection_no_transaction;
    use super::*;

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
    fn test_establish_connection() {

        let pg_connection: PgConnection = pg_connection();

        // let conn = establish_connection().unwrap();
        // assert_eq!(conn.batch_execute("SELECT 1").unwrap(), 0);
    }
}