program demoStdLibs;

var a: integer;
var b: integer;
var c,d,e: real;

begin
    a := 2;
    b := 8;
    writeln("'a' value is:", a);
    writeln("'b' value is:", b);
	// using pow
    writeln("");
    writeln("2^8 = ", pow(a, b));

	c:=47;
	d:= pow(c, 2); // elevar a la 2
	e:= pow(c, 4); // elevar a la N

    writeln("");
    writeln("'c' value is:", c);
    writeln("'d' value is:", d);
    writeln("'e' value is:", e);

    writeln("");
    writeln("pow((a+b), 3)", pow((a+b), 3));

    writeln("");
    writeln("pow(0, 0)", pow(0, 0));
    writeln("pow(1, 1)", pow(1, 1));
    writeln("pow(-1, 1)", pow(-1, 1));
    writeln("pow(1, -1)", pow(1, -1));
    writeln("pow(-1, -1)", pow(-1, -1));

    writeln("");
    writeln("pow(-1, -1)+47", pow(-1, -1)+47);
    writeln("(pow(-1, -1)+47)", (pow(-1, -1)+47));
    writeln("1-(pow(-1, -1)+47)", 1-(pow(-1, -1)+47));
    writeln("(1-pow(-1,-1)+47)", (1-pow(-1,-1)+47));

	// using abs()
    writeln("");
    writeln("abs(0)", abs(0));
    writeln("abs(-1)", abs(-1));
    writeln("abs(-1.25)", abs(-1.25));
    writeln("abs(1.234)", abs(1.234));
	writeln("abs(-8798789)", abs(-8798789));

	// using sin()
    writeln("");
    writeln("sin(0)", sin(0));

	// using sin() -
    writeln("");
    writeln("sin(-1)", sin(-1));
    writeln("sin(-1.25)", sin(-1.25));
    writeln("sin(-1.234)", sin(-1.234));
	writeln("sin(-8798789)", sin(-8798789));

	// using sin() +
    writeln("");
    writeln("sin(1)", sin(1));
    writeln("sin(1.234)", sin(1.234));
	writeln("sin(8798789)", sin(8798789));

	// using ln()
    writeln("");
	writeln("ln(1.234)", ln(1.234));

	// using sqrt()
    writeln("");
	writeln("sqrt(1.234)", sqrt(1.234));
end.
