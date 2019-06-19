use std::fs::File;
use std::io::{self, Read, Write};

pub fn encrypt_file<W: Write>(_f: &File, _dest: W) -> io::Result<()> {
    unimplemented!();
}

pub fn decrypt_file<R: Read>(_f: &File, _dest: R) -> io::Result<()> {
    unimplemented!();
}
