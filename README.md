# bioinformatics-compeau-pevzner
compeau-pevzner bioinformatic algorithms book challenges

Se presenta la solución a retos del libro Bioinformatics Algorithms, de Compeau y Pevzner.
Los retos fueron validados en Rosalind.org, y los datos de prueba se encuentran disponibles.
Actualmente se cuenta con un archivo llamado main.rs que recibe argumentos de línea de comandos para las llamadas.
El archivo con las dos funciones 1A y 1B, se encuentra en el archivo patterns.rs.

Sintaxis:
Actualmente la llamada se hace sobre main, pero cambiará a biocptools.
Las posibles opciones son:
-pc para el reto 1A-Pattern Count
-fw para el reto 1B-Frequent Words

Ejemplos de llamadas a main con los archivos de Rosalind.org, son:

./main -pc < 1Adata.txt

./main -fw < 1Bdata.txt
