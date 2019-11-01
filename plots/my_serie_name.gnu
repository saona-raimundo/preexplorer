set key
set title "More comparisons"
plot "data/my_serie_name_0.txt" using 1:2 with lines title "First" dt 1, \
"data/my_serie_name_1.txt" using 1:2 with lines title "Second" dt 2, \
"data/my_serie_name_2.txt" using 1:2 with lines title "Third" dt 3, 
pause -1
