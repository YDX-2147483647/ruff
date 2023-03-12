use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use url::Url;

/// Relative size of a test case. Benchmarks can use it to configure the time for how long a benchmark should run to get stable results.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TestCaseSpeed {
    /// A test case that is fast to run
    Fast,

    /// A normal test case
    Normal,

    /// A slow test case
    Slow,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    file: TestFile,
    speed: TestCaseSpeed,
}

impl TestCase {
    pub fn fast(file: TestFile) -> Self {
        Self {
            file,
            speed: TestCaseSpeed::Fast,
        }
    }

    pub fn normal(file: TestFile) -> Self {
        Self {
            file,
            speed: TestCaseSpeed::Normal,
        }
    }

    pub fn slow(file: TestFile) -> Self {
        Self {
            file,
            speed: TestCaseSpeed::Slow,
        }
    }
}

impl TestCase {
    pub fn code(&self) -> &str {
        &self.file.code
    }

    pub fn name(&self) -> &str {
        &self.file.name
    }

    pub fn speed(&self) -> TestCaseSpeed {
        self.speed
    }

    pub fn path(&self) -> PathBuf {
        Path::new("target").join(self.name())
    }
}

#[derive(Debug, Clone)]
pub struct TestFile {
    name: String,
    code: String,
}

impl TestFile {
    pub fn new(name: String, code: String) -> Self {
        Self { name, code }
    }

    pub fn try_download(name: &str, url: &str) -> Result<TestFile, TestFileDownloadError> {
        let url = Url::parse(url)?;

        let cached_filename = Path::new("target").join(name);

        match std::fs::read_to_string(&cached_filename) {
            Ok(content) => Ok(TestFile::new(name.to_string(), content)),
            Err(_) => {
                // File not yet cached, download and cache it in the target directory
                let response = ureq::get(url.as_str()).call()?;

                let content = response.into_string()?;

                // SAFETY: There's always the `target` directory
                let parent = cached_filename.parent().unwrap();
                if let Err(error) = std::fs::create_dir_all(parent) {
                    eprintln!("Failed to crate the directory for the test case {name}: {error}")
                } else if let Err(error) = std::fs::write(cached_filename, &content) {
                    {
                        eprintln!("Failed to cache test case file downloaded from {url}: {error}")
                    }
                }

                Ok(TestFile::new(name.to_string(), content))
            }
        }
    }
}

#[derive(Debug)]
pub enum TestFileDownloadError {
    UrlParse(url::ParseError),
    Request(ureq::Error),
    Download(std::io::Error),
}

impl From<url::ParseError> for TestFileDownloadError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParse(value)
    }
}

impl From<ureq::Error> for TestFileDownloadError {
    fn from(value: ureq::Error) -> Self {
        Self::Request(value)
    }
}

impl From<std::io::Error> for TestFileDownloadError {
    fn from(value: std::io::Error) -> Self {
        Self::Download(value)
    }
}

impl Display for TestFileDownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFileDownloadError::UrlParse(inner) => {
                write!(f, "Failed to parse url: {inner}")
            }
            TestFileDownloadError::Request(inner) => {
                write!(f, "Failed to download file: {inner}")
            }
            TestFileDownloadError::Download(inner) => {
                write!(f, "Failed to download file: {inner}")
            }
        }
    }
}

impl std::error::Error for TestFileDownloadError {}
