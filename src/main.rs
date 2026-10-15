use std::{
    io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, print,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }
}


fn handle_connection(mut stream:TcpStream){
    let buf = BufReader::new(&stream);
    // let http_req: Vec<_> = buf
    //             .lines()
    //             .map(|result| result.unwrap())
    //             .take_while(|line| !line.is_empty())
    //             .collect();

    // println!("Request: {http_req:#?}");


    // let status_line = "HTTP/1.1 200 OK";
    // let contents = std::fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    let req_line = buf.lines().next().unwrap().unwrap();
    println!("{req_line:?}");

    let (status_line, filename) = if req_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = std::fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
