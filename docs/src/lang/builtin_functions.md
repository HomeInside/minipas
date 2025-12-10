# Funciones integradas

Las **funciones integradas** (o **built-in functions**) son funciones predefinidas que están disponibles en **Minipas** sin necesidad de que el programador las defina explícitamente. Estas funciones permiten realizar operaciones comunes y son parte del runtime del lenguaje, facilitando tareas de programación frecuentes.


## Características

**Disponibilidad**: Están siempre accesibles durante la programación sin necesidad de definiciones adicionales o importaciones.
  
**Tipos de Operaciones**: Realizan una variedad de tareas, incluyendo:
   - Manipulación de cadenas.
   - Operaciones matemáticas.
   - Entrada y salida de datos.
   - Conversión de tipos de datos.

y muchas cosas más!.


## Ventajas

- **Ahorro de Tiempo**: Permiten realizar tareas complejas sin necesidad de codificarlas desde cero.
- **Reducción de Errores**: Al utilizar funciones ya probadas y testeadas, se minimizan los errores en el código.
- **Facilidad de Mantenimiento**: El uso de funciones integradas puede hacer el código más limpio y fácil de mantener.


## Lista de funciones integradas

A continuación se describe la lista de funciones integradas en **Minipas**.

### entrada/salida

- **writeln**: Escribe un texto en la salida estándar y añade un salto de línea al final.
- **write**: Escribe un texto en la salida estándar sin añadir un salto de línea.
- **readln**: Lee una línea de texto de la entrada estándar y la asigna a una variable.
- **format**: Da formato a cadenas de texto, permitiendo la inclusión de variables de manera estructurada.

### constantes

- **PI**: Constante que representa el valor de π (**pi**), aproximadamente *3.141592653589793*.
- **E**: Constante que representa el número **e** (conocido en ocasiones como número de Euler o constante de Napier), la base del logaritmo natural, aproximadamente *2.718281828459045*.

### funciones matemáticas básicas

- **pow**: Calcula la potencia de un número dado, elevando una base a un exponente.
- **abs**: Devuelve el valor absoluto de un número, eliminando cualquier signo negativo.

### funciones trigonométricas

- **sin**: Calcula el seno de un ángulo dado en radianes.
- **cos**: Calcula el coseno de un ángulo dado en radianes.
- **tan**: Calcula la tangente de un ángulo dado en radianes.
- **cot**: Calcula la cotangente de un ángulo dado en radianes.
- **asin**: Calcula el arco seno, retornando el ángulo correspondiente a un valor dado.
- **acos**: Calcula el arco coseno, retornando el ángulo correspondiente a un valor dado.
- **atan**: Calcula el arco tangente, retornando el ángulo correspondiente a un valor dado.

### funciones logarítmicas y exponenciales

- **exp**: Calcula el valor de e elevado a la potencia de un número.
- **ln**: Devuelve el logaritmo natural (base e) de un número.
- **log10**: Calcula el logaritmo en base 10 de un número.
- **sqrt**: Devuelve la raíz cuadrada de un número.

### funciones auxiliares

- **floor**: Retorna el mayor número entero menor o igual al número dado.
- **ceil**: Retorna el menor número entero mayor o igual al número dado.
- **round**: Redondea un número al entero más cercano.
- **trunc**: Elimina la parte decimal de un número, retornando solo la parte entera.
- **fract**: Devuelve la parte fraccionaria de un número.
- **max**: Retorna el mayor de dos o más números proporcionados.
- **min**: Retorna el menor de dos o más números proporcionados.
- **sign**: Devuelve el signo de un número: 1 si es positivo, -1 si es negativo, y 0 si es cero.

### sistema

- **random**: Genera un número aleatorio dentro de un rango especificado.
- **exit**: Termina la ejecución del programa en cualquier punto.
- **clrscr**: Limpia la pantalla de la consola, eliminando todo el texto visible.
- **typeinfo**: Proporciona información sobre el tipo de dato de una variable en tiempo de ejecución.

### fechas

- **date**: Retorna la fecha actual del sistema en un formato específico.
- **time**: Retorna la hora actual del sistema en un formato específico.
- **date_time**: Devuelve la fecha y hora actuales combinadas en un solo valor.

### cadenas de caracteres

- **len**: Retorna la longitud de una cadena o la cantidad de elementos en un array.
- **concat**: Une dos o más cadenas en una sola cadena.

### conversiones de tipos

- **to_int**: Convierte un valor a su representación entera.
- **to_real**: Convierte un valor a su representación de punto flotante (real).
- **to_str**: Convierte un valor a su representación como cadena de texto.

### pruebas

- **assert**: Verifica si una condición es verdadera y genera un error si no lo es.
- **assert_equals**: Compara dos valores y asegura que sean iguales, generando un error si no lo son.
- **panic**: Termina el programa y despliega un mensaje de error en caso de una situación crítica.
