{
	es un método eficiente para calcular el máximo común divisor (MCD)
	de dos números enteros, el número más grande que los divide
	a ambos sin dejar resto.

	- https://es.wikipedia.org/wiki/Algoritmo_de_Euclides
	- https://www.calculatorsoup.com/calculators/math/gcf-euclids-algorithm.php
	- https://rosettacode.org/wiki/Greatest_common_divisor
}

program test_Euclid_mcd;

var num1, num2: integer;
var result: integer;

{
// con while
// no funciona aun
def mcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

// con restas sucesivas
function mcd(a: integer; b: integer): integer;
begin
    if a = b then
        return a;
    else if a > b then
        return mcd(a - b, b);
    else
        return mcd(a, b - a);
end

// simular con la operación de división entera (div)
y restas:
function mcd(a:integer; b:integer) : integer;
begin
    if b = 0 then
        return a;
    else
        return mcd(b, a - to_int((a / b)) * b);
end

}

function mcd(a:integer; b:integer) : integer;
begin
	if b = 0 then
		return a;
	else
		return mcd(b, a%b);
end

begin
	writeln( 'algoritmo de Euclides' );
	writeln();

	writeln( '1. Introduce un número entero (no muy grande)  ;-)' );
	num1 := to_int(readln());
	writeln();
	writeln( '2. Introduce un número entero (no muy grande)  ;-)' );
	num2 := to_int(readln());
	writeln();
	result := mcd(num1, num2);
	// 48, 18 -> 6
	// 47, 14 -> 1
	// 80, 19 -> 1
	// 80675, 21945 -> 35
	writeln( 'el MCD es:', result);
end.
