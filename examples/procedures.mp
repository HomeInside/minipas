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

// - https://www.nachocabanes.com/pascal/curso/cupas08.php
// forward
procedure B(n: integer);
begin
  writeln('Entrando a B, con parámetro ', n);
  if n>0 then A (n-1);
end

procedure A(n:integer);
begin
  writeln('Entrando a A, con parámetro ', n);
  if n>0 then B(n-1);
end

begin
	writeln();
	saludo("minipas!");
	writeln();
	saludar();
	writeln();
	sumar(5,11);
	writeln();
  	A(10);
end.
