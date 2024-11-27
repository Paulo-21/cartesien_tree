set terminal png
set output "plot.png"
set xlabel "Nombre de noeuds"
set ylabel "Temps d'exécution en milliseconde"
set title "Temps d'interaction Arbre Cartésien"
plot "insert.dat" using 1:2 with linespoints title "Temps d'insertion", \
     "search.dat" using 1:2 with linespoints title "Temps de recherche", \
     "remove.dat" using 1:2 with linespoints title "Temps de suppression"
