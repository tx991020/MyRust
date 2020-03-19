use lazy_static::lazy_static;
use r2d2;
use redis::{Client, ConnectionLike};


type Pool = r2d2::Pool<Client>;
pub type CacheConnection = r2d2::PooledConnection<Client>;



lazy_static! {
    static ref POOL: Pool = {
        let redis_url = "redis://127.0.0.1/";
        let client = redis::Client::open(redis_url).expect("Failed to create redis client");
        Pool::new(client).expect("Failed to create redis pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get redis connection");
    assert_eq!(true, conn.check_connection(), "Redis connection check failed");
}

pub fn connection() -> Result<CacheConnection, &'static str> {
    POOL.get()
        .map_err(|_| "can get connection")
}



