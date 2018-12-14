use crate::util::datetime::{Date,Time};
use crate::util::coord::RawPosition;
use crate::util::parse_error::ParseError;

/// The first flavor of C Record - a task record which defines some properties of the whole task.
#[derive(Debug, PartialEq, Eq)]
pub struct CRecordDeclaration {
    pub date: Date,
    pub time: Time,
    pub flight_date: Date,
    pub task_id: u16,
    pub turnpoint_count: u8,
    pub name: Option<String>,
}

impl CRecordDeclaration {
    pub fn parse(line: &str) -> Result<Self, ParseError> {
        assert!(line.len() >= 25);
        assert!(line.as_bytes()[0] == b'C');

        let date = Date::parse(&line[1..7])?;
        let time = Time::parse(&line[7..13])?;
        let flight_date = Date::parse(&line[13..19])?;
        let task_id = line[19..23].parse::<u16>()?;
        let turnpoint_count = line[23..25].parse::<u8>()?;
        let name = if line.len() > 25 {
            Some(String::from(&line[25..]))
        } else {
            None
        };

        Ok(CRecordDeclaration { date, time, flight_date, task_id, turnpoint_count, name })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CRecordTurnpoint {
    pub position: RawPosition,
    pub name: Option<String>,
}

impl CRecordTurnpoint {
    pub fn parse(line: &str) -> Result<Self, ParseError> {
        assert!(line.len() >= 18);
        assert!(line.as_bytes()[0] == b'C');

        let position = RawPosition::parse_lat_lon(&line[1..18])?;
        let name = if line.len() > 18 {
            Some(String::from(&line[18..]))
        } else {
            None
        };

        Ok(CRecordTurnpoint { position, name })
    }
}


#[cfg(test)]
mod test {
    use super::{*};
    use crate::util::coord::{Compass,RawCoord,RawPosition};

    #[test]
    fn c_record_declaration_parse() {
        let sample_string = "C230718092044000000000204Foo task";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        let mut expected = CRecordDeclaration {
            date: Date { day: 23, month: 07, year: 2018 },
            time: Time { hours: 09, minutes: 20, seconds: 44 },
            flight_date: Date { day: 00, month: 00, year: 2000 },
            task_id: 2,
            turnpoint_count: 4,
            name: Some("Foo task".to_string())
        };
        assert_eq!(parsed_declaration, expected);

        let sample_string = "C230718092044000000000204";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        expected.name = None;
        assert_eq!(parsed_declaration, expected);

    }

    #[test]
    fn c_record_turnpoint_parse() {
        let sample_string = "C5156040N00038120WLBZ-Leighton Buzzard NE";
        let parsed_turnpoint = CRecordTurnpoint::parse(sample_string).unwrap();
        let expected = CRecordTurnpoint {
            position: RawPosition {
                lat: RawCoord { degrees: 51, minutes: 56, minutes_fraction: 40, sign: Compass::North },
                lon: RawCoord { degrees: 00, minutes: 38, minutes_fraction: 120, sign: Compass::West },
            },
            name: Some("LBZ-Leighton Buzzard NE".to_string()),
        };

        assert_eq!(parsed_turnpoint, expected);

    }
}