# preexplorer
Easy plotter and saver of simple data. Handy tool for development stage or small computational projects. 



Handy way to save data, have a quick view and an initial [gnuplot](http://www.gnuplot.info/) script to plot it. 
if you are looking for a pure Rust plotter, check out [plotters](https://crates.io/crates/plotters).



Rust is meant for computations and plotting is usually an iterative process, done best in tools like [gnuplot](http://www.gnuplot.info/). That is way we separate both steps: compute in Rust, save the data, find the best plot with [gnuplot](http://www.gnuplot.info/). 

# Purpose

Do you have a costly process in Rust and want to save the data for postprocessing?
Would you like to still have a basic glance to check it and leave fine-tuning of the plot for later?
This is the crate for you!

# Main features

- Easy plotting
- Easy comparison
- Implementable traits
- Extensive documentation

# Examples

For more, see the folder [examples](https://github.com/rasa200/preexplorer/tree/master/examples). 

# Warning

For now, this crate is Windows only, because of extensions in file name. Help wanted.

# Installation

- Download and [install gnuplot](http://www.gnuplot.info/download.html), a command line engine for plotting. (Note that the gnuplot project has nothing to do with GNU).
- Add ``preexplorer = "0.2"`` to your ``Cargo.toml`` file under ``[dependencies]``.
- I suggest to simply go with ``use preexplorer::prelude::*;`` in your binary, and then use the short-name ``pre::...`` and the ``preexplore`` method directly in iterators and tuple of iterators.



# Future

- [ ] New Structs for error plotting
  - [ ] SequenceWithError 
  - [ ] ProcessWithError
- [ ] More senses
  - [ ] Audio
    - [ ] audify()
    - [ ] sonify()

# Q & A

1. Why processes must be the same structs to be compared? 
   Because of Rust explicit typing: comparisons need to save an explicit type. 
2. Are there n-dimensional variants?
   No, it is out of scope. Please implement your own plot script for that. You can do so easily based in the Data struct. 
3. How to deactivate some options, e.g. tics?
   What would represent the empty value instead of None. This happens because of inferring type errors in Rust.

# Disclaimer

There is no connection with the gnuplot project.
