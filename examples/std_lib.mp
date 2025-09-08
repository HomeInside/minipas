program demoStdLibs;

var a: integer;
var b: integer;
var c,d,e: real;

begin
    a := 2;
    b := 8;
    writeln("'a' value is:", a);
    writeln("'b' value is:", b);

    writeln("2^8 = ", pow(a, b));

	c:=47;
	d:= pow(c, 2); // elevar a la 2
	e:= pow(c, 4); // elevar a la N

    writeln("'c' value is:", c);
    writeln("'d' value is:", d);
    writeln("'e' value is:", e);

    writeln("pow((a+b), 3)", pow((a+b), 3));

    writeln("pow(0, 0)", pow(0, 0));
    writeln("pow(1, 1)", pow(1, 1));
    writeln("pow(-1, 1)", pow(-1, 1));
    writeln("pow(1, -1)", pow(1, -1));
    writeln("pow(-1, -1)", pow(-1, -1));

    writeln("pow(-1, -1)+47", pow(-1, -1)+47);
    writeln("(pow(-1, -1)+47)", (pow(-1, -1)+47));
    writeln("1-(pow(-1, -1)+47)", 1-(pow(-1, -1)+47));
    writeln("(1-pow(-1,-1)+47)", (1-pow(-1,-1)+47));
end.
