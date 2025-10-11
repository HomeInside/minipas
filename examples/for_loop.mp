program testFor;
var tabla, num: integer;
var letra: string;
var i: integer;

function fibonacci(num:integer) : integer;
begin
	if num < 2 then
		return num;
	return fibonacci(num-1) + fibonacci(num-2);
end

function test_return_loop(): integer;
var j: integer;
begin
  for j := 1 to 5 do
  begin
    writeln("valor de j:", j);
    if j = 3 then return j;
  end
  writeln("esto nunca se imprime...");
end

function busca(): integer;
var i: integer;
begin
  for i := 1 to 5 do
  begin
    if i = 3 then
    begin
      writeln('encontrado', i);
      return i;
    end
  end
  return 0;
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

  for i := 1 to 10 do
  begin
    writeln("entrando al for con break/continue, i es:", i);
    if i = 3 then continue;
    if i = 8 then break;
    writeln("el valor de i, es", i);
  end

  for num := 10 downto 1 do
    writeln("fibonacci de:", num, "es:", fibonacci(num));

  // pendiente
  {for letra := "a" to "z" do
    writeln( letra );}

  writeln();
  writeln("test con break");

  for i := 1 to 5 do
  begin
    if i = 3 then break;
    writeln(i);
  end

  writeln();
  writeln("test con continue");

  for i := 1 to 5 do
  begin
    if i = 3 then continue;
    writeln(i);
  end

  writeln();
  writeln("el valor es:", test_return_loop());

  writeln();

  for i := 1 to 5 do
  begin
    if i = 3 then
    begin
      writeln('saliendo en', i);
      break;
    end
    writeln('i =', i);
  end

  writeln();

  for i := 1 to 5 do
  begin
    if i = 3 then
    begin
      writeln('salta', i);
      continue;
    end
    writeln('i =', i);
  end

  writeln();
  writeln('resultado de busca:', busca());
end.
