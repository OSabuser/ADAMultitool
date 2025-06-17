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

fn main() {
    env_logger::init();

    log::trace!("Hello, world!");
    log::debug!("Hello, world!");
    log::info!("Hello, world!");
    log::warn!("Hello, world!");
    log::error!("Hello, world!");
}
