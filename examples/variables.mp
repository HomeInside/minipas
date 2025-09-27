program demo_variables;
var a:integer;
var b:real;
var c:string;
var d:boolean;

var demo1,
demo2,
demoEs3,
DemoEs4,
DEMOES5,
DeMo3Es6: integer;

var snake_case: string;
var _snake_case: string;
var __1_uno_: string;
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
    writeln("e es:", e);
    writeln("f es:", f);
    writeln("g es:", g);
    writeln("h es:", h);
    writeln("i es:", i);
    return 0;
end

function sumar(a:integer; b: integer): integer;
var c: integer;
begin
	writeln("el valor de a es:", a);
	writeln("el valor de b es:", b);
	c := a + b;
	return c;
end

procedure procedimiento();
var j:integer; k:real; l:string; m:boolean;
var n:integer;
begin
    writeln("dentro del procedimiento");
    writeln("j es:", j);
    writeln("k es:", k);
    writeln("l es:", l);
    writeln("m es:", m);
    writeln("n es:", n);
end

begin
    writeln("la suma es:", sumar(1,2));
    writeln("'a' value is:", a);
    writeln("'b' value is:", b);
    writeln("'c' value is:", c);
    writeln("'d' value is:", d);

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
	writeln("False es", FALSE, "ó es", False);
	writeln("True es", TRUE, "ó es", True);
end.
