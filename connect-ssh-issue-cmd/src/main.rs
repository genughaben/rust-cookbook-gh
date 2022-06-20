use dotenv::dotenv;
use ssh2::Session;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn connect_ssh() {
    let home_server_ip = env::var("SERVER_IP").expect("Server Ip not set in .env file");
    let username = env::var("SERVER_USERNAME").expect("Server username not set in .env file");
    let password = env::var("SERVER_PASSWORD").expect("Server password not set in .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB password not set in .env file");

    let tcp = TcpStream::connect(&home_server_ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&username, &password).unwrap();
    let mut channel = sess.channel_session().unwrap();
    let release_command = "lsb_release -a";
    // let command = format!("PGPASSWORD={db_password} docker exec -it rocketblogdb psql -U test_user -d blog -f insert.sql", db_password = &db_password);
    // let command2 = format!("PGPASSWORD={db_password} docker exec -i rocketblogdb psql -U test_user -d blog < insert.sql", db_password = &db_password);
    channel.exec(release_command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}

fn main() {
    dotenv().ok();
    connect_ssh();
}
