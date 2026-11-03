{
    Minipas maneja dos tipos de constantes:
    - internas del lenguaje: 'PI','E','TRUE','FALSE'
    - definidas por el usuario, utilizando 'const',
        estas deben ser declaradas en Mayúsculas y antes
        de la sección 'var', el orden de definición es:
        - const
        - var
        - ...
        consulta la documentación para mejor comprensión
        de su uso, en `https://cutt.ly/minipas`
}

program demoConstant;
const MAX = 1;
const
    MAX_INTENTOS = 3;
    DOUBLE_PI = 6.2830;
    TITLE = "constantes en MiniPas";
    MODO_DEBUG = True;
    _IS_ACTIVE = False;
    //TRUE = False; // ❌ error, no se puede utilizar una builtin
    //sin = 1; // ❌ error, no se puede utilizar una builtin
    //FALSE = True; // ❌ error, no se puede utilizar una builtin

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
    writeln("suma utilizando constantes:", MAX_INTENTOS + DOUBLE_PI);
    writeln("resta utilizando constantes:", DOUBLE_PI - MAX);

    writeln();
    //MAX_INTENTOS := 1; // ❌ error, no se puede modificar la constante

    writeln("validando con 'assert'");
    assert(MAX_INTENTOS = 3);
    assert(DOUBLE_PI =  6.2830);
    assert(TITLE = "constantes en MiniPas");
    assert(MODO_DEBUG = True);
    assert(_IS_ACTIVE = False);
    writeln("validando con 'assert' OK");

    writeln();
    writeln("validando con 'assert_equals'");
    assert_equals(typeinfo(MAX_INTENTOS), "Integer");
    assert_equals(typeinfo(DOUBLE_PI), "Real");
    assert_equals(typeinfo(TITLE), "String");
    assert_equals(typeinfo(MODO_DEBUG), "Boolean");
    assert_equals(typeinfo(_IS_ACTIVE), "Boolean");
    writeln("validando con 'assert_equals' OK");
end.
