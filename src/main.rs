use log::{info, warn};
use pretty_env_logger;

mod args;
mod repl;
mod tcp;

use args::Socket;
use repl::Interactor;

fn main() {
    pretty_env_logger::init();

    let config = args::parse_args();
    info!(
        "Connection to {} on port {} with protocol {}",
        config.address.ip(),
        config.address.port(),
        config.socket,
    );

    let mut interactor = Interactor::standard();

    let res = match config.socket {
        Socket::TCP => tcp::start_tcp(config, &mut interactor),
        _ => {
            warn!("{} Protocol not implemented.", config.socket);
            Ok(())
        }
    };

    match res {
        Ok(_) => println!("Goodbye"),
        Err(err) => println!("Error occured: {}", err),
    }
}
