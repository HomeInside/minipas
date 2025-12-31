# for

El ciclo **for for-do)** se utiliza para iterar sobre un rango específico de valores. Es ideal cuando se conoce de antemano cuántas veces se debe repetir el bloque de código.

La sintaxis del bucle **for-do** es la siguiente:


## to

La palabra clave **to** indica que el ciclo se ejecutará desde un `valor inicial` hasta un `valor final`, incrementando la variable de control en cada iteración.

```mp
for nombre_variable := valor_inicial to valor_final do 
  Sentencia;
```

usando un bloque **begin**/**end** (recomendado)

```mp
for nombre_variable := valor_inicial to valor_final do
  begin
    Sentencia1;
    Sentencia2;
    ...
  end
```

Donde:

`nombre_variable` especifica una variable de tipo ordinal, denominada variable de control o variable índice; los valores `valor_inicial` y `valor_final` son valores que puede tomar la variable de control; y **Sentencia** (ó Sentencia1, Sentencia2 ...) es el cuerpo del bucle `for-do`, que puede ser una instrucción simple o un grupo de instrucciones.


<br>

**A tener en cuenta:**

La variable `nombre_variable` debe ser declarada antes de ser utilizada en el ciclo `for`.

Para el ciclo/bucle **for** se recomienda utilizar siempre un bloque **begin**/**end**.

<br>


### Ejemplo

cuando existe una sola sentencia, se puede utilizar el ciclo `for-do` en una linea.

```mp
for i := 1 to 5 do writeln("hello foor loop in minipas", i);
```

sin embargo, por legibilidad, siempre recomendaremos el uso de un bloque **begin**/**end**.

```mp
for i := 1 to 5 do
begin
  writeln("hello foor loop in minipas", i);
end
```

### salida

la salida para ambos casos, será algo como:
```
minipas v.1.9.0
hello foor loop in minipas 1
hello foor loop in minipas 2
hello foor loop in minipas 3
hello foor loop in minipas 4
hello foor loop in minipas 5
```


## down-to

La palabra clave **downto** se utiliza para iterar de manera decreciente, comenzando desde un `valor final` hasta un `valor inicial`, decrementando la variable de control en cada iteración.


```mp
for nombre_variable := valor_final downto valor_inicial do 
  Sentencia;
```

usando un bloque **begin**/**end** (recomendado)

```mp
for nombre_variable := valor_final downto valor_inicial  do
  begin
    Sentencia1;
    Sentencia2;
    ...
  end
```

Donde:

`nombre_variable` especifica una variable de tipo ordinal, denominada variable de control o variable índice; los valores `valor_inicial` y `valor_final` son valores que puede tomar la variable de control; y **Sentencia** (ó Sentencia1, Sentencia2 ...) es el cuerpo del bucle `for-do`, que puede ser una instrucción simple o un grupo de instrucciones.


<br>

**A tener en cuenta:**

La variable `nombre_variable` debe ser declarada antes de ser utilizada en el ciclo `for`.

Para el ciclo/bucle **for** se recomienda utilizar siempre un bloque **begin**/**end**.

<br>


### Ejemplo

cuando existe una sola sentencia, se puede utilizar el ciclo `for-do` en una linea.

```mp
for i := 5 downto 1 do writeln("hello foor loop in minipas", i);
```

sin embargo, por legibilidad, siempre recomendaremos el uso de un bloque **begin**/**end**.

```mp
for i := 5 downto 1 do
begin
  writeln("hello foor loop in minipas", i);
end
```

### salida

la salida para ambos casos, será algo como:
```
minipas v.1.9.0
hello foor loop in minipas 5
hello foor loop in minipas 4
hello foor loop in minipas 3
hello foor loop in minipas 2
hello foor loop in minipas 1
```


## Recomendaciones

**Definición clara de los límites**: Asegúrate de que los límites de inicio y fin sean claros y correctos para evitar bucles que no se comportan como se esperaba.

**Evitar bucles infinitos**: Verifica que las condiciones de terminación de tus ciclos estén bien definidas para evitar que tu programa se quede atrapado en un bucle infinito.

**Incrementos y decrementos correctos**: En ciclos `for`, asegúrate de que el incremento (usando `to`) o el decremento (usando `downto`) sean apropiados para la lógica del programa.

**Uso de variables de control**: Mantén el uso de la variable de control (como `i`) simple y único dentro del ciclo para evitar confusiones y errores en la lógica.

**Comentarios claros**: Agrega comentarios para explicar la lógica detrás de los ciclos, especialmente si tienen condiciones complejas o no evidentes.

**Ciclos anidados**: Si utilizas ciclos anidados, mantén la claridad en el control de las variables para evitar errores lógicos y verifica que la lógica sea comprensible.

**Preferir `for` para iteraciones contables**: Usa `for-do` cuando sepas de antemano cuántas veces necesitas iterar, ya que es más legible y fácil de entender.

**Evaluar el rendimiento**: Considera el impacto en el rendimiento de bucles anidados o extensos, y optimiza cuando sea necesario para evitar problemas de eficiencia.

**Pruebas exhaustivas**: Realiza pruebas con diferentes valores de entrada para asegurar que los ciclos funcionan como se espera en todos los casos, especialmente en los límites.

**Evitar lógica complicada dentro del ciclo**: Mantén las instrucciones dentro del ciclo simples y directas. Si la lógica es compleja, considera extraerla en funciones o procedimientos separados.
