//! The `templating` module contains the logic for passing templates
//! and values to templating engines. It abstracts rendering those away.

mod mustache;
pub use self::mustache::Mustache;

use std::fmt::Display;
use std::fmt::Formatter;
use std::error::Error;
use std::collections::HashMap;

/// The trait for integrating templating engines.
pub trait TemplatingEngine {
    fn render_template(
        &self,
        template: &str,
        values: &HashMap<&str, &str>,
    ) -> Result<String, RenderError>;
}

/// A type representing any error that could happen when attempting to render
/// from a template.
#[derive(Debug)]
pub struct RenderError {
    /// The error that caused the rendering failure.
    // We should probably look into error_chain at some point to replace this!
    inner: Box<dyn Error>,
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "template rendering failed: {}", self.inner)?;

        Ok(())
    }
}

impl Error for RenderError {}