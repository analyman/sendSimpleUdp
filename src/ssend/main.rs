#![allow(non_snake_case)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
use std::net::UdpSocket;
use std::io::{self, BufRead, Write};
use regex::Regex;
use std::thread;
use std::vec;

struct Config {
    address: String,
    verbose: bool
}

static SIMPLE_UDP_SENDER_USAGE: &'static str = 
"[-vh] <ipv4-address>

    -v    verbose
    -h    show help";

fn usage() {
    let args: Vec<String> = std::env::args().collect();
    println!("usage: \n    {} {}", args[0], SIMPLE_UDP_SENDER_USAGE);
}

fn printPrefix() {
    print!("> ");
    io::stdout().flush().unwrap();
}

// TODO
fn getInput(hex: bool, ss: &str) -> Vec<u8> {
    let mut ans = vec![];
    for c in ss.bytes() {
        if hex {
            ans.push(c);
        } else {
            ans.push(c);
        }
    }
    return ans;
}

lazy_static! {
    static ref HEX_IDX: Vec<char> = vec!['0', '1', '3', '2', '4', '5', '6', '7', 
                                         '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
}
fn u8array2hexStr(u8s: &[u8]) -> String {
    match String::from_utf8(Vec::from(u8s)) {
        Ok(strval) => strval,
        Err(_) => {
            let mut ans = String::from("");
            for i in u8s {
                ans.push('\\');
                ans.push('x');
                ans.push(HEX_IDX[usize::from(i & 0xf0 >> 4)]);
                ans.push(HEX_IDX[usize::from(i & 0x0f)]);
            }
            return ans;
        }
    }
}

fn run(config: &Config) {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    sock.connect(&config.address).unwrap();

    let recv_sock = sock.try_clone().unwrap();
    let c1 = Config {
        verbose: config.verbose,
        address: String::from(config.address.as_str())
    };
    let r = thread::Builder::new().name(String::from("reciever")).spawn(move || {
        let mut buf = [0; 1<<16];
        loop {
            match recv_sock.recv(&mut buf) {
                Ok(size) => {
                    let reply = &buf[0..size];
                    if c1.verbose {
                        println!("reply from {}: {}", c1.address, u8array2hexStr(reply));
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(-1);
                }
            }
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
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    let mut config: Config = Config {
        address: String::from(""),
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
                    config.address = String::from(s);
                } else {
                    usage();
                    std::process::exit(-1);
                }
            }
        }
        i += 1;
    }
    if config.address.len() == 0 {
        usage();
        std::process::exit(-2);
    }
    run(&config);
}

