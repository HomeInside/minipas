program fizzbuzz;

var n: integer;

begin
	n := 1;

	while n < 101 do
	begin
		if n % 15 = 0 then
			writeln("fizzbuzz");
		else if n % 3 = 0 then
			writeln("fizz");
		else if n % 5 = 0 then
			writeln("buzz");
		else
			writeln(n);

		n := n+1;
	end
end.

