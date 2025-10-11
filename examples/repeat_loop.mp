{--------------------------}
{  Este fuente procede de  }
{  CUPAS, curso de Pascal  }
{  por Nacho Cabanes       }
{                          }
{  Comprobado con:         }
{    - Minipas 1.7.0       }
{--------------------------}

program testRepeat;
var y: integer;
var i: integer;
var
  x, suma: integer;

begin
  y := 5;


  repeat
    // dentro del repeat
    writeln('ejecutado');
    // iterando
  until y = 5; // agregar parentesis


  i := 0;

  writeln();

  repeat
    i := i + 2;
    writeln('i = ', i);
  until i >= 5; // agregar parentesis

  x := 0;
  suma := 0;

  writeln();

  repeat
    x := x + 1;
    suma := suma + x;

    writeln('Iteración ', x, ': suma = ', suma);

    if (x % 2) = 0 then
      writeln(' -> ', x, ' es par');
    else
      writeln(' -> ', x, ' es impar');
  until x >= 5; // agregar parentesis

  writeln('Final: x = ', x, ', suma = ', suma);

  writeln();
  writeln("test con break");

  i := 0;
  repeat
    i := i + 1;
    if i = 3 then
    begin
      writeln('saliendo en', i);
      break;
    end
    writeln('i =', i);
  until i >= 5;


  writeln();
  writeln("test con continue");

  i := 0;
  repeat
    i := i + 1;
    if i = 3 then
    begin
      writeln('salta', i);
      continue;
    end
    writeln('i =', i);
  until i >= 5;

end.
