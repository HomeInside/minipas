# Instalación desde el código fuente

La mejor manera de obtener la última y mejor versión de **Minipas** es instalarla desde el código fuente. Es fácil y solo lleva unos segundos:

**Nota:**

Si utilizas Windows, fuera de [WSL](https://learn.microsoft.com/es-es/windows/wsl/), se recomienda utilizar [MSVC](https://visualstudio.microsoft.com/downloads/), revisa los requisitos para esto, [por aquí...](https://rust-lang.github.io/rustup/installation/windows.html)


## Clona el repo

```bash
$ git clone https://github.com/HomeInside/minipas.git
```

## Construye

```bash
$ cd minipas
```

```bash
$ cargo build --release
```

## Prueba

```bash
$ minipas run examples/hello_world.mp
```

Realiza las modificaciones que creas convenientes y envíanos un [pull request.](https://github.com/HomeInside/minipas/pulls)

Para cualquier duda, comentario, sugerencia ó aporte, dirigete a la seccion de [issues.](https://github.com/HomeInside/minipas/issues)

Antes de abrir un issue nuevo, revisa los ya existentes en busca de una solución (posiblemente ya planteada) para el problema que se te presenta.
