use crate::message::Message;
use std::io::Cursor;

pub fn encode(message: &Message) -> Vec<u8> {
    rmp_serde::to_vec(message).expect("Failed to encode message")
}

pub fn decode(message: &[u8]) -> Vec<Message> {
    let mut messages = Vec::new();
    let mut c = Cursor::new(message);

    // TODO: introduce upper bound to prevent vulnerability to DoS?
    while let Ok(msg) = rmp_serde::from_read::<_, Message>(&mut c) {
        messages.push(msg);
    }

    messages
}

#[cfg(test)]
mod tests {
    use crate::{
        bidirectional::PropertiesV1,
        client,
        codec::{decode, encode},
        server::{self},
    };

    #[test]
    fn can_decode_multi_object_buffer() {
        let m1 = server::HelloV1::new();
        let m2 = server::ChangePropertiesV1::new(PropertiesV1::new(true, 1234.0, 1.0));
        let m3 = client::HelloV1::new(PropertiesV1::new(true, 1234.0, 1.0));
        let enc_1 = encode(&m1);
        let enc_2 = encode(&m2);
        let enc_3 = encode(&m3);
        let combined = vec![enc_1, enc_2, enc_3].concat();

        let mut decoded = decode(combined.as_slice());

        assert_eq!(decoded.len(), 3);
        assert_eq!(m3, decoded.pop().unwrap());
        assert_eq!(m2, decoded.pop().unwrap());
        assert_eq!(m1, decoded.pop().unwrap());
    }
}
