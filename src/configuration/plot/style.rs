use core::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Style {
    Default,
    Lines,
    Points,
    Linespoints,
    Impulses,
    Dots,
    Steps,
    Fsteps,
    Histeps,
}

impl From<&str> for Style {
    fn from(s: &str) -> Self {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "default" => Style::Default,
            "lines" => Style::Lines,
            "points" => Style::Points,
            "linespoints" => Style::Linespoints,
            "impulses" => Style::Impulses,
            "dots" => Style::Dots,
            "steps" => Style::Steps,
            "fsteps" => Style::Fsteps,
            "histeps" => Style::Histeps,
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
            _ => Style::Lines,
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
        }
    }
}
