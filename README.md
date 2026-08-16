# tlpm-rs
Safe Rust FFI bindings for Thorlabs PM-series optical power meters.

This crate provides a comprehensive, domain-driven Rust wrapper around the proprietary Thorlabs C-API (`TLPMX.h`). It enables programmatic control, automated data acquisition, array/burst measurements, and hardware-triggered sequences for compatible Thorlabs instruments.

## System Requirements
Since this is a `-sys` style FFI wrapper, it relies on proprietary Thorlabs binaries to compile and execute.
1. **Thorlabs Software**: You must install the official [Thorlabs Optical Parameter Monitor](https://www.thorlabs.com/software-pages/OPM) software on your machine. This installs the required VISA drivers, headers, and compiled libraries.
2. **Platform Restrictions**: Building this crate is currently restricted to **Windows**. The build script dynamically links against the proprietary `TLPMX_64.lib` library provided by Thorlabs. Running `cargo check` will work on UNIX systems for development purposes, but `cargo build` will fail.

## Environment Variables & Configuration
The crate's `build.rs` script needs to locate the Thorlabs C header (`TLPMX.h`) and the corresponding 64-bit library directory. By default, the build script looks in the standard IVI Foundation installation path:
```
C:\Program Files\IVI Foundation\VISA\Win64\Include
```
If you have installed the Thorlabs software in a custom location, you **must** set the `TLPM_DIR` environment variable to point to the directory containing `TLPMX.h`.

*Note: The build script assumes that the library file (`TLPMX_64.lib`) is located in a `Lib_x64\msc` folder one level up from your specified include directory.*

## Disclaimer: Experimental & LLM-Assisted
**Please read before using this crate in a production laboratory environment.**
- **Untested Functionality**: While the core API logic and module architecture are in place, the vast majority of the functions in this crate remain untested on physical Thorlabs hardware.
- **LLM Generation**: A Large Language Model (LLM) was heavily utilized to generate the safe Rust wrapper methods, macros, and FFI bindings across the massive surface area of the Thorlabs C-API.
- **Potential Errors**: Due to the automated nature of the generation and the lack of comprehensive hardware tests, there may be undiscovered mistakes. This includes potential pointer mismanagement in array measurements, incorrect FFI length assumptions, or improperly mapped VISA status codes.

### Found a Bug?
If you encounter a segmentation fault, unexpected hardware behaviour, or a mapping error, please [open an issue](../../issues) on GitHub. Include the specific function that failed, the expected behaviour, and any relevant error codes or crash logs. Pull requests with fixes are highly encouraged!

## License
This projeect is dual-licensed under the MIT and Apache 2.0 licenses. See the [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) files for more details.

