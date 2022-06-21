use std::{
    error::Error,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread, time,
};

use tcp_webserver::pool::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting webserver...");

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4)?;

    println!("Listening on port 7878...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("\nRequest: {}", String::from_utf8_lossy(&buffer[..]));

    let response = if buffer.starts_with(b"GET") {
        if buffer.starts_with(b"GET /sleep") {
            thread::sleep(time::Duration::from_secs(5));
        }

        fs::read_to_string("responses/get.txt").unwrap()
    } else {
        fs::read_to_string("responses/non_get.txt").unwrap()
    };

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
