use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Decimal resolution.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DecimalResolution {
    width: usize,
    height: usize,
}

impl DecimalResolution {
    /// Creates a new DecimalResolution.
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Horizontal pixel dimension.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Sets Horizontal pixel dimension.
    pub fn set_width(&mut self, value: usize) -> &mut Self {
        self.width = value;
        self
    }

    /// Vertical pixel dimension.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Sets Vertical pixel dimension.
    pub fn set_height(&mut self, value: usize) -> &mut Self {
        self.height = value;
        self
    }
}

impl fmt::Display for DecimalResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FromStr for DecimalResolution {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens = input.splitn(2, 'x').collect::<Vec<_>>();

        if tokens.len() != 2 {
            return Err(Error::custom(format!(
                "InvalidInput: Expected input format: [width]x[height] (ex. 1920x1080), got {:?}",
                input,
            )));
        }

        let width = tokens[0];
        let height = tokens[1];

        Ok(DecimalResolution {
            width: width.parse()?,
            height: height.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            DecimalResolution::new(1920, 1080).to_string(),
            "1920x1080".to_string()
        );

        assert_eq!(
            DecimalResolution::new(1280, 720).to_string(),
            "1280x720".to_string()
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            DecimalResolution::new(1920, 1080),
            "1920x1080".parse::<DecimalResolution>().unwrap()
        );

        assert_eq!(
            DecimalResolution::new(1280, 720),
            "1280x720".parse::<DecimalResolution>().unwrap()
        );

        assert!("1280".parse::<DecimalResolution>().is_err());
    }

    #[test]
    fn test_width() {
        assert_eq!(DecimalResolution::new(1920, 1080).width(), 1920);
        assert_eq!(DecimalResolution::new(1920, 1080).set_width(12).width(), 12);
    }

    #[test]
    fn test_height() {
        assert_eq!(DecimalResolution::new(1920, 1080).height(), 1080);
        assert_eq!(
            DecimalResolution::new(1920, 1080).set_height(12).height(),
            12
        );
    }
}