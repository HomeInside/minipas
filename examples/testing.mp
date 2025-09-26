program testingProg;

var a, b: integer;
var numero:real;


begin
	writeln("testing module");
	writeln("inicio -> el valor de a es:", a);
	writeln("inicio -> el valor de b es:", b);
	a:= 1;
	b:= 2;
	writeln("asignacion -> el valor de a es:", a);
	writeln("asignacion -> el valor de b es:", b);

	assert(2 * 2 = 4);
	assert(a<>b);

	//assert_equals(a, b);
	assert_equals(2*3, 6);
	//assert_equals(3+3, 7);

	writeln("final -> el valor de a es:", a);
	writeln("final -> el valor de b es:", b);
	assert(nil = nil);
	//assert(a = nil);
	//assert(nil > 3);
	assert("example"<> nil);
	assert(nil= nil);
	//assert(nil> nil);

	assert(a<>nil);
	assert(b<>nil);
	assert_equals(a, 1);
	assert_equals(b, 2);
	//panic("aqui usamos panic!");
	writeln("testing finalizado");
end.
