# preexplorer
Easy plotter and saver of simple data. Handy tool for development stage or small computational projects. 



Handy way to save data, have a quick view and an initial [gnuplot](http://www.gnuplot.info/) script to plot it. 
if you are looking for a pure Rust plotter, check out [plotters](https://crates.io/crates/plotters).



Rust is meant for computations and plotting is usually an iterative process, done best in tools like [gnuplot](http://www.gnuplot.info/). That is way we separate both steps: compute in Rust, save the data, find the best plot with [gnuplot](http://www.gnuplot.info/). 

# Purpose

TO DOCUMENT

# Examples

For more, see the folder [examples](https://github.com/rasa200/preexplorer/tree/master/examples). 

# Installation

- Download and [install gnuplot](http://www.gnuplot.info/download.html), a command line engine for plotting. (Note that the gnuplot project has nothing to do with GNU).
- Add ``preexplorer = "0.1"`` to your ``Cargo.toml`` file under ``[dependencies]``.
- I suggest to simply go with ``use preexplorer::prelude::*;`` in your binary, and then use the short-name ``pre::...`` and the ``preexplore`` method directly in iterators and tuple of iterators.



# To do

- Rethink the distribution so that it integrates one
- Use [std](https://doc.rust-lang.org/std/index.html)::[path](https://doc.rust-lang.org/std/path/index.html)::[PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html) for the path of files
- Interoperability
  - In basic struct
    - Add method 
      - to_comparison
        - self.into()
    - Implement own trait
      - Sequence: Configurable + Plotable + Saveable
  - In comparison
    - Change name to "add an s"
    - Implement
      - From<basic>
      - crate::traits::Comparison
- Why processes must be the same structs to be compared? 
  - Can comparisons not simply have basic traits as objects?
- Warning: Windows only, because of extensions in file name. Help wanted.
- More settings
  - Extensible Configuration struct
    - through a HashMap
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
- New Structs for error plotting
  - SequenceWithError 
  - ProcessWithError
- Allow user defined configuration parameters
  - HashMap
- More customization
  - Audio
    - audify()
    - sonify()
- ndimensional variants
- Document project
  - Add bingenes crate as an example of implementing traits

# Questions

Why the following code does not work?

```rust
MarkovChain::new(init_state, &transition)
    .take(max_steps)
	.preexplore()
```



# Q & A

1. Why do the structs not implement Eq traits?
   Because we use f64
2. Why do the structs not implement PartialOrd traits?
   Because we use HashMap
3. Why do we not have two Configurations? SaveConfiguration and PlotConfiguration?
   To make life simpler for those outside the crate

# Disclaimer

There is no connection with the gnuplot project.
