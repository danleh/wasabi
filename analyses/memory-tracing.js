{
	// TODO possibly compress accesses to avoid OOM
	const accesses /* : [{func, instr, addr, write: bool}] */ = [];

	function access({func, instr}, {addr, offset}, write) {
		accesses.push({func, instr, addr: addr+offset, write});
	};

	Wasabi.analysis = {
		load(loc, op, memarg) { access(loc, memarg, false) },
		store(loc, op, memarg) { access(loc, memarg, true) },
	};

	Wasabi.analysisResult = accesses;
}