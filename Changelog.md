# Changelog

## 0.3.1

- Drop the trait bound PartialOrd for Density, as it is not needed anymore.
- Change Density plotting: instead of (incorrect) binning, use smooth kdensity from gnuplot.
- More intuitive and idiomatic crate structure: 
  - Add and AddAssign traits.
  - From/Into traits.
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