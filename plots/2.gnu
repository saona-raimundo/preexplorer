set key
set title "More comparisons"
plot "data/2_0.txt" using 1:2 with lines title "Second", "data/2_1.txt" using 1:2 with lines title "First", "data/2_2.txt" using 1:2 with lines title "Third", 
pause -1
