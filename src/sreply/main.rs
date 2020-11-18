#![allow(non_snake_case)]
use std::net::UdpSocket;
use regex::Regex;

static SIMPLE_UDP_SENDER_USAGE: &'static str = 
"[-vh] <ipv4-address>

    -v    verbose
    -h    show help

    <ipv4-address> has format likes '127.0.0.1:8080'";

struct Config {
    sockaddr: String,
    verbose:  bool
}

fn usage() {
    let args: Vec<String> = std::env::args().collect();
    println!("usage:\n    {} {}", args[0], SIMPLE_UDP_SENDER_USAGE);
}

fn run(config: &Config) {
    let sock = UdpSocket::bind(&config.sockaddr).unwrap();

    let mut buf = [0; 16*16];
    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                sock.send_to(&buf[0..size], addr).unwrap();
                if config.verbose {
                    println!("{} -> {}: {:?}", addr, config.sockaddr, &buf[0..size]);
                }
            },
            Err(e) => {
                println!("{}", e);
                std::process::exit(-1);
            }
        };
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    let mut config: Config = Config {
        sockaddr: String::from(""),
        verbose: false
    };
    let valid_ipv4_port = Regex::new(r"^([0-9]{1,3}.){3}[0-9]{1,3}:[0-9]{1,5}$").unwrap();
    while i < args.len() {
        let s = args[i].as_str();
        match s {
            "-v" => {
                config.verbose = true;
            },
            "-h" => {
                usage();
                std::process::exit(0);
            }
            _ => {
                if valid_ipv4_port.is_match(s) && i == args.len() -1 {
                    config.sockaddr = String::from(s);
                } else {
                    usage();
                    std::process::exit(-1);
                }
            }
        }
        i += 1;
    }
    if config.sockaddr.len() == 0 {
        usage();
        std::process::exit(-2);
    }
    run(&config);
}

