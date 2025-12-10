# build

El comando **build** es utilizado para compilar un archivo con código fuente de **minipas**.


```bash
$ minipas build hello_world.mp
```

ó de la forma
```bash
$ minipas b hello_world.mp
```

para ambos casos, la salida será algo como:

```
minipas v.1.9.0
building "hello_world.mp"...
generating AST file (bin): "a.mpc"

OK.
```

esto creará un archivo con formato binario (similar a [bytecode](https://es.wikipedia.org/wiki/Bytecode)) con la extensíon **.mpc**.

**Nota:**

Para más información sobre los archivos **.mpc**, consulte el comando [run](./run.md).

**Importante:**

el comando `build`/`b` solo acepta archivos con extensión **.mp**

en caso contrario obtendrá un error como este:

```
minipas v.1.9.0
minipas error: extensión de archivo de entrada, no valido.
utilice '.mp', para las extensiones de archivo.

try 'minipas --help' for more information
```
