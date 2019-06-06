use serial::SerialPort;
use std::env;
use std::io::{Read, Write};
use std::net::UdpSocket;

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
        // Receive the command via socket and display it.
        let mut buf = [0; 10];
        let (amt, _src) = udp.recv_from(&mut buf)?;
        let buf = &buf[0..amt];
        println!("received: {}", String::from_utf8_lossy(buf).to_owned());

        let idx = dbg!(base36(dbg!(buf[0]))).unwrap_or(0);
        let color = base36(buf[1]).unwrap_or(0);

        // Write the command into the serial port.
        let out_buf = [idx, color, color, color];

        println!("{:?}", out_buf);
        port.write_all(&out_buf)?;
        port.flush()?;


        // Read data from the serial port and display it.
        let mut read_buf = [0; 10];
        if let Ok(size) = port.read(&mut read_buf) {
            println!("device sent: {:?}", &read_buf[0..size]);
        }
    }
}

/// Takes a character encoded in base64 and returns corresponding numeric value.
fn base36(value: u8) -> Option<u8> {
    // The index of in the transpose table corresponds to the value encoded by the input.
    const TRANSPOSE_TABLE_LOWER: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    const TRANSPOSE_TABLE_UPPER: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    TRANSPOSE_TABLE_LOWER
        .iter()
        .position(|&x| x == value)
        .or_else(|| TRANSPOSE_TABLE_UPPER.iter().position(|&x| x == value))
        .map(|x| x as u8)
}
