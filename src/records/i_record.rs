use std::{fmt, str};

use crate::records::extension::ExtensionDefRecord;
use crate::util::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct IRecord<'a>(pub ExtensionDefRecord<'a>);

impl<'a> IRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let first_byte = line.as_bytes()[0];
        assert!(first_byte == b'I');
        Ok(IRecord(ExtensionDefRecord::parse(line)?))
    }
}

impl<'a> fmt::Display for IRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f, 'I')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::extension::Extension;

    #[test]
    fn irecord_format() {
        let expected_string = "I033638FXA3941ENL4246TAS";
        let record = IRecord(ExtensionDefRecord {
            num_extensions: 3,
            extensions: vec![
                Extension::new("FXA", 36, 38),
                Extension::new("ENL", 39, 41),
                Extension::new("TAS", 42, 46),
            ],
        });

        assert_eq!(format!("{}", record), expected_string);
    }
}
