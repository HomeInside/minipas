program demo_variables;
var a:integer;
var b:real;
var c:string;
var d:boolean;
var x:byte;

var demo1,
demo2,
demoEs3,
DemoEs4,
DEMOES5,
DeMo3Es6: integer;

var snake_case: string;
var _snake_case: string;
var __1_uno_: real;
var _1234_: string;

var func_tion: string;
integers, nombre: string;
reals:real;

var
is_18: boolean;
trauesa: boolean;

var btrues, atruesa: boolean;
sina: boolean;
formato: boolean;

var truesa, falsea: boolean;
var beging: boolean;


function funcion(): integer;
var e:integer; f:real; g:string; h:boolean;
var i:integer;
begin
    writeln("dentro de la funcion");
    writeln("el valor de 'e' es:", e);
    writeln("el valor de 'f' es:", f);
    writeln("el valor de 'g' es:", g);
    writeln("el valor de 'h' es:", h);
    writeln("el valor de 'i' es:", i);
    writeln("el valor de 'x' es:", x);
    return 0;
end

function sumar(a:integer; b: integer): integer;
var c: integer;
begin
    writeln("dentro de la función sumar");
	writeln("el valor de a es:", a);
	writeln("el valor de b es:", b);
	c := a + b;
	writeln("el valor de c es:", c);
	return c;
end

procedure procedimiento();
var j:integer; k:real; l:string; m:boolean;
var n:integer;
begin
    writeln("dentro del procedimiento");
    writeln("el valor de 'j' es:", j);
    writeln("el valor de 'k' es:", k);
    writeln("el valor de 'l' es:", l);
    writeln("el valor de 'm' es:", m);
    writeln("el valor de 'n' es:", n);
end

begin
    writeln();
    writeln("la suma es:", sumar(1,2));
    writeln("el valor de 'a' value is:", a);
    writeln("el valor de 'b' value is:", b);
    writeln("el valor de 'c' value is:", c);
    writeln("el valor de 'd' value is:", d);
    writeln("el valor de 'x' value is:", x);

	a:=47;
	b:= (a*a);
	c:= (b*b)*(a)+1;
	c:= c % 3;
    writeln();
    funcion();
    procedimiento();

    writeln();
    writeln("'demo1' value is:", demo1);
    writeln("'demo2' value is:", demo2);
    writeln("'demoEs3' value is:", demoEs3);
    writeln("'DemoEs4' value is:", DemoEs4);
    writeln("'DEMOES5' value is:", DEMOES5);
    writeln("'DeMo3Es6' value is:", DeMo3Es6);
    writeln();

    snake_case := "soy snake_case";
    _snake_case := "soy _snake_case";
    __1_uno_ := 1.234;

    writeln("'snake_case' value is:", snake_case);
    writeln("'_snake_case' value is:", _snake_case);
    writeln("'__1_uno_' value is:", __1_uno_);
    writeln("'_1234_' value is:", _1234_);
    //writeln("'function' value is:", function);
    writeln("'fun_ction' value is:", func_tion);
    writeln();
	writeln("integers: es", integers);
	writeln("reals: es", reals);
	writeln("nombre: es", nombre);
	writeln("is_18: es", is_18);
	writeln("atruesa: es", atruesa);
	writeln("btrues: es", btrues);
	writeln("formato: es", formato);
	writeln("sina: es", sina);
	writeln("truesa: es", truesa);
	writeln("falsea: es", falsea);
	writeln("beging: es", beging);
    writeln();
	writeln("False es", FALSE, "ó es", False);
	writeln("True es", TRUE, "ó es", True);

    writeln();
    writeln(typeinfo(a));
    writeln(typeinfo(b));
    writeln(typeinfo(c));
    writeln(typeinfo(d));
    writeln(typeinfo(demo1));
    writeln(typeinfo(demo2));
    writeln(typeinfo(demoEs3));
    writeln(typeinfo(DemoEs4));
    writeln(typeinfo(DEMOES5));
    writeln(typeinfo(DeMo3Es6));
    writeln(typeinfo(snake_case));
    writeln(typeinfo(_snake_case));
    writeln(typeinfo(__1_uno_));
    writeln(typeinfo(_1234_));
    writeln(typeinfo(func_tion));
    writeln(typeinfo(integers));
    writeln(typeinfo(nombre));
    writeln(typeinfo(reals));
    writeln(typeinfo(is_18));
    writeln(typeinfo(trauesa));
    writeln(typeinfo(btrues));
    writeln(typeinfo(atruesa));
    writeln(typeinfo(sina));
    writeln(typeinfo(formato));
    writeln(typeinfo(truesa));
    writeln(typeinfo(falsea));
    writeln(typeinfo(beging));
    writeln(typeinfo(x));

    writeln();
    assert_equals(typeinfo(a), "Integer");
    assert_equals(typeinfo(b), "Integer");
    assert_equals(typeinfo(c), "Integer");
    assert_equals(typeinfo(d), "Boolean");
    assert_equals(typeinfo(demo1), "Integer");
    assert_equals(typeinfo(demo2), "Integer");
    assert_equals(typeinfo(demoEs3), "Integer");
    assert_equals(typeinfo(DemoEs4), "Integer");
    assert_equals(typeinfo(DEMOES5), "Integer");
    assert_equals(typeinfo(DeMo3Es6), "Integer");
    assert_equals(typeinfo(snake_case), "String");
    assert_equals(typeinfo(_snake_case), "String");
    assert_equals(typeinfo(__1_uno_), "Real");
    assert_equals(typeinfo(_1234_), "String");
    assert_equals(typeinfo(func_tion), "String");
    assert_equals(typeinfo(integers), "String");
    assert_equals(typeinfo(nombre), "String");
    assert_equals(typeinfo(reals), "Real");
    assert_equals(typeinfo(is_18), "Boolean");
    assert_equals(typeinfo(trauesa), "Boolean");
    assert_equals(typeinfo(btrues), "Boolean");
    assert_equals(typeinfo(atruesa), "Boolean");
    assert_equals(typeinfo(sina), "Boolean");
    assert_equals(typeinfo(formato), "Boolean");
    assert_equals(typeinfo(truesa), "Boolean");
    assert_equals(typeinfo(falsea), "Boolean");
    assert_equals(typeinfo(beging), "Boolean");
    assert_equals(typeinfo(x), "Byte");
end.
