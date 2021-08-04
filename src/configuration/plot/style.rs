//! Styles for plotting in gnuplot.

// Traits
use core::convert::TryFrom;
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

/// A small collection of all possible styles.
///
/// # Remarks
///
/// There are ``From<>`` implementations for ease of use.
#[derive(
    Debug,
    PartialOrd,
    PartialEq,
    Clone,
    Display,
    AsRefStr,
    EnumCount,
    EnumIter,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Style {
    /// Default "default" or 0
    #[strum(ascii_case_insensitive)]
    Default,
    /// Continuous lines "-" or "lines" or 1
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "lines", serialize = "-")]
    Lines,
    /// Points "+" or "points" or 2
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "points", serialize = "+")]
    Points,
    /// Points and lines together "-+-" or "linepoints" or 3
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "linespoints", serialize = "-+-")]
    Linespoints,
    /// Vertical line per data point "|" or "impulses" or 4
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "impulses", serialize = "|")]
    Impulses,
    /// Smallest point possible "." or "dots" or 5
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "dots", serialize = ".")]
    Dots,
    /// Piecewise constant with jumps on data points "_|" or "steps" or 6
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "steps", serialize = "_|")]
    Steps,
    /// Piecewise constant with jumps previous to data points "|-"or "fsteps" or 7
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "fsteps", serialize = "|-")]
    Fsteps,
    /// Piecewise constant centered in data points "_-_" or "histeps" or 8
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "histeps", serialize = "_-_")]
    Histeps,
    /// Adjustable piecewise constant centered in data points "_--_" or "boxes" or 9
    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "boxes", serialize = "_--_")]
    Boxes,
}

impl TryFrom<&str> for Style {
    type Error = strum::ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Style::from_str(s)
    }
}

impl From<u32> for Style {
    fn from(s: u32) -> Self {
        match s {
            0 => Style::Default,
            1 => Style::Lines,
            2 => Style::Points,
            3 => Style::Linespoints,
            4 => Style::Impulses,
            5 => Style::Dots,
            6 => Style::Steps,
            7 => Style::Fsteps,
            8 => Style::Histeps,
            9 => Style::Boxes,
            _ => Style::Default,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let style = Style::default();
        assert_eq!(style, Style::Default);
    }

    #[test]
    fn from_str() {
        let s = "linespoints";
        let style = Style::from_str(s).unwrap();
        assert_eq!(style, Style::Linespoints);
    }

    #[cfg(feature = "use-serde")]
    #[test]
    fn serde() -> Result<(), ron::Error> {
        // Serializing
        let style = Style::default();
        let string = ron::ser::to_string(&style)?;
        assert_eq!(string, "Default");
        // Deserializing
        let string = "Default";
        let style: Style = ron::de::from_str(string)?;
        assert_eq!(style, Style::default());

        Ok(())
    }
}
