use derive_builder::Builder;
use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Indicate the number of skipped Media Segments [`MediaPlaylist`].
///
/// It applies to the entire [`MediaPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord, Builder)]
pub struct ExtXSkip {
    skipped_segments: u32,
}

impl ExtXSkip {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SKIP:";

    pub fn builder() -> ExtXSkipBuilder { ExtXSkipBuilder::default() }
}

/// This tag requires [`ProtocolVersion::V8`].
impl RequiredVersion for ExtXSkip {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V8 }
}

impl fmt::Display for ExtXSkip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut directives: Vec<String> = vec![];
        directives.push(format!("SKIPPED-SEGMENTS={}", self.skipped_segments));
        write!(f, "{}{}", Self::PREFIX, directives.join(","))
    }
}

impl TryFrom<&str> for ExtXSkip {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;
        Err(Error::custom(format!("Unsupported: {:?}", input)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        let control = ExtXSkip::builder().skipped_segments(12).build().unwrap();
        assert_eq!(
            "#EXT-X-SKIP:SKIPPED-SEGMENTS=12".to_string(),
            control.to_string(),
        );
    }
}
