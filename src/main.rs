use serde_derive::Deserialize;
use std::{
    fs::read_to_string,
    io::Result,
    net::{SocketAddr, UdpSocket},
    process::exit,
};
use structopt::StructOpt;
use tracing::{error, info, trace, warn, Level};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    config: String,

    #[structopt(short, long, default_value = "info")]
    log_level: Level,
}

#[derive(Deserialize)]
struct Config {
    bind_address: SocketAddr,
    targets: Vec<SocketAddr>,
}

fn main() {
    if let Err(e) = run() {
        error!("{}", e);
        exit(1)
    }
}

fn run() -> Result<()> {
    // Parse command line args
    let opt = Opt::from_args();

    // Init logging
    tracing_subscriber::fmt()
        .with_max_level(opt.log_level)
        .init();

    let config: Config = toml::from_str(&read_to_string(opt.config)?)?;

    // Bind to address
    info!("binding to {}", config.bind_address);
    let socket = UdpSocket::bind(&config.bind_address)?;

    let mut target_sockets = vec![];
    for addr in config.targets {
        let target_socket = UdpSocket::bind("0.0.0.0:0")?;
        trace!(
            "connecting from {} to {}",
            target_socket.local_addr()?,
            addr
        );
        target_socket.connect(addr)?;
        target_sockets.push(target_socket);
    }

    // This buffer is the largest size of a single UDP packet
    let mut buf = [0; 1024];

    loop {
        // Wait for a message
        let (len, _) = match socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };
        let payload = &buf[..len];

        // Send payload
        for target_socket in target_sockets.iter() {
            if let Err(e) = target_socket.send(payload) {
                warn!("failed to send message: {}", e);
            }
        }
    }
}
