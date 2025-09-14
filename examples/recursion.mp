{
	calcula el factorial de un numero
	n! = 20!
	n! = 2.432902008 E+18
	si el numero es muy grande
	genera un error de "attempt to multiply with overflow"

	- https://www.nachocabanes.com/pascal/curso/cupas08.php
	- https://www.calculatorsoup.com/calculators/discretemathematics/factorials.php
}

program PruebaDeFactorial;

var numero,result: integer;

function factorial(num:integer) : integer;
begin
	if num = 1 then
	return 1; //Aseguramos que tenga salida siempre
else
	return num * factorial( num-1 ); // Caso general
end

begin
	writeln( 'Introduce un número entero (no muy grande)  ;-)' );
	numero := to_int(readln());
	result := factorial(numero);
	writeln( 'Su factorial es ',  result);
end.
