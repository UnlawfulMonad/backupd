use error_chain::error_chain;

error_chain! {
    errors {
        HandshakeError(msg: Option<String>) {
            description("Handshake with endpoint failed")
            display("failed to authenicate: {:?}", msg)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        VarError(::std::env::VarError);
        Bincode(Box<::bincode::ErrorKind>);
    }
}
