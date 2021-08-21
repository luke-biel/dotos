export RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=-Tsrc/bsp/rpi3/link.ld -C link-arg=-otarget/kernel.elf"

cargo rustc --target=aarch64-unknown-none-softfloat --release --features=rpi3 --no-default-features

rust-objcopy -O binary --strip-all target/kernel.elf kernel8.img
