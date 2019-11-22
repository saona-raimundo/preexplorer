set key
set title "All together"
plot "data/my_serie_name_0.txt" using 1:2 with lines title "0" dashtype 1, \
"data/my_serie_name_1.txt" using 1:2 with lines title "1" dashtype 2, 
pause -1
