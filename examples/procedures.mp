program test_procedures;

procedure saludar();
begin
	writeln("Hola mundo desde un procedimiento");
end

procedure saludo(nombre:string);
begin
	writeln("Hola", nombre, "saludos desde un procedimiento");
	//return 0;
end

begin
	saludar();
	saludo("minipas!");
end.
