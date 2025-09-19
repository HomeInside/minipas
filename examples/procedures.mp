program test_procedures;

procedure saludar();
begin
	writeln("Hola mundo desde un procedimiento");
end

procedure saludo(nombre:string);
begin
	writeln("Hola", nombre, "saludos desde un procedimiento");
end

procedure sumar(a:integer; b:integer);
var c: integer;
begin
	writeln("el valor de a es:", a);
	writeln("el valor de b es:", b);
	writeln("el valor de c es:", c);
	c := a + b;
	writeln("la suma es:", c);
end

begin
	writeln();
	saludo("minipas!");
	writeln();
	saludar();
	writeln();
	sumar(5,11);
end.
