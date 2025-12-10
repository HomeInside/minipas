# boleanos

Representan valores de verdad. Generalmente, un boleano puede tener solo dos posibles valores.

En **Minipas**, un valor boleano se representa de la siguiente manera, con la primera letra en Mayúscula:

- **True** (verdadero)
- **False** (falso)

Recuerda que los valores booleanos son lógicos y no numéricos. Esto lo hace útil para la toma de decisiones en la lógica de programación y control de flujo.


### Ejemplo

```mp
program hello_booleans;

var
esMayorDeEdad: boolean;

begin
    writeln("esMayorDeEdad al inicio es:", esMayorDeEdad);
    esMayorDeEdad := True;

    if esMayorDeEdad then
        writeln('Es mayor de edad.');
    else
        writeln('No es mayor de edad.');
end.
```

<br>
<br>
 
**Nota:**

Para una mejor compresión recomendamos leer

- [Primitive Type bool](https://doc.rust-lang.org/std/primitive.bool.html)
- [pascal_booleans](https://www.tutorialspoint.com/pascal/pascal_booleans.htm)
