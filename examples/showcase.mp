{
  programming in Minipas
  Minipas showcase program
  Author: diniremix <diniremix@gmail.com>
  Minipas version: 1.3.0
}

program showcase;
var
    a, b, c    : integer;
    r          : real;
    s1, s2, s3 : string;
    flag       : boolean;

// -------------------------
// Funciones
// -------------------------
function suma(x: integer; y: integer): integer;
begin
    return x + y;
end

function es_par(n: integer): boolean;
begin
    if (n % 2) = 0 then
        return True;
    else
        return False;
end

function division_real(x: integer; y: integer): real;
begin
    return x / y;
end

function cmp_str(a: string; b: string): string;
begin
    // devuelve una etiqueta según comparación lexicográfica
    if a = b then
        return "equal";
    else if a > b then
        return "greater";
    else
        return "less";
end

// -------------------------
// Recursividad
// -------------------------
function mcd(a:integer; b:integer) : integer;
begin
	if b = 0 then
		return a;
	else
		return mcd(b, a%b);
end

// -------------------------
// Procedimientos
// -------------------------
procedure saludar();
begin
	writeln("Hola mundo desde un procedimiento");
end

procedure saludo(nombre:string);
begin
	writeln("Hola", nombre, "saludos desde un procedimiento");
end

// -------------------------
// Procedimiento de impresión / demostración
// -------------------------
procedure mostrar_estadisticas();
var t: integer;
var result: integer;
var num1: integer;
var num2: integer;

begin
    writeln("----- Demo de expresiones -----");
    writeln();

    // operaciones aritméticas y modulo
    a := 7;
    b := 3;
    writeln("a = ", a, ", b = ", b);
    writeln("a + b ->", a + b);
    writeln("a - b ->", a - b);
    writeln("a * b ->", a * b);
    writeln("a / b ->", a / b);
    writeln("a % b ->", a % b);
    writeln();

    // uso de funciones
    c := suma(a, b);
    writeln("suma(a,b) ->", c);
    writeln("es_par(c) ->", es_par(c));
    r := division_real(a, b);
    writeln("division_real(a,b) ->", r);
    writeln();

    // uso de procedimientos
    writeln();
    saludar();
    saludo("minipas");

    // comparaciones numéricas
    writeln("1 = 2 ->", 1 = 2);
    writeln("1 < 2 ->", 1 < 2);
    writeln("1 <= 1 ->", 1 <= 1);
    writeln();

    // strings: concatenación y comparaciones
    s1 := "hola";
    s2 := "holanda";
    s3 := s1 + " " + s2;
    writeln("s1 ->", s1, ", s2 ->", s2);
    writeln("concat ->", s3);
    writeln("'hola' = 'hola' ->", ("hola" = "hola"));
    writeln("('a' > 'b') ->", ("a" > "b"));
    writeln("'hola' > 'holanda' ->", s1 > s2);
    writeln("cmp_str(s1, s2) ->", cmp_str(s1, s2));
    writeln();

    // mostrar evaluación booleana en if
    t := 10;
    if es_par(t) then
        writeln("t es par (t=", t, ")");
    else
        writeln("t es impar (t=", t, ")");

    writeln();

    // combinación: usar expresiones complejas como argumentos
    // -- notar cuándo pongo paréntesis para no confundir el parser en expr_list
    writeln("combinado ->", (1 + 2 = 3), ", texto ->", "uno" + "dos");

    // ejemplo de recursividad
    writeln();
    writeln("hallar el MCD (recursivo)");
    writeln("escriba el primer numero para hallar el MCD");
    num1 := to_int(readln());

    writeln();
    writeln("escriba el segundo numero para hallar el MCD");
    num2 := to_int(readln());
    resultado := mcd(num1, num2);
    writeln("el MCD de:", num1, "y", num2, "es:", resultado);

    writeln();
    writeln("usando el modulo date_time");

    writeln("Fecha actual: ", date());
	writeln("Hora actual: ", time());
	writeln("Fecha y hora: ", date_time());

    writeln("----- fin del demo -----");
end

// -------------------------
// Programa principal
// -------------------------
begin
    writeln("Inicio del demo_all");
    writeln();
    writeln("escribe un numero");
	input := to_int(readln());
    writeln("el numero es:", input);

    // inicializa y llama al procedimiento ppal
    mostrar_estadisticas();

    writeln();
    writeln("Demo finalizado.");
end.
