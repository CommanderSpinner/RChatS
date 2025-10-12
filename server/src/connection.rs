use postgres::{Client, NoTls, Error};

struct Connection {
    connection_string: String,
}

impl Connection {
    fn new(connection_string: &str) -> Connection {
        Connection { connection_string: connection_string.to_string() }
    }
}