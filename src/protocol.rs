use serde::{Serialize, Deserialize};

/// A handshake message is the first thing sent by the server after establishing
/// a connection to the agent.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Handshake {
    pub version: u64,
    pub name: String,
    pub secret: String,
}

// The start of a file stream.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileHeader {
    pub path: String,
    pub size: u64,
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
        let hs = Handshake { version: 0 };
        let hs_bin = pack(&hs).unwrap();
        let hs_deserialized = unpack(&hs_bin[..]).unwrap();
        assert_eq!(hs, hs_deserialized);
    }
}
