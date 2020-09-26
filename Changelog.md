# Changelog

## 0.3.3

- Update documentation
- Warning added when saving a struct when no data
- Implement statistical data representation
  - Sequence and Process variants of: bin, error and violin plots.
- Implement heatmap
- Add bins to Density

## 0.3.2

- More intuitive and idiomatic crate structure: 
  - Add and AddAssign traits.
  - From/Into traits.
- Drop the trait bound PartialOrd for Density, as it is not needed anymore.
- Change Density plotting: instead of (incorrect) binning, use smooth kdensity from gnuplot.


## 0.3.1

- Idiomatic implementation of Setter and Getter methods.
- Moved from failure to thiserror crate.

## 0.2.4

- Documentation bug: rustdoc and images.

## 0.2.3

- Update dependencies. 
- More general input for id's. 
- Documentation improved: now images in rustdoc.

## 0.2.2

- Documentation improved.

## 0.2.1

- Documentation added, specially triats.
- New default plot for Density 

## 0.1.2

- Documentation added.