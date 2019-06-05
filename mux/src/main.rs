use serial::SerialPort;
use std::env;
use std::net::UdpSocket;
use std::io::Write;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("Please provide a serial port as argument (ex: /dev/ttyACM0)");
    }
    let serial_path = &args[0];

    println!("Opening port: {:?}", serial_path);
    let mut port = serial::open(&serial_path).unwrap();
    port.configure(&SETTINGS).unwrap();
    // TODO: set timeout?

    let udp = UdpSocket::bind("127.0.0.1:49161")?;

    loop {
        let mut buf = [0; 10];
        let (amt, _src) = udp.recv_from(&mut buf)?;
        let cmd = String::from_utf8_lossy(&buf[0..amt]).to_owned();

        println!("received: {}", cmd);

        write!(port, "{}", cmd)?;
    }
}
