

use std::{
    io::{BufRead, BufReader, Write, Read}, // Add Read trait
    net::{TcpListener, TcpStream},
    thread,
};


fn main() {
    println!("Starting the server...");
    
    let server = TcpListener::bind("0.0.0.0:8002").unwrap();
    println!("Server started on {}", server.local_addr().unwrap().to_string().as_str());
    println!("Waiting for connections...\n\r\n\r");
    
    for stream in server.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream.peer_addr());
        println!("Connection established!");          
        thread::spawn(|| { redirect(stream); });
    }
    
}



fn redirect(mut stream: TcpStream) {    
    let mut buffer = [0; 1024];
    
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    
    loop {

        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_lines: Vec<&str> = request.lines().collect();
        let first_line = request_lines.get(0).unwrap_or(&"");
        let method = first_line.split_whitespace().next().unwrap_or("GET");
        let url = first_line.split_whitespace().nth(1).unwrap_or("/error");
        let redirect_param = url.split("redirect?=").nth(1).unwrap_or("test.yae.gay");
        
        if redirect_param.is_empty() {

            break;   
        }

        println!("method: {}", method);
        println!("first_line: {}", url);
        println!("redirect_param: '{}'", redirect_param);


        let response = format!("HTTP/1.1 302 Found\r\nLocation: https://{}\r\n\r\n", redirect_param);
        

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();  stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}




