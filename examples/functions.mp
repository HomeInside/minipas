program test_functions;
var i, j: integer;
var x: boolean;
var
y,z:string;

function saludar(): integer;
begin
	writeln("hola mundo desde una función");
	return 0;
end

function sumar(a:integer; b:integer): integer;
var
c: integer;
d: integer;
var e,f:string;

begin
	writeln("el valor de a es:", a);
	writeln("el valor de b es:", b);
	writeln("el valor de c es:", c);
	writeln("el valor de d es:", d);
	writeln("el valor de e es:", e);
	c := a + b;
	return c;
end

// bloque principal
begin
	writeln();
	writeln("3 + 5 = ", sumar(3, 5));
	writeln("10 + 20 = ", sumar(10, 20));
	writeln();
	writeln(sumar(2, 9));
	writeln();
	saludar();
end.
