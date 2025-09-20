program testFor;

var tabla, num: integer;
var letra: string;

function fibonacci(num:integer) : integer;
begin
	if num < 2 then
		return num;
	return fibonacci(num-1) + fibonacci(num-2);
end

begin
  for tabla := 1 to 10 do
  begin
    for num := 1 to 10 do
    begin
      writeln(tabla,'*', num, ' = ', (tabla*num));
    end
    writeln();
  end

  for num := 5 downto 1 do
    writeln("hello foor loop in minipas", num);

  writeln();

  for num := 10 downto 1 do
    writeln("fibonacci de:", num, "es:", fibonacci(num));

  // pendiente
  {for letra := "a" to "z" do
    writeln( letra );}
end.
