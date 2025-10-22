#!/bin/bash

echo "开始下载二进制包...."

wget https://dl.google.com/go/go1.24.6.linux-amd64.tar.gz

echo "开始解压二进制包...."

tar -xzf go1.24.6.linux-amd64.tar.gz

echo "开始安装到/usr/lib目录...."

sudo mv go /usr/lib/go-1.24.6

cd /usr/lib

sudo rm go

sudo ln -s go-1.24.6 go

cd /usr/bin

sudo rm go

sudo ln -s ../lib/go-1.24.6/bin/go go

echo "安装完成!"
