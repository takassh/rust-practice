# What is this?
- Run pytorch on rust using [tch](https://github.com/LaurentMazare/tch-rs) crate

# How to run?
You need to pass `DYLD_LIBRARY_PATH` and `LIBTORCH` (`LIBTORCH` is already passed by `settings.json`)
- `DYLD_LIBRARY_PATH=./libtorch/lib:/opt/homebrew/opt/libomp/lib cargo run`