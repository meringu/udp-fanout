use std::{
    io::Result,
    net::{SocketAddr, UdpSocket},
};
use structopt::StructOpt;
use tracing::{error, info, trace, Level};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "0.0.0.0:8125")]
    bind_address: SocketAddr,

    #[structopt(short = "t", long)]
    fanout_targets: Vec<SocketAddr>,

    #[structopt(short, long, default_value = "info")]
    log_level: Level,
}

fn main() -> Result<()> {
    // Parse command line args
    let opt = Opt::from_args();

    // Init logging
    tracing_subscriber::fmt()
        .with_max_level(opt.log_level)
        .init();

    // Bind to address
    info!("binding to {}", opt.bind_address);
    let socket = UdpSocket::bind(&opt.bind_address)?;

    let mut target_sockets = vec![];
    for addr in opt.fanout_targets {
        let target_socket = UdpSocket::bind("0.0.0.0:0")?;
        trace!(
            "connecting from {} to {}",
            target_socket.local_addr()?,
            addr
        );
        target_socket.connect(addr)?;
        target_sockets.push(target_socket);
    }

    // The buffer is the largest size of a single UDP packet
    let mut buf = [0; 1024];

    loop {
        // wait for a message
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
            match target_socket.send(payload) {
                Err(e) => error!("failed to send message: {}", e),
                _ => {}
            }
        }
    }
}
