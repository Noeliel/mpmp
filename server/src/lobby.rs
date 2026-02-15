use crate::{
    bitflags::BitFlag,
    cached_properties::{CachedProperties, Property},
};
use api::{client::ClientMessage, message::Message, server};
use common::{Result, error::Error, socket::PeerConnection};
use std::{
    collections::VecDeque,
    ptr::eq,
    sync::mpsc::{Receiver, Sender, channel},
};

pub struct Lobby<'m> {
    clients: Vec<PeerConnection>,
    receiver: Receiver<LobbyMessage<'m>>,
    properties: Option<CachedProperties>,
}

impl<'m> Lobby<'m> {
    pub fn new() -> (Self, Sender<LobbyMessage<'m>>) {
        let (tx, rx) = channel();

        (
            Lobby {
                clients: Vec::new(),
                receiver: rx,
                properties: None,
            },
            tx,
        )
    }

    pub fn add_client(&mut self, connection: PeerConnection) -> Result<&PeerConnection> {
        self.clients.push(connection);
        self.clients
            .last()
            .ok_or(Error::OtherError("Failed to retrieve client ref"))
    }

    pub fn remove_client_by_index(&mut self, index: &usize) -> Result<PeerConnection> {
        if *index >= self.clients.len() {
            return Err(Error::OtherError("Index OOB"));
        }

        let c = self.clients.remove(*index);
        Ok(c)
    }

    pub fn remove_client_by_ref(&mut self, connection: &PeerConnection) -> Result<PeerConnection> {
        let c = self
            .clients
            .pop_if(|e| eq(e, connection))
            .ok_or(Error::OtherError("Unknown client"))?;

        Ok(c)
    }

    pub fn tick(&mut self) -> std::result::Result<(), ()> {
        if let Ok(lobby_message) = self.receiver.try_recv() {
            match lobby_message {
                LobbyMessage::NewPeer(new_peer) => {
                    _ = self.add_client(new_peer.peer);
                }
                LobbyMessage::RemovePeer(remove_peer) => {
                    _ = self.remove_client_by_ref(remove_peer.peer);
                }
                LobbyMessage::Exit => return Err(()),
            }
        }

        let current_properties = if let Some(ref props) = self.properties {
            props
        } else {
            &CachedProperties::default()
        };

        // will be Some variant after iterating through clients if a property
        // update takes place this tick
        let mut new_properties: Option<CachedProperties> = None;

        // clients that are to be removed this tick
        let mut disconnected_clients: VecDeque<usize> = VecDeque::new();

        self.clients.iter_mut().enumerate().for_each(|(i, client)| {
            let msg = client.read_messages();

            match msg {
                Ok(Some(msg)) => {
                    msg.into_iter().for_each(|msg| {
                        if let Message::ClientMessage(msg) = msg {
                            match msg {
                                ClientMessage::HelloV1(hello) => {
                                    _ = client.write_message(server::HelloV1::new());

                                    if self.properties.is_none() {
                                        // initialize with the first client
                                        new_properties = Some((&hello.current_properties).into());
                                    } else {
                                        // new client to existing lobby
                                        // -> inform of current state in any case (even if we don't update props this tick)
                                        _ = client.write_message(server::ChangePropertiesV1::new(
                                            current_properties.into(),
                                        ));
                                    }
                                }
                                ClientMessage::ChangePropertiesV1(p) => {
                                    let mut new_state: CachedProperties = (&p).into();

                                    let prop_diff = current_properties.diff(&new_state);

                                    // pause on seek to help with sync since it can generate a lot of events;
                                    // (note: prop_diff pretty much always contains Property::TimePos unless we're paused
                                    // so we specifically look for events *only* changing Property::TimePos)
                                    if prop_diff.is(Property::TimePos as u32) {
                                        new_state.paused = true;
                                    }

                                    // store them
                                    new_properties = Some(new_state);

                                    eprintln!("Received new properties");
                                }
                                ClientMessage::GetPropertiesV1 => {
                                    // TODO: do this in the latter part of this tick instead
                                    // (and remember which client requested the properties, if we're not sending them to all anyways)
                                    _ = client.write_message(server::ChangePropertiesV1::new(
                                        current_properties.into(),
                                    ))
                                }
                            }
                        }
                    });
                }
                Err(Error::SocketClosedError) => {
                    disconnected_clients.push_front(i);
                }
                _ => {}
            }
        });

        // drop disconnected clients
        disconnected_clients.iter().for_each(|i| {
            _ = self.remove_client_by_index(i);
        });

        // inform clients about updated playback state
        if let Some(ref new) = new_properties {
            self.clients.iter_mut().for_each(|client| {
                _ = client.write_message(server::ChangePropertiesV1::new(new.into()));
            });
        }

        // finally, commit the new props if we have clients left
        if self.clients.is_empty() {
            self.properties = None
        } else if new_properties.is_some() {
            self.properties = new_properties;
        }

        Ok(())
    }
}

impl<'m> Drop for Lobby<'m> {
    fn drop(&mut self) {
        // TODO: notify peers of disconnect?
    }
}

pub enum LobbyMessage<'m> {
    NewPeer(NewPeer),
    RemovePeer(RemovePeer<'m>),
    Exit,
}

pub struct NewPeer {
    peer: PeerConnection,
}

impl<'m> NewPeer {
    pub fn new(peer: PeerConnection) -> LobbyMessage<'m> {
        LobbyMessage::NewPeer(NewPeer { peer })
    }
}

pub struct RemovePeer<'p> {
    peer: &'p PeerConnection,
}

impl<'p> RemovePeer<'p> {
    pub fn new(peer: &'p PeerConnection) -> LobbyMessage<'p> {
        LobbyMessage::RemovePeer(RemovePeer { peer })
    }
}
