## Ruff Micro-benchmarks

Benchmarks for the different Ruff-tools.

### Run Benchmark

You can run the benchmarks with

```shell
cargo benchmark
```

### Comparing Benchmarks
You can save the benchmark results and compare them using `--save-baseline`. This is useful to prove that changes improve performance compared to `main`.

```shell
# On main
cargo benchmark --save-baseline=main

# After applying your changes
cargo benchmark --save-baseline=pr

critcmp main pr
```

You must install [`critcmp`](https://github.com/BurntSushi/critcmp) for the comparsion.

```bash
cargo install critcmp
```

## Profiling

### Linux

Install `perf` and build `ruff_benchmark` with the `release-debug` profile and then run it with perf

```shell
cargo build --profile=release-debug -p ruff_benchmark && perf record -g -F 999 ./target/release-debug/ruff_benchmark
```

Then convert the recorded profile

```shell
perf script -F +pid > /tmp/test.perf
```

You can now view the converted file with [firefox profiler](https://profiler.firefox.com/)

You can find a more in-depth guide [here](https://profiler.firefox.com/docs/#/./guide-perf-profiling)

### Mac

Use [cargo-instruments](https://crates.io/crates/cargo-instruments) for profiling.
