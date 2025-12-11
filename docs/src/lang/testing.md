# Testing

Aunque Turbo Pascal V7 (del cual se inspira **Minipas**) no existía la función `assert`, en **Minipas** se incluye para comprobar condiciones y manejar errores de una manera más controlada en tu programa.

**A tener en cuenta:**

El uso de `assert` y de `assert_equals` causan **pánico** (**el programa se detiene**) si las condiciones no se cumplen:

<br>
    
## **assert**

verifica que una condición sea verdadera. Si la condición no se cumple, se lanza una excepción y se detiene la ejecución del programa, indicando que se ha producido un error.

### Ejemplo

```mp
program testExample;
var a,b:integer;
begin
    a:= 1;
    b:= 1;
    
    assert(2 * 2 = 4);
    assert(a<>b); // <- falla
end.
```

### Salida

la salida es algo como:

```
minipas v.1.9.0

thread 'main' panicked at src\runtime\std_lib\testing.rs:11:13:
Assertion failed: expected `true`, got `false`

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<br>
    
## **assert_equals**

compara dos valores: uno esperado y otro real. Si los valores no son iguales, se lanza una excepción y se detiene la ejecución del programa, indicando que se ha producido un error.

### Ejemplo

```mp
program testSys;
var msg1: string;
begin

msg1 := "Hola";
assert_equals(msg1.lower(), "hola");
writeln();

writeln("el valor de PI es:", PI);
assert_equals(PI, 3.141592); // <- falla
end.
```

### Salida

la salida es algo como:

```
minipas v.1.9.0

el valor de PI es: 3.141592653589793

thread 'main' panicked at src\runtime\std_lib\testing.rs:46:9:
Assertion failed: expected left: `3.141592653589793`, got right: `3.141592`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
