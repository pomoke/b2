#!/bin/sh
cargo b --target=x86_64-unknown-uefi -p b2 && \
cp target/x86_64-unknown-uefi/debug/b2.efi esp/efi/boot/bootx64.efi && \
qemu-system-x86_64 -accel kvm -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_VARS.fd  -drive format=raw,file=fat:rw:esp -m 1024M
