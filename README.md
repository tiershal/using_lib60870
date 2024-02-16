Using lib60870 in a Rust project
---

This is a very rudimentary example of how to build the lib60870 library in a Rust project as a -sys crate.

NOTE: This example is not complete and is not intended to be used in a production environment.

## Requirements

### Clang

The bindgen crate requires this to be installed on the host system.

### CMake

The lib60870-sys build process requires CMake to be installed on the host system.

### lib60870

https://github.com/mz-automation/lib60870

This library is added as a submodule within the `lib60870-sys` folder.

This project was tested against the `v2.3.2` tag release of the project.
