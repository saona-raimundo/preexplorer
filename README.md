# preexplorer
Easy plotter and saver of simple data. Handy tool for development stage or small computational projects. 



Handy way to save data, have a quick view and an initial [gnuplot](http://www.gnuplot.info/) script to plot it. 
if you are looking for a pure Rust plotter, check out [plotters](https://crates.io/crates/plotters).



Rust is meant for computations and plotting is usually an iterative process, done best in tools like [gnuplot](http://www.gnuplot.info/). That is way we separate both steps: compute in Rust, save the data, find the best plot with [gnuplot](http://www.gnuplot.info/). 



# Installation

- Download and [install gnuplot](http://www.gnuplot.info/download.html), a command line engine for plotting. (Note that the gnuplot project has nothing to do with GNU).
- Add ``preexplorer = "0.1"`` to your ``Cargo.toml`` file under ``[dependencies]``.
- I suggest to simply go with ``use preexplorer::prelude::*;`` in your binary, and then use the short-name ``pre::...`` .



# To do

- Interoperability
  - to_comparison
    - In all basic structures
    - Is this part of the trait?
    - type: Comparison?
- Duplicate methods, for easier use
  - labelx and xlabel
- Warning: Windows only, beacuse of extensions in file name. Help wanted.
- More settings
  - Extensible Configuration struct
    - through a HashMap
  - rangex, and xrange
  - rangey, and yrange
  - set or unset 
    - palette
    - tics
      - xrange
      - yrange
      - colorbox
- pub traits should be externally implementable
  - raw_data
    - Change to plotable_data
      raw data can be seen as something else
  - Include 
    - pub use configuration::*; 
      in lib level.
  - Expose base_plot_script
    - At least examples for plot script in Plotable
  - Make a personalized plot_script writable
    - Add a macro for it? 
  - Make data and plot directory constants public
  - Add method
    - .path_to_data
      - Gives the full path to the data file
        with extension
    - .path_to_plot_script
      - Gives the full path to the plot script file
        with extension
  - Document 
    - Trait structure
      - Configurable + -> Saveable + -> Plotable
    - Saveable
      - raw_data
    - Plotable
      - plot_script
      - base_plot_script
- Add plot_later command, 
  - save and write_plot_script together
- Add SequenceWithError and ProcessWithError for error plotting
- Allow user defined configuration parameters
  - HashMap
- Write proper presentation of the crate:
  - Purpose
  - Installation
    - Recall gnuplot
  - Use with Rust
- More customization
  - Audio
    - audify()
    - sonify()
- ndimensional variants
- Document project
  - Add bingenes crate as an example of implementing traits

# Disclaimer

There is no connection with the gnuplot project.
