fn main() {
    let mut frame = protocol::mu_frame::MUFrame::new();
    frame.set_data(b"Hello, server!\n".to_vec()).unwrap();
    println!("{}", frame);
}
