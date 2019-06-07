use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Handshake {
    pub version: u64,

}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileHeader {
    pub path: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub last: bool,
    pub data: Vec<u8>,
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
