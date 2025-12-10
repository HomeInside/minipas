# sentencia if

## Sentencia if-then

La instrucción `if-then` es la forma más simple de instrucción de control, utilizada con frecuencia en la toma de decisiones y para cambiar el flujo de control de la ejecución del programa.

La sintaxis de la instrucción `if-then` es:

```mp
if condición then sentencias;
```

Donde **condición** es una condición booleana o relacional y **sentencias** es una instrucción simple o compuesta. Un ejemplo de una instrucción if-then es:

```
if a <= 20 then
   c := c+1;
```

Si la condición de la expresión booleana se evalúa como **verdadera**, se ejecutará el bloque de código **dentro de la instrucción if**. Si la expresión booleana se evalúa como **falsa**, se ejecutará el primer conjunto de código **después del final de la instrucción if**.


Miremos con un ejemplo completo:

### Ejemplo

```mp
program ifExample;

var a: integer;

begin
  a:= 10;

  // se puede usar parentesis
  //
  // if (a < 20) then
  //
  if a < 20 then
      writeln("a menor que 20");
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
a menor que 20
```


## Sentencia if-then-else

Una instrucción `if-then` puede ir seguida de una instrucción `else` **opcional**, que se ejecuta cuando la expresión booleana es **falsa**.

La sintaxis de la instrucción `if-then-else` es:

```mp
if condición then Sentencias1; else Sentencias2;
```

Donde **Sentencias1** y **Sentencias2** son sentencias diferentes.

En las sentencias **if-then-else**, cuando la condición de prueba es **verdadera**, se ejecuta la sentencia **Sentencias1** y se omite **Sentencias2**, cuando la condición de prueba es **falsa**, se omite **Sentencias1** y se ejecuta la sentencia **Sentencias2**.


<br>

**A tener en cuenta:**

En **minipas**, cada sentencia **debe** terminar en **punto y coma** (`;`).

### Ejemplo

```mp
program ifElseExample;
var
  a : integer;

begin
  a := 100;
  // se puede usar parentesis
  //
  // if (a < 20) then
  //
  if a < 20 then
      writeln("a es menor que 20");
  else
      writeln("a es mayor, el valor de a es:", a);
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
a es mayor, el valor de a es: 100
```


## Sentencias if-then else if-then

Una instrucción `if-then` puede ir seguida de una instrucción `else if-then-else` **opcional**, lo cual resulta muy útil para comprobar varias condiciones utilizando una única instrucción `if-then-else`.

### Ejemplo

```mp
program ifElse_ifElseExample;
var
  a: integer;

begin
  a := 100;

  if a = 10 then
      writeln("el valor de a es: 10");
  else if a = 20 then
      writeln("el valor de a es: 20");
  else if a = 30 then
      writeln("el valor de a es: 30");
  else
      writeln("Ninguno de los valores coincide.");
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
Ninguno de los valores coincide.
```


## Instrucciones simples ó compuestas

Una instrucción simple es una línea de código que realiza una única acción. Estas instrucciones son directas:

```
if a <= 20 then
   c := c+1; // <- instrucción simple
```

Una instrucción compuesta es un bloque de código que puede contener cero o más instrucciones simples, ó incluso instrucciones compuestas dentro (anidadas). Se utiliza para agrupar varias acciones bajo un mismo control de flujo.

La sintaxis de una instrucción compuesta incluye **begin** y **end**, que delimitan el bloque de instrucciones:

```
if a <= 20 then begin // <- 'begin' indica el inicio del bloque
   c := c+1;
   writeln("El valor de C es:", c);
end // <- 'end' indica el fin del bloque
```

<br>

**A tener en cuenta:**

En **minipas**, cada bloque **begin** debe terminar con un **end**, pero este `end` **no** debe llevar **punto y coma** (`;`) al final.

<br>

Los ejemplos anteriores se componen de sentencias simples, ahora veamos un ejemplo utilizando sentencias compuestas:

### Ejemplo

```mp
program ifStatemensExample;

var a: integer;

begin
  a := 50;

  if a = 10 then begin
      writeln("el valor de a es 10");
      a := a + 1;
      writeln("el valor de a es:", a);
  end
  else if a = 20 then begin
      writeln("el valor de a es 20");
      a := a * 2;
      writeln("el valor de a es:", a);
  end
  else begin
      writeln("el valor de a es diferente de 10 y 20");
      a := a - 1;
      writeln("el valor de a es:", a);
  end
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
el valor de a es diferente de 10 y 20
el valor de a es: 49
```

<br>

**A tener en cuenta:**

Se recomienda usar un bloque **begin**/**end**, cuando se tenga dos ó más instrucciones en un bloque `if-then`, `if-then-else` etc...


## Sentencias if anidadas

Siempre es legal anidar sentencias `if-else`, lo que significa que se puede utilizar una sentencia if o `else if` dentro de otra sentencia `if` o `else if`. **Minipas** permite anidar hasta cualquier nivel, sin embargo, las sentencias anidadas pueden volverse complicadas, así que es importante mantener el código legible y organizado.

La sintaxis de una instrucción `if` anidada es la siguiente:

```mp
if condicion_1 then
   if condicion_2 then sentencias1;
else
   sentencias2;
```

otro ejemplo:

```mp
if condicion_1 then
   if condicion_2 then begin
      sentencias1
      sentencias2
   end
else begin
   if condicion_3 then begin
     if condicion_4 then begin
       sentencias3;
       sentencias4;
     end
   end
end
```

**A tener en cuenta:**

Recuerda que las sentencias anidadas pueden volverse complicadas, así que es importante mantener el código legible y organizado.

### Ejemplo

```
program NestedIfElseExample;
var
   a, b: integer;

begin
   a := 100;
   b := 200;

   if a = 100 then
      if b = 200 then
         writeln("el valor de a es 100 y el valor de b es 200");
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
el valor de a es 100 y el valor de b es 200
```

### Uso de if Anidadas en Situaciones Prácticas

Las sentencias if anidadas son útiles, por ejemplo, en situaciones donde deban verificarse múltiples criterios para cumplir con ciertas condiciones lógicas. Esto puede incluir:

- Validaciones en entradas de usuario.
- Cálculos que dependen de diferentes rangos.
- Procesamiento de datos donde ciertos parámetros dependen de otros.

### Consideraciones

- **Legibilidad**: Las sentencias anidadas pueden volverse complicadas, así que es importante mantener el código legible y organizado. si anidas mucho, considera el uso de bloques **begin**/**end**.
- **Indentación**: Usar una buena indentación ayuda a visualizar mejor la estructura anidada y la lógica del flujo del programa.
- **Manejo de Errores**: Las sentencias anidadas pueden dificultar la identificación de errores, así que es aconsejable descomponer problemas complejos en funciones más manejables cuando sea posible.
