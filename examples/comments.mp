{
  programming in Minipas
  demo with comments
  Author: diniremix <diniremix@gmail.com>
}
//
{}
// Programa de prueba con comentarios
program comentarios;
begin
{
los comentarios pueden ser multilineas
como este comentarios
}
// o ser para una linea
var a, b: integer;

a := 12;  { asignamos 10 a 'a' }
b := 13;   // otro comentario

if a > b then
    writeln("el valor de a es:", a); // imprime a
else
    writeln("el valor de b es:", b); // imprime b
end.
