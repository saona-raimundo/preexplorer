set key
set title "My title"
nbins_0 = 20.0 #number of bins
max_0 = 199 #max value
min_0 = 0 #min value
len_0 = 300.0 #number of values
width_0 = 199 / nbins_0 #width

#function used to map a value to the intervals
hist_0(x,width_0) = width_0 * floor(x/width_0) + width_0 / 2.0

nbins_1 = 20.0 #number of bins
max_1 = 299 #max value
min_1 = 100 #min value
len_1 = 420.0 #number of values
width_1 = 199 / nbins_1 #width

#function used to map a value to the intervals
hist_1(x,width_1) = width_1 * floor(x/width_1) + width_1 / 2.0

plot "data/1_0.txt" using (hist_0($1,width_0)):(1.0/len_0) smooth frequency with steps title "My legend", "data/1_1.txt" using (hist_1($1,width_1)):(1.0/len_1) smooth frequency with steps title "1", 
pause -1
