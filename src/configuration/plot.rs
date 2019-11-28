
pub(crate) mod style;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct PlotConfiguration {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
    labelx: Option<String>,
    labely: Option<String>,
    style: crate::configuration::plot::style::Style,
}

impl PlotConfiguration {
    pub(crate) fn default() -> PlotConfiguration {
        let title = None;
        let logx = None;
        let logy = None;
        let labelx = None;
        let labely = None;
        let style = crate::configuration::plot::style::Style::Default;

        PlotConfiguration { title, logx, logy, labelx, labely, style }
    }

    pub(crate) fn base_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();
        gnuplot_script += "unset key\n";

        match self.title() {
            Some(title) => {
                gnuplot_script += &format!("set title \"{}\"\n", title);
            },
            None => {
                gnuplot_script += &format!("set title \"\"\n");
            },
        }

        match self.labelx() {
            Some(labelx) => {
                gnuplot_script += &format!("set xlabel \"{}\"\n", labelx);
            },
            None => {
                gnuplot_script += &format!("set xlabel \"\"\n");
            },
        }

        match self.labely() {
            Some(labely) => {
                gnuplot_script += &format!("set ylabel \"{}\"\n", labely);
            },
            None => {
                gnuplot_script += &format!("set ylabel \"\"\n");
            },
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
    pub(crate) fn set_style(&mut self, style: crate::configuration::plot::style::Style) -> &mut Self {
        self.style = style;
        self
    }


    pub(crate) fn title(&self) -> Option<String> {
        match &self.title {
            Some(title) => Some(title.to_string()),
            None => None,
        }
    }
    pub(crate) fn logx(&self) -> Option<f64> {
        self.logx
    }
    pub(crate) fn logy(&self) -> Option<f64> {
        self.logy
    }
    pub(crate) fn labelx(&self) -> Option<&str> {
        match &self.labelx {
            Some(labelx) => Some(labelx),
            None => None,
        }
    }
    pub(crate) fn labely(&self) -> Option<&str> {
        match &self.labely {
            Some(labely) => Some(labely),
            None => None,
        }
    }
    pub(crate) fn style(&self) -> &crate::configuration::plot::style::Style {
        &self.style
    }
}
