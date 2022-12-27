use std::{
    env,
    fmt::Display,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::{PathBuf},
};

fn main() {

    let port = env::var("PORT").unwrap_or("7878".to_string());
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);

    let wwwroot: &String = &match env::var("WWWROOT") {
        Ok(s) => s,
        Err(_) => env::current_dir().unwrap().into_os_string().into_string().unwrap(),
    };


    println!("Binding to {}", &address);
    let listener = match TcpListener::bind(&address) {
        Err(what) => return error(&format!("Unable to bind '{}': {}", &address, what)),
        Ok(l) => l
    };

    println!("Serving from {}", &wwwroot);

    for stream in listener.incoming() { 
        match stream {
            Ok(stream) => handle_connection(stream, &wwwroot),
            Err(e) => eprintln!("Connection failed: {}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream, wwwroot: &String) {
    let buf_reader = BufReader::new(&mut stream);

    let content:Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect(); 

    let request_line = &content[0];
    let _headers = &content[1..];

    let mut split = request_line.split(" ");
    let command = split.next();
    let path = split.next();
    let version = split.next();

    let body = match (command, path, version) {
        (Some("GET"), Some("/"), Some("HTTP/1.1")) =>
            response_for_path("/index.html", wwwroot),

        (Some("GET"), Some(path), Some("HTTP/1.1")) =>
            response_for_path(path, wwwroot),

        _ =>
            "HTTP/1.1 501 NotImplemented\r\n\r\nNot Implemented.\r\n".to_string(),
    };

    match stream.write_all(body.as_bytes()) {
        Err(e) => eprintln!("Unable to write response: {}", e),
        Ok(r) => r
    }
}

fn response_for_path(path: &str, wwwroot: &String) -> String {
    let mut full_path = PathBuf::new();
    full_path.push(wwwroot);
    full_path.push(&path[1..]);

    let contents = fs::read_to_string(&full_path.as_path());

    match contents {
        Ok(contents) => {
            format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), &contents)
        },
        Err(_) =>
            format!("HTTP/1.1 404 NOT FOUND\r\n\r\nFour Owe For...\r\n")
    }

}


fn error(why: &dyn Display) {
    eprintln!("{}", why);
    std::process::exit(1);
}