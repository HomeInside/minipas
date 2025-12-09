# funciones

Una función es un grupo de sentencias que juntas realizan una tarea.

## Definición de una función

Una declaración de función le indica al interprete el nombre, el tipo de retorno y los parámetros de una función. Una definición de función proporciona el cuerpo real de la función.

Las funciones pueden aceptar (ó no) parámetros (que son variables definidas en la cabecera de la función) y estos parámetros son utilizados para recibir valores al momento de invocar la función.

En **Minipas**, una función se define utilizando la palabra clave **function**.

La forma general de una definición de función es la siguiente:

**ejemplo de función que acepta argumentos:**

```mp
function nombre_funcion(param1: tipo1; param2: tipo2; ...): tipo_de_retorno;
// declaraciones locales usando `var`, si aplica

begin // <- el cuerpo de la funcion, comienza con 'begin'
   sentencia1
   sentencia2
   ...
   return valor, ó sentencia
end // <- el cuerpo de la funcion, finaliza con 'end'
```

<br>

**ejemplo de función que no acepta argumentos:**

```mp
function nombre_funcion(): tipo_de_retorno;
// declaraciones locales usando `var`, si aplica

begin // <- el cuerpo de la funcion, comienza con 'begin'
   sentencia1
   sentencia2
   ...
   return valor, ó sentencia
end // <- el cuerpo de la funcion, finaliza con 'end'
```

**Parámetros**: establecen el vínculo entre el programa de llamada y los identificadores de función, y también se denominan parámetros formales. Un parámetro es como un marcador de posición. Cuando se invoca una función, se pasa un valor al parámetro. Este valor se denomina parámetro real o argumento. La lista de parámetros se refiere al tipo, el orden y el número de parámetros de una función. **El uso de dichos parámetros formales es opcional**. Estos parámetros pueden tener un tipo de dato estándar, un tipo de datos definido por el usuario o un tipo de datos de subrango.

**Argumento**: por otro lado, es el valor real que se pasa a la función o procedimiento al momento de su invocación. Es el dato que ocupa la posición del parámetro definido en la cabecera.

resumiendo un poco:
- Parámetros son las variable en la declaración de la función.
- Argumentos son los valores reales pasado en la llamada, usados en la invocación de la función.

**Tipo de retorno:** todas las funciones **deben devolver un valor**, por lo que a todas las funciones se les debe asignar un tipo. El tipo de función es el tipo de datos del valor que devuelve la función. Puede ser un tipo estándar, escalar, definido por el usuario o de subrango, pero no puede ser un tipo estructurado.

**Declaraciones locales:** las declaraciones locales se refieren a las declaraciones de etiquetas, variables, que se aplican únicamente al cuerpo de la función.

**Cuerpo de la función:** contiene un conjunto de sentencias que definen lo que hace la función. Siempre debe estar entre las palabras reservadas **begin** y **end**. Es la parte de una función donde se realizan todos los cálculos.

**Retorno**: cada función termina con la palabra reservada **return** y una valor asociado a un tipo de dato ó una expresión.


## Llamar a una función

Al crear una función, se define lo que debe hacer dicha función. Para utilizar una función, hay que **llamarla**(ó invocarla) para que realice la tarea definida. Cuando un programa llama a una función, el control del programa se transfiere a la función llamada, esta realiza la tarea definida y, cuando se ejecuta su instrucción de retorno (**return**), devuelve el control del programa, al programa principal.

Llamar/invocar a una función implica:
- usar el nombre definido para ella
- enviarle argumentos (si los acepta)
- esperar (ó no) una respuesta


```
// si la función recibe argumentos
nombre_funcion(arg1, arg2, ...);
```

```
// si la función NO recibe argumentos
nombre_funcion();
```

<br>
veamos con un ejemplo

### Ejemplo

```mp
program test_functions;

// nombre de la función, parametros y tipo de retorno
function sumar(a:integer; b:integer): integer;

// declaraciones locales
var c: integer;

//cuerpo de la función
begin
    writeln("el valor de a es:", a);
    writeln("el valor de b es:", b);
    c := a + b;
    writeln("el valor de c es:", c);
    
    // valor ó sentencia de retorno
    return c;

end // fin de la función

// bloque principal del programa
begin
    writeln();
    writeln("3 + 5 = ", sumar(3, 5)); // <- llamada a la función
    writeln("10 + 20 = ", sumar(10, 20)); // <- llamada a la función
    writeln();
    sumar(2, 9); // <- llamada a la función
    writeln();
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0

el valor de a es: 3
el valor de b es: 5
el valor de c es: 8
3 + 5 =  8
el valor de a es: 10
el valor de b es: 20
el valor de c es: 30
10 + 20 =  30

el valor de a es: 2
el valor de b es: 9
el valor de c es: 11
```

**A tener en cuenta:**

Recuerda que las funciones (definidas usando `function`) **siempre** devuelven un valor.

### valor de retorno

Como se especificó anteriormente, las funciones **siempre** devuelven un valor, asi que podemos "capturar" ese valor en una variable y hacer alguna operación ó simplemente mostralo.

### Ejemplo

el mismo ejemplo sin comentarios, y omitiendo mostrar muchos valores por la consola.

```mp
program test_functions;
var resultado: integer;

function sumar(a:integer; b:integer): integer;
var c: integer;

begin
    c := a + b;
    return c;
end

begin
    resultado := sumar(2, 9);
    writeln("el resultado de la suma es:", resultado);
end.
```

### salida

la salida será algo como:
```
minipas v.1.9.0
el resultado de la suma es: 11
```

## Consideraciones
**Definición Clara**: Asegúrate de que la función tenga un propósito específico y bien definido.

**Nombres Descriptivos**: Usa nombres significativos que reflejen la acción que realiza la función (por ejemplo, `sumar`).

**Número de Parámetros y Argumentos**: Los argumentos deben coincidir en número y tipo con los parámetros definidos en la función o procedimiento.

Limita el número de parámetros para mantener la simplicidad.

**Valor de Retorno**: Especifica el tipo de retorno de la función claramente y asegúrate de que devuelve un valor adecuado.

**Documentación**: Comenta el código para explicar qué hace la función, sus parámetros y valor de retorno.

**Evitando efectos Secundarios**:Trata de evitar modificar variables globales a menos que sea necesario; esto ayuda a mantener la función predecible.

**Pruebas y validaciones**: Implementa pruebas para garantizar que la función produzca los resultados esperados con varios datos de entrada.

Considera agregar validaciones de entrada para evitar errores.

**Código modular**: Divide la lógica del programa en funciones más pequeñas y manejables; esto facilita la lectura y el mantenimiento del código.

**Anidamiento**: Puedes llamar a otras funciones dentro de una función, pero ten cuidado con la profundidad anidada para no complicar la lógica.

**Gestión de errores**: Implementa manejo de errores donde sea necesario para evitar fallos inesperados durante la ejecución.

**Recursividad**: Si usas funciones recursivas, asegúrate de tener una condición de salida clara para evitar bucles infinitos.
