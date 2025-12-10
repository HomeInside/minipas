# emit

El comando **emit** es utilizado para generar archivos que contienen información del parser y del interprete de **minipas**.

información util para depuración.


```bash
$ minipas emit hello_world.mp
```

ó de la forma
```bash
$ minipas e hello_world.mp
```

para ambos casos, la salida será algo como:

```
minipas v.1.9.0

generating Pairs file: "hello_world.mpp"
generating AST file (text): "hello_world.mpa"

OK.
```

esto creará dos archivos con formato JSON

- archivo **.mpp**, información del parser
    ```json
    [
        Pair {
            rule: program,
            span: Span {
                str: "program helloworld;\r\nbegin\r\n    writeln(\"Hello World in minipas!\");\r\nend.\r\n",
                start: 0,
                end: 75,
            },
            inner: [
                Pair {
                    rule: ident,
                    span: Span {
                        str: "helloworld",
                        start: 8,
                        end: 18,
                    },
                    inner: [],
                },
                Pair {
                    rule: semicolon,
                    span: Span {
                        str: ";",
                        start: 18,
                        end: 19,
                    },
                    inner: [],
                },
                Pair {
                    rule: block,
                    span: Span {
                        str: "begin\r\n    writeln(\"Hello World in minipas!\");\r\nend",
                        start: 21,
                        end: 72,
                    },
                    inner: [
                        Pair {
                            rule: stmt,
                            span: Span {
                                str: "writeln(\"Hello World in minipas!\");",
                                start: 32,
                                end: 67,
                            },
                            inner: [
                                Pair {
                                    rule: expr_stmt,
                                    span: Span {
                                        str: "writeln(\"Hello World in minipas!\");",
                                        start: 32,
                                        end: 67,
                                    },
                                    inner: [
                                        Pair {
                                            rule: expr,
                                            span: Span {
                                                str: "writeln(\"Hello World in minipas!\")",
                                                start: 32,
                                                end: 66,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: sum,
                                                    span: Span {
                                                        str: "writeln(\"Hello World in minipas!\")",
                                                        start: 32,
                                                        end: 66,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: product,
                                                            span: Span {
                                                                str: "writeln(\"Hello World in minipas!\")",
                                                                start: 32,
                                                                end: 66,
                                                            },
                                                            inner: [
                                                                Pair {
                                                                    rule: method_chain,
                                                                    span: Span {
                                                                        str: "writeln(\"Hello World in minipas!\")",
                                                                        start: 32,
                                                                        end: 66,
                                                                    },
                                                                    inner: [
                                                                        Pair {
                                                                            rule: factor,
                                                                            span: Span {
                                                                                str: "writeln(\"Hello World in minipas!\")",
                                                                                start: 32,
                                                                                end: 66,
                                                                            },
                                                                            inner: [
                                                                                Pair {
                                                                                    rule: func_call,
                                                                                    span: Span {
                                                                                        str: "writeln(\"Hello World in minipas!\")",
                                                                                        start: 32,
                                                                                        end: 66,
                                                                                    },
                                                                                    inner: [
                                                                                        Pair {
                                                                                            rule: ident,
                                                                                            span: Span {
                                                                                                str: "writeln",
                                                                                                start: 32,
                                                                                                end: 39,
                                                                                            },
                                                                                            inner: [],
                                                                                        },
                                                                                        Pair {
                                                                                            rule: lparen,
                                                                                            span: Span {
                                                                                                str: "(",
                                                                                                start: 39,
                                                                                                end: 40,
                                                                                            },
                                                                                            inner: [],
                                                                                        },
                                                                                        Pair {
                                                                                            rule: expr_list,
                                                                                            span: Span {
                                                                                                str: "\"Hello World in minipas!\"",
                                                                                                start: 40,
                                                                                                end: 65,
                                                                                            },
                                                                                            inner: [
                                                                                                Pair {
                                                                                                    rule: expr_item,
                                                                                                    span: Span {
                                                                                                        str: "\"Hello World in minipas!\"",
                                                                                                        start: 40,
                                                                                                        end: 65,
                                                                                                    },
                                                                                                    inner: [
                                                                                                        Pair {
                                                                                                            rule: expr,
                                                                                                            span: Span {
                                                                                                                str: "\"Hello World in minipas!\"",
                                                                                                                start: 40,
                                                                                                                end: 65,
                                                                                                            },
                                                                                                            inner: [
                                                                                                                Pair {
                                                                                                                    rule: sum,
                                                                                                                    span: Span {
                                                                                                                        str: "\"Hello World in minipas!\"",
                                                                                                                        start: 40,
                                                                                                                        end: 65,
                                                                                                                    },
                                                                                                                    inner: [
                                                                                                                        Pair {
                                                                                                                            rule: product,
                                                                                                                            span: Span {
                                                                                                                                str: "\"Hello World in minipas!\"",
                                                                                                                                start: 40,
                                                                                                                                end: 65,
                                                                                                                            },
                                                                                                                            inner: [
                                                                                                                                Pair {
                                                                                                                                    rule: method_chain,
                                                                                                                                    span: Span {
                                                                                                                                        str: "\"Hello World in minipas!\"",
                                                                                                                                        start: 40,
                                                                                                                                        end: 65,
                                                                                                                                    },
                                                                                                                                    inner: [
                                                                                                                                        Pair {
                                                                                                                                            rule: factor,
                                                                                                                                            span: Span {
                                                                                                                                                str: "\"Hello World in minipas!\"",
                                                                                                                                                start: 40,
                                                                                                                                                end: 65,
                                                                                                                                            },
                                                                                                                                            inner: [
                                                                                                                                                Pair {
                                                                                                                                                    rule: string_literal,
                                                                                                                                                    span: Span {
                                                                                                                                                        str: "\"Hello World in minipas!\"",
                                                                                                                                                        start: 40,
                                                                                                                                                        end: 65,
                                                                                                                                                    },
                                                                                                                                                    inner: [],
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                        Pair {
                                                                                            rule: rparen,
                                                                                            span: Span {
                                                                                                str: ")",
                                                                                                start: 65,
                                                                                                end: 66,
                                                                                            },
                                                                                            inner: [],
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ],
                                                                        },
                                                                    ],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        Pair {
                                            rule: semicolon,
                                            span: Span {
                                                str: ";",
                                                start: 66,
                                                end: 67,
                                            },
                                            inner: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                Pair {
                    rule: EOI,
                    span: Span {
                        str: "",
                        start: 75,
                        end: 75,
                    },
                    inner: [],
                },
            ],
        },
    ]
    ```

- archivo **.mpa**, información del AST (interprete)
    ```json
    [
        Block(
            [
                Expr(
                    Call {
                        name: "writeln",
                        args: [
                            StringLiteral(
                                "Hello World in minipas!",
                            ),
                        ],
                    },
                ),
            ],
        ),
    ]
    ```


**Importante:**

El comando `emit`/`e` solo acepta archivos con extensión **.mp**

en caso contrario obtendrá un error como este:

```
minipas v.1.9.0
minipas error: extensión de archivo de entrada, no valido.
utilice '.mp', para las extensiones de archivo.

try 'minipas --help' for more information
```
