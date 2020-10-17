mod snowflake;

use log::info;
use snowflake::Snowflake;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut snowflake = Snowflake::new(1, 1);
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?, &mut snowflake);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, snowflake: &mut Snowflake) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, content) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", snowflake.next_id())
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", -1)
    };

    info!("snowflake id {}", content);

    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
