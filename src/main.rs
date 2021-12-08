extern crate tun_tap;

use clap::Parser;
use std::io::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::Arc;
use std::thread;
use tun_tap::{Iface, Mode};

//     https://github.com/rust-vsock/vsock-rs/blob/master/echo_server/src/main.rs

#[derive(Parser)]
#[clap(version = "1.0", author = "Pennefather S. <pennefather.sean@gmail.com>")]
struct AppArgs {
    /// Client = mode 0, server = mode 1
    mode: u32,
}

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = Command::new("ip").args(args).spawn().unwrap().wait().unwrap();
    assert!(ecode.success(), "Failed to execte {}", cmd);
}

// [TUN a] <--> [Socket] <--> [TUN b]
fn tun_to_sock(tun: std::sync::Arc<tun_tap::Iface>, sock: &mut std::net::TcpStream) {
    let mut buf = [0; 4096];
    println!("Creating TUN consumer");
    loop {
        let amount = tun.recv(&mut buf).unwrap();
        sock.write(&buf[0..amount]).unwrap();
    }
}

fn sock_to_tun(tun: std::sync::Arc<tun_tap::Iface>, sock: &mut std::net::TcpStream) {
    let mut buf = [0; 4096];
    println!("Creating SOCK consumer");
    loop {
        let amount = sock.read(&mut buf).unwrap();
        tun.send(&buf[0..amount]).unwrap();
    }
}

fn create_server_socket() -> Result<TcpStream, Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    match listener.accept() {
        Ok((socket, _addr)) => Ok(socket),
        Err(e) => Err(e),
    }
}

fn create_client_socket() -> TcpStream {
    return TcpStream::connect("127.0.0.1:8000").expect("failed to create a TCP socket");
}

fn client_application() {
    let tun = Iface::new("tun_client", Mode::Tun).unwrap();
    cmd("ip", &["addr", "add", "dev", "tun_client", "10.0.0.3/24"]);
    cmd("ip", &["link", "set", "up", "dev", "tun_client"]);
    let tun = Arc::new(tun);
    let tun_writer = Arc::clone(&tun);
    let tun_reader = Arc::clone(&tun);
    let mut sock_reader = create_client_socket();
    let mut sock_writer = sock_reader.try_clone().expect("Failed to clone socket");

    let sock_tun_handle = thread::spawn(move || {
        sock_to_tun(tun_writer, &mut sock_reader);
    });
    let tun_sock_handle = thread::spawn(move || {
        tun_to_sock(tun_reader, &mut sock_writer);
    });

    tun_sock_handle.join().unwrap();
    sock_tun_handle.join().unwrap();
}

fn server_application() {
    let tun = Iface::new("tun_server", Mode::Tun).unwrap();
    cmd("ip", &["addr", "add", "dev", "tun_server", "10.0.0.2/24"]);
    cmd("ip", &["link", "set", "up", "dev", "tun_server"]);
    let tun = Arc::new(tun);
    let tun_writer = Arc::clone(&tun);
    let tun_reader = Arc::clone(&tun);
    let mut sock_reader = create_server_socket().unwrap();
    let mut sock_writer = sock_reader.try_clone().expect("Failed to clone socket");

    let sock_tun_handle = thread::spawn(move || {
        sock_to_tun(tun_writer, &mut sock_reader);
    });
    let tun_sock_handle = thread::spawn(move || {
        tun_to_sock(tun_reader, &mut sock_writer);
    });

    tun_sock_handle.join().unwrap();
    sock_tun_handle.join().unwrap();
}

fn main() {
    let args: AppArgs = AppArgs::parse();

    if args.mode == 0 {
        client_application();
    } else {
        server_application();
    }
}
