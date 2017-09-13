use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    errors {
        MoreDataNeeded
        InvalidData
    }
}
