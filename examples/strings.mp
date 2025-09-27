program testStrings;
var cadena, trimmerStr: string;
var ancho: integer;
begin
	cadena:= " Anita lava la tinA ";
	ancho := len(cadena);

	writeln("cadena de caracteres:","'",cadena,"'");
	writeln();
	writeln("el ancho de la cadena de caracteres es (len):", ancho);
	writeln("el ancho de la cadena de caracteres es (length):", length(cadena));
	writeln();
	writeln("a mayusculas:", upper(cadena));
	writeln("a minusculas:", lower(cadena));
	writeln();
	writeln("antes del trim:", "'",cadena,"'");
	writeln();
	writeln("despues del trim");
	trimmerStr := trim(cadena);
	writeln("trim2:", "'",trim(cadena),"'");
	writeln("trim3:", "'",trimmerStr,"'");

	writeln();
	writeln("concatenar:", concat("concatena", "una", "lista", "de", "cadenas", "de", "caracteres"));
	writeln("concatenar:", concat("caracteres", "y", PI));
	writeln("concatenar:", concat("caracteres", "y", E));
	writeln("concatenar:", concat(PI, E));
	writeln("concatenar:", concat(TRUE, True));
	writeln("concatenar:", concat(FALSE, False));

	writeln();
	writeln(concat("--",trim(cadena), "--"));
	writeln(concat("\\", "\\"));
	writeln(concat("caracter \\", "\\"));
	writeln(concat("caracter \\r", "\r", "y", "otro texto"));
	writeln(concat("caracter \\n", "\n", "y", "otro texto"));
	writeln(concat("caracter \\t", "\t", "y", "otro texto"));

	writeln();
	writeln("uso de expresiones");

	writeln();
	writeln("1+2 ->", 1+2);
	writeln("1-2 ->", 1-2);
	writeln("1*2 ->", 1*2);
	writeln("1/2 ->", 1/2);
	writeln("1%2 ->", 1%2);

	writeln();
	writeln("1=2 ->", 1=2);
	writeln("1>2 ->", 1>2);
	writeln("1>=2 ->", 1>=2);
	writeln("1<2 ->", 1<2);
	writeln("1<=2 ->", 1<=2);
	writeln("1<>2 ->", 1<>2);

	writeln();
	writeln("uso de expresiones con caracteres");
	writeln("utilizando parentesis");

	writeln();
	writeln("('a' = 'a') ->", ("a" = "a"));
	writeln("('a' > 'a') ->", ("a" > "a"));
	writeln("('a' >= 'a') ->", ("a" >= "a"));
	writeln("('a' < 'a') ->", ("a" < "a"));
	writeln("('a' <= 'a') ->", ("a" <= "a"));
	writeln("('a' <> 'a') ->", ("a" <> "a"));

	writeln();
	writeln("sin utilizar parentesis");
	writeln("'a' > 'a' ->", "a" > "a");
	writeln("'a' >= 'a' ->", "a" >= "a");
	writeln("'a' < 'a' ->", "a" < "a");
	writeln("'a' <= 'a' ->", "a" <= "a");
	writeln("'a' <> 'a' ->", "a" <> "a");

	writeln();
	writeln("('hola' = 'holanda') ->", ("hola" = "holanda"));
	writeln("('hola' >= 'holanda') ->", ("hola" > "holanda"));
	writeln("('holanda' >= 'hola') ->", ("holanda" > "hola"));
	writeln();
	writeln("'hola' = 'holanda' ->", "hola" = "holanda");
	writeln("'hola' >= 'holanda' ->", "hola" > "holanda");
	writeln("'holanda' >= 'hola' ->", "holanda" > "hola");
end.
