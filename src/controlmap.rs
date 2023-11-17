use crate::parser::{control_map_parser, Line};
use crate::scan_code::ScanCodeError;
use core::fmt;
use core::slice::Iter;
use nom::error::convert_error;
use std::vec::IntoIter;

/// Structure with an array containing `controlmap.txt` line by line.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlMap {
    lines: Vec<Line>,
}

impl fmt::Display for ControlMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub type Result<T, E = ControlMapError> = core::result::Result<T, E>;

impl ControlMap {
    /// ControlMap from text input
    pub fn from_txt(txt: &str) -> Result<Self> {
        let (remain, lines) = control_map_parser(txt).map_err(|err| {
            let err = match err {
                nom::Err::Incomplete(_) => "Incomplete error".into(),
                nom::Err::Error(err) => convert_error(txt, err),
                nom::Err::Failure(err) => convert_error(txt, err),
            };

            ControlMapError::ParseError(err)
        })?;

        match remain.is_empty() {
            true => Ok(Self { lines }),
            false => Err(ControlMapError::Incomplete(remain.into())),
        }
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> Iter<'_, Line> {
        self.lines.iter()
    }
}

impl IntoIterator for ControlMap {
    type Item = Line;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlMapError {
    #[error("ParseError. Reason:\n{0}")]
    ParseError(String),
    #[error("Incomplete parse.  Remain:\n{0}")]
    Incomplete(String),
    #[error(transparent)]
    ScanCodeError(#[from] ScanCodeError),
}
