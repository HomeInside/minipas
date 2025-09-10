program useWriteln;
var a, b: integer;
var c: integer;
var x,y: real;
var numero:real;
begin

    writeln("Hello, World in minipas!");
    writeln("suma:", 4+1);
    writeln("resta:", 4-3);
    writeln("multiplicacion:", 4*2);
    writeln("division:", (4/5));
    writeln("precede?:", 4+1-3*2/5);
    writeln("con parentesis:", (4+1-3)*(2/5));

	a := 42;
	b := 8;
	c := a+b;
	writeln("espacio 1");
	x:= sin(1.234);
	writeln("espacio 2");
	writeln("ejemplo de sin:", x);
	writeln("espacio 3");
	sin(1.234);
	writeln("espacio 4");
	writeln("el valor de 'a' es:", a, "el valor de 'b' es:", b, "y el valor de 'c' es:", c);

	writeln();
	writeln("espacio 5");
	numero := 3.1415926535;
	//writeln(numero:0:3);
	writeln(numero);
	writeln("espacio 6");
	writeln();
	writeln("espacio 7");
	writeln("");
	writeln("=========================");
	writeln("el numero es {}", numero);
	writeln("el numero es {:.3}", numero);
	writeln("el numero es {:.3f}", numero);

	writeln("el numero es trunc", trunc(numero));

	writeln("el numero es fract", fract(numero));

	writeln();
	y:= pow(a, 2);
	writeln(" el pow", y);

	writeln();
	writeln("antes");
	fract(numero);
	writeln("despues");
	y := fract(numero);
	writeln("el numero es fract (y) es:", y);

	writeln();
	writeln("el numero es format", format(numero));
	writeln("otra forma de format", (format("pi ≈ {:.2f}", numero)));
	write("esto", "va", "en", "la", "misma", "linea");
	write();
	write("...finalizando");

end.
