.PHONY: all clean fmt clippy qemu run

# 检查是否需要进行fmt --check
# 解析命令行参数  
FMT_CHECK?=0

ifeq ($(FMT_CHECK), 1)
	FMT_CHECK=--check
else
	FMT_CHECK=
endif

export ARCH ?= riscv64

ifeq ($(ARCH), riscv64)
	RUST_TARGET=riscv64imac-unknown-none-elf
else
	@echo "ARCH=$(ARCH) is not supported"
	@exit 1
endif

EFI_OUTPUT_DIR?=output

OBJCOPY_FLAGS=

# OBJCOPY_FLAGS+=-j .header -j .text -j .plt -j .sdata -j .data -j .dynamic -j .dynstr -j .dynsym -j .rel -j .rel.*  -j .rela* -j .reloc -j .reloc* -j .sbss


OBJCOPY_FLAGS+= --output-target=binary

export RUSTFLAGS=-Crelocation-model=pic

ifeq ($(ARCH), riscv64)
	OBJCOPY_FLAGS+= --binary-architecture=riscv
else
	@echo "ARCH=$(ARCH) is not supported"
	@exit 1
endif

all:
	@mkdir -p $(EFI_OUTPUT_DIR)
ifeq ($(ARCH), riscv64)
	$(MAKE) riscv64imac
else
	@echo "ARCH=$(ARCH) is not supported"
	@exit 1
endif

riscv64imac:
	RUSTFLAGS=$(RUSTFLAGS) cargo build --release --target riscv64imac-unknown-none-elf
	rust-objcopy $(OBJCOPY_FLAGS) target/$(RUST_TARGET)/release/dragon_boot $(EFI_OUTPUT_DIR)/dragon_boot-riscv64imac.efi

run:
	@$(MAKE) all || exit 1
	@$(MAKE) qemu

clean:
	@cargo clean


fmt:
	@cargo fmt --all $(FMT_CHECK)

clippy:
	@cargo clippy --all --target $(RUST_TARGET) --all-features


qemu:
	cd tools && ./run-qemu.sh && cd ..
