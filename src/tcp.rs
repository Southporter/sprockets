use crate::args::Config;
use crate::repl::{self, Interactor};
use std::io::{BufRead, Write};
use std::net::TcpStream;

pub fn start_tcp<R, W>(config: Config, interactor: &mut Interactor<R, W>) -> std::io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut conn = TcpStream::connect(config.address)?;

    repl::start_repl(&mut conn, interactor)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{Config, Socket};
    use std::io::{BufReader, Cursor};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
    use std::panic;

    fn get_tcp_config(port: u16) -> Config {
        Config {
            socket: Socket::TCP,
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port),
        }
    }

    fn setup(port: u16) -> TcpListener {
        TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap()
    }

    #[test]
    fn quits() {
        let port = 11012u16;
        let listener = setup(port);
        let reader = BufReader::new("quit".as_bytes());
        let writer = Cursor::new(vec![]);
        let mut interactor = Interactor::new(reader, writer);
        let res = start_tcp(get_tcp_config(port), &mut interactor);
        let (stream, addr) = listener.accept().unwrap();
        println!("Stream {:?}, addr: {:?}", stream, addr);
        assert!(res.is_ok());
    }
    // #[test]
    // fn echos_to_socket() {
    //     let port = 11013u16;
    //     let listener = setup(port);

    //     let reader = Cursor::new()
    // }
}
