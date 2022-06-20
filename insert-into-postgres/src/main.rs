use std::env;
use dotenv::dotenv;
use postgres::{Client, NoTls};

fn insert_in_postgres() {
    let db_conn = env::var("DB_CONNECTION").expect("Database connection is missing or incorrect");
    let mut client = Client::connect(
        &db_conn,
        NoTls)
        .unwrap();

    let body = "<p>Example DB postet post</p> <p> Some more text</p>";
    let published = true;
    client.execute(
        "INSERT INTO posts (body, published) VALUES ($1, $2)",
        &[&body, &published],
    ).unwrap();

    for row in client.query("SELECT body FROM posts", &[]).unwrap() {
        // let id: i32 = row.get(0);
        let name: &str = row.get(0);
        // let data: Option<&[u8]> = row.get(2);

        println!("found post: {}", name);
    }

}

fn main() {
    dotenv().ok();
    insert_in_postgres();
}

