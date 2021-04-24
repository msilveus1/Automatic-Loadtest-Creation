use mysql::*;
use mysql::prelude::*;

static db_url : &str = "mysql://root:root@172.17.0.1:3306";

pub fn get_db_connection() -> mysql::PooledConn {
    let pool = Pool::new(db_url).unwrap();
    return pool.get_conn().unwrap();
}

