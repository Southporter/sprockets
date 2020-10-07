use log::info;
use pretty_env_logger;
mod args;

fn main() {
    pretty_env_logger::init();

    let config = args::parse_args();
    info!(
        "Connection to {} on port {} with protocol {}",
        config.address.ip(),
        config.address.port(),
        config.socket,
    )
}
