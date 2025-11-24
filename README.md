# File Tagging

A project for file tagging, consisting of:

- A Rust library
- Rust bindings to Python, using PyO3
- A Python GUI app, using PyQT, that utilizes the Rust library, providing a visual method for interacting with file tags
- A Rust CLI app that utilizes the Rust library, providing a terminal-based method for interacting with file tags


## Develop install

- Clone the repo
- Create a python virtual environment
- Install the following `pip` packages:
    - maturin
    - pyinstall
    - PySide6

To create the bindings in tagbinding_py, run `maturin develop`.
