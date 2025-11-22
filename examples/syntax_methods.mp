
program test_syntax_methods;
var b, c: string;
var d: integer;
var
    s1, s2, s3: string;
    n, m: integer;
    r: real;

begin
	b := "Hola";
	c := b.upper();
	writeln("uso de upper:", c);

	writeln();
	writeln("uso de lower: ", c.lower());
	assert_equals(c.lower(), "hola");

	writeln("uso de length: ", c.length());
	assert_equals(c.length(), 4);

	writeln();
	writeln("uso de encadenamiento: ", c.lower().length());
	assert_equals(c.lower().length(), 4);

	writeln("uso de toString: ", typeinfo(123.toString()));
	assert_equals(123.toString(), "123");
	assert_equals(typeinfo(123.toString()), "String");

	writeln("uso de abs: ", -3.abs());
	assert_equals(-3.abs(), 3);

	writeln();
	writeln(3.14159.trunc());
	assert_equals(3.14159.trunc(), 3);
	//writeln("hola".upper().copy(1,2)); // no implementado

	writeln();
	writeln(12.abs());
	assert_equals(12.trunc(), 12);

	writeln(42.to_str());
	assert_equals(42.to_str(), "42");

	writeln();
	writeln("trunc PI:", PI.trunc().to_str());
	assert_equals(PI.trunc().to_str(), "3.0");
	assert_equals(typeinfo(PI.trunc().to_str()), "String");

	writeln("uso de concat:", PI.trunc().to_str().concat("aloha"));

	writeln();
	writeln("uso de trim:", " con espacios aqui ".trim());
	assert_equals(" con espacios aqui ".trim(), "con espacios aqui");

	writeln();
	writeln("=== STRING METHODS ===");
	s1 := "hola mundo";
	writeln("upper:", s1.upper());
	writeln("lower:", s1.lower());
	writeln("length:", s1.length());
	//writeln("copy(6,5):", s1.copy(6,5)); // no implementado
	//writeln("pos('mun'):", s1.pos("mun")); // no implementado
	//writeln("insert('MiniPas ',1):", s1.insert("MiniPas ",1)); // no implementado
	writeln("concat:", s1.upper().concat("!!!"));
	writeln("trim (alias):", "   trim  ".trim());
	writeln("len (builtin):", len(s1));

	writeln();
	writeln("=== NUMERIC METHODS ===");
	n := -42;
	r := 3.14159;
	writeln("abs:", n.abs());
	writeln("abs builtin:", abs(n));
	writeln("trunc:", r.trunc());
	writeln("to_str:", r.to_str());
	writeln("toString (alias):", r.toString());
	writeln("encadenado:", PI.trunc().to_str().concat(" pi aprox"));
	writeln("alias encadenado:", (-9.8).abs().toString().concat(" m/s^2"));

	writeln();
	writeln("=== MIXED EXPRESSIONS ===");
	s2 := "Mini".concat("Pas");
	writeln("concat directo:", s2);
	writeln("encadenado con método:", "Hola".concat(" ").concat("Mundo").upper());
	writeln("uso en función builtin:", writeln("len->", "Probando".len()));

	writeln();
	writeln("=== ALIASES ===");
	writeln("Length() alias:", "alias".length());
	writeln("Upper() alias:", "alias".upper());
	writeln("toString alias numérico:", 1.234.toString());

	writeln();
	writeln("=== ERROR CASES (deberían fallar) ===");
	// descomentar para probar control de errores
	//writeln("fallo:", s1.noExiste()); // panic
	assert_equals(typeinfo(n), "Integer");
	writeln("fallo:", n.lower());

	assert_equals(typeinfo(s1), "String");
	//writeln("fallo:", s1.upper(123)); // upper() no toma argumentos

	writeln();
	writeln("->", ("abc".upper().lower().concat("x")).len());
	assert_equals(("abc".upper().lower().concat("x")).len(), 5);

	writeln();
	writeln(((-1.23).abs().to_str().concat(" ok")));
	assert_equals(((-1.23).abs().to_str().concat(" ok")), "1.23  ok");

	writeln();
	writeln(("  Minipas  ".trim().upper()).concat("!"));
	assert_equals(("  Minipas  ".trim().upper()).concat("!"), "MINIPAS !");

	writeln();
	writeln(typeinfo(True.to_str()));
end.
