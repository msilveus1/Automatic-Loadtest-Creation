use mysql::*;
use mysql::prelude::*;

static db_url : &str = "mysql://msilveus:@localhost:3306";

pub fn get_db_connection() -> Result<PooledConn> {
    let pool = Pool::new(db_url)?;
    return pool.get_conn();
}

