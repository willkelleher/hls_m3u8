use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

use shorthand::ShortHand;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Specifies the path to a prefetch segment.
#[derive(ShortHand, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[shorthand(enable(must_use, into))]
pub struct ExtXPrefetch<'a> {
    uri: Cow<'a, str>,
}

impl<'a> ExtXPrefetch<'a> {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PREFETCH:";

    /// Makes a new [`ExtXPrefetch`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXPrefetch;
    /// let prefetch = ExtXPrefetch::new("https://path/to/segment.ts");
    /// ```
    #[must_use]
    pub fn new<T: Into<Cow<'a, str>>>(uri: T) -> Self {
        Self{
            uri: uri.into(),
        }
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtXPrefetch<'static> {
        ExtXPrefetch {
            uri: Cow::Owned(self.uri.into_owned()),
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl<'a> RequiredVersion for ExtXPrefetch<'a> {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl<'a> fmt::Display for ExtXPrefetch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.uri)
    }
}

impl<'a> TryFrom<&'a str> for ExtXPrefetch<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        Ok(Self{uri: Cow::Borrowed(input)})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXPrefetch::new("http://path/to/segment.ts").to_string(),
            "#EXT-X-PREFETCH:http://path/to/segment.ts".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXPrefetch::new("http://path/to/segment.ts").required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXPrefetch::new("http://path/to/segment.ts"),
            ExtXPrefetch::try_from("#EXT-X-PREFETCH:http://path/to/segment.ts").unwrap()
        );
    }
}
