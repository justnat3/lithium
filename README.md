# Lithium

## What is this?

This is a rust x86_64 kernel that I am building purely for fun. Its not really meant to be 
anything too exciting. I have always been quite interested in digging through kernel internals
so I decided to make my own for experimentation.

## Building

You will need to use rust nightly, if you are using rustup(which if you are not, you should)

```sh
rustup default nightly
rustup toolchain install nightly --force nightly-2021-03-24
```

You will also need to install bootimage 

```
cargo install bootimage
```

Then you will be able to build the kernel

```
cargo bootimage --target x86_64-lithium.json 
```

## Running The Kernel

To run the kernel you will need qemu installed, then you can run

```
./bootloader/run.sh
```
