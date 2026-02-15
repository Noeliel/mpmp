use api::{client::HelloV1, message::Message, server::ServerMessage};
use client::events::PlaybackState;
use common::socket::{Connection, ConnectionRole};
use std::{io::stdin, net::TcpStream, sync::mpsc, thread, time::Duration};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8015").expect("Failed to connect");
    let mut conn =
        Connection::tls_wrap(stream, ConnectionRole::Client, true).expect("Failed to wrap in TLS");

    _ = conn.write_message(HelloV1::new(
        PlaybackState::new(
            /* paused */ false, /* time_pos */ 0.0, /* speed */ 1.0,
        )
        .into(),
    ));
    let (tx, _rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            _ = stdin().read_line(&mut buf);
            if tx.send(buf).is_err() {
                break; // receiver is dead, quit
            }
        }
    });

    loop {
        if let Ok(Some(msg)) = conn.read_messages() {
            msg.iter().for_each(|msg| {
                if let Message::ServerMessage(msg) = msg {
                    match msg {
                        ServerMessage::HelloV1(_) => {
                            eprintln!("[Hello] Received server hello v1");
                        }
                        ServerMessage::ChangePropertiesV1(p) => {
                            eprintln!(
                                "Received property change: Paused -> {}, TimePos -> {}",
                                p.paused(),
                                p.time_pos()
                            )
                        }
                    }
                }
            });
        }

        // chat is not implemented yet
        // if let Some(msg) = rx.try_recv().ok() {
        //     _ = conn.write_message(Message::ClientMessage(
        //         api::client::ClientMessage::ChatMessageV1(ChatMessageV1 { msg: msg }),
        //     ));
        // }

        thread::sleep(Duration::from_millis(1));
    }
}
