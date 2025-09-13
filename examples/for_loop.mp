program testFor;

var
tabla, numero: integer;

begin
  for tabla := 1 to 5 do
  begin
    for numero := 1 to 10 do
    begin
      writeln( tabla, ' por ', numero ,' es ', tabla * numero );
      writeln("algo aqui");
    end
  end
  writeln("aqui al final");
end.

