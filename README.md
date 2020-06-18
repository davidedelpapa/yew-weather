# YEW Weather

[![Built With: RUST](https://img.shields.io/badge/Built%20With-RUST-lightgrey)](https://www.rust-lang.org/) [![WASM: Yew](https://img.shields.io/badge/WASM-Yew-brightgreen)](https://yew.rs/)

Codebase for the [Yew Megatutorial](https://github.com/davidedelpapa/yew-tutorial), part 8. Based on [Yew Development Environment](https://github.com/davidedelpapa/yew-devenv)

## Usage

Just run the `run` script; it should correctly initialize the environment at the first run (both cargo and npm)

```bash
./run
```

Of course, this is intended as code for a [tutorial](https://github.com/davidedelpapa/yew-tutorial), so better to follow it.

### Caveat: Cleanup

Sometimes the hot-reload server or the `thttp` server just stay up, while building fails, etc.
In case of "zombie" servers, try to fix it with `--clean`

```bash
./run --clean
```