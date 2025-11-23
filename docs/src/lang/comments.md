# Comentarios

Los comentarios en **minipas** pueden ser de la siguiente forma:
- una sola linea, usando `//`

    ```mp
    // un comentario antes de todo
    program helloworld;
    begin
        // dentro del bloque principal
        writeln("Hello World in minipas!"); // al lado de una sentencia
    end.
    // un comentario despues del bloque principal
    ```

- una sola linea, usando `{}`

    ```mp
    program helloworld;
    begin
        { dentro del bloque principal }
        writeln("Hello World in minipas!");
    end.
    ```

- varias lineas, usando `{}`

    ```mp
    {
    programming in Minipas
    demo with comments
    Author: John Doe <johndoe@example.com>
    }
    program helloworld;
    begin
        { dentro del bloque principal }
        writeln("Hello World in minipas!");
    end.
    ```

- anidados, usando `{}`

    ```mp
    program helloworld;
    begin
        {
            programming in Minipas
            demo with comments
            Author: John Doe <johndoe@example.com>
            // aqui un comentario de una linea
            // dentro de las llaves
        }
        writeln("Hello World in minipas!");
    end.
    ```

**Tenga en cuenta:**

Los comentarios en **minipas** pueden ir en cualquier parte del programa, o incluso estar anidados y estos serán ignorados por el interprete, aunque se **recomienda precaución en estos casos.**
