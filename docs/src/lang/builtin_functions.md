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
```
writeln("hola mundo en minipas!");
```
- **write**: Escribe un texto en la salida estándar sin añadir un salto de línea.
```
write("esto");
write("va");
write("en");
write("la");
write("misma");
write("linea");
// la salida será algo como:
esto va en la misma linea
```
- **readln**: Lee una línea de texto de la entrada estándar y la asigna a una variable.
```
writeln("Escriba un nombre");
name:= readln();
writeln("el nombre es:", name);
```
- **format**: Da formato a cadenas de texto, permitiendo la inclusión de variables de manera estructurada.

### constantes

- **PI**: Constante que representa el valor de π (**pi**), aproximadamente *3.141592653589793*.
```
writeln("el valor de PI es:", PI);
```
- **E**: Constante que representa el número **e** (conocido en ocasiones como número de Euler o constante de Napier), la base del logaritmo natural, aproximadamente *2.718281828459045*.
```
writeln("el valor de E es:", E);
```


### funciones matemáticas básicas

- **pow**: Calcula la potencia de un número dado, elevando una base a un exponente.
```
a := 2;
b := 8;
writeln("2^8 = ", pow(a, b));
```
- **abs**: Devuelve el valor absoluto de un número, eliminando cualquier signo negativo.
```
writeln("abs(0)", abs(0));
writeln("abs(-1)", abs(-1));
writeln("abs(-1.25)", abs(-1.25));
writeln("abs(1.234)", abs(1.234));
```

### funciones trigonométricas

- **sin**: Calcula el seno de un ángulo dado en radianes.
```
writeln("sin(-1.234)", sin(-1.234));
```
- **cos**: Calcula el coseno de un ángulo dado en radianes.
```
writeln("cos(-1.234)", cos(-1.234));
```
- **tan**: Calcula la tangente de un ángulo dado en radianes.
```
writeln("tan(PI/4) = ", tan(PI/4));
```
- **cot**: Calcula la cotangente de un ángulo dado en radianes.
```
writeln("cot(PI/4) = ", cot(PI/4));
```
- **asin**: Calcula el arco seno, retornando el ángulo correspondiente a un valor dado.
```
writeln("asin(1) = ", asin(1));
```
- **acos**: Calcula el arco coseno, retornando el ángulo correspondiente a un valor dado.
```
writeln("acos(0) = ", acos(0));
```
- **atan**: Calcula el arco tangente, retornando el ángulo correspondiente a un valor dado.
```
writeln("atan(1) = ", atan(1));
```
### funciones logarítmicas y exponenciales

- **exp**: Calcula el valor de e elevado a la potencia de un número.
```
writeln("exp(1) = ", exp(1));
```
- **ln**: Devuelve el logaritmo natural (base e) de un número.
```
writeln("ln(E) = ", ln(E));
```
- **log10**: Calcula el logaritmo en base 10 de un número.
```
writeln("log10(100) = ", log10(100));
```
- **sqrt**: Devuelve la raíz cuadrada de un número.
```
writeln("sqrt(16) = ", sqrt(16));
```


### funciones auxiliares

- **floor**: Retorna el mayor número entero menor o igual al número dado.
```
writeln("floor(3.7) = ", floor(3.7));
```
- **ceil**: Retorna el menor número entero mayor o igual al número dado.
```
writeln("ceil(3.2) = ", ceil(3.2));
```
- **round**: Redondea un número al entero más cercano.
```
writeln("round(2.6) = ", round(2.6));
```
- **trunc**: Elimina la parte decimal de un número, retornando solo la parte entera.
```
writeln("trunc(-2.71828) = ", trunc(-2.71828));
```
- **fract**: Devuelve la parte fraccionaria de un número.
```
⚠️ missing example
```
- **max**: Retorna el mayor de dos o más números proporcionados.
```
writeln("max(3,7) = ", max(3,7));
```
- **min**: Retorna el menor de dos o más números proporcionados.
```
writeln("min(3,7) = ", min(3,7));
```
- **sign**: Devuelve el signo de un número: 1 si es positivo, -1 si es negativo, y 0 si es cero.
```
writeln("sign(-5) = ", sign(-5));
writeln("sign(0) = ", sign(0));
writeln("sign(7) = ", sign(7));
```


### sistema

- **random**: Genera un número aleatorio dentro de un rango especificado, el valor debe ser mayor que cero (0)
```
writeln("funcion random 1:", random(0)); // <- genera un error ⚠️
writeln("funcion random 2:", random(1));
writeln("funcion random 3:", random(10));
writeln("funcion random 4:", random(1.234));
```
- **exit**: Termina la ejecución del programa en cualquier punto.
```
program testSys;
begin
	writeln("usando el modulo sys");
	exit();
	writeln("terminado"); // nunca se ejecuta
end.
```
- **clrscr**: Limpia la pantalla de la consola, eliminando todo el texto visible.
```
program testSys;
begin
    clrscr();
end.
```
- **typeinfo**: Proporciona información sobre el tipo de dato de una variable en tiempo de ejecución.
```
writeln("uso de Integer:", typeinfo(123));
writeln("uso de toString:", typeinfo(123.toString()));
```

### fechas

- **date**: Retorna la fecha actual del sistema en un formato específico.
```
writeln("Fecha actual:", date());
```
- **time**: Retorna la hora actual del sistema en un formato específico.
```
writeln("Hora actual:", time());
```
- **date_time**: Devuelve la fecha y hora actuales combinadas en un solo valor.
```
writeln("Fecha y hora:", date_time());
```


### cadenas de caracteres

- **len**: Retorna la longitud de una cadena de caracteres.
```
cadena:= " Anita lava la tinA ";
total := len(cadena);
writeln("la cantidad de caracteres es (len):", total);
```
- **concat**: permite unir dos ó más valores en una cadena de caracteres.
```
writeln("concatenar:", concat("caracteres", "y", PI, "ademas", "de", 1.234));
```

### conversiones de tipos

- **to_int**: Convierte un valor a su representación entera.
```
writeln("Ingresa un número entero");
num1 := to_int(readln());
```
- **to_real**: Convierte un valor a su representación de punto flotante (real).
```
writeln("Ingresa un número con parte decimal");
num2 := to_real(readln());
```
- **to_str**: Convierte un valor a su representación como cadena de texto.
```
writeln(42.to_str());
writeln(True.to_str());
```

### pruebas

- **assert**: Verifica si una condición es verdadera y genera un error si no lo es.
```
assert((4 % 2) = 0);
```
- **assert_equals**: Compara dos valores y asegura que sean iguales, generando un error si no lo son.
```
assert_equals(1, 1);
assert_equals(True, False);
```
- **panic**: Termina el programa y despliega un mensaje de error en caso de una situación crítica, espera un argumento.
```
program testSys;
begin
	panic("error al salir!");
	writeln("terminado"); // nunca se ejecuta
end.
```
