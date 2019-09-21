use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::Error;

/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]
///
/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]: https://tools.ietf.org/html/rfc8216#section-4.3.2.6
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtXProgramDateTime(DateTime<FixedOffset>);

impl ExtXProgramDateTime {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PROGRAM-DATE-TIME:";

    /// Makes a new `ExtXProgramDateTime` tag.
    pub fn new<T: Into<DateTime<FixedOffset>>>(date_time: T) -> Self {
        Self(date_time.into())
    }

    /// Returns the date-time of the first sample of the associated media segment.
    pub const fn date_time(&self) -> &DateTime<FixedOffset> {
        &self.0
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXProgramDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date_time = self.0.to_rfc3339();
        write!(f, "{}{}", Self::PREFIX, date_time)
    }
}

impl FromStr for ExtXProgramDateTime {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        // TODO: parse with chrono
        let date_time = DateTime::parse_from_rfc3339(input)?;
        Ok(Self::new(date_time))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let date_time = "2010-02-19T14:54:23.031+08:00"
            .parse::<DateTime<FixedOffset>>()
            .unwrap();

        let program_date_time = ExtXProgramDateTime::new(date_time);

        assert_eq!(
            program_date_time.to_string(),
            "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00".to_string()
        );
    }

    #[test]
    fn test_parser() {
        "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00"
            .parse::<ExtXProgramDateTime>()
            .unwrap();
    }

    #[test]
    fn test_requires_version() {
        let date_time = "2010-02-19T14:54:23.031+08:00"
            .parse::<DateTime<FixedOffset>>()
            .unwrap();

        let program_date_time = ExtXProgramDateTime::new(date_time);

        assert_eq!(program_date_time.requires_version(), ProtocolVersion::V1);
    }
}