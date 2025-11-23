# Variables

La definición de una variable se coloca en un bloque que comienza con la palabra clave **var**, seguida de las definiciones de las variables como se muestra a continuación:

- ejemplo 1:
    ```mp
    // cada variable en una linea
    var nombre: string;
    var edad: integer;
    var mayor_de_edad: boolean;
    ```

- ejemplo 2:
    ```mp
    // si son del mimos tipo, pueden ir en una sola sentencia
    var nombre, apellidos: string;
    ```

- ejemplo 3:
    ```mp
    // en un bloque de definición
    var
    nombre: string;
    edad: integer;
    mayor_de_edad: boolean;
    ```

Este tipo de variables definidas despues de la declaracion del programa serán **variables globales** y estarán disponibles para cualquier función, procedimiento y el bloque principal del programa, durante su ejecución.

Es decir, las variables globales se definen después del encabezado del programa.
