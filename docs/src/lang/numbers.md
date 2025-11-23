# numéricos

## Integer

Un entero es un número sin componente fraccionario. Esto incluye todos los números sin parte decimal, tanto positivos como negativos.

### Rango de Valores

El rango específico de valores que puede almacenar un **integer** puede variar dependiendo de la implementación, pero generalmente en Pascal es de **-32,768** a **32,767**. En **minipas**, este valor se incrementa ya que se utiliza [i64](https://doc.rust-lang.org/std/primitive.i64.html) de Rust, lo que amplía significativamente el valor a usar en una variable.

- **Turbo Pascal v 7**
\begin{cases}
\text -2^{15} \text{ hasta }\ 2^{15} - 1 \\
\end{cases}

- **Minipas**
\begin{cases}
\text -2^{63} \text{ hasta }\ 2^{63} - 1 \\
\end{cases}

### Ejemplo

```mp
program hello_integers;
var
    a,b,c: integer;
begin
    writeln("Hello World in minipas!");
    writeln();
    writeln("el valor de a es:", a);
    writeln("el valor de b es:", b);
    writeln("el valor de c es:", c);
end.
```


## Real

Se refiere a un tipo de dato que se utiliza para representar números que tienen parte decimal. Estos tipos son esenciales para realizar cálculos que requieren precisión con decimales, como operaciones matemáticas complejas, mediciones, y otros casos donde los números enteros no son suficientes.

### Rango de Valores

El rango específico de valores que puede almacenar un **integer** puede variar dependiendo de la implementación, pero generalmente en Pascal es de **-32,768** a **32,767**. En **minipas**, este valor se incrementa ya que se utiliza [f64](https://doc.rust-lang.org/std/primitive.f64.html) de Rust, lo que amplía significativamente el valor a usar en una variable.

- **Turbo Pascal v 7**
\begin{cases}
\text 2.9 \times 10^{-39} \;\text{ hasta }\; 1.7 \times 10^{38} \\
\end{cases}

- **Minipas**
\begin{cases}
\text 1.8 \times 10^{308} \\
\end{cases}

### Ejemplo

```mp
program hello_real;
var
    a,b,c: real;
begin
    writeln("Hello World in minipas!");
    writeln();
    writeln("el valor de a es:", a);
    writeln("el valor de b es:", b);
    writeln("el valor de c es:", c);
end.
```

## Byte

Es un tipo especial de entero que tiene un rango de valores de **0** a **255**, por lo que no soporta valores negativos.

### Ejemplo

```mp
program hello_byte;
var
    edad: byte;
begin
    writeln("Hello World in minipas!");
    writeln();
    edad:= 256; //genera un error, ya que el valor maximo es 255
    writeln("el valor de edad es:", edad);
end.
```

<br>
<br>
 
**Nota:**

Para una mejor compresión recomendamos leer

- [Primitive Type i64](https://doc.rust-lang.org/std/primitive.i64.html)
- [Primitive Type f64](https://doc.rust-lang.org/std/primitive.f64.html)
- [pascal_data_types](https://www.tutorialspoint.com/pascal/pascal_data_types.htm)
