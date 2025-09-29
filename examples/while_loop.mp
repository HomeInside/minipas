program testWhile;

var
  frase: string;
  i,j,n,
  total: integer;
  resultado: integer;

function factorial(num:integer) : integer;
begin
  if num = 1 then
    return 1; //Aseguramos que tenga salida siempre
  else
    return num * factorial( num-1 ); // Caso general
end

function sumar(a:integer; b:integer): integer;
var c: integer;
begin
	// writeln("el valor de a es:", a);
	// writeln("el valor de b es:", b);
	// writeln("el valor de c es:", c);
	c := a + b;
	return c;
end

begin
  writeln();
  writeln('Escribe frases, y deja una línea en blanco para salir');
  writeln( '¿Primera frase?' );
  frase := readln();
  writeln();

  while frase <> '' do
  begin
    writeln( 'Su longitud es ', len(frase) );
    write( '¿Siguiente frase?' );
    frase := readln();
  end

  writeln();
  writeln("con incremento");
  i := 1;
  while i <= 5 do
  begin
    writeln("el valor de i es:", i);
    writeln("la suma de 'i + i' es:", sumar(i,i));
    i := i + 1;
  end

  writeln();
  writeln("con decremento");
  i := 10;
  while i > 0 do
  begin
    writeln("el valor de i es:", i);
    writeln("el factorial de:", i, "es:", factorial(i));
    i := i - 1;
  end
  writeln("final valor de i:", i);

  writeln();
  writeln("con sentencias y funciones");
  j := 1;
  while sumar(j, i)<=10 do
  begin
    writeln("el valor de j es:", j);
    j := j + 1;
    i := i + 1;
  end

  writeln();
  writeln("sumatorias con while");
  writeln("para salir escriba 0");
  total := 0;
  n := to_int(readln());

  while n > 0 do
  begin
    total := total + n;
    n := to_int(readln());
  end

  writeln('Suma total: ', total);

  writeln();
  writeln("while anidados");
  i := 1;

  while i <= 3 do
  begin
    j := 1;
    while j <= 2 do
    begin
      writeln('i=', i, ' j=', j);
      j := j + 1;
    end
    i := i + 1;
  end

  // pendiente
  // expresiones booleanas combinadas
  {writeln();
  writeln("expresiones booleanas combinadas");
  a := 0;
  b := 10;
  ok := True;

  while (a < b) and ok do
  begin
    writeln('a=', a, ' b=', b);
    a := a + 1;

    if a = 5 then
      ok := False;
  end}

  i := 1;

  writeln();
  writeln("tablas de multiplicar");

  while (i <= 5) do
  begin
    j := 1;
    while j <= 10 do
    begin
      resultado := i * j;
      writeln(i, ' x ', j, ' = ', resultado);
      j := j + 1;
    end
    writeln('----------------------');
    i := i + 1;
  end

end.
