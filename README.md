# File Tagging

A project for file tagging, consisting of:

- A Rust library
- Rust bindings to Python, using PyO3
- A Python GUI app, using PyQT, that utilizes the Rust library, providing a visual method for interacting with file tags
- A Rust CLI app that utilizes the Rust library, providing a terminal-based method for interacting with file tags


## Develop install

- Clone the repo
- Create a python virtual environment
- Install the following `pip` packages inside the virtual environment:
    - maturin
    - pyinstall
    - PySide6
    - pyinstaller (only for packaging to an executable)

Running `cargo build` in the root directory will build:
- The Rust library
- The CLI app

To create the Python bindings, run `maturin develop` inside of `tagbind_py`. This will also install the bindings (as `rs_tags`) in the virtual environment

