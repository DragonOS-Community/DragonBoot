# DragonBoot

尝试在 DragonOS 上实现 LinuxBoot 所做的事~

--- 

## 开发情况

- [x] x86_64架构支持
- [x] u-root 根文件系统支持
- [x] u-root 使用 kexec 切换到 Linux 内核
- [ ] 多架构支持
- [ ] 更完善的 kexec 执行流程
- [ ] 更多的 kexec 系列系统调用, 如 kexec_file_load()

## 项目 release 成果

当前此项目只在一个环境在测试通过, 测试日期为 2025.10.20, 如未在下方标注特定版本，则为默认测试日期时的最新版本。

测试使用的特定软件版本为：

- OS: Ubuntu 22.04
- go: 1.24.6 amd64
- Linux kernel(for kexec): 6.15.4

## 项目使用流程

### 基本运行环境

项目构建需要基本的运行环境, 一般来说, DragonOS 的构建环境可以基本满足此项目的构建要求, 如遇到缺少软件包等情况, 使用 apt install 安装即可。

### Go 环境支持

项目需要 Go 语言环境支持, 你可以通过命令检查是否有 Go 语言环境: 

```bash
go version
```

如果没有获取到版本, 可以使用下面的脚本安装, 仅支持 x86_64 架构, 版本固定为已经过测试的 1.24.6:

```bash
sudo ./go_install.sh
```

安装完成后可以使用 `go version` 查看是否可以正常找到.

> 请注意查看[项目目前存在的 BUG 与暂时性的补丁中 go runtime 的更改](#go-runtime)，对 go 源码做一定的更改。

### Linux 构建

为了能够运行 kexec 功能, 我们需要一个内核, 这里选用 Linux 6.15.4, 当然, 其他版本也是可以的, 如果你没有现成的编译好的内核, 可以跟随下面的步骤构建:

首先拉取源码并解压:

```bash
wget https://www.kernel.org/pub/linux/kernel/v6.x/linux-6.15.4.tar.xz
tar -xf linux-6.15.4.tar.xz
```

这里使用默认内核配置, 开始编译:

```bash
cd linux-6.15.4
make defconfig
make -j12
```

我们需要的内核位于`arch/x86/boot/bzImage`, 这是待会包括进 initram 的内核文件.

### u-root 安装

我们需要安装 u-root 工具, 首先配置环境变量:

```bash
mkdir go
export GOPATH="{path to pwd}/go"
export PATH="$PATH:$GOPATH/bin"
```

随后下载并安装 u-root:

```bash
git clone https://github.com/u-root/u-root
cd u-root
go install
```

> 请注意查看[项目目前存在的 BUG 与暂时性的补丁中 u-root 的更改](#u-root)，对 u-root 源码做一定的更改。

随后在 `u-root` 目录下执行构建命令:

```bash
u-root -uinitcmd="kexec --loadsyscall ./bzImage" -files {path to bzImage}:./bzImage core
```

现在我们得到了一个 `.cpio` 文件, DragonOS 支持 xz 压缩的 cpio 格式, 我们再对其进行压缩:

```bash
xz --check=crc32 --lzma2=dict=512KiB /tmp/initramfs.linux_amd64.cpio
```

### 配置并启动内核

确保你能够构建并运行 x86 架构的 DragonOS, 详细过程见对应仓库.

首先打开内核的配置, 在 cargo.toml 中启用 `initram` 的 feature(可以把他加到 default 中).

随后将刚刚的压缩文件移到 `DragonOS/kernel/initram` 目录下, 并使用架构重命名:

```bash
cp /tmp/initramfs.linux_amd64.cpio.xz ~/DragonOS/kernel/initram/x86.cpio.xz
```

此时即可使用 `make build` 构建内核.

随后修改 `tools/run-qemu.sh` 脚本, 在 243 行取消多核启动参数 `-smp ${QEMU_SMP}`, 当前内核还未适配多 cpu 核心的情况.

所有事情执行完成之后, 可以通过命令 `make qemu-nographic` 启动了.

## 项目目前存在的 BUG 与暂时性的补丁

### go runtime

目前 DragonOS 对 netpoll_epoll 的支持尚不完善，不能正常实现系统调用的功能，导致 go runtime 的 poll 线程会抛出错误，使得整个进程结束。

需要对 go 源码做一定的修改，在 go 的源码目录(如果你使用的是 `go_install.sh` 安装的, 那么在 `/usr/lib/go-1.24.6`)中，修改 `src/runtime/netpoll_epoll.go`:

```go
// Line 122

if errno != _EINTR {
    //      println("runtime: epollwait on fd", epfd, "failed with", errno)
    //      throw("runtime: netpoll failed")
}
```

在 122 行注释掉抛出错误, 最好添加一行 `return gList{}, 0` 来返回暂未准备好的结果让线程休眠。

### u-root

因为 Linux 对根文件系统的初始化是由根文件系统中的 `init` 来完成的，如挂载 `dev/` `sys/` 等文件夹。但是 DragonOS 默认在进入用户态之前就会自己挂载这些文件夹初始化，所以二者存在一定的行为差距。

u-root 的 init 程序会执行挂载和初始化 rootfs 的部分，这对 Linux 来说是没问题且必须的，但 DragonOS 并不需要且目前 DragonOS 也不支持这些功能。

因此需要对 u-root 的源码进行更改，注释掉此部分，具体在 u-root 源码目录的 `cmds/core/init/init.go`:

```go
// Line 58

//libinit.SetEnv()
//libinit.CreateRootfs()
libinit.NetInit()
```

在 58 行注释掉 `libinit.SetEnv()` 和 `libinit.CreateRootfs()`。
