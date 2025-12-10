# while

El ciclo **while(while-do)** se repite mientras la condición especificada sea verdadera. Es útil cuando no se conoce de antemano el número de iteraciones.

La sintaxis del bucle **while-do** es la siguiente:

```mp
while condición do
begin
  Sentencia1;
  Sentencia2;
  ...
end
```

<br>

**Condición:** La expresión booleana que se evalúa antes de cada iteración. Si la condición es verdadera, el bloque de instrucciones se ejecuta. Si es falsa, el ciclo termina.

Recuerda que debes controlar la condición, para evitar caer en un ciclo infinito.

<br>

**A tener en cuenta:**

Aunque es posible hacer que un ciclo `while` ocupe una sola línea:

```mp
while condición do Sentencia;
```

se recomienda **siempre utilizar** un bloque **begin**/**end**. 

Usar un estilo claro y mantener una buena legibilidad del código es fundamental para evitar errores en la lógica tu programa.


<br>


### Ejemplo

- ejemplo 1:

⚠️ Este ejemplo causa un **bucle infinito**, debido a que la sentencia termina en el punto y coma (`;`) despues de `writeln` y **no** al final del incremento de `i`;

```mp
program test_loops;
var i: integer;

begin
   // este ejemplo causa un bucle infinito ⚠️
   while i <= 5 do writeln("el número es:", i); i := i + 1;
end.
```

por eso siempre se recomienda el uso de un bloque **begin**/**end**. 

- ejemplo 2:

```mp
program test_loops;
var i: integer;

begin
  i := 1; // inicializamos el valor de `i` en 1

  while i <= 5 do
  begin
    writeln("el número es:", i);
    i := i + 1; // incrementamos `i`
  end
end.
```

en el ejemplo anterior, la condición `i <= 5` se evalúa en cada iteración del ciclo, y al final del bloque del ciclo se utiliza `i := i + 1;` como control, para poder validar la condición y poder salir del ciclo `while-do`.

### salida

la salida será algo como:
```
minipas v.1.9.0
el número es:  1
el número es:  2
el número es:  3
el número es:  4
el número es:  5
```


### Ejemplo

hagamos el clasico ejemplo de las tablas de multiplicar:

```mp
program test_loops;
var i,j: integer;
var resultado: integer;

begin
  writeln("tablas de multiplicar");
  writeln();

  // inicializamos el valor de `i` en 1
  i := 1;

  while (i <= 10) do // <- primer ciclo while
  begin
    // inicializamos el valor de `j` en 1
    // en cada iteración del ciclo principal
    j := 1;

    while j <= 10 do // <- ciclo while anidado
    begin
      resultado := i * j;
      writeln(i, " x ", j, " = ", resultado);
      j := j + 1; // incrementamos `j`
    end

    writeln("----------------------");
    i := i + 1; // incrementamos `i`
  end
end.
```

Aquí utilizamos ciclos anidados!.

### salida

la salida será algo como:
```
minipas v.1.9.0
tablas de multiplicar

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


## Recomendaciones

**Evitar bucles infinitos**: Verifica que las condiciones de terminación de tus ciclos estén bien definidas para evitar que tu programa se quede atrapado en un bucle infinito.

**Uso de variables de control**: Mantén el uso de la variable de control (como `i`) simple y único dentro del ciclo para evitar confusiones y errores en la lógica.

**Comentarios claros**: Agrega comentarios para explicar la lógica detrás de los ciclos, especialmente si tienen condiciones complejas o no evidentes.

**Ciclos anidados**: Si utilizas ciclos anidados, mantén la claridad en el control de las variables para evitar errores lógicos y verifica que la lógica sea comprensible.

**Evaluar el rendimiento**: Considera el impacto en el rendimiento de bucles anidados o extensos, y optimiza cuando sea necesario para evitar problemas de eficiencia.

**Pruebas exhaustivas**: Realiza pruebas con diferentes valores de entrada para asegurar que los ciclos funcionan como se espera en todos los casos, especialmente en los límites.

**Evitar lógica complicada dentro del ciclo**: Mantén las instrucciones dentro del ciclo simples y directas. Si la lógica es compleja, considera extraerla en funciones o procedimientos separados.
