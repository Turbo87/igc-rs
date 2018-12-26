use crate::util::datetime::{Date,Time};
use crate::util::coord::RawPosition;
use crate::util::parse_error::ParseError;

/// The first flavor of C Record - a task record which defines some properties of the whole task.
///
/// The IGC specification states that a conforming file containing a task declaration will contain
/// a CRecordDeclaration, immediately followed (turnpoint_count + 4) CRecordTurnpoints's.
/// The extra 4 turnpoints are for the takeoff/land locations, and the task start/finish locations
#[derive(Debug, PartialEq, Eq)]
pub struct CRecordDeclaration<'a> {
    pub date: Date,
    pub time: Time,
    pub flight_date: Date,
    pub task_id: u16,
    pub turnpoint_count: u8,
    pub task_name: Option<&'a str>,
}

impl<'a> CRecordDeclaration<'a> {
    /// Parse a string as a C record task declaration
    ///
    /// ```
    /// # extern crate igc_rs;
    /// # use igc_rs::{ records::CRecordDeclaration, util::{Time,Date} };
    /// let record = CRecordDeclaration::parse("C230718092044000000000204Foo task").unwrap();
    /// assert_eq!(record.date, Date::from_dmy(23, 7, 2018));
    /// assert_eq!(record.time, Time::from_hms(9, 20, 44));
    /// assert_eq!(record.task_id, 2);
    /// assert_eq!(record.turnpoint_count, 4);
    /// assert_eq!(record.task_name, Some("Foo task"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert!(line.len() >= 25);
        assert!(line.as_bytes()[0] == b'C');

        let date = Date::parse(&line[1..7])?;
        let time = Time::parse(&line[7..13])?;
        let flight_date = Date::parse(&line[13..19])?;
        let task_id = line[19..23].parse::<u16>()?;
        let turnpoint_count = line[23..25].parse::<u8>()?;
        let task_name = if line.len() > 25 {
            Some(&line[25..])
        } else {
            None
        };

        Ok(CRecordDeclaration { date, time, flight_date, task_id, turnpoint_count, task_name })
    }
}

/// The second flavor of C Record - a start / turn / end point for a task.
#[derive(Debug, PartialEq, Eq)]
pub struct CRecordTurnpoint<'a> {
    pub position: RawPosition,
    pub turnpoint_name: Option<&'a str>,
}

impl<'a> CRecordTurnpoint<'a> {
    /// Parse a string as a C record task turnpoint
    ///
    /// ```
    /// # extern crate igc_rs;
    /// # use igc_rs::{ records::CRecordTurnpoint, util::{Compass,RawCoord} };
    /// let record = CRecordTurnpoint::parse("C5156040N00038120WLBZ-Leighton Buzzard NE").unwrap();
    /// assert_eq!(record.position.lat,
    ///            RawCoord { degrees: 51, minute_thousandths: 56040, sign: Compass::North });
    /// assert_eq!(record.position.lon,
    ///            RawCoord { degrees: 0, minute_thousandths: 38120, sign: Compass::West });
    /// assert_eq!(record.turnpoint_name, Some("LBZ-Leighton Buzzard NE"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert!(line.len() >= 18);
        assert!(line.as_bytes()[0] == b'C');

        let position = RawPosition::parse_lat_lon(&line[1..18])?;
        let turnpoint_name = if line.len() > 18 {
            Some(&line[18..])
        } else {
            None
        };

        Ok(CRecordTurnpoint { position, turnpoint_name })
    }
}


#[cfg(test)]
mod tests {
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
            task_name: Some("Foo task")
        };
        assert_eq!(parsed_declaration, expected);

        let sample_string = "C230718092044000000000204";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        expected.task_name = None;
        assert_eq!(parsed_declaration, expected);

    }

    #[test]
    fn c_record_turnpoint_parse() {
        let sample_string = "C5156040N00038120WLBZ-Leighton Buzzard NE";
        let parsed_turnpoint = CRecordTurnpoint::parse(sample_string).unwrap();
        let expected = CRecordTurnpoint {
            position: RawPosition {
                lat: RawCoord { degrees: 51, minute_thousandths: 56040, sign: Compass::North },
                lon: RawCoord { degrees: 0, minute_thousandths: 38120, sign: Compass::West },
            },
            turnpoint_name: Some("LBZ-Leighton Buzzard NE"),
        };

        assert_eq!(parsed_turnpoint, expected);

    }
}
