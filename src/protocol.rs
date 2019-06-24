use serde::{Serialize, Deserialize};


pub const VERSION: u64 = 1;

/// A handshake message is the first thing sent by the server after establishing
/// a connection to the agent.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Handshake {
    pub version: u64,
    pub name: String,
    pub secret: String,
}

/// The start of a file stream.
/// Following this message is a file `size` bytes long to be included in the current
/// backup operation.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileHeader {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ack {
    pub success: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use rmp_serde::{
        to_vec as pack,
        from_slice as unpack,
    };

    #[test]
    fn test_serialize_handshake() {
        let hs = Handshake { version: 0, name: String::new(), secret: String::new() };
        let hs_bin = pack(&hs).unwrap();
        let hs_deserialized = unpack(&hs_bin[..]).unwrap();
        assert_eq!(hs, hs_deserialized);
    }
}
