#!/bin/bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-lithium/debug/bootimage-lithium.bin
