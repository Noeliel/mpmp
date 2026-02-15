use crate::{
    client::ConnectionState::{Connected, Disconnected},
    events::ClientEvent,
    mediaplayer::MediaPlayer,
};
use api::{
    client,
    server::ServerMessage::{self},
};
use api::{client::ChangePropertiesV1, message::Message};
use common::{
    Result,
    config::Config,
    error::Error,
    socket::{Connection, ConnectionRole, PeerConnection},
};
use std::net::TcpStream;
use std::time::{Duration, Instant};

enum ConnectionState {
    Disconnected(Instant),
    Connected(PeerConnection),
}

pub struct Client<'p, T> {
    player: &'p T,
    config: Config,
    state: ConnectionState,
}

static NOT_CONNECTED_ERR: &str = "Not connected";

impl<'p, T: MediaPlayer> Client<'p, T> {
    pub fn new(player: &'p T, config: Config) -> Self {
        let mut client = Client {
            player,
            config,
            state: Disconnected(Instant::now()),
        };

        _ = client.try_connect();

        client
    }

    pub fn fetch_events(&mut self) -> Result<Vec<ClientEvent>> {
        self.ensure_connected_intermittently();

        if let Connected(ref mut c) = self.state {
            let msgs = c.read_messages().inspect_err(|_| {
                self.handle_disconnect();
            })?;

            let events = msgs
                .unwrap_or(Vec::new())
                .into_iter()
                .filter_map(|msg| match msg {
                    Message::ServerMessage(msg) => match msg {
                        ServerMessage::HelloV1(_) => None, // TODO
                        ServerMessage::ChangePropertiesV1(p) => {
                            Some(ClientEvent::PropertyChange(p.into()))
                        }
                    },
                    _ => None,
                })
                .collect();

            Ok(events)
        } else {
            Err(Error::OtherError(NOT_CONNECTED_ERR))
        }
    }

    pub fn signal_property_change(&mut self) -> Result<()> {
        self.ensure_connected_now();

        if let Connected(ref mut c) = self.state {
            let state = self
                .player
                .get_playback_state()
                .map_err(|e| Error::OtherError(e))?;

            c.write_message(ChangePropertiesV1::new(state.into()))
                .inspect_err(|_| {
                    self.handle_disconnect();
                })
        } else {
            Err(Error::OtherError(NOT_CONNECTED_ERR))
        }
    }

    pub fn ensure_connected_intermittently(&mut self) {
        if let Disconnected(since) = self.state
            && since.elapsed() >= Duration::from_secs(5)
        {
            self.ensure_connected_now();
        }
    }

    pub fn ensure_connected_now(&mut self) {
        if let Disconnected(_) = self.state {
            _ = self.try_connect();
        }
    }

    fn try_connect(&mut self) -> Result<()> {
        let stream = TcpStream::connect(self.config.get_host())
            .or_else(self.connection_failure(/* notify */ false))?;

        Connection::tls_wrap(stream, ConnectionRole::Client, /* nonblocking */ true)
            .and_then(|mut c| {
                let state = self
                    .player
                    .get_playback_state()
                    .map_err(|e| Error::OtherError(e))?;

                c.write_message(client::HelloV1::new(state.into()))?;
                Ok(c)
            })
            .and_then(self.connection_success(/* notify */ true))
            .or_else(self.connection_failure(/* notify */ true))
    }

    fn connection_success<E>(
        &mut self,
        notify: bool,
    ) -> impl FnOnce(PeerConnection) -> std::result::Result<(), E> {
        move |c| {
            self.state = Connected(c);

            if notify {
                self.player.display("[mpmp] Connected");
            }

            Ok(())
        }
    }

    fn connection_failure<O, E>(
        &mut self,
        notify: bool,
    ) -> impl FnOnce(E) -> std::result::Result<O, E> {
        move |e| {
            self.state = Disconnected(Instant::now());

            if notify {
                self.player.display("[mpmp] Failed to connect");
            }

            Err(e)
        }
    }

    fn handle_disconnect(&mut self) {
        self.player.display("[mpmp] Connection lost");
        self.state = Disconnected(Instant::now());
    }
}
