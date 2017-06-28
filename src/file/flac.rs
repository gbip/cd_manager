use flac::metadata::get_vorbis_comment;
use file::{Tags, AudioFile};

/// This struct represent a flac file.
pub struct Flac {
    filename: String,
}

impl Flac {
    fn new(filename: String) -> Flac {
        Flac { filename: filename }
    }
}

impl AudioFile for Flac {
    fn get_tags(&self) -> Result<Tags, &str> {
        match get_vorbis_comment(&self.filename) {
            Ok(vorbis_comments) => Ok(vorbis_comments.comments),
            Err(_) => Err("Error while reading flac metadata"),
        }
    }
}
