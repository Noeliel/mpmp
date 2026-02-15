use crate::{
    Result,
    crypto_impl::{CryptoStream, ssl_start_client, ssl_start_server},
    error::Error,
};
use api::{
    codec::{decode, encode},
    message::Message,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const BUF_SIZE: usize = 1024;

type InnerConnection = TcpStream;
pub type PeerConnection = Connection<CryptoStream<InnerConnection>>;

pub enum ConnectionRole {
    Client,
    Server,
}

#[derive(PartialEq, Hash, Debug)]
pub struct Connection<T>
where
    T: Read + Write,
{
    stream: T,
}

// TODO: clean up Result types and maybe try to reconnect (not here)
impl<T: Read + Write> Connection<T> {
    // TODO: maybe rewrite with peeking -> consider case when data can be read but no message decoded
    pub fn read_messages(&mut self) -> Result<Option<Vec<Message>>> {
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

        // TODO: do we pass the underlying error?
        // returns std::io::Error with std::io::ErrorKind::WouldBlock if queue is empty
        let read = self.stream.read(&mut buf);

        match read {
            Ok(read) => {
                if read > 0 {
                    Ok(Some(decode(&buf)))
                } else {
                    Err(Error::SocketClosedError)
                }
            }
            Err(_) => Ok(None),
        }
    }

    pub fn write_message(&mut self, message: Message) -> Result<()> {
        let encoded = encode(&message);
        self.stream.write_all(encoded.as_slice())?;
        self.stream.flush()?;

        Ok(())
    }

    // pub fn peek(&mut self) -> Result<usize> {
    //     self.stream.peek()
    // }
}

impl Connection<InnerConnection> {
    pub fn tls_wrap(
        stream: InnerConnection,
        conntype: ConnectionRole,
        nonblocking: bool,
    ) -> Result<PeerConnection> {
        let mut stream = match conntype {
            ConnectionRole::Client => ssl_start_client(stream),
            ConnectionRole::Server => ssl_start_server(stream),
        }?;

        if nonblocking {
            _ = stream.get_mut().set_nonblocking(true);
        }

        Ok(Connection { stream })
    }
}
