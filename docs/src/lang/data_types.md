# Tipos de datos

Los tipos de datos indican el significado, las restricciones, los valores posibles, las operaciones, las funciones y el modo de almacenamiento de un valor asociado a el.

Cada valor en **minipas** es de un determinado tipo de dato, lo que le indica cómo trabajar con ellos al al interprete.

Ten en cuenta que **minipas** es un lenguaje de **tipado fuerte y estático**, lo que significa que se debe definir y conocer los tipos de todas las variables antes de la ejecución del programa.

**minipas** soporta los siguientes tipos de datos:

- [**integer**](./numbers.md#integer) números enteros.
- [**real**](./numbers.md#real) números con parte decimal.
- [**string**](./strings.md) cadenas de caracteres.
- [**boolean**](./boolean.md) indica solo dos posibles valores, **verdadero** o **falso**.
- [**byte**](./numbers.md#byte) número entero sin signo que tiene un valor entre 0 y 255. 


## Valor especial Nulo

**minipas** usa el valor especial `nil` (nulo), sin embargo `nil`, no es un tipo de dato en sí mismo, sino más bien un valor especial que indica **la ausencia de un valor** ó un valor **no asignado**.

**A tener en cuenta:**

En **minipas**, no puedes asignar un valor nulo (`nil`) a una variable de la misma manera que en otros lenguajes de programación.
