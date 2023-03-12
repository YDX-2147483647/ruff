use criterion::Criterion;
use ruff_benchmark::test_case::{TestCase, TestFile, TestFileDownloadError};
use ruff_benchmark::Tool;
use std::ffi::OsString;
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

#[allow(clippy::print_stderr)]
fn main() -> Result<(), Error> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("--help") {
        eprintln!(
            r#"cargo benchmark
Ruff Microbenchmarks
Usage:
    cargo benchmark [options]
OPTIONS:
    --tool=<linter>             The tool to benchmark.
    --save-baseline=<name>      Names an explicit baseline and enables overwriting the previous results.
    --retain-baseline=<name>    Names an explicit baseline and disables overwriting the previous results.
    --filter=<name>             Only runs test cases that match filter
    --help                      Prints this help
        "#
        );
        return Ok(());
    }

    let mut tools = Vec::new();

    while let Some(tool) = args.opt_value_from_str::<_, Tool>("--tool")? {
        tools.push(tool);
    }

    if tools.is_empty() {
        tools.extend(Tool::all());
    }

    let mut criterion = Criterion::default()
        .without_plots()
        .with_output_color(supports_color::on(supports_color::Stream::Stdout).is_some());
    if let Some(baseline) = args.opt_value_from_str("--save-baseline")? {
        criterion = criterion.save_baseline(baseline);
    }

    if let Some(retain_baseline) = args.opt_value_from_str("--retain-baseline")? {
        criterion = criterion.retain_baseline(retain_baseline, true);
    }

    let filter: Option<String> = args.opt_value_from_str("--filter")?;

    let remaining = args.finish();
    if !remaining.is_empty() {
        return Err(Error::Unsupported(remaining));
    }

    let mut test_cases = create_test_cases()?;

    if let Some(filter) = filter {
        test_cases.retain(|case| case.name().contains(&filter))
    };

    for tool in tools {
        tool.benchmark(&mut criterion, &test_cases);
    }

    criterion.final_summary();

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Download(TestFileDownloadError),
    Args(pico_args::Error),
    Unsupported(Vec<OsString>),
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
            Error::Unsupported(arguments) => {
                write!(f, "Unsupported arguments: '{arguments:?}'")
            }
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
