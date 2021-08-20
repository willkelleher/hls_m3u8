use derive_builder::Builder;
use std::convert::TryFrom;
use std::fmt;

use crate::types::Float;
use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Indicate support for DeliverDirectives [`MediaPlaylist`].
///
/// It applies to the entire [`MediaPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord, Builder)]
pub struct ExtXServerControl {
    #[builder(setter(into, strip_option), default)]
    can_skip_until: Option<Float>,
    #[builder(setter(into, strip_option), default)]
    can_skip_dateranges: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    hold_back: Option<Float>,
    #[builder(setter(into, strip_option), default)]
    part_hold_back: Option<Float>,
    #[builder(setter(into, strip_option), default)]
    can_block_reload: Option<bool>,
}

impl ExtXServerControl {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SERVER-CONTROL:";

    pub fn builder() -> ExtXServerControlBuilder { ExtXServerControlBuilder::default() }
}

/// This tag requires [`ProtocolVersion::V8`].
impl RequiredVersion for ExtXServerControl {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V8 }
}

impl fmt::Display for ExtXServerControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut directives: Vec<String> = vec![];
        if let Some(can_skip_until) = self.can_skip_until {
            directives.push(format!("CAN-SKIP-UNTIL={}", can_skip_until));
        }
        write!(f, "{}{}", Self::PREFIX, directives.join(","))
    }
}

impl TryFrom<&str> for ExtXServerControl {
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
        let control = ExtXServerControl::builder()
            .can_skip_until(12u16)
            .build()
            .unwrap();
        assert_eq!(
            "#EXT-X-SERVER-CONTROL:CAN-SKIP-UNTIL=12".to_string(),
            control.to_string(),
        );
    }
}
