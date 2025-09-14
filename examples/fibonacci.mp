{
	La sucesión de Fibonacci es una sucesión `Fn` de números
	naturales definida de forma recursiva.

	- https://www.calculatorsoup.com/calculators/discretemathematics/fibonacci-calculator.php
	- https://rosettacode.org/wiki/Fibonacci_sequence
}

program test_fibonacci;

var numero, result: integer;

function fibonacci(num:integer) : integer;
begin
	if num < 2 then
		return num;
	return fibonacci(num-1) + fibonacci(num-2);
end

begin
	writeln( 'Introduce un número entero (no muy grande)  ;-)' );
	numero := to_int(readln());
	result := fibonacci(numero);
	writeln( 'el fibonacci de', numero, 'es',  result);
end.
