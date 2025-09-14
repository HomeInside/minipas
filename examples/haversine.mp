{
  A pure Minipas implementation of Haversine formula
  Based on Bartek Górny implementation and others
  Author: Jorge Brunal Pérez <diniremix@gmail.com>

  How to use:

  //from haversine import calculate_distance

  origin := [4.6565, -74.564654];
  destiny := [4.6748, -74.05485];

  result_in_kms := calculate_distance(origin, destiny, 'kms')
  writeln("result in kms:", result_in_kms);

  Otherwise
  result_in_mts := calculate_distance(origin, destiny)
  writeln("result in mts:", result_in_mts);

  programming in Minipas
  Author: diniremix <diniremix@gmail.com>
}

program Haversine2;
var origin1:real;
var origin2:real;
var destiny1:real;
var destiny2:real;
var result: real;

function calculate_distance(origin1:real; origin2:real; destiny1:real; destiny2:real; typeMeasure:string): real;
// TODO
// soporte para arrays
// variables con `_`
// `r` como variable standar o algo funciona sola
var r: real;
var rad: real;
var lat1: real;
var lat2: real;
var lon1: real;
var lon2: real;
var dlat: real;
var dlon: real;
var a: real;
var b: real;
var c: real;
var d: real;

var resp: real;

begin
	r := 6371;
	rad := PI / 180;

	lat1 := origin1; // lat1 = lat[0]
	lat2 := destiny1; // lat2 = lon[0]
	lon1 := origin2; // lon1 = lat[1]
	lon2 := destiny2; // lon2 = lon[1]

	dlat := (lat2 - lat1) * rad;
	dlon := (lon2 - lon1) * rad;

	a := sin(dlat/2) *sin(dlat/2) + cos(lat1*rad)  *cos(lat2*rad) * sin(dlon/2) * sin(dlon/2);
	// writeln("a:", a);

	c := 2* asin(sqrt(a));
	// writeln("c:", c);

	d := to_real(r*c);
	//writeln("d:", d);

	resp := 0.0;

	if typeMeasure = "kms" then
		resp := floor(d);
	else if typeMeasure = "mts" then
		resp := floor(d*1000);

	return resp;
end

begin
	// origin = [4.6565, -74.564654]
	origin1 := 4.6565;
	origin2 := -74.564654;

	// destiny = [4.6748, -74.05485]
	destiny1 := 4.6748;
	destiny2 := -74.05485;

	writeln("havesine en minipas");
	writeln();

	result := calculate_distance(origin1, origin2, destiny1, destiny2, "kms");
	writeln("result in kms:", result);

	writeln();

	result := calculate_distance(origin1, origin2, destiny1, destiny2, "mts");
	writeln("result in mts:", result);
end.
