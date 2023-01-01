# bgos

Bulgarian-themed Operating System -- just for fun.

Started on 22-12-31.

## Dev Workflow

### Format

`cargo fmt`

### Build

`cargo build`

### Produce Bootable Image

`cargo bootimage`

### Run

Emulate via QEMU:

`qemu-system-x86_64 -drive format=raw,file=target/amd64-bgos/debug/bootimage-bgos.bin`
