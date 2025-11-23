# strings

Se utilizan para representar cadenas de caracteres, el rango de un tipo de dato **string** depende de la longitud de la cadena y del uso de memoria.

Las cadenas de caracteres, son en realidad una secuencia de caracteres con una especificación de tamaño opcional. Los caracteres pueden ser numéricos, letras, espacios en blanco, caracteres especiales o una combinación de todos ellos. Extended Pascal proporciona numerosos tipos de objetos de cadena dependiendo del sistema y la implementación. Analizaremos los tipos de cadenas más comunes utilizados en los programas.

En **minipas** un tipo de dato **string** es internamente [String](https://doc.rust-lang.org/std/string/struct.String.html) de Rust, para asignar un valor se puede utilizar indistintamente comillas (`""`) o apóstrofos (`''`), aquí recomendamos el uso de comillas.

### Ejemplo

```mp
program hello_strings;
var
    nombres: string;
begin
    writeln("Hello World in minipas!");
    writeln();
    nombres := "john";
    writeln("el nombres es:", nombres);
end.
```

<br>
<br>
 
**Nota:**

Para una mejor compresión recomendamos leer

- [String](https://doc.rust-lang.org/std/string/struct.String.html)
- [pascal_strings](https://www.tutorialspoint.com/pascal/pascal_strings.htm)
