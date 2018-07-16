use serde_plain;
use std::collections::HashSet;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighLevelHook {
    Start,

    Nop,
    Unreachable,

    Br,
    BrIf,
    BrTable,

    If,
    Begin,
    End,

    // together for call_pre and call_post
    Call,
    Return,

    Drop,
    Select,

    Const,
    Unary,
    Binary,

    Load,
    Store,

    MemorySize,
    MemoryGrow,

    Local,
    Global,
}

#[derive(Debug)]
pub struct EnabledHooks(HashSet<HighLevelHook>);

impl EnabledHooks {
    pub fn all() -> Self {
        use self::HighLevelHook::*;
        static VARIANTS: [HighLevelHook; 21] = [Nop, Unreachable, Br, BrIf, BrTable, If, Begin, End, Call, Return, Drop, Select, Const, Unary, Binary, Load, Store, MemorySize, MemoryGrow, Local, Global];
        EnabledHooks(VARIANTS.iter().cloned().collect())
    }

    /// if this option is given, instrument no hook by default, only the given ones
    pub fn from_hooks(s: &str) -> io::Result<Self> {
        let mut result = HashSet::new();
        for hook in s.split(',') {
            result.insert(
                serde_plain::from_str(hook)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid hook".to_string()))?);
        }
        Ok(EnabledHooks(result))
    }

    /// if this option is given, instrument all hooks by default, except for the given ones
    pub fn from_no_hooks(s: &str) -> io::Result<Self> {
        let result = Self::all().0
            .difference(&Self::from_hooks(s)?.0)
            .cloned().collect();
        Ok(EnabledHooks(result))
    }

    pub fn is_enabled(&self, hook: HighLevelHook) -> bool {
        self.0.contains(&hook)
    }
}