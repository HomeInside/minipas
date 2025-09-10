{
  A pure Minipas implementation of Haversine formula
  Based on Bartek Górny implementation and others
  Author: Jorge Brunal Pérez <diniremix@gmail.com>

  How to use:

  //from haversine import calculate_distance

  origin = [4.6565, -74.564654]
  destiny = [4.6748, -74.05485]

  result_in_kms = calculate_distance(origin, destiny, 'kms')
  print "result in kms", result_in_kms

  Otherwise
  result_in_mts = calculate_distance(origin, destiny)
  print "result in mts", result_in_mts

  programming in Minipas
  demo with all
  Author: diniremix <diniremix@gmail.com>
}

{
	//calculate_distance
	r = 6371
	rad = math.pi / 180
	lat1 = lat[0]
	lat2 = lon[0]

	lon1 = lat[1]
	lon2 = lon[1]

	dlat = (lat2 - lat1) * rad
	dlon = (lon2 - lon1) * rad

	a = math.sin(dlat / 2) * math.sin(dlat / 2) + math.cos(lat1 * rad) * math.cos(lat2 * rad) * math.sin(
	    dlon / 2
	) * math.sin(dlon / 2)
	c = 2 * math.asin(math.sqrt(a))
	d = r * c

	resp = 0
	if type_measure == "kms":
	    resp = round(d, 2)
	if type_measure == "mts":
	    resp = round((d * 1000), 2)
	return resp
}

program Haversine1;
// TODO
// soporte para arrays
// variables con `_`
// `r` como variable standar o algo funciona sola
//var origin, destiny:real;
//result_in_kms, result_in_mts:real;
var origin1, origin2, destiny1, destiny2: real;
var r, rad: real;
var lat1, lat2: real;
var lon1, lon2: real;
var dlat, dlon: real;
var a, b, c, d: real;
var typeMeasure:string;
var resp: real;
begin
	// origin = [4.6565, -74.564654]
	origin1 := 4.6565;
	origin2 := -74.564654;

	// destiny = [4.6748, -74.05485]
	destiny1 := 4.6748;
	destiny2 := -74.05485;

	r := 6371;
	rad := PI / 180;
	lat1 := origin1; // lat1 = lat[0]
	lat2 := destiny1; // lat2 = lon[0]
	lon1 := origin2; // lon1 = lat[1]
	lon2 := destiny2; // lon2 = lon[1]

	dlat := (lat2 - lat1) * rad;
	dlon := (lon2 - lon1) * rad;

	writeln("origin_1:", origin1);
	writeln("origin_2:", origin2);
	writeln("destiny_1:", destiny1);
	writeln("destiny_2:", destiny2);
	writeln("rad:", rad);
	writeln("lat1:", lat1);
	writeln("lat2:", lat2);
	writeln("lon1:", lon1);
	writeln("lon2:", lon2);
	writeln("dlat:", dlat);
	writeln("dlon:", dlon);

	a := sin(dlat/2) *sin(dlat/2) + cos(lat1*rad)  *cos(lat2*rad) * sin(dlon/2) * sin(dlon/2);
	writeln("a:", a);

	c := 2* asin(sqrt(a));
	writeln("c:", c);

	d := r*c;
	writeln();
	writeln("========D==========");
	writeln("d:", d);
	writeln();

	typeMeasure := "kms";
	writeln("type_measure:", typeMeasure);
	resp := 0;

	{
	if typeMeasure = "kms" then
		//writeln("entrando a kms");
		resp := round(d); //round(d,2);
	else
		//writeln("entrando a mts");
		resp := round(d*1000); // round((d*1000),2);

	writeln("resp in kms:", resp);
	}
	// result in kms 56.54
	// result in mts 56536.4
	writeln();
	writeln("==================");
	writeln("resp in kms(round):", round(d));
	writeln();
	writeln("resp in kms(fract):", fract(d));
	writeln("resp in kms(floor):", floor(d));
	writeln("resp in kms(ceil):", ceil(d));
	writeln("==================");
	writeln();
	writeln("resp in mts(round):", round(d*1000));
	writeln();
	writeln("resp in mts(fract):", fract(d*1000));
	writeln("resp in mts(floor):", floor(d*1000));
	writeln("resp in mts(ceil):", ceil(d*1000));

end.
