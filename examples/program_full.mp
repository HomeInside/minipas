{
  programming in Minipas
  demo with all
  Author: diniremix <diniremix@gmail.com>
  Minipas version: 1.7.0
}

program miprograma1;

var a, b: integer;
begin
clrscr();

//
// comentario de una linea
// * comentario con asteriscos*
//
a := 4;
b := 5;

if a > b then
	writeln("a es:", a);
	// writeln("el valor de a es:", a); // TO FIX
else
	writeln("b es:", b);
	// writeln("el valor de b es:", b); // TO FIX

// comentarios usando '/'😆
writeln("funciona bien😆");
writeln("con emojis: 😆😎🥳🤘");

{otro comentario}
writeln("texto", "con", "mas", "parametros", "en", "writeln");
writeln("a+b es:", a+b);
writeln("suma:", 4+1);
writeln("resta:", 4-3);
writeln({"aqui no hay datos:"} "el valor es cero(0)", 0);
writeln("multiplicacion:", 4*2); // al lado
writeln("division:", (4/5));
writeln("todo junto:", 4+1-3*2/5);
writeln("todo con parentesis:", (4+1-3)*(2/5));
{ con esto finalizamos }
writeln("todo con parentesis:", ("aa"+"bb"));
writeln("todo con parentesis:", (1+4));
writeln("todo con parentesis:", 1+3);
writeln("funcion random 1:", random());
//writeln("funcion random 2:", random(0));
writeln("funcion random 3:", random(1));
writeln("funcion random 4:", random(10));
writeln("funcion random 5:", random(1.234));
//writeln("funcion random 6:", random("1.234"));
writeln("funcion random 7:", random(1.0));
writeln("funcion random 8:", random(1.01));
writeln("funcion random 9:", random(1.001));
writeln("funcion random 10:", random(1.01));
writeln("funcion random 11", random(2.564));
writeln("funcion random 12:", random(3));

writeln("valor de a:", a);
a:= random();
writeln("valor de a con random:", a);
writeln("funcion random:", a);
a:= random(12);
writeln("funcion random(12):", a);
writeln("1 / 1 -> ", 1 / 1);
//writeln("1 / 0 -> ", 1 / 0);
//writeln("0 / 0 -> ", 0 / 0);
writeln("1 % 1 -> ", 1 % 1);
writeln("1.2345 % 2.3456 -> ", 1.2345 % 2.3456);
writeln("PI % 2.3456 -> ", PI % 2.3456);
//writeln("1 % 0 -> ", 1 % 0);
//writeln("0 % 0 -> ", 0 % 0);
//writeln("1.234 % 0 -> ", 1.234 % 0);
//writeln("1.234 % 0.0 -> ", 1.234 % 0.0);
//writeln("1 % 0.0 -> ", 1 % 0.0);
end.
// aqui va uno al final
