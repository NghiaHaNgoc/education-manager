use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn database_connection() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("FER201m").use_db("FER201m").await.unwrap();
    db
}
