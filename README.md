# `probe-rs` Flash Algorithm for TMS570

This repository represents a flash algorithm to support programming a TMS570 device.

## Compiling

To build this repository, you will need to download the F021 flash API from Texas Instruments. Then place the following file in the root of this repository:

* `F021_API_CortexR4_BE_L2FMC_NDS.lib`

Then, build with either:

* cargo build --release
* cargo build

Finally, generate the `tms570lc4357.yaml` file with:

* target-gen test template.yaml tms570lc4357.yaml target/armebv7r-none-eabi/release/tms570

## Using

To use this file with `probe-rs`, specify this chip description, along with the correct chip name. For example:

* probe-rs read --chip TMS570LC4357 --chip-description-path ./tms570lc4357.yaml b32 0x1000 4

## Notes on Performance

Performance of the debug bridge can be improved. However, one easy fix you can make is to use a tool such as [turbo-110](https://github.com/xobs/turbo-110) to switch your probe into CMSIS-DAP 2.0 mode, providing a 20x speedup in JTAG performance.

Other fixes are ongoing within `probe-rs` itself.
