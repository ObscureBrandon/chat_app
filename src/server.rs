use std::{io::Read, net::TcpListener};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6969")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => loop {
                let mut buff = vec![0; 32];
                match stream.read(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("invalid");
                        println!("{}", msg);
                    }

                    Err(_) => {
                        eprint!("nono");
                        break;
                    }
                }
            },
            Err(_) => {
                eprint!("nono");
            }
        }
    }
    Ok(())
}
