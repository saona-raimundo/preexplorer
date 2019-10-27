unset key
nbins = 20.0 #number of bins
max = 299 #max value
min = 100 #min value
len = 420.0 #number of values
width = 199 / nbins #width

#function used to map a value to the intervals
hist(x,width) = width * floor(x/width) + width / 2.0

plot "data/1_1.txt" using (hist($1,width)):(1.0/len) smooth frequency with steps
pause -1
