use crate::lobby::{Lobby, NewPeer};
use common::{
    config::Config,
    socket::{Connection, ConnectionRole},
};
use std::{net::TcpListener, thread, time::Duration};

mod bitflags;
mod cached_properties;
mod lobby;

fn open_socket() {
    let config = Config::new();
    let host = if let Ok(ref config) = config {
        config.get_host()
    } else {
        "0.0.0.0:8015"
    };

    eprintln!("Listening on {}", host);

    let listener = listen(host);

    let (mut lobby, l_tx) = Lobby::new();

    thread::spawn(move || {
        loop {
            if lobby.tick().is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(2));
        }
    });

    for stream in listener.incoming().flatten() {
        eprintln!("New connection from {}", stream.peer_addr().unwrap());

        if let Ok(conn) = Connection::tls_wrap(stream, ConnectionRole::Server, true) {
            _ = l_tx.send(NewPeer::new(conn));
        } else {
            eprintln!("Failed to wrap in TLS");
        }
    }
}

fn listen(host: &str) -> TcpListener {
    TcpListener::bind(host).unwrap()
}

fn main() {
    open_socket();
}
