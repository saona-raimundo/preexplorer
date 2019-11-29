# external_gnuplot
Handy way to save data, have a quick view and an initial [gnuplot](http://www.gnuplot.info/) script to plot it. 
if you are looking for a pure Rust plotter, check out [plotters](https://crates.io/crates/plotters).



Rust is meant for computations and plotting is usually an iterative process, done best in tools like [gnuplot](http://www.gnuplot.info/). That is way we separate both steps: compute in Rust, save the data, find the best plot with [gnuplot](http://www.gnuplot.info/). 



# Installation

- Download and [install gnuplot](http://www.gnuplot.info/download.html), a command line engine for plotting. (Note that the gnuplot project has nothing to do with GNU).
- Add ``preexplorer = "0.1"`` to your ``Cargo.toml`` file under ``[dependencies]``.
- I suggest to simply go with ``use preexplorer::prelude::*;`` in your binary, and then use the short-name ``pre::...`` .



# To do

- Go back to
  - I: IntoIterator,
  - and use I::IntoIter: ExactSizeIterator, in new method. 
- More customization
  - Audio
    - audify()
    - sonify()
- Add to options
  - dashtype 
    - for each plot
    - for comparisons
      - Default variant to use in comparisons
      - respect inner options
- ndimensional variants
- Write proper presentation of the crate:
  - Purpose
  - Installation
    - Recall gnuplot
  - Use with Rust
- Document project
- Publish it in crates.io



# Disclaimer

I have no connection with the gnuplot project.
