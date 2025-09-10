program testSys;
var x:real;
begin
	clrscr();
	writeln("usando el modulo sys");
	writeln();
	writeln("espera 1 segundo");
	writeln("");
	x := sleep(); // ignorando x por el null
	writeln("x",x);
	writeln("espera 5 segundos");
	//writeln("", sleep(5));
	writeln("");
	writeln("esperando 1.234 segundos:", sleep(1.234));
	writeln("");
	writeln("informacion del sistema");
	writeln();
	writeln("platform:", platform());
	writeln("version:", version());
	writeln("exit:", exit());
	writeln("terminado");
end.
