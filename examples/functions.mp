program test_functions;

function saludar(): integer;
begin
	writeln("hola mundo desde una función");
	return 0;
end

function sumar(a:integer; b:integer): integer;
var c: integer;
begin
	writeln("el valor de a es:", a);
	writeln("el valor de b es:", b);
	writeln("el valor de c es:", c);
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
