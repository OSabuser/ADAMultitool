# Графическое меню настроек

> Проект TUI меню для Raspberry Pi Zero 2W. Используется совместно с крейтом `config_utility`

## Кросс-компиляцияs

> [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
> [cross-rs](https://github.com/cross-rs/cross)
  
```bash
cargo zigbuild --target=arm-unknown-linux-gnueabihf.2.28 --release # Cборка 1
cross build --target=arm-unknown-linux-gnueabihf --bin config_utility --release # Cборка 2
scp -P 1996 target/arm-unknown-linux-gnueabihf/release/config_utility pi@192.168.88.87:/home/pi # Передача файла на девайс
serialport = {version = "4.7.2", default-features = false}` # Для кросскомпиляции
```

## Настройки target

> [# lxsession/LXDE-pi/autostart override](https://stackoverflow.com/questions/36466500/on-raspberry-pi-auto-start-terminal-after-login)

```bash
# Запуск приложения
cd /home/pi
export RUST_LOG=warn #Включить дальнейший вывод логов уровня WARN
./config_utility --config=pizero --mode=pull # Загрузка настроек, хранящихся на устройстве
wait
sync
timeout --foreground 90s ./rpi_menu --config=pizero # Запуск меню с таймаутом 90 сек
wait
sync
./config_utility --config=pizero --mode=push # Загрузка измененных (или нет) настроек на устройство
```
