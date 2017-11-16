use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    errors {
        MoreDataNeeded(needed: usize) {
            description("More data is needed to continue")
            display("At least {} bytes are needed", needed)
        }
        InvalidData
    }
}
