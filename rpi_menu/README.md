# Графическое меню настроек

> Raspberry Pi Zero 2W

## Кросс-компиляция

- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
- Сборка: `cargo zigbuild --target=arm-unknown-linux-gnueabihf.2.28 --release`
- `scp -P 1996 target/arm-unknown-linux-gnueabihf/release/config_utility pi@192.168.88.87:/home/pi`
- `serialport = {version = "4.7.2", default-features = false}`
