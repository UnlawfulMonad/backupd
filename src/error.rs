use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        VarError(::std::env::VarError);
        Bincode(Box<::bincode::ErrorKind>);
    }
}
