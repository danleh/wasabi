use core::fmt;
use std::str::FromStr;

use serde::Serialize;

use crate::ValType;

// TODO crazy, even faster idea: use a single number for function types
// create a static mapping from number <-> function type, 
// e.g., 0 == [] -> []
//       1 == [i32] -> []
//       2 == [] -> [i32]
//       3 == [i32] -> [i32]
// etc.

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize)]
pub struct FunctionType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

impl FunctionType {
    pub fn new(params: &[ValType], results: &[ValType]) -> Self {
        FunctionType { 
            params: params.into(), 
            results: results.into() 
        }
    }

    pub fn inputs(&self) -> &[ValType] {
        &self.params
    }

    pub fn results(&self) -> &[ValType] {
        &self.results
    }
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?} -> {:?}", self.params, self.results).to_lowercase())
    }
}

impl FromStr for FunctionType {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        fn trim_filter(s: &str) -> Option<&str> {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }

        // Split by the arrow.
        let mut splitted = str.split("->");
        let params = splitted.next().ok_or(())?;
        let results = splitted.next().ok_or(())?;
        if splitted.next().is_some() {
            // More than one arrow in the type is invalid.
            return Err(());
        }

        // Split individual types by comma, and remove brackets.
        let params = params
            .trim()
            .strip_prefix('[').ok_or(())?
            .strip_suffix(']').ok_or(())?
            .split(',')
            .filter_map(trim_filter)
            .map(ValType::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let results = results
            .trim()
            .strip_prefix('[').ok_or(())?
            .strip_suffix(']').ok_or(())?
            .split(',')
            .filter_map(trim_filter)
            .map(ValType::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FunctionType { params, results })
    }
}

impl From<crate::FunctionType> for FunctionType {
    fn from(ty: crate::FunctionType) -> Self {
        ty.as_ref().clone()
    }
}
