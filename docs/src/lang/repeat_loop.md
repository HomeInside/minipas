# repeat

A diferencia de los bucles **for** y **while**, que comprueban la condición del bucle en la parte superior del mismo, el bucle **repeat ... until** comprueba su condición en la parte inferior del bucle.

Un ciclo/bucle **repeat ... until** es similar a un bucle `while`, excepto que este garantiza que se ejecutará al menos una vez, y luego continúa repitiéndolo hasta que se cumpla una condición.

La sintaxis del bucle **repeat ... until** es la siguiente:

```mp
repeat
   Setencia1;
   Setencia2;
   ...
   SetenciaN;
until condición;
```

<br>

Observa que la expresión condicional aparece al final del bucle, por lo que las instrucciones dentro, se ejecutan una vez antes de que se compruebe la condición.

Si la condición es **falsa**, el flujo de control vuelve al principio para repetirse, y las instrucciones del bucle se ejecutan de nuevo. Este proceso se repite hasta que la condición dada se vuelve **verdadera**.


**A tener en cuenta:**

Recuerda que debes controlar la condición, para evitar caer en un ciclo infinito.

El ciclo **repeat ... until** no necesariamente necesita de un bloque **begin**/**end**. 

Usar un estilo claro y mantener una buena legibilidad del código es fundamental para evitar errores en la lógica tu programa.

<br>


### Ejemplo

```mp
program test_loops;
var
   i, j: integer;
   resultado: integer;

begin
   writeln("Tablas de multiplicar");
   writeln();

   // Inicializamos el valor de `i` en 1
   i := 1;

   repeat
      // Inicializamos el valor de `j` en 1
      j := 1;

      repeat
         resultado := i * j;
         writeln(i, " x ", j, " = ", resultado);
         j := j + 1; // incrementamos `j`
      until j > 10; // condición de terminación para el ciclo anidado

      writeln("----------------------");
      i := i + 1; // incrementamos `i`
   until i > 10; // condición de terminación para el ciclo principal
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
Tablas de multiplicar

1  x  1  =  1
1  x  2  =  2
1  x  3  =  3
1  x  4  =  4
1  x  5  =  5
1  x  6  =  6
1  x  7  =  7
1  x  8  =  8
1  x  9  =  9
1  x  10  =  10
----------------------
2  x  1  =  2
2  x  2  =  4
2  x  3  =  6
2  x  4  =  8
2  x  5  =  10
2  x  6  =  12
2  x  7  =  14
2  x  8  =  16
2  x  9  =  18
2  x  10  =  20
----------------------
3  x  1  =  3
3  x  2  =  6
3  x  3  =  9
3  x  4  =  12
3  x  5  =  15
3  x  6  =  18
3  x  7  =  21
3  x  8  =  24
3  x  9  =  27
3  x  10  =  30
----------------------

...
```

**A tener en cuenta:**

Recuerda que debes controlar la condición, para evitar caer en un ciclo infinito.


## Recomendaciones

**Evitar bucles infinitos**: Verifica que las condiciones de terminación de tus ciclos estén bien definidas para evitar que tu programa se quede atrapado en un bucle infinito.

**Uso de variables de control**: Mantén el uso de la variable de control (como `i`) simple y único dentro del ciclo para evitar confusiones y errores en la lógica.

**Comentarios claros**: Agrega comentarios para explicar la lógica detrás de los ciclos, especialmente si tienen condiciones complejas o no evidentes.

**Ciclos anidados**: Si utilizas ciclos anidados, mantén la claridad en el control de las variables para evitar errores lógicos y verifica que la lógica sea comprensible.

**Evaluar el rendimiento**: Considera el impacto en el rendimiento de bucles anidados o extensos, y optimiza cuando sea necesario para evitar problemas de eficiencia.

**Pruebas exhaustivas**: Realiza pruebas con diferentes valores de entrada para asegurar que los ciclos funcionan como se espera en todos los casos, especialmente en los límites.

**Evitar lógica complicada dentro del ciclo**: Mantén las instrucciones dentro del ciclo simples y directas. Si la lógica es compleja, considera extraerla en funciones o procedimientos separados.
