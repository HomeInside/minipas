program helloworld;
var a, b: integer;
begin

    writeln("Hello, World in minipas!");
    writeln("suma:", 4+1);
    writeln("resta:", 4-3);
    writeln("multiplicacion:", 4*2);
    writeln("division:", (4/5));
    writeln("precede?:", 4+1-3*2/5);
    writeln("con parentesis:", (4+1-3)*(2/5));

a := 42;
b := 8;
c := a+b;
writeln("el valor de 'a' es:", a, "el valor de 'b' es:", b, "y el valor de 'c' es:", c);
end.
