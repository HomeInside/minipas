# run

El comando **run** es utilizado para ejecutar un archivo con código fuente de **minipas**.

las extensiones de archivos permitidas son:

- **.mp** archivo con código fuente de **minipas**.
- **.mpc** archivo en formato binario (similar a [bytecode](https://es.wikipedia.org/wiki/Bytecode)) de un archivo con código fuente **.mp**.


```bash
$ minipas run hello_world.mp
```

ó de la forma
```bash
$ minipas r hello_world.mp
```

para ambos casos, la salida será algo como:

```
minipas v.1.9.0
Hello World in minipas!
```


## archivos binarios

**minipas** soporta la ejecución de archivos previamente compilados (comando [build](./build.md))

```bash
$ minipas run a.mpc
```

ó de la forma
```bash
$ minipas r a.mpc
```

para ambos casos, la salida será algo como:

```
minipas v.1.9.0
Hello World in minipas!
```


**Importante:**

el comando `run`/`r` solo acepta archivos con extensión **.mp** ó **.mpc**.

en caso contrarió obtendrá un error como este:

```
minipas v.1.9.0
minipas error: extensión de archivo no válida.
run: utilice '.mp' ó '.mpc', para extensiones de archivo.

try 'minipas --help' for more information
```
