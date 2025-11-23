# Linea de comandos

La herramienta de línea de comandos **Minipas**, se utiliza para compilar y revisar codigo fuente de archivos `.mp`. Una vez instalado **Minipas**, puede ejecutar el comando `minipas --help` en su terminal para ver los comandos disponibles.

Las siguientes secciones proporcionan información detallada sobre los diferentes comandos disponibles.
- [minipas build](./build.md) Compila un archivo fuente, y genera el AST en formato binario.
- [minipas run](./run.md) Ejecuta un archivo con extensión `.mp`|`.mpc` de **minipas**.
- [minipas emit](./emit.md) Genera los Pairs y el AST de un archivo fuente.
- [minipas check](./check.md) Verifica un archivo fuente sin ejecutarlo.

## extensiones de archivo
**Minipas** utiliza las siguientes extensiónes de archivos

- **.mp** archivo con código fuente de **minipas**.
- **.mpc** archivo en formato binario (similar a [bytecode](https://es.wikipedia.org/wiki/Bytecode)) de un archivo con código fuente **.mp**.
- **.mpa** archivo en formato JSON, que contiene las [Reglas](https://docs.rs/pest/latest/pest/iterators/struct.Pair.html) generadas por el parser, de un archivo con código fuente.
- **.mpp** archivo en formato JSON, que contiende información del [AST](https://es.wikipedia.org/wiki/%C3%81rbol_de_sintaxis_abstracta) generado por el interprete, de un archivo con código fuente.
