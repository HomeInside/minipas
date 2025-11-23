# MiniPas

**MiniPas** es un lenguaje de programación interpretado **inspirado en Turbo Pascal V7**, diseñado para ser sencillo, educativo y fácil de usar.

⚠️ **MiniPas** **no busca ser compatible** con [Pascal](https://es.wikipedia.org/wiki/Pascal_(lenguaje_de_programaci%C3%B3n)), [Turbo Pascal V7](https://es.wikipedia.org/wiki/Turbo_Pascal) ó [FreePascal](https://www.freepascal.org/): muchas características de Pascal **no están soportadas**.

Su objetivo principal es permitir experimentar con conceptos básicos de programación estructurada y scripting, ideal para aprendizaje ó pequeños proyectos.

☢️ **MiniPas** Es un lenguaje experimental y minimalista.

Ejemplo:

```pascal,ignore
// example.mp
program helloworld;
var r: real;

begin
    // comentarios de una linea
    writeln("Hello World in minipas!");
    r := 2.0;
    writeln(); // un espacio en blanco
    writeln("Circunferencia:", 2 * PI * r);// usamos PI
    {   comentarios de
        varias lineas
    }
    writeln("uso de pow:", pow(2,3));
    writeln("uso de sqrt", sqrt(16));
    writeln();
    writeln("platform:", platform());
    writeln("version:", version());
end.
```

proporciona una salida como la siguiente:

```bash
welcome to minipas v0.9.0
cargando archivo: 'example.mp'
Hello, World in minipas!

Circunferencia: 12.566370614359172
uso de pow: 8
uso de sqrt 4

platform: linux
version: 0.9.0 (MiniPas build, unknown, linux) [rustc]
```


## Licencia

Este proyecto está disponible bajo dos licencias distintas.

Puede seleccionar la licencia que mejor se ajuste a sus necesidades, entre las siguientes opciones:
- Licencia [Apache versión 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- Licencia [MIT](https://opensource.org/license/mit/)

El texto completo de estas licencias, se encuentran en los archivos **LICENSE-APACHE** y **LICENSE-MIT**
