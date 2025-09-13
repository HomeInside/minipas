program test_functions;
var x:integer;

function saludo(): integer;
begin
	writeln("Hola mundo desde la funcion saludo");
	x := saludar();
	return x;
end

function saludar():integer;
begin
	writeln("Hola mundo desde una función");
	return 0;
end

{
// funcion comentada, no funciona actualmente
function sumar(a:integer): integer;
begin
	writeln("el valor de a:", a);
	return 0;
end
sumar(1);
}

begin
	x:=saludo();
	writeln("el valor de X es:", x);
end.
