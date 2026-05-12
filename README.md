# rusty-watch-from-scratch

### Specs:
ESP32-S3R2 - The SoC with WiFi and Bluetooth, up to 240MHz operating frequency, with onboard 2MB PSRAM (ESP32-S3 182025 R2MTL341000)

### Tools used:
esp-generate, esp-flash, 

### Useful links:
ESP32 S3 Datasheet:
https://documentation.espressif.com/esp32-s3_datasheet_en.html
Waveshare WIKI for this board : 
https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.28 <br/>
Board schema:
https://files.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.28/ESP32-S3-Touch-LCD-1.28-Sch.pdf

### Useful commans:
```. $HOME/export-esp.sh``` - to source the espup

```
espflash flash target/xtensa-esp32s3-espidf/debug/rusty-watch-from-scratch
sudo chmod 666 /dev/ttyACM0
espflash monitor /dev/ttyUSB0
```

### To forward USB from Host (Windows) to WSL :
```
# on Host :
usbipd list
usbipd attach --busid 2-4 --wsl

# then on WSL :
lsusb
ls /dev/ttyACM*
```