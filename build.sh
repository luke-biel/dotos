#!/usr/bin/env sh

cargo build --release
rust-objcopy -O binary --strip-all target/dotos.elf kernel8.img

if [ "$MOUNT" = "ON" ] && [ -n "$MOUNT_DEVICE" ]; then
  sudo mkdir -p /media/usb
  sudo mount "${MOUNT_DEVICE}" /media/usb
  sudo cp kernel8.img /media/usb/kernel8.img
  sudo umount /media/usb
fi
