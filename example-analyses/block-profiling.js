{
	const counts = [];

	Wasabi.analysis.begin = function ({func, instr}, type) {
		counts[func] = counts[func] || [];
		counts[func][instr] = counts[func][instr] || { count: 0, type };
		counts[func][instr].count++;
	};

	Wasabi.analysisResult = counts;
}