
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct PlotConfiguration {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
    labelx: Option<String>,
    labely: Option<String>,
}

impl PlotConfiguration {
    pub(crate) fn default() -> PlotConfiguration {
        let title = None;
        let logx = None;
        let logy = None;
        let labelx = None;
        let labely = None;

        PlotConfiguration { title, logx, logy, labelx, labely }
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
                gnuplot_script += &format!("set labelx \"{}\"\n", labelx);
            },
            None => {
                gnuplot_script += &format!("set labelx \"\"\n");
            },
        }

        match self.labely() {
            Some(labely) => {
                gnuplot_script += &format!("set labely \"{}\"\n", labely);
            },
            None => {
                gnuplot_script += &format!("set labely \"\"\n");
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
    pub(crate) fn labelx(&self) -> Option<String> {
        match &self.labelx {
            Some(labelx) => Some(labelx.to_string()),
            None => None,
        }
    }
    pub(crate) fn labely(&self) -> Option<String> {
        match &self.labely {
            Some(labely) => Some(labely.to_string()),
            None => None,
        }
    }
}
