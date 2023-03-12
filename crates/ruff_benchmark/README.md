## Ruff Micro-benchmarks

Benchmarks for the different Ruff-tools.

### Run Benchmark

You can run the benchmarks with

```shell
cargo benchmark
```

### Iterating on improvement

You can use `--save-baseline=<name>` to store an initial baseline benchmark (e.g. on `main`) and then use
`--retain-benchmark=<name>` to compare against that benchmark. Criterion will print a message telling you if the benchmark improved/regressed compared to that baseline.

```shell
# Run once on your "baseline" code
cargo benchmark --save-baseline=main

# Then iterate with
cargo benchmark --retain-baseline=main
```



### Print Comparison
You can use `--save-baseline` and `critcmp` to get a pretty comparison between two recordings.
This is useful to illustrate the improvements of a PR.

```shell
# On main
cargo benchmark --save-baseline=main

# After applying your changes
cargo benchmark --save-baseline=pr

critcmp main pr
```

You must install [`critcmp`](https://github.com/BurntSushi/critcmp) for the comparison.

```bash
cargo install critcmp
```

## Profiling

### Linux

Install `perf` and build `ruff_benchmark` with the `release-debug` profile and then run it with perf

```shell
cargo build --profile=release-debug -p ruff_benchmark && perf record -g -F 9999 ./target/release-debug/ruff_benchmark
```

Then convert the recorded profile

```shell
perf script -F +pid > /tmp/test.perf
```

You can now view the converted file with [firefox profiler](https://profiler.firefox.com/)

You can find a more in-depth guide [here](https://profiler.firefox.com/docs/#/./guide-perf-profiling)

### Mac

Use [cargo-instruments](https://crates.io/crates/cargo-instruments) for profiling.
