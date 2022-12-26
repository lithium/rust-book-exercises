use std::{
    env,
    fmt::Display,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {

    let port = env::var("PORT").unwrap_or("7878".to_string());
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);

    println!("Binding to {}", &address);
    let listener = match TcpListener::bind(&address) {
        Err(what) => return error(&format!("Unable to bind '{}': {}", &address, what)),
        Ok(l) => l
    };

    for stream in listener.incoming() { 
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let content:Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect(); 

    let request_line = &content[0];
    let headers = &content[1..];

    dbg!(request_line, headers);

    let response = "HTTP/1.1 200 OK\r\n\r\nHello\r\n";

    match stream.write_all(response.as_bytes()) {
        Err(e) => eprintln!("Unable to write response: {}", e),
        Ok(r) => r
    }
}


fn error(why: &dyn Display) {
    eprintln!("{}", why);
    std::process::exit(1);
}