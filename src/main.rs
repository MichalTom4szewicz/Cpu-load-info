use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use systemstat::{System, Platform};
use std::thread;
use std::time::Duration;
use chrono::{Datelike, Timelike, Utc};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Data {
    time: String,
    load: String,
}

fn handle_connection(mut stream: TcpStream) {
    println!("req");

    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            println!("Measuring CPU load...");
            thread::sleep(Duration::from_millis(1000));

            let cpu = cpu.done().unwrap();
            let now = Utc::now();
            let (is_pm, hour) = now.hour12();
            let (_, year) = now.year_ce();

            let timestamp = format!("{}:{}:{} {}   {}.{}.{}", hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" }, year, now.month(), now.day());
            let load = format!("{}%", cpu.user*100.0 );
            println!("ts {} and cpu {}", timestamp, load);

            let data = Data {
                time: timestamp,
                load: load
            };

            let serialized = serde_json::to_string(&data).unwrap();

            let response = format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nConnection: keep-alive\r\nContent-Length: {}\r\nContent-Type: application/json;charset=utf-8\r\n\r\n{}",
            serialized.len(),
            serialized
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("sent! {} bytes\n", serialized.len());
        },
        Err(x) => println!("CPU load: error: {}", x)
    }
}

fn main() {
    let listener = TcpListener::bind("192.168.1.19:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 4096];

//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }