#!/bin/zsh

cargo bootimage && qemu-system-x86_64 -drive format=raw,file=./target/x86_64-frankos/debug/bootimage-frankos.bin
