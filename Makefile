.PHONY: all clean

# 检查是否需要进行fmt --check
# 解析命令行参数  
FMT_CHECK?=0

ifeq ($(FMT_CHECK), 1)
	FMT_CHECK=--check
else
	FMT_CHECK=
endif

export ARCH ?= riscv64

all:
ifeq ($(ARCH), riscv64)
	$(MAKE) riscv64imac
else
	@echo "ARCH=$(ARCH) is not supported"
	@exit 1
endif

riscv64imac:
	@cargo build --release --target riscv64imac-unknown-none-elf

clean:
	@cargo clean


fmt:
	@cargo fmt --all $(FMT_CHECK)
