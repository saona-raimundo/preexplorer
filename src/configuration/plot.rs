pub(crate) mod style;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct PlotConfiguration {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
    labelx: Option<String>,
    labely: Option<String>,
    rangex: Option<(f64, f64)>,
    rangey: Option<(f64, f64)>,
    ticsx: Option<String>,
    ticsy: Option<String>,
    style: crate::configuration::plot::style::Style,
    dashtype: Option<usize>,
    pause: Option<f64>,
}

impl PlotConfiguration {
    pub(crate) fn opening_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();
        gnuplot_script += "unset key\n";
        gnuplot_script += &self.opening_plot_script_comparison();

        gnuplot_script
    }

    pub(crate) fn opening_plot_script_comparison(&self) -> String {
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

        if let Some(rangex) = &self.get_rangex() {
            gnuplot_script += &format!("set xrange [{}:{}]\n", rangex.0, rangex.1);
        }
        if let Some(rangey) = &self.get_rangey() {
            gnuplot_script += &format!("set yrange [{}:{}]\n", rangey.0, rangey.1);
        }

        match self.get_ticsx() {
            Some(ticsx) => {
                gnuplot_script += &format!("set xtics {}\n", ticsx);
            }
            None => {
                gnuplot_script += &format!("unset xtics\n");
            }
        }

        match self.get_ticsy() {
            Some(ticsy) => {
                gnuplot_script += &format!("set ytics {}\n", ticsy);
            }
            None => {
                gnuplot_script += &format!("unset ytics\n");
            }
        }

        gnuplot_script
    }

    pub(crate) fn ending_plot_script(&self) -> String {
        let mut gnuplot_script = String::new();


        if let Some(pause) = &self.get_pause() {
            gnuplot_script += &format!("pause {}", pause);
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
    pub(crate) fn rangex(&mut self, rangex: (f64, f64)) -> &mut Self {
        self.rangex = Some(rangex);
        self
    }
    pub(crate) fn rangey(&mut self, rangey: (f64, f64)) -> &mut Self {
        self.rangey = Some(rangey);
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
    pub(crate) fn ticsx<T>(&mut self, ticsx: T) -> &mut Self 
    where
        T: Into<Option<String>>,
    {
        self.ticsx = ticsx.into();
        self
    }
    pub(crate) fn ticsy<T>(&mut self, ticsy: T) -> &mut Self 
    where
        T: Into<Option<String>>,
    {
        self.ticsy = ticsy.into();
        self
    }
    pub(crate) fn pause<T>(&mut self, pause: T) -> &mut Self 
    where
        T: Into<Option<f64>>,
    {
        self.pause = pause.into();
        self
    }

    //////////////////////////////////////////////////////////
    // Getting
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
    pub(crate) fn get_rangex(&self) -> Option<(f64, f64)> {
        self.rangex
    }
    pub(crate) fn get_rangey(&self) -> Option<(f64, f64)> {
        self.rangey
    }
    pub(crate) fn get_style(&self) -> &crate::configuration::plot::style::Style {
        &self.style
    }
    pub(crate) fn get_dashtype(&self) -> Option<usize> {
        self.dashtype
    }
    pub(crate) fn get_ticsx(&self) -> Option<&String> 
    {
        self.ticsx.as_ref()
    }
    pub(crate) fn get_ticsy(&self) -> Option<&String> 
    {
        self.ticsy.as_ref()
    }
    pub(crate) fn get_pause(&self) -> Option<f64> 
    {
        self.pause
    }

}

impl Default for PlotConfiguration {
    fn default() -> PlotConfiguration {
        let title = None;
        let logx = None;
        let logy = None;
        let labelx = None;
        let labely = None;
        let rangex = None;
        let rangey = None;
        let style = crate::configuration::plot::style::Style::Default;
        let dashtype = None;
        let ticsx = Some(String::from(""));
        let ticsy = Some(String::from(""));
        let pause = Some(-1.0);

        PlotConfiguration {
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