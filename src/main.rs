use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use sysinfo::System;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:32762").unwrap();
    println!("Server listening on port 32762");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut sys: System = System::new();
    sys.refresh_all();
  

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let xml_response: String = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<data>

    <total_mem>{}</total_mem>
    <used_mem>{}</used_mem>
</data>"#,
            sys.total_memory(),
            sys.used_memory()
        );

        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/xml\r\nContent-Length: {}\r\n\r\n{}",
            xml_response.len(),
            xml_response
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents: &str = "404 Not Found";

        let response: String = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
