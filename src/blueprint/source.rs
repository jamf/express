use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use tempdir::TempDir;
use git2::Repository;

type DynError = Box<dyn Error>;

pub enum Source {
    Local(PathBuf),
    Remote(RemoteSource),
}

impl Source {
    pub fn new(source: &str) -> Result<Self, DynError> {
        let path = Path::new(source);

        if path.exists() {
            return Ok(Self::local(path)?);
        }
        Ok(Self::remote(source)?)
    }

    fn local(path: impl AsRef<Path>) -> Result<Self, DynError> {
        Ok(Source::Local(path.as_ref().canonicalize()?))
    }

    fn remote(url: &str) -> Result<Self, DynError> {
        let dir = TempDir::new("checked_out_blueprint")?;

        Repository::clone(url, &dir)?;

        Ok(Source::Remote(RemoteSource {
            url: url.to_string(),
            checked_out: dir,
        }))
    }

    /// The local path where the blueprint data can be found for parsing.
    pub fn path(&self) -> &Path {
        use Source::*;

        match self {
            Remote(tmpdir) => tmpdir.path(),
            Local(path)    => &path,
        }
    }

    /// A blueprint locator to be used in the .rendr.yaml file
    /// in the rendered project.
    pub fn to_string(&self, from: impl AsRef<Path>) -> String {
        use Source::*;

        let from = from.as_ref().canonicalize().unwrap();

        match self {
            Local(path) => pathdiff::diff_paths(path, from)
                                .unwrap()
                                .into_os_string()
                                .into_string()
                                .unwrap(),
            Remote(src) => src.url().to_string(),
        }
    }
}

pub struct RemoteSource {
    url: String,
    checked_out: TempDir,
}

impl RemoteSource {
    fn path(&self) -> &Path {
        self.checked_out.path()
    }

    fn url(&self) -> &str {
        &self.url
    }
}

#[test]
fn source_canonicalizes_its_path_on_init() {
    let source = Source::new("test_assets").unwrap();

    assert!(source.path().is_absolute());
}

#[test]
fn source_calculates_relative_path_correctly() {
    let source = Source::new("test_assets/example_blueprint").unwrap();
    let project_dir = ".";

    assert_eq!(source.to_string(project_dir), "test_assets/example_blueprint");
}