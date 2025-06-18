// Assumes the binary is main:

// $ RUST_LOG=error ./main
// [2017-11-09T02:12:24Z ERROR main] this is printed by default
// $ RUST_LOG=info ./main
// [2017-11-09T02:12:24Z ERROR main] this is printed by default
// [2017-11-09T02:12:24Z INFO main] the answer was: 12
// $ RUST_LOG=debug ./main
// [2017-11-09T02:12:24Z DEBUG main] this is a debug message
// [2017-11-09T02:12:24Z ERROR main] this is printed by default
// [2017-11-09T02:12:24Z INFO main] the answer was: 12

use misc::serial_port::SerialInterface;
use log::{debug, error, info, trace, warn};
use std::thread;
use protocol::mu_frame::MUFrame;

fn main() {
    env_logger::init();

    let mut port_instance = SerialInterface::new("/dev/ttyAMA0", 9600, std::time::Duration::from_millis(4500));

    let mut frame = MUFrame::new();
    frame.set_data(b"get loadcapacity\n".to_vec()).unwrap();

    let bytes = frame.serialize();

    log::debug!("Sent: {:?}", bytes);
 
    port_instance.write_data(&bytes).unwrap();

    thread::sleep(std::time::Duration::from_millis(250));

    let mut read_buffer = [0; 256];

    port_instance.read_data(&mut read_buffer).unwrap();

    let binding = String::from_utf8_lossy(&read_buffer);
    let composed_string = binding.trim_matches(|c: char| (c.is_whitespace() || c.is_control()));
    log::debug!("Composed string: {:?}",composed_string);

    // let mut port_instance = SerialInterface::new("/dev/ttyAMA0", 9600, std::time::Duration::from_millis(4500));

    // port_instance.write_data(b"get loadcapacity\n").unwrap();

    // thread::sleep(std::time::Duration::from_millis(150));

    // let mut read_buffer = [0; 256];

    // port_instance.read_data(&mut read_buffer).unwrap();

    // let binding = String::from_utf8_lossy(&read_buffer);
    // let composed_string = binding.trim_matches(|c: char| (c.is_whitespace() || c.is_control()));
    // log::debug!("Composed string: {:?}",composed_string);

    // let message = composed_string.split("\n\r\n").collect::<Vec<&str>>()[0];
    // let device_name = composed_string.split("\n\r\n").collect::<Vec<&str>>()[1];

    // log::debug!("Message: {:?}, Device name: {:?}",message, device_name);
    
}
