program testSys;
begin
	clrscr();
	writeln("usando el modulo sys");
	writeln();
	writeln("espera 1 segundo...");
	writeln("");
	sleep();
	writeln("espera 5 segundos...");
	writeln("", sleep(5));
	writeln("");
	writeln("esperando 1.234 segundos:", sleep(1.234));
	writeln("");
	writeln("espera 0 segundos...");
	sleep(0.1); // panic si N<=0
	writeln("");
	writeln("informacion del sistema");
	writeln();
	writeln("platform:", platform());
	writeln("version:", version());
	writeln();
	writeln("exiting...");
	exit();
	writeln("terminado");
end.
