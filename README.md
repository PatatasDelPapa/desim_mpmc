# desim_mpmc
A benchmark of a model of multiple producers and consumers with bounded capacity using the desim library.

## Requirements
The desim library requires a nightly compiler to set it you can use `rustup override set nightly`

## Running the benchmark
You can run the benchmark using either `cargo bench` or `cargo criterion` the former does not require any configuraction but the later is recommended. To use `cargo criterion` first you have to install it
```
cargo install cargo-criterion`
```
Then you can use the command to run the benchmark.

### Known problems
I couldn't get `cargo criterion` to work on my machine, a windows 10 notebook, but with WSL2 with Ubuntu 20.04 it works fine
