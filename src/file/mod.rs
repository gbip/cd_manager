pub mod flac;

/// This module is intended to contain everything related to files.
use std::collections::HashMap;

/// This type represent tags.
pub type Tags = HashMap<String, String>;

/// This trait define what an AudioFile is.
pub trait AudioFile {
    fn get_tags(&self) -> Result<Tags, &str>;
}
