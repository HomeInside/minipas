# Estructura de un programa
Un programa en **minipas**, sigue una estructura específica que incluye distintos componentes.

A continuación, se detallan los elementos clave:


### Nombre (declaración) del programa

el programa comienza con la palabra clave **program**, seguido de un nombre cualquiera, este nombre no debe:

- iniciar con numeros
- incluir espacios
- utilizar guión medio (**-**), 

se acepta el guión bajo (_)

```mp
program helloworld;
```

### Declaraciones de Variables

Se declaran las variables que se usarán en el programa, especificando su tipo de dato.
```mp
var r: real;
var result: real;
```

### Declaraciones de funciones y procedimientos

Incluye la lógica que se ejecutará cuando se llame al procedimiento y/ó función, en caso de que las definas en tu programa.

```mp
function calcular(radio:real): real;
begin
    return 2 * PI * radio;
end
```

### Cuerpo del programa
comienza con la palabra clave **begin** después de la declaración del programa.

```mp
begin
```

### Código del Programa
Aquí es donde se implementa la lógica principal, incluyendo operaciones, condiciones, ciclos, etc.

```mp
    r := 2.0;
    result := calcular(r);
    writeln("el valor de la Circunferencia es:", result);
```

### Final del Programa
Todo programa en **minipas** finaliza con la palabra **end** y un signo de punto (**.**), al final.

```mp
end.
```


## ejemplo completo

```mp
program helloworld;

var r: real;
var result: real;

function calcular(radio:real): real;
begin
    return 2 * PI * radio;
end

begin
    r := 2.0;
    result := calcular(r);
    writeln("el valor de la Circunferencia es:", result);
end.
```

## salida

la salida será algo como:
```
minipas v.1.9.0
el valor de la Circunferencia es: 12.566370614359172
```
