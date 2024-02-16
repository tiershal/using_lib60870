use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use lib60870_sys::{
    cs104::ConnectionBuilder, CS101_CauseOfTransmission, CommonAddr, QualifierOfInterrogation,
    Timestamp, IEC_60870_5_104_DEFAULT_PORT,
};

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Args::parse();

    let (ip, port) = (
        cfg.ip.unwrap_or("localhost".into()),
        cfg.port.unwrap_or(IEC_60870_5_104_DEFAULT_PORT as u16),
    );
    println!("Connecting to {}:{}", ip, port);

    let connection =
        ConnectionBuilder::new(IpAddr::V4(Ipv4Addr::from_str(&ip).expect("Invalid IP")))
            .with_port(port)
            .with_local_ip(IpAddr::V4(
                Ipv4Addr::from_str(&cfg.local_ip.unwrap_or("localhost".to_string()))
                    .expect("Invalid local IP"),
            ))
            .with_local_port(
                cfg.local_port
                    .unwrap_or(IEC_60870_5_104_DEFAULT_PORT as u16),
            )
            .build();

    let connection = connection.connect().map_err(|e| {
        eprintln!("Could not connect: {:?}", e);
        e
    })?;

    println!("Connected");

    connection.start_transmission();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // Send interrogation command
    connection.send_interrogation_command(
        CS101_CauseOfTransmission::CS101_COT_ACTIVATION,
        QualifierOfInterrogation::Station,
        CommonAddr(1),
    );

    std::thread::sleep(std::time::Duration::from_secs(5));

    // Send clock synchronization command
    connection.send_test_command_with_timestamp(0x4938, Timestamp::now_ms(), CommonAddr(1));

    connection.stop_transmission();

    println!("Wait...");

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = "localhost")]
    ip: Option<String>,
    #[clap(short, long)]
    port: Option<u16>,
    #[clap(long)]
    local_ip: Option<String>,
    #[clap(long)]
    local_port: Option<u16>,
}
