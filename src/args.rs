use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use log::info;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub enum Socket {
    TCP,
    UDP,
    UNIX,
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Socket::TCP => "TCP",
            Socket::UDP => "UDP",
            Socket::UNIX => "Unix",
        };
        write!(f, "{}", display)
    }
}

#[derive(Debug)]
pub struct Config {
    pub socket: Socket,
    pub address: SocketAddr,
}

pub fn parse_args() -> Config {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("ADDRESS")
                .help("IPv4 or IPv6 address to connect to.")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PORT")
                .required(true)
                .takes_value(true)
                .index(2),
        )
        .arg(
            Arg::with_name("protocol")
                .help("Protocol used to connect to the address and port. One of TCP, UDP, or UNIX"),
        )
        .get_matches();

    Config {
        socket: Socket::TCP,
        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9874),
    }
}
