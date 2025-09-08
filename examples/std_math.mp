program testMath;

var a: real;
var b: real;
var c: real;
var d: real;

begin
    a := 2;
    b := -3;
    c := 0;

    // === Constantes ===
    writeln("PI = ", PI);
    writeln("E = ", E);

    // === Potencia y valor absoluto ===
    writeln("pow(2,3) = ", pow(2,3));
    writeln("pow(5,0) = ", pow(5,0));
    writeln("abs(-5) = ", abs(-5));
    writeln("abs(3) = ", abs(3));

    // === Trigonométricas ===
    writeln("sin(PI/2) = ", sin(PI/2));
    writeln("cos(PI) = ", cos(PI));
    writeln("tan(PI/4) = ", tan(PI/4));
    writeln("cot(PI/4) = ", cot(PI/4));

    writeln("asin(1) = ", asin(1));
    writeln("acos(0) = ", acos(0));
    writeln("atan(1) = ", atan(1));

    // === Exponenciales y logaritmos ===
    writeln("exp(1) = ", exp(1));
    writeln("ln(E) = ", ln(E));
    writeln("log10(100) = ", log10(100));

    // === Raíz cuadrada ===
    writeln("sqrt(16) = ", sqrt(16));

    // === Redondeos ===
    writeln("floor(3.7) = ", floor(3.7));
    writeln("ceil(3.2) = ", ceil(3.2));
    writeln("round(2.6) = ", round(2.6));

    // === trunc ===
    writeln("trunc(3.14159) = ", trunc(3.14159));
    writeln("trunc(-2.71828) = ", trunc(-2.71828));

    // === Mínimo y máximo ===
    writeln("min(3,7) = ", min(3,7));
    writeln("max(3,7) = ", max(3,7));

    writeln("sign(-5) = ", sign(-5));
    writeln("sign(0) = ", sign(0));
    writeln("sign(7) = ", sign(7));
end.
