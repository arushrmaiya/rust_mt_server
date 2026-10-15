use std::{
    io::{BufReader, prelude::*}, 
    net::{TcpListener, TcpStream}, 
    thread, 
    time::Duration
};

use rust_mt_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        // handle_connection(stream);
        pool.execute(|| {
            handle_connection(stream);
        });

    println!("Shutting down.");

    }
}


fn handle_connection(mut stream:TcpStream){
    let buf = BufReader::new(&stream);

    let req_line = buf.lines().next().unwrap().unwrap();
    println!("{req_line:?}");

    let (status_line, filename) = match req_line.as_str(){
        "GET / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ =>{
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }

    };

    let contents = std::fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
