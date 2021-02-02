//! Styles for plotting in gnuplot.

use core::fmt::Display;

/// A small collection of all possible styles.
///
/// # Remarks
///
/// There are ``From<>`` implementations for ease of use.
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Style {
    /// Default "default" or 0
    Default,
    /// Continuous lines "-" or "lines" or 1
    Lines,
    /// Points "+" or "points" or 2
    Points,
    /// Points and lines together "-+-" or "linepoints" or 3
    Linespoints,
    /// Vertical line per data point "|" or "impulses" or 4
    Impulses,
    /// Smallest point possible "." or "dots" or 5
    Dots,
    /// Piecewise constant with jumps on data points "_|" or "steps" or 6
    Steps,
    /// Piecewise constant with jumps previous to data points "|-"or "fsteps" or 7
    Fsteps,
    /// Piecewise constant centered in data points "_-_" or "histeps" or 8
    Histeps,
    /// Adjustable piecewise constant centered in data points "_--_" or "boxes" or 9
    Boxes,
}

impl From<&str> for Style {
    fn from(s: &str) -> Self {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "default" => Style::Default,
            "lines" | "-" => Style::Lines,
            "points" | "+" => Style::Points,
            "linespoints" | "-+-" => Style::Linespoints,
            "impulses" | "|" => Style::Impulses,
            "dots" | "." => Style::Dots,
            "steps" | "_|" => Style::Steps,
            "fsteps" | "|-" => Style::Fsteps,
            "histeps" | "_-_" => Style::Histeps,
            "boxes" | "_--_" => Style::Boxes,
            _ => Style::Lines,
        }
    }
}

impl From<String> for Style {
    fn from(s: String) -> Self {
        Style::from(s.as_str())
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

impl Display for Style {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Style::Default => write!(f, "lines"),
            Style::Lines => write!(f, "lines"),
            Style::Points => write!(f, "points"),
            Style::Linespoints => write!(f, "linespoints"),
            Style::Impulses => write!(f, "impulses"),
            Style::Dots => write!(f, "dots"),
            Style::Steps => write!(f, "steps"),
            Style::Fsteps => write!(f, "fsteps"),
            Style::Histeps => write!(f, "histeps"),
            Style::Boxes => write!(f, "boxes"),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::Default
    }
}
