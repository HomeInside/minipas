program testStrings;
var cadena, trimmerStr: string;
var ancho: integer;
begin
	cadena:= " Anita lava la tinA ";
	ancho := len(cadena);

	writeln("cadena de caracteres:","'",cadena,"'");
	writeln();
	writeln("el ancho de la cadena de caracteres es (len):", ancho);
	writeln("el ancho de la cadena de caracteres es (length):", length(cadena));
	writeln();
	writeln("a mayusculas:", upper(cadena));
	writeln("a minusculas:", lower(cadena));
	writeln();
	writeln("antes del trim:", "'",cadena,"'");
	writeln();
	writeln("despues del trim");
	trimmerStr := trim(cadena);
	//writeln("trim2:", "'",trim(cadena),"'");
	writeln("trim2:", "'",trimmerStr,"'");
end.
