use criterion::Criterion;
use ruff_benchmark::test_case::{TestCase, TestFile, TestFileDownloadError};
use ruff_benchmark::Tool;
use std::fmt::{Debug, Display, Formatter};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "openbsd"),
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64"
    )
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> Result<(), Error> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("--help") {
        eprintln!(
            r#"cargo benchmark
Ruff Microbenchmarks
Usage:
    cargo benchmark [options]
OPTIONS:
    --tool=<linter>         The tool to benchmark.
    --save-baseline=name    Stores the benchmark results under the given name so that you can compare them with `critcmp`
    --help                  Prints this help
        "#
        );
    }

    let mut tools = Vec::new();

    while let Some(tool) = args.opt_value_from_str::<_, Tool>("--tool")? {
        tools.push(tool);
    }

    if tools.is_empty() {
        tools.extend(Tool::all());
    }

    let mut criterion = Criterion::default().without_plots();
    if let Some(baseline) = args.opt_value_from_str("--save-baseline")? {
        criterion = criterion.save_baseline(baseline);
    }

    let test_cases = create_test_cases()?;

    for tool in tools {
        tool.benchmark(&mut criterion, &test_cases);
    }

    drop(criterion);

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Download(TestFileDownloadError),
    Args(pico_args::Error),
}

impl From<pico_args::Error> for Error {
    fn from(value: pico_args::Error) -> Self {
        Error::Args(value)
    }
}

impl From<TestFileDownloadError> for Error {
    fn from(value: TestFileDownloadError) -> Self {
        Error::Download(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Download(error) => std::fmt::Display::fmt(error, f),
            Error::Args(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {}

fn create_test_cases() -> Result<Vec<TestCase>, TestFileDownloadError> {
    Ok(vec![
        TestCase::fast(TestFile::try_download("numpy/globals.py", "https://github.com/numpy/numpy/blob/89d64415e349ca75a25250f22b874aa16e5c0973/numpy/_globals.py")?),
        TestCase::normal(TestFile::try_download(
            "pydantic/types.py",
            "https://raw.githubusercontent.com/pydantic/pydantic/main/pydantic/types.py",
        )?),
        TestCase::normal(TestFile::try_download("numpy/ctypeslib.py", "https://github.com/numpy/numpy/blob/main/numpy/ctypeslib.py")?),
        TestCase::slow(TestFile::try_download(
            "large/dataset.py",
            "https://raw.githubusercontent.com/DHI/mikeio/b7d26418f4db2909b0aa965253dbe83194d7bb5b/tests/test_dataset.py",
        )?),
    ])
}
