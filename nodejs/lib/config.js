const HIGH_LEVEL_HOOK = Object.freeze({
  Start: "Start",

  Nop: "Nop",
  Unreachable: "Unreachable",

  Br: "Br",
  BrIf: "BrIf",
  BrTable: "BrTable",

  If: "If",
  Begin: "Begin",
  End: "End",

  // together for call_pre and call_post
  Call: "Call",
  Return: "Return",

  Drop: "Drop",
  Select: "Select",

  Const: "Const",
  Unary: "Unary",
  Binary: "Binary",

  Load: "Load",
  Store: "Store",

  MemorySize: "MemorySize",
  MemoryGrow: "MemoryGrow",

  Local: "Local",
  Global: "Global"
});

module.exports = HIGH_LEVEL_HOOK;
