#![allow(non_snake_case)]
use std::net::UdpSocket;
use std::io::{self, BufRead, Write};
use std::thread;

static SIMPLE_UDP_SENDER_USAGE: &'static str = 
"usage:
    sudp [-ovh] <ipv4-address> [message]

    -o    run once
    -v    verbose
    -h    show help";

fn usage() {
    println!("{}", SIMPLE_UDP_SENDER_USAGE);
}

fn printPrefix() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn run() {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    sock.connect("127.0.0.53:53").unwrap();

    let recv_sock = sock.try_clone().unwrap();
    let r = thread::Builder::new().name(String::from("reciever")).spawn(move || {
        let mut buf = [0; 16*16];
        loop {
            match recv_sock.recv(&mut buf) {
                Ok(size) => {
                    let reply = String::from_utf8_lossy(&buf[0..size]);
                    println!("reply: {}", reply);
                },
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(-1);
                }
            };
        }
    });

    let s = thread::Builder::new().name(String::from("sender")).spawn(move || {
        let mut buf = String::new();
        let stdin = io::stdin();
        loop {
            buf.clear();
            printPrefix();
            stdin.lock().read_line(&mut buf).unwrap();
            sock.send(buf.as_bytes()).unwrap();
        }
    });
    r.unwrap().join().unwrap();
    s.unwrap().join().unwrap();
}

fn main() {
    usage();
    run();
}

