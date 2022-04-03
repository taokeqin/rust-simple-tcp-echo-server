use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let port = 9000;
    println!("TCP Server Listening at: {}...", port);
    //start server
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let mut thread_vec = Vec::new();
    //get a tcp connection
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("{} Joined.", stream.peer_addr().unwrap());
                //start a new thread to handle the connection/stream
                let thread = thread::spawn(|| handle_connection(stream));
                thread_vec.push(thread);
            },
            Err(_) => {
                //do nothing.
            }
        }

    }
    for t in thread_vec {
        // try to join the thread.
        match t.join() {
            Ok(_) => {
                println!("Join thread success.")
            },
            Err(_) => {
                println!("Join thread error.")
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop{
        //create a buffer, for every cycle, 
        //TODO: create a shared buffer, and zero it every time before using.
        let mut buffer = [0; 1024];
        //read content from network to the buffer
        let read_result = stream.read(&mut buffer);
        match read_result {
            Ok(len) => {
                if len > 0 {
                    // if read and with content, do the echo
                    println!("{} says: {}", stream.peer_addr().unwrap(), String::from_utf8_lossy(&buffer[..]));
                    stream.write(&buffer).unwrap();
                } else {
                    // if read but 0 content, normally peer closed the connection.
                    println!("{} says: bye.", stream.peer_addr().unwrap());
                    break;
                }
            },
            Err(_) => {
                println!("{} says: bye.", stream.peer_addr().unwrap())
            }
        }
    }
}