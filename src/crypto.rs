use std::io::{self, Write, Read};
use openssl::symm::{Cipher, Crypter, Mode};

const TAG_SIZE: usize = 16;
const BUFFER_SIZE: usize = 8192;

pub struct Encrypter<W> {
    crypter: Crypter,
    writer: W,
    buffer: Vec<u8>,
}

impl<W: Write> Encrypter<W> {
    pub fn new(writer: W, key: &[u8], iv: &[u8]) -> Encrypter<W> {
        assert_eq!(key.len(), 32);
        assert_eq!(iv.len(), 32);

        let crypter = Crypter::new(
            Cipher::aes_256_gcm(),
            Mode::Encrypt,
            key,
            Some(iv)).unwrap();
        Encrypter { crypter, writer, buffer: vec![0; BUFFER_SIZE] }
    }

    pub fn finalize(mut self) -> io::Result<(W, Vec<u8>)> {
        let finalized_bytes = self.crypter.finalize(&mut self.buffer)?;
        self.writer.write_all(&self.buffer[..finalized_bytes])?;

        self.crypter.get_tag(&mut self.buffer[..TAG_SIZE])?;
        Ok((self.writer, self.buffer[..TAG_SIZE].into()))
    }
}

impl<W: Write> Write for Encrypter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        assert!(buf.len() < self.buffer.len());

        let bytes_generated = self.crypter.update(buf, &mut self.buffer)?;
        self.writer.write_all(&self.buffer[..bytes_generated])?;
        Ok(bytes_generated)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub struct Decrypter<R> {
    crypter: Crypter,
    reader: R,
    buffer: Vec<u8>,
}

impl<R: Read> Decrypter<R> {
    pub fn new(reader: R, key: &[u8], iv: &[u8]) -> Decrypter<R> {
        assert_eq!(key.len(), 32);
        assert_eq!(iv.len(), 32);

        let crypter = Crypter::new(
            Cipher::aes_256_gcm(),
            Mode::Decrypt,
            key,
            Some(iv)).unwrap();
        Decrypter { crypter, reader, buffer: vec![0; BUFFER_SIZE] }
    }
}

impl<R: Read> Read for Decrypter<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(&mut self.buffer)?;
        Ok(self.crypter.update(&self.buffer[..bytes_read], buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_encrypter() {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        let mut iv = [0u8; 32];
        rng.fill(&mut key);
        rng.fill(&mut iv);

        let mut buf = Vec::with_capacity(1024);

        let mut e = Encrypter::new(&mut buf, &key, &iv);
        e.write(b"test").unwrap();
        let (_, tag) = e.finalize().unwrap();
        assert_eq!(tag.len(), 16);
    }
}
