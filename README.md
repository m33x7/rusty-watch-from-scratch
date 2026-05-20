# rusty-watch-from-scratch

### Specs:
ESP32-S3R2 - The SoC with WiFi and Bluetooth, up to 240MHz operating frequency, with onboard 2MB PSRAM (ESP32-S3 182025 R2MTL341000)

### Tools used:
esp-generate, esp-flash, xtensa-esp32s3-elf-gdb

### Useful links:
ESP32 S3 Datasheet:
https://documentation.espressif.com/esp32-s3_datasheet_en.html <br/>
Waveshare WIKI for this board : 
https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.28 <br/>
Board schema:
https://files.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.28/ESP32-S3-Touch-LCD-1.28-Sch.pdf
Battery charger ETA6098:
https://www.eta-semi.com/wp-content/uploads/2022/03/ETA6098_V1.1.pdf

### Useful commans:
```. $HOME/export-esp.sh``` - to source the espup

```
rustup show
rustup override set esp
```

```
espflash flash target/xtensa-esp32s3-espidf/debug/rusty-watch-from-scratch
espflash monitor
```

### To forward USB from Host (Windows) to WSL :
```
# on Host :
usbipd list
usbipd attach --busid 2-4 --wsl

# then on WSL :
lsusb
ls /dev/ttyACM0
sudo chmod 666 /dev/ttyACM0
```

### Install xtensa-esp-elf-gdb
```
wget https://github.com/espressif/binutils-gdb/releases/download/esp-gdb-v16.3_20250913/xtensa-esp-elf-gdb-16.3_20250913-x86_64-linux-gnu.tar.gz
tar -xzf xtensa-esp-elf-gdb-16.3_20250913-x86_64-linux-gnu.tar.gz
xtensa-esp32s3-elf-gdb
```