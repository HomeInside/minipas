program demoStdLibs;

var a: integer;
var b: integer;
var c,d,e: real;

begin
    a := 2;
    b := 8;
    writeln("2^8 = ", pow(a, b));

	c:=47;
	d:= pow(c, 2); // elevar a la 2
	e:= pow(c, 4); // elevar a la N
    writeln("'c' value is:", a);
    writeln("'d' value is:", b);
    writeln("'e' value is:", c);
end.
