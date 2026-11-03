# Constantes

⚠️ En **Minipas** las Constantes están disponibles a partir de la versión **2.x**.

La definición de una constante se coloca en un bloque que comienza con la palabra clave **const**, seguida de las definiciones de las constantes como se muestra a continuación:

- ejemplo 1:
```mp
// cada constante en una linea
const MAX_INTENTOS = 3;
const DOUBLE_PI = 6.2830;
const TITLE = "constantes en MiniPas";
```

- ejemplo 2:
```mp
// en un bloque de definición
const
    MAX_INTENTOS = 3;
    DOUBLE_PI = 6.2830;
    TITLE = "constantes en MiniPas";
    MODO_DEBUG = True;
    _IS_ACTIVE = False;
```


## Tipos de constantes

**minipas** maneja **dos tipos** de constantes:
1. **Constantes internas del lenguaje** (builtins): `PI`, `E`, `TRUE`, `FALSE`.
2. **Constantes definidas por el usuario**, declaradas con la sección `const`.

### Ubicación

⚠️ La sección `const` debe ir **antes** de `var` en el programa principal

```mp
program <nombre>;
const   ...     // opcional
var     ...     // opcional
...             // procedimientos y funciones

begin
...
end.
```

No se permite intercalar `const` después de `var`.

## Tipos de datos

Las constantes definen el valor al momento de su declaración y el tipo de dato es [inferido automáticamente](https://es.wikipedia.org/wiki/Inferencia_de_tipos), revisa la sección de [Tipos de datos permitidos](../lang/data_types.html) en **Minipas**, los tipos de datos admitidos, para valores literales en constantes son:

| Tipo | Ejemplo | Tipo inferido |
|---|---|---|
| Entero | `const MAX_INTENTOS = 3;` | `Integer` |
| Real | `const DOUBLE_PI = 6.2830;` | `Real` |
| String | `const TITLE = "hola";` | `String` |
| Booleano | `const MODO_DEBUG = True;` | `Boolean` |

**Nota:**

Los booleanos en `const` usan la forma canónica `True`/`False`. Las formas `TRUE`/`FALSE` son [builtins](../lang/builtin_functions.md#constantes) del entorno y no se permite su uso como valor literal.

### Reglas para el uso de constantes

Consulta la [guía de estilo](../lang/style_guide.md#3-estilo-de-nombres) para más información.

- **Solo MAYÚSCULAS, dígitos y `_`**. Ejemplos válidos: `MAX_INTENTOS`, `PI_2`, `_PRIV`.
- Nombres mixtos o en minúsculas (`maxIntentos`, `_is_active`, `lower`) producen error en tiempo de parseo.
- No se pueden usar palabras reservadas del lenguaje como nombre.
- No se puede **redefinir** cualquier [builtins](../lang/builtin_functions.md) como `const`.
- inmutabilidad: Una constante **no puede reasignarse**.
- redeclaración: Una constante **no puede redeclarse**.
- aunque el tipo de dato se infiere en la declaración, las operaciones aritmeticas están restringidas al tipo de dato resultante.

### Ejemplo

```mp
program demoConstant;
const MAX = 1;

const
    MAX_INTENTOS = 3;
    DOUBLE_PI = 6.2830;
    TITLE = "constantes en MiniPas";
    MODO_DEBUG = True;
    _IS_ACTIVE = False;

var r: real;

begin

    writeln("el valor de MAX:", MAX);
    writeln("valor inicial de r:", r);
    
    r := 2.0;
    writeln("Circunferencia:", 2 * PI * r);
    r := 1.65;
    writeln("valor de r, después:", r);

    writeln();
    writeln("el valor de PI:", PI);
    writeln("el valor de E:", E);
    writeln("el valor de TRUE:", TRUE);
    writeln("el valor de FALSE:", FALSE);

    writeln();
    writeln("el valor de MAX_INTENTOS:", MAX_INTENTOS);
    writeln("el valor de DOUBLE_PI:", DOUBLE_PI);
    writeln("el valor de TITLE:", TITLE);
    writeln("el valor de MODO_DEBUG:", MODO_DEBUG);
    writeln("el valor de _IS_ACTIVE:", _IS_ACTIVE);
    writeln();
end.
```
### salida

la salida será algo como:
```
minipas v.2.0.0-alpha.0
el valor de MAX: 1
valor inicial de r: 0.0
Circunferencia: 12.566370614359172
valor de r, después: 1.65

el valor de PI: 3.141592653589793
el valor de E: 2.718281828459045
el valor de TRUE: true
el valor de FALSE: false

el valor de MAX_INTENTOS: 3
el valor de DOUBLE_PI: 6.283
el valor de TITLE: constantes en MiniPas
el valor de MODO_DEBUG: true
el valor de _IS_ACTIVE: false
```


<br>
<br>

**Nota:**

Este tipo de constantes definidas despues de la declaracion del programa serán **constantes globales** y estarán disponibles para cualquier función, procedimiento y el bloque principal del programa, durante su ejecución.

Es decir, las constantes globales se definen después del encabezado del programa, **antes** de la definición de [variables](../lang/vars.md).


<br>
Para una mejor compresión recomendamos leer

- [Primitive Type bool](https://doc.rust-lang.org/std/primitive.bool.html)
- [pascal_constants](https://www.tutorialspoint.com/pascal/pascal_constants.htm)
- [pascal_variable_types](https://www.tutorialspoint.com/pascal/pascal_variable_types.htm)
- [pascal_booleans](https://www.tutorialspoint.com/pascal/pascal_booleans.htm)
