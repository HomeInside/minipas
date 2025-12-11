# Guía de estilo

**Minipas 1.9.0**

Este documento define el **estilo oficial de código** para Minipas.
Su objetivo es garantizar que todos los programas escritos en Minipas sean:

* claros
* consistentes
* fáciles de leer
* simples de mantener
* didácticos

La guía se inspira en prácticas de Free Pascal, GNU Pascal, Object Pascal y otras guías de estilo.


# 1. Filosofía del lenguaje

**Minipas** toma muchas caracteristicas y referencias del lenguage Pascal, sin embargo, hay que recalcar algunas cosas: 

- ⚠️ **MiniPas** **no busca ser compatible** con [Pascal](https://es.wikipedia.org/wiki/Pascal_(lenguaje_de_programaci%C3%B3n)), [Turbo Pascal V7](https://es.wikipedia.org/wiki/Turbo_Pascal) ó [FreePascal](https://www.freepascal.org/): muchas características de Pascal **no están soportadas**.

- **MiniPas** es un **lenguaje interpretado, inspirado en Turbo Pascal V7**, diseñado para ser sencillo, educativo y fácil de usar, la gran mayoría (si no todos...) de los dialectos de Pascal son **compilados**.

- Pascal es un lenguaje que no distingue entre mayúsculas y minúsculas (Case Insensitive), lo que significa que puedes escribir tus variables, funciones y procedimientos con cualquiera de las dos. En **Minipas** no se acepta esto, todas las palabras claves deben escribirse en **minúsculas**, *salvo muy pequeñas excepciones*.

- En **minipas**, cada sentencia **debe** terminar en **punto y coma** (`;`).

- ☢️ **MiniPas** Es un lenguaje experimental y minimalista.

- **Minipas** es un lenguaje **minimalista** y **educativo**, por lo que la guía enfatiza:

    * simplicidad sobre optimización
    * claridad sobre trucos
    * coherencia sobre preferencias personales

Cuando existan dudas, elegir siempre **lo más legible**.


# 2. Organización de archivos

### 2.1 Un archivo, un programa

Cada archivo de **Minipas** debe contener **un único programa** o módulo.

### 2.2 Nombre del archivo

Utilizar siempre **minúsculas** y evitar espacios:

```
calculadora.mp
mi_programa.mp
```

### 2.3 Estructura básica del archivo

Un archivo debe seguir este orden lógico:

```
# Comentarios iniciales / propósito
# Declaraciones globales (si las hay)
# Programa principal
```

Mantener el archivo limpio y sin secciones huérfanas.

---

# 3. Estilo de nombres

**Minipas** adopta un estilo [**snake_case**](https://es.wikipedia.org/wiki/Snake_case) simple, consistente y fácil de leer.

### 3.1 Variables

* minúsculas
* palabras separadas con `_`
* nombres descriptivos

Ejemplos:

```
contador
suma_total
nombre_usuario
```

### 3.2 Constantes

⚠️ **Minipas** no admite constantes, cuando llegue el momento...

* MAYÚSCULAS
* separadas por `_`

Ejemplo:

```
MAX_INTENTOS = 3;
PI = 3.14159;
```

### 3.3 Funciones / procedimientos

Igual que las variables:

```
leer_archivo();
calcular_area(base, altura);
```

### 3.4 Boleanos

De ser necesario, preferir nombres que indiquen verdadero/falso:

```
es_valido
tiene_error
modo_debug
```

---

# 4. Formato e indentación

### 4.1 Indentación estándar

Usar **2 espacios** por nivel.
No usar tabs.

Ejemplo:

```mp
if x > 0 then
  writeln("positivo");
```

### 4.2 Líneas

* longitud recomendada: **máximo 120 caracteres**
* sin espacios al final

### 4.3 Espacios alrededor de operadores

Correcto:

```
a := b + c;
x := y * 2;
```

Incorrecto:

```
a:=b+c;
```

### 4.4 Paréntesis

Sin espacios internos:

Correcto:

```
imprimir(x + 1);
```

Incorrecto:

```
imprimir( x + 1 );
```


# 5. Bloques y estructuras de control

### 5.1 Bloques siempre multilínea

Siempre abrir un bloque en la línea siguiente.

#### Correcto:

```
program test_loops;
var i: integer;

begin
  i := 1;

  while i <= 5 do
  begin
    writeln("el número es:", i);
    i := i + 1;
  end
end.
```

#### Incorrecto:

```
program test_loops;
var i: integer;

begin
   while i <= 5 do writeln("el número es:", i); i := i + 1;
end.
```

### 5.2 Estructura de control estándar

#### If:

```
if condicion then
  Sentencia1;
else
  Sentencia2;
```

#### Bucle:

```
while condicion do
begin
  Sentencia1;
  Sentencia2;
  ...
  SentenciaN;
end
```

### 5.3 Evitar `goto`

⚠️ **Minipas** no admite la sentencia **goto**.


# 6. Comentarios

### 6.1 Comentarios de línea

Usar `//`.

```
// Este bucle acumula el total
```

### 6.2 Comentar el *por qué*, no el *qué*

❌ No útil:

```
// suma dos números
suma := a + b;
```

✔ Mejor:

```
// Reiniciamos suma para evitar arrastre de valores previos
suma := a + b;
```

### 6.3 Comentarios de bloque

Permitido para documentación o explicaciones largas:

```
{
 Este módulo valida el archivo de entrada,
 convierte el contenido a una estructura útil
 y luego ejecuta el programa principal.
}
```

---

# 7. Buenas prácticas del lenguaje

### 7.1 Código simple y claro

Evitar expresiones complejas.
Evitar magia o algoritmos innecesariamente enredados.

### 7.2 No duplicar código

Extraer funciones cuando sea posible.

### 7.3 Evitar efectos colaterales ocultos

Una función debería hacer **solo una cosa**.

### 7.4 Inicializar variables

No depender de valores implícitos.

### 7.5 Manipulación defensiva

Comprobar casos especiales si aplica (divisiones, rangos, etc.).

---

# 8. Ejemplos oficiales

## 8.1 Ejemplo completo

```
{
	La sucesión de Fibonacci es una sucesión `Fn` de números
	naturales definida de forma recursiva.

	- https://www.calculatorsoup.com/calculators/discretemathematics/fibonacci-calculator.php
	- https://rosettacode.org/wiki/Fibonacci_sequence
}

program test_fibonacci;
var numero, resultado: integer;

function fibonacci(num:integer): integer;
begin
	if num < 2 then
		return num;
	return fibonacci(num-1) + fibonacci(num-2);
end

begin
	writeln("calcular numeros de fibonacci");
	writeln("Introduce un número entero (no muy grande)");
	numero := to_int(readln());
	resultado := fibonacci(numero);
	writeln("el fibonacci de", numero, "es",  resultado);
end.
```

---

# 9. Errores comunes a evitar

* nombres como `x`, `t`, `c1` sin contexto
* bloques en una sola línea
* comentarios redundantes
* confiar en comportamiento no definido
* mezclar estilos
* indentación inconsistente

# 10. Resumen rápido (cheat-sheet)

* **Indentación:** 2 espacios
* **Nombres:** snake_case
* **Constantes:** cuando se soporten serán en MAYÚSCULAS
* **Bloques:** siempre multilínea
* **Comentarios:** explicar el porqué
* **Longitud de línea:** 120 caracteres
* **Priorizar:** claridad, simpleza, consistencia
