# check

Verifica un archivo fuente sin ejecutarlo.

El comando **check** es utilizado para validar el código fuente, buscando errores de sintaxis.

```bash
$ minipas check hello_world.mp
```

ó de la forma
```bash
$ minipas c hello_world.mp
```

para ambos casos, la salida será algo como:

```
minipas v.1.9.0
validando pairs...
validando AST...

OK.
```

**Importante:**

El comando `check`/`c` solo acepta archivos con extensión **.mp**

en caso contrarió obtendrá un error como este:

```
minipas v.1.9.0
minipas error: extensión de archivo de entrada, no valido.
utilice '.mp', para las extensiones de archivo.

try 'minipas --help' for more information
```
