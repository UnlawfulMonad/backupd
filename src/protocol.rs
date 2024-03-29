use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ack {
    pub success: bool,
    pub message: Option<String>,
}

macro_rules! message_helper {
    // We should be able to simplify this once `concat_idents` is stablized.
    ($read_name:ident, $write_name:ident, $type:ident) => {
        #[allow(dead_code)]
        #[inline]
        pub fn $read_name<R: ::std::io::Read>(r: R) -> super::error::Result<$type> {
            Ok(::bincode::deserialize_from(r)?)
        }

        #[allow(dead_code)]
        #[inline]
        pub fn $write_name<W: ::std::io::Write>(w: W, t: &$type) -> super::error::Result<()> {
            ::bincode::serialize_into(w, t)?;
            Ok(())
        }
    };
}

message_helper!(read_handshake, write_handshake, Handshake);
message_helper!(read_ack, write_ack, Ack);
message_helper!(read_file_header, write_file_header, FileHeader);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_handshake() {
        let hs = Handshake {
            version: 0,
            name: String::new(),
            secret: String::new(),
        };
        let hs_bin = bincode::serialize(&hs).unwrap();
        let hs_deserialized = bincode::deserialize(&hs_bin[..]).unwrap();
        assert_eq!(hs, hs_deserialized);
    }
}
