program testAbbreviatedOperations;
var x,y:integer;
var z: real;
var a: integer;
var b: real;

begin
    writeln("usando operaciones abreviadas");
    writeln();
    writeln("el valor de 'x':", x);
    writeln("el valor de 'y':", y);
    writeln("el valor de 'z':", z);
    writeln();

    x +=1;
	assert_equals(x, 1);

    y -=1;
	assert_equals(y, -1);

	z *=1;
	assert_equals(z, 0.0);

	z /=2;
	assert_equals(z, 0.0);

	a %=3;
	assert_equals(a, 0);

	b %=4;
	assert_equals(b, 0.0);

	b +=y;
	assert_equals(b, -1.0);

	b %=5;
	assert_equals(b, 4.0);

	writeln();
    writeln("el valor de 'x':", x);
    writeln("el valor de 'y':", y);
    writeln("el valor de 'z':", z);
    writeln("el valor de 'a':", a);
    writeln("el valor de 'b':", b);
end.
