program testSys;
var x:real;
begin
    writeln("usando el modulo sys");

    writeln("espera 1 segundo");
    writeln("");
    x := sleep(); // ignorando x

    writeln("");
    writeln("espera 5 segundos");
    writeln("", sleep(5));

    writeln("");
    writeln("esperando 1.234 segundos:", sleep(1.234));
    writeln("");
    writeln("platform:", platform());
    writeln("version:", version());
    writeln("exit:", exit());
    writeln("terminado");
end.
