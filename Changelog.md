# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - Unreleased

- Doc: correcting an image
- More traits for `Style` and be more careful with conversions
- Add `serde` support 
- Add z axis configuration

## [0.4.1] - 2021-07-19

- Fix: Heatmaps are now plotted correctly using `pm3d` instead of `image` 
- Updated dependencies
- Errors are now non-exhaustive for less breaking changes

## [0.3.5] - 2021-01-07

- Correct Style::Point symbol documentation
- Add a global `preexplorer::clean()` function to remove all generated files 

## [0.3.4] - 2020-11-12

- Make Style Enum public

## [0.3.3] - 2020-09-26

- Update documentation
- Warning added when saving a struct when no data
- Implement statistical data representation
  - Sequence and Process variants of: bin, error and violin plots.
- Implement heatmap
- Add bins to Density

## [0.3.2] - 2020-07-13

- More intuitive and idiomatic crate structure: 
  - Add and AddAssign traits.
  - From/Into traits.
- Drop the trait bound PartialOrd for Density, as it is not needed anymore.
- Change Density plotting: instead of (incorrect) binning, use smooth kdensity from gnuplot.


## [0.3.1] - 2020-06-04

- Idiomatic implementation of Setter and Getter methods.
- Moved from failure to thiserror crate.

## [0.2.4] - 2020-05-02

- Documentation bug: rustdoc and images.

## [0.2.3] - 2020-05-02

- Update dependencies. 
- More general input for id's. 
- Documentation improved: now images in rustdoc.

## [0.2.2] - 2020-04-16

- Documentation improved.

## [0.2.1] - 2020-04-15

- Documentation added, specially traits.
- New default plot for Density 

## [0.1.1] - 2020-03-24

- Documentation added.

## [0.1.0] - 2020-03-24

- First working version

