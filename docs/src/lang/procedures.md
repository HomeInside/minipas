# procedimientos

Los procedimientos (a diferencia de las funciones) no devuelven un único valor, sino que realizan una serie de acciones, su propósito principal es ejecutar tareas específicas en lugar de calcular y retornar un valor único.


## Definición de un procedimiento

Se define utilizando la palabra clave **procedure**. La forma general de la definición de un procedimiento es la siguiente:

**ejemplo de un procedimiento que acepta argumentos:**

```mp
procedure nombre_procedimiento(param1: tipo1; param2: tipo2; ...);
// declaraciones locales usando `var`, si aplica

begin // <- el cuerpo del procedimiento, comienza con 'begin'
   sentencia1;
   sentencia2;
   ...
end // <- el cuerpo del procedimiento, finaliza con 'end'
```

<br>

**ejemplo de un procedimiento que no acepta argumentos:**

```mp
procedure nombre_procedimiento();
// declaraciones locales usando `var`, si aplica

begin // <- el cuerpo del procedimiento, comienza con 'begin'
   sentencia1;
   sentencia2;
   ...
end // <- el cuerpo del procedimiento, finaliza con 'end'
```

**Parámetros**: establecen el vínculo entre el programa de llamada y los identificadores del procedimiento, y también se denominan parámetros formales. Un parámetro es como un marcador de posición. Cuando se invoca a un procedimiento, se pasa un valor al parámetro. Este valor se denomina parámetro real o argumento. La lista de parámetros se refiere al tipo, el orden y el número de parámetros de un procedimiento. **El uso de dichos parámetros formales es opcional**. Estos parámetros pueden tener un tipo de dato estándar, un tipo de datos definido por el usuario o un tipo de datos de subrango.

**Argumento**: por otro lado, es el valor real que se pasa al procedimiento al momento de su invocación. Es el dato que ocupa la posición del parámetro definido en la cabecera.

resumiendo un poco:
- Parámetros son las variable en la declaración del procedimiento.
- Argumentos son los valores reales pasado en la llamada, usados en la invocación del procedimiento.

**Declaraciones locales:** las declaraciones locales se refieren a las declaraciones de etiquetas, variables, que se aplican únicamente al cuerpo del procedimiento.

**Cuerpo del procedimiento:** contiene un conjunto de sentencias que definen lo que hace el procedimiento. Siempre debe estar entre las palabras reservadas **begin** y **end**. Es la parte de un procedimiento donde se realizan todos los cálculos.


## Llamar a un procedimiento

Al crear un procedimiento, se define lo que debe hacer dicho procedimiento. Para utilizar un procedimiento, hay que **llamarlo**(ó invocarlo) para que realice la tarea definida. Cuando un programa llama a un procedimiento, el control del programa se transfiere al procedimiento llamado, este realiza la tarea definida y, cuando se ejecuta su ultima instrucción, devuelve el control del programa, al programa principal.

Llamar/invocar a un procedimiento implica:
- usar el nombre definido para él
- enviarle argumentos (si los acepta)

```
// si el procedimiento recibe argumentos
nombre_procedimiento(arg1, arg2, ...);
```

```
// si el procedimiento NO recibe argumentos
nombre_procedimiento();
```

<br>

veamos un ejemplo

### Ejemplo

```mp
program test_procedures;

procedure saludo(nombre: string);
begin
    writeln("Hola", nombre, "saludos desde un procedimiento");
end

begin
    saludo("Pedro");
end.
```

### salida

la salida será algo como:

```
minipas v.1.9.0
Hola Pedro saludos desde un procedimiento
```

**A tener en cuenta:**

Recuerda que los procedimientos (definidos usando `procedure`) **NO** devuelven un valor.


## Consideraciones

**Definición clara**: Asegúrate de que el procedimiento tenga un propósito específico y bien definido.

**Nombres descriptivos**: Usa nombres significativos que reflejen la acción que realiza el procedimiento (por ejemplo, `saludo`).

**Número de parámetros y argumentos**: Los argumentos deben coincidir en número y tipo con los parámetros definidos en el procedimiento.

Limita el número de parámetros para mantener la simplicidad.

**Valor de retorno**: Especifica el tipo de retorno del procedimiento claramente.

**Documentación**: Comenta el código para explicar qué hace el procedimiento, sus parámetros y  en general que hace.

**Evitando efectos secundarios**: Trata de evitar modificar variables globales a menos que sea necesario; esto ayuda a mantener el procedimiento predecible.

**Pruebas y validaciones**: Implementa pruebas para garantizar que el procedimiento produzca los resultados esperados con varios datos de entrada.

Considera agregar validaciones de entrada para evitar errores.

**Código modular**: Divide la lógica del programa en funciones y procedimientos más pequeños y manejables; esto facilita la lectura y el mantenimiento del código.

**Anidamiento**: Puedes llamar a otras funciones y procedimientos dentro de un procedimiento, pero ten cuidado con la profundidad anidada para no complicar la lógica.

**Gestión de errores**: Implementa manejo de errores donde sea necesario para evitar fallos inesperados durante la ejecución.

**Recursividad**: Si usas procedimientos recursivos, asegúrate de tener una condición de salida clara para evitar bucles infinitos, La clave está en gestionar adecuadamente las condiciones de salida para asegurar que la recursión se detenga de manera adecuada.
