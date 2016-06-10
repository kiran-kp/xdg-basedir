use std::convert::From;
use std::io;

error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links {
    }

    foreign_links {
        io::Error, Io, "IO error";
    }

    errors {
        NoHomeDir {
            description("Could not find home dir")
            display("$HOME not defined or is not valid")
        }
    }
}
