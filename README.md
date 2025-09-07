# MiniPas

MiniPas es un lenguaje de programación interpretado **inspirado en Tubo Pascal V7**, diseñado para ser **simple, educativo y fácil de usar**.

⚠️ MiniPas **no busca ser compatible** con [Pascal](https://es.wikipedia.org/wiki/Pascal_(lenguaje_de_programaci%C3%B3n)), [Turbo Pascal](https://es.wikipedia.org/wiki/Turbo_Pascal) ó [FreePascal](https://www.freepascal.org/): muchas características del Pascal clásico **no están soportadas**.

Su objetivo principal es permitir experimentar con conceptos básicos de programación estructurada y scripting, ideal para aprendizaje o pequeños proyectos.

Actualmente es un lenguaje experimental y minimalista.

Ejemplos:

```mp
program helloworld;

begin
    writeln("Hello, World in minipas!");
end.
```

## Como Contribuir

### Clona este repositorio:

```bash
$ git clone git@gitlab.com:HomeInside/minipas.git
```

### Construye

```bash
cd minipas
```

```bash
cargo build --release
```

### Prueba

```bash
$ minipas examples/hello_world.mp
```

Realiza las modificaciones que creas convenientes y envíanos un [pull request.](https://gitlab.com/HomeInside/minipas/-/merge_requests)

Para cualquier duda, comentario, sugerencia ó aporte, dirigete a la seccion de [issues.](https://gitlab.com/HomeInside/minipas/-/issues)

Antes de abrir un issue nuevo, revisa los ya existentes en busca de una solución (posiblemente ya planteada) para el problema que se te presenta.


## Licencia

Este proyecto está disponible bajo dos licencias distintas.

Puede seleccionar la licencia que mejor se ajuste a sus necesidades, entre las siguientes opciones:
- Licencia [Apache versión 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- Licencia [MIT](https://opensource.org/license/mit/)

El texto completo de estas licencias, se encuentran en los archivos **LICENSE-APACHE** y **LICENSE-MIT**
