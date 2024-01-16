use std::{
    io::{self, Write},
    net::TcpStream,
};
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:6969")?;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let msg = line.trim().to_string();
        stream.write(msg.as_bytes())?;
    }
}
