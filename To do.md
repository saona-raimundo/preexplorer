- Statistical analysis

  - SequenceError

    - [x] Plotting 
      - [x] yerrorbars (x:y:error with yerrorbars or w yerr)
      - [x] Trend Line of centers (x:y w lines)
    - [x] new
      - [x] Iterator of iterators
    - [x] Comparison
      - [x] ... 

  - ProcessError

    - [x] continuous yerrorbars (x:y:error with yerrorbars) or (x:y:error with filledc fs transparent solid 0.5 lc rgb "dark-grey")
    - [x] trend 
    - [x] Check out: [gnuplot](http://gnuplot.sourceforge.net/demo_5.4/errorbars.html) 

  - SequenceViolin

    - [x] Violin


  - SequenceBin

    - [ ] Bin
      - [ ] smooth bins 

      - [ ] gnuplot script

        ``` gnuplot
        unset key 
        
        array DataPoints[3]
        DataPoints[1] = 10
        DataPoints[2] = 20
        DataPoints[3] = 10
        
        do for [i=0:2] {
        	# Plotting each histogram
        	set table 'partial_plot'.i
        	plot "0.txt" index i using 2:(1. / DataPoints[i+1]) bins binwidth=1 with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
        	unset table
        }
        
        # Plotting the histograms
        set style fill solid 0.5
        plot for [i=0:2] 'partial_plot'.i using (i):1:(i):(i+$2):3:4 with boxxyerrorbars # using x:y:xlow:xhigh:ylow:yhigh
        
        pause -1
        ```

        - Fixed binwidth for consistency and plotting constant values (make it part of the construction)

  - Heatmaps
      - [ ] [simple](http://www.labbookpages.co.uk/software/gnuplot.html#heatmaps)
      - [ ] [doc](http://www.bersch.net/gnuplot-doc/image.html)
      - [ ] Constructor
          - [ ] new(xs: I, ys: J, zs: K)
          - [ ] From<Array2>
  - [ ] Clean the source file
  - [ ] From Densities
- Examples
  - [x] SequenceError
  - [x] ProcessError
  - [x] SequenceViolin
  - [ ] SequenceBin
- Tests
  - [x] SequenceError
  - [ ] ProcessError
  - [ ] SequenceViolin
- Documentation
  - Library level
  - Module level
  - Describe choices and data saved!
  - Images
  
  - [ ] Comparison
    - Do comparisons by grid-plotting.
  - [gnuplot examples](http://gnuplot.sourceforge.net/demo_5.4/heatmaps.html)
- [ ] Check out: 
  - https://serialmentor.com/dataviz/
  - https://serialmentor.com/