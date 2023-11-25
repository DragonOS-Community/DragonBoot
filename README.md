# DragonBoot

A stage2 UEFI bootloader of DragonOS in pure Rust.

--- 

## 功能

- [ ] 从UEFI启动DragonBoot
- [ ] 显示启动菜单
- [ ] 从磁盘启动DragonOS
- [ ] 启动配置
- [ ] 平坦设备树解析

## 目标架构

- [x] riscv64

## 关于DragonBoot

由于目前Risc-V上，许多操作系统都是要把DTB编译进内核，导致操作系统无法作为一个与开发板无关的二进制文件进行传播，因此DragonBoot的目标是，作为一个第二阶段的引导加载程序，加载DragonOS内核，并把uboot传来的平坦设备树传递给内核。

## Maintainer

- longjin <longjin@dragonos.org>

## License

DragonBoot is licensed under the GPLv2 License. See [LICENSE](LICENSE) for details.
