# bgos

Bulgarian-themed Operating System -- just for fun.

Started on 22-12-31.

## Dev Workflow

For OS emulation, install [QEMU](https://www.qemu.org/) and add it to PATH if necessary. Afterwards, ensure to restart your terminals / IDEs to refresh their environment variables.

### Format

`cargo fmt`

### Build

`cargo build`

### Produce Bootable Image

`cargo bootimage`

### Run (via QEMU)

`cargo run`

### Unit Test (via QEMU)

`cargo test`
