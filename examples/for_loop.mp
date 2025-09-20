program testFor;

var tabla, num: integer;

begin
  for tabla := 1 to 10 do
  begin
    for num := 1 to 10 do
    begin
      writeln(tabla,'*', num, ' = ', (tabla*num));
    end
    writeln();
  end
end.
