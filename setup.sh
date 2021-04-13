docker exec mariadbtest /usr/bin/mysql mysql < sql/setup.sql
cargo run --manifest-path rust/RestAPI/Cargo.toml