pub mod test_case;

use crate::test_case::{TestCase, TestCaseSpeed};
use criterion::{black_box, BenchmarkId, Criterion, Throughput};
use ruff::linter::lint_only;
use ruff::settings::{flags, Settings};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
pub enum Tool {
    Linter,
}

impl Tool {
    pub fn all() -> impl Iterator<Item = Tool> {
        [Tool::Linter].into_iter()
    }

    pub fn benchmark(&self, criterion: &mut Criterion, cases: &[TestCase]) {
        match self {
            Tool::Linter => benchmark_linter(criterion, cases),
        }
    }
}

fn benchmark_linter(criterion: &mut Criterion, cases: &[TestCase]) {
    let mut group = criterion.benchmark_group("linter");

    for case in cases {
        group.throughput(Throughput::Bytes(case.code().len() as u64));
        group.measurement_time(match case.speed() {
            TestCaseSpeed::Fast => Duration::from_secs(5),
            TestCaseSpeed::Normal => Duration::from_secs(10),
            TestCaseSpeed::Slow => Duration::from_secs(20),
        });
        group.bench_with_input(BenchmarkId::from_parameter(case.name()), case, |b, case| {
            b.iter(|| {
                lint_only(
                    case.code(),
                    &case.path(),
                    None,
                    &black_box(Settings::default()),
                    flags::Noqa::Enabled,
                    flags::Autofix::Enabled,
                )
            });
        });
    }

    group.finish();
}

#[derive(Debug)]
pub struct UnknownToolError {
    name: String,
}

impl Display for UnknownToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown Tool '{}'. Valid tools are: 'linter'", self.name)
    }
}

impl std::error::Error for UnknownToolError {}

impl FromStr for Tool {
    type Err = UnknownToolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tool = match s {
            "linter" => Tool::Linter,
            _ => {
                return Err(UnknownToolError {
                    name: s.to_string(),
                })
            }
        };

        Ok(tool)
    }
}
