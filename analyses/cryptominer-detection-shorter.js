const signature = {};
Wasabi.analysis.binary = function(loc, op) {
	switch (op) {
		case "i32.add":
		case "i32.and":
		case "i32.shl":
		case "i32.shr_u":
		case "i32.xor":
			signature[op] = (signature[op] || 0) + 1;
}};