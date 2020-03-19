use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;


type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;




lazy_static! {
    static ref POOL: Pool = {
        let db_url = "postgres://postgres:123456@localhost/goods";
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {

    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
}

pub fn connection() -> Result<DbConnection, &'static str> {
    POOL.get()
        .map_err(|_| "can get connection")
}
