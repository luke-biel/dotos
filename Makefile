MODE ?= debug

ifeq ($(MODE),release)
	CARGO_FLAGS = --release
else
endif

.PHONY: all clean cargo-clean

all: kernel.img

kernel.img: target/dotos.elf
	rust-objcopy --strip-all -O binary target/dotos.elf kernel.img

target/dotos.elf: **/*.rs
	cargo build $(CARGO_FLAGS)

clean: cargo-clean
	rm kernel.img

cargo-clean:
	cargo clean
