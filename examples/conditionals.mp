{--------------------------}
{  Ejemplo en Pascal:      }
{                          }
{    Tercera prueba de     }
{    "if"                  }
{    IF3.PAS               }
{                          }
{  Este fuente procede de  }
{  CUPAS, curso de Pascal  }
{  por Nacho Cabanes       }
{                          }
{  Comprobado con:         }
{    - Free Pascal 2.2.0w  }
{    - Turbo Pascal 7.0    }
{    - Turbo Pascal 5.0    }
{    - Surpas 1.00         }
{    - Minipas 0.9.0       }
{--------------------------}

program testConditionals;

var
numero: integer;

begin
	clrscr();
	writeln('Escriba un número');
	numero:= to_int(readln());
	//numero:= toreal(readln());
	writeln();

	if numero < 0 then
		begin
			writeln('El número es negativo. Pulse INTRO para seguir.');
			writeln("con un bloque begin");
		end
	else if numero > 5 then
		begin
			writeln("el numero es mayor que 5:", numero);
			writeln("otras sentencias");
			writeln("dentro del bloque");
		end
	else if numero = 0 then writeln("el numero es 0:", numero);
	else writeln("el numero es otro diferente:", numero);
end.
