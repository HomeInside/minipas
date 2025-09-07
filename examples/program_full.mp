{
  programming in Minipas
  demo with all
  Author: diniremix <diniremix@gmail.com>
}
program miprograma1;

var a, b: integer;
begin

//
// que pasa aqui
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
end.
// aqui va uno al final
