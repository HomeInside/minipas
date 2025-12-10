# break y continue

## **break**

Es una instrucción que se utiliza para salir inmediatamente de un ciclo (como **for**, **while** o **repeat...until**). Cuando se utiliza, control de flujo se transfiere a la siguiente línea después del ciclo que contiene la instrucción `break`.

```mp
program breakExample;
var a: integer;

begin
   a := 10;

   while a < 20 do
   begin
      writeln("el valor de a es:", a);
      a:= a+1;

      if a > 15 then
        break; // <- uso de break
   end
end.
```

la salida será algo como

```
minipas v.1.9.0
el valor de a es: 10
el valor de a es: 11
el valor de a es: 12
el valor de a es: 13
el valor de a es: 14
el valor de a es: 15
```


## continue

Esta instrucción se utiliza para saltar directamente a la siguiente iteración de un ciclo. Al encontrar un `continue`, el resto del código en la iteración actual se omite y se pasa a la siguiente.

En el bucle `for`, la instrucción **continue** hace que se ejecuten las partes de prueba condicional e incremento del bucle. En los bucles `while` y `repeat...until`, la instrucción `continue` hace que el control del programa pase a las pruebas condicionales.

```mp
program continueExample;
var i: integer;

begin
   for i := 1 to 10 do
   begin
      if i mod 2 = 0 then
         continue; // salta la iteración para números pares
      writeln("número impar:", i);
   end
end.
```

la salida será algo como

```
minipas v.1.9.0
número impar: 1
número impar: 3
número impar: 5
número impar: 7
número impar: 9
```


## goto

⚠️ **Minipas** no admite la sentencia **goto**.

## Recomendaciones

al utilizar **break:**

**Claridad:** Usa break solamente cuando sea necesario salir de un ciclo prematuramente, demasiados break pueden hacer que el flujo del código sea confuso.

**Uso Moderado:** si encuentras que estás usando break frecuentemente, considera reestructurar el ciclo o la lógica para que sea más clara.

**Documenta su Uso:** comenta por qué es necesario para que futuros lectores del código comprendan la lógica detrás de su uso.

<br>

al utilizar **continue:**


**Claridad:** usa continue para mejorar la legibilidad del código al evitar anidamientos excesivos de condicionales dentro de un ciclo.

**Cuidado con Saltos:** asegúrate de que utilizar continue no cause que se omitan acciones críticas en el ciclo.

**Comentarios:** similar a break, documenta el uso de continue, haciendo explícito el motivo de saltar ciertas iteraciones.
