// see SEISMIC: SEcure In-lined Script Monitors forInterrupting Cryptojacks
// https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=1&cad=rja&uact=8&ved=0ahUKEwjHt8KE0rfcAhWMalAKHY75Bl4QFggoMAA&url=http%3A%2F%2Fwww.utdallas.edu%2F~hamlen%2Fwang18esorics.pdf&usg=AOvVaw2u-ajqPIRKG1B7beQ5fVvL
{
	Wasabi.analysisResult = {
		add: 0,
		and: 0,
		shl: 0,
		shr: 0,
		xor: 0,
	};

	Wasabi.analysis = {
		binary(loc, op) {
			switch (op) {
				case "i32.add":
					Wasabi.analysisResult.add++;
					break;
				case "i32.and":
					Wasabi.analysisResult.and++;
					break;
				case "i32.shl":
					Wasabi.analysisResult.shl++;
					break;
				case "i32.shr_u":
					Wasabi.analysisResult.shr++;
					break;
				case "i32.xor":
					Wasabi.analysisResult.xor++;
			}
		}
	};
}
