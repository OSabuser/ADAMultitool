# ADAMultitool [rpi-branch]

> Набор утилит для работы с MCU по последовательному порту

## Кросс-компиляция

> [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
> [cross-rs](https://github.com/cross-rs/cross)
  
```bash
cargo zigbuild --target=arm-unknown-linux-gnueabihf.2.28 --release # Cборка 1
cross build --target=arm-unknown-linux-gnueabihf --bin config_utility --release # Cборка 2
serialport = {version = "4.7.2", default-features = false}` # Для кросскомпиляции
scp -P 1996 target/arm-unknown-linux-gnueabihf/release/[bin] pi@[ip_address]:/home/pi # Передача файла на девайс
```

## Настройки целевого устройства

> [# lxsession/LXDE-pi/autostart override](https://stackoverflow.com/questions/36466500/on-raspberry-pi-auto-start-terminal-after-login)

```bash
# Запуск приложения
lxterminal --title="ADAMultitool" -e bash -c "path_to_script"

# Содержимое скрипта
cd %ADA_DIR
export RUST_LOG=warn #Включить дальнейший вывод логов уровня WARN
./config_utility --config=pizero --mode=pull # Загрузка настроек, хранящихся на устройстве
wait
sync
timeout --foreground 90s ./rpi_menu --config=pizero # Запуск меню с таймаутом 90 сек
wait
sync
./config_utility --config=pizero --mode=push # Загрузка измененных (или нет) настроек на устройство
```

### config_utility

> Утилита для загрузки настроек индикатора с MCU, расположенного на материнской плате

### rpi_menu

> TUI меню для изменения настроек индикатора
