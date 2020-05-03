// Structs
use crate::constants::PLOT_DIR;
use std::ffi::OsStr;
use std::path::PathBuf;

// Traits
use getset::{Getters};

pub mod style;

pub use style::*;

#[derive(Getters, Debug, PartialOrd, PartialEq, Clone)]
#[getset(get = "pub")]
pub(crate) struct PlotConfiguration {
    path_buf: PathBuf,
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
    labelx: Option<String>,
    labely: Option<String>,
    rangex: Option<(f64, f64)>,
    rangey: Option<(f64, f64)>,
    ticsx: Option<String>,
    ticsy: Option<String>,
    style: Style,
    dashtype: Option<usize>,
    pause: Option<f64>,
}

impl PlotConfiguration {
    pub(crate) fn set_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.path_buf.set_extension(extension);
        self
    }

    pub(crate) fn opening_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();
        gnuplot_script += "unset key\n";
        gnuplot_script += &self.opening_plot_script_comparison();

        gnuplot_script
    }

    pub(crate) fn opening_plot_script_comparison(&self) -> String {
        let mut gnuplot_script = String::new();

        match self.title() {
            Some(title) => {
                gnuplot_script += &format!("set title \"{}\"\n", title);
            }
            None => {
                gnuplot_script += "set title \"\"\n";
            }
        }

        match self.labelx() {
            Some(labelx) => {
                gnuplot_script += &format!("set xlabel \"{}\"\n", labelx);
            }
            None => {
                gnuplot_script += "set xlabel \"\"\n";
            }
        }

        match self.labely() {
            Some(labely) => {
                gnuplot_script += &format!("set ylabel \"{}\"\n", labely);
            }
            None => {
                gnuplot_script += "set ylabel \"\"\n";
            }
        }

        if let Some(logx) = &self.logx() {
            if *logx <= 0.0 {
                gnuplot_script += "set logscale x\n";
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.logy() {
            if *logy <= 0.0 {
                gnuplot_script += "set logscale y\n";
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        if let Some(rangex) = &self.rangex() {
            gnuplot_script += &format!("set xrange [{}:{}]\n", rangex.0, rangex.1);
        }
        if let Some(rangey) = &self.rangey() {
            gnuplot_script += &format!("set yrange [{}:{}]\n", rangey.0, rangey.1);
        }

        match self.ticsx() {
            Some(ticsx) => {
                gnuplot_script += &format!("set xtics {}\n", ticsx);
            }
            None => {
                gnuplot_script += "unset xtics\n";
            }
        }

        match self.ticsy() {
            Some(ticsy) => {
                gnuplot_script += &format!("set ytics {}\n", ticsy);
            }
            None => {
                gnuplot_script += "unset ytics\n";
            }
        }

        gnuplot_script
    }

    pub(crate) fn ending_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();

        if let Some(pause) = &self.pause() {
            gnuplot_script += &format!("pause {}", pause);
        }

        gnuplot_script
    }

    pub(crate) fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }
    pub(crate) fn set_logx(&mut self, logx: f64) -> &mut Self {
        self.logx = Some(logx);
        self
    }
    pub(crate) fn set_logy(&mut self, logy: f64) -> &mut Self {
        self.logy = Some(logy);
        self
    }
    pub(crate) fn set_labelx(&mut self, labelx: String) -> &mut Self {
        self.labelx = Some(labelx);
        self
    }
    pub(crate) fn set_labely(&mut self, labely: String) -> &mut Self {
        self.labely = Some(labely);
        self
    }
    pub(crate) fn set_rangex(&mut self, rangex: (f64, f64)) -> &mut Self {
        self.rangex = Some(rangex);
        self
    }
    pub(crate) fn set_rangey(&mut self, rangey: (f64, f64)) -> &mut Self {
        self.rangey = Some(rangey);
        self
    }
    pub(crate) fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }
    pub(crate) fn set_dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.dashtype = Some(dashtype);
        self
    }
    pub(crate) fn set_ticsx<T>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.ticsx = ticsx.into();
        self
    }
    pub(crate) fn set_ticsy<T>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.ticsy = ticsy.into();
        self
    }
    pub(crate) fn set_pause<T>(&mut self, pause: T) -> &mut Self
    where
        T: Into<Option<f64>>,
    {
        self.pause = pause.into();
        self
    }
    pub(crate) fn set_id<S: AsRef<OsStr>>(&mut self, id: S) -> &mut Self {
        if let Some(extension) = self.path_buf.clone().extension() {
            self.path_buf.set_file_name(id);
            self.path_buf.set_extension(extension);
        } else {
            self.path_buf.set_file_name(id);
        }

        self
    }
    pub(crate) fn extension(&self) -> Option<&OsStr> {
        self.path_buf.extension()
    }
}

impl Default for PlotConfiguration {
    fn default() -> PlotConfiguration {
        let mut path_buf: PathBuf = PLOT_DIR.iter().collect();
        path_buf.push("none");
        path_buf.set_extension("gnu");
        let title = None;
        let logx = None;
        let logy = None;
        let labelx = None;
        let labely = None;
        let rangex = None;
        let rangey = None;
        let style = Style::Default;
        let dashtype = None;
        let ticsx = Some(String::from(""));
        let ticsy = Some(String::from(""));
        let pause = Some(-1.0);

        PlotConfiguration {
            path_buf,
            title,
            logx,
            logy,
            rangex,
            rangey,
            labelx,
            labely,
            style,
            dashtype,
            ticsx,
            ticsy,
            pause,
        }
    }
}
