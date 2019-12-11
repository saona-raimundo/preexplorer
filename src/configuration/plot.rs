pub(crate) mod style;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct PlotConfiguration {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
    labelx: Option<String>,
    labely: Option<String>,
    style: crate::configuration::plot::style::Style,
    dashtype: Option<usize>,
}

impl PlotConfiguration {
    pub(crate) fn base_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();
        gnuplot_script += "unset key\n";
        gnuplot_script += &self.base_plot_script_comparison();

        gnuplot_script
    }

    pub(crate) fn base_plot_script_comparison(&self) -> String {
        let mut gnuplot_script = String::new();

        match self.get_title() {
            Some(title) => {
                gnuplot_script += &format!("set title \"{}\"\n", title);
            }
            None => {
                gnuplot_script += &format!("set title \"\"\n");
            }
        }

        match self.get_labelx() {
            Some(labelx) => {
                gnuplot_script += &format!("set xlabel \"{}\"\n", labelx);
            }
            None => {
                gnuplot_script += &format!("set xlabel \"\"\n");
            }
        }

        match self.get_labely() {
            Some(labely) => {
                gnuplot_script += &format!("set ylabel \"{}\"\n", labely);
            }
            None => {
                gnuplot_script += &format!("set ylabel \"\"\n");
            }
        }

        if let Some(logx) = &self.get_logx() {
            if *logx <= 0.0 {
                gnuplot_script += "set logscale x\n";
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.get_logy() {
            if *logy <= 0.0 {
                gnuplot_script += "set logscale y\n";
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script
    }

    pub(crate) fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }
    pub(crate) fn logx(&mut self, logx: f64) -> &mut Self {
        self.logx = Some(logx);
        self
    }
    pub(crate) fn logy(&mut self, logy: f64) -> &mut Self {
        self.logy = Some(logy);
        self
    }
    pub(crate) fn labelx(&mut self, labelx: String) -> &mut Self {
        self.labelx = Some(labelx);
        self
    }
    pub(crate) fn labely(&mut self, labely: String) -> &mut Self {
        self.labely = Some(labely);
        self
    }
    pub(crate) fn style(&mut self, style: crate::configuration::plot::style::Style) -> &mut Self {
        self.style = style;
        self
    }
    pub(crate) fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.dashtype = Some(dashtype);
        self
    }

    pub(crate) fn get_title(&self) -> Option<&String> {
        self.title.as_ref()
    }
    pub(crate) fn get_logx(&self) -> Option<f64> {
        self.logx
    }
    pub(crate) fn get_logy(&self) -> Option<f64> {
        self.logy
    }
    pub(crate) fn get_labelx(&self) -> Option<&String> {
        self.labelx.as_ref()
    }
    pub(crate) fn get_labely(&self) -> Option<&String> {
        self.labely.as_ref()
    }
    pub(crate) fn get_style(&self) -> &crate::configuration::plot::style::Style {
        &self.style
    }
    pub(crate) fn get_dashtype(&self) -> Option<usize> {
        self.dashtype
    }
}

impl Default for PlotConfiguration {
    fn default() -> PlotConfiguration {
        let title = None;
        let logx = None;
        let logy = None;
        let labelx = None;
        let labely = None;
        let style = crate::configuration::plot::style::Style::Default;
        let dashtype = None;

        PlotConfiguration {
            title,
            logx,
            logy,
            labelx,
            labely,
            style,
            dashtype,
        }
    }
}