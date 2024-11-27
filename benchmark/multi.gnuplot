set terminal png
set output "plot.png"
set xlabel "Nombre de noeuds"
set ylabel "Temps d'exécution"
set title "Plusieurs courbes"
plot "insert.dat" using 1:2 with lines title "Temps d'insertion", \
     "search.dat" using 1:2 with points title "Temps de recherche", \
     "remove.dat" using 1:2 with linespoints title "Temps de suppression"
