use mysql as my;

pub fn db_pool() -> my::Pool {
    my::Pool::new(
        format!(
            "mysql://{}:{}@{}/{}",
            "root", "root123", "localhost", "rust_test"
        ),
    )
        .expect("Failed to create MySQL pool")
}
