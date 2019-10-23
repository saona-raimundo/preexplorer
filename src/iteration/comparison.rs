/// Various iterations mixed together to be compared
pub use crate::traits::PlotableStructure;

// Trait bounds
use core::fmt::Display;
use failure::Fallible;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<'a, I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    pub(crate) data_set: &'a mut Vec<crate::iteration::Iteration<I>>,
    pub(crate) options: crate::iteration::IterationOptions,
}
impl<'a, I> Comparison<'a, I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    pub fn new(data_set: &'a mut Vec<crate::iteration::Iteration<I>>) -> Comparison<'a, I> {
        let options = crate::iteration::IterationOptions::default();

        Comparison { data_set, options }
    }

    pub fn set_title<S: Display>(mut self, title: S) -> Self {
        self.options.set_title(title.to_string());
        self
    }
    pub fn set_logx(mut self, logx: f64) -> Self {
        self.options.set_logx(logx);
        self
    }
    pub fn set_logy(mut self, logy: f64) -> Self {
        self.options.set_logy(logy);
        self
    }

    pub fn add(&mut self, simulations: &Vec<crate::iteration::Iteration<I>>) {
        self.data_set.extend_from_slice(simulations)
    }
}

impl<'a, I> crate::traits::PlotableStructure for Comparison<'a, I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    fn save<S: Display>(self, serie: &S) -> Fallible<()> {
        for i in (0..self.data_set.len()).rev() {
            match self.data_set.pop() {
                Some(iteration) => {
                    crate::iteration::Iteration::save(iteration, &format!("{}_{}", serie, i))?
                }
                None => break,
            }
        }

        Ok(())
    }

    fn plot<S: Display>(self, serie: &S) -> Fallible<()> {
        self.write_plot_script(serie)?;
        self.save(serie)?;

        let gnuplot_file = format!("{}.gnu", serie);

        let gnuplot_file = &format!("plots\\{}", gnuplot_file);
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(())
    }

    fn write_plot_script<S: Display>(&self, serie: &S) -> Fallible<()> {
        std::fs::create_dir_all("plots").unwrap();
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = String::new();
        gnuplot_script += &format!("set key\n");
        if let Some(title) = &self.options.title {
            gnuplot_script += &format!("set title \"{}\"\n", title);
        }
        if let Some(logx) = &self.options.logx {
            if *logx == -1.0 {
                gnuplot_script += &format!("set logscale x\n");
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.options.logy {
            if *logy == -1.0 {
                gnuplot_script += &format!("set logscale y\n");
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script += &format!("plot ");
        for i in 0..self.data_set.len() {
            let legend = match &self.data_set[i].options.title {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using 1:2 with lines title \"{}\", ",
                serie, i, legend
            );
        }
        gnuplot_script += &format!("\n");
        gnuplot_script += &format!("pause -1\n");

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(())
    }
}
