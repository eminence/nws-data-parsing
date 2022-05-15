//! NWS Public Forecast Zones
//!
//! NWS's description of forecast zones:
//!
//! > The NWS issues forecasts and some watches and warnings for public
//! > zones which usually are the same as counties but in many cases are
//! > subsets of counties.  Counties are subset into zones to allow for
//! > more accurate forecasts because of the differences in weather within
//! > a county due to such things as elevation or proximity to large
//! > bodies of water.
//!
//! This crate contains information about all 3882 public [forecast zones](ForecastZone),
//! based on the April 5, 2022 data dump from [NWS's website](https://www.weather.gov/gis/PublicZones).
//!
//! You can get a [ForecastZone] by calling [`from_str`](ForecastZone::from_str), or by referencing one
//! of the enum variants.  Note that the variants are not docmented (because there are so many of them),
//! but you can still access them.
//!
//!
//!
//! # Example
//!
//! ```rust
//! # use nws_forecast_zones::*;
//! let parsed = parse_zoneset("MAZ012-013-017-RIZALL").unwrap();
//! assert!(parsed.contains(ForecastZone::RI001));
//! assert!(parsed.contains(ForecastZone::MA013));
//! assert!(! parsed.contains(ForecastZone::MA014));
//! ```
//!
//! This crate uses data published from NWS, but is otherwise unaffiliated with the National Weather Service,
//! and is not an official NWS library.

mod gen;

/// Details about an NWS forecast zone
pub struct ZoneDetails {
    /// Two-letter state abbreviation
    pub state: &'static str,
    /// Three-digit zone number
    pub zone: &'static str,
    /// The zone number, as a u16
    pub zone_numeric: u16,
    /// Name of this zone
    pub name: &'static str,
    /// WFO (Weather forecasting office) for this zone
    pub wfo: &'static str,
}

use std::{num::ParseIntError, str::FromStr};

pub use gen::ForecastZone;

#[cfg_attr(test, derive(Debug))]
enum ZoneSetEnum {
    /// All zones in a specific state
    All(String),
    /// A inclusive range of zones for a specific state
    Range(String, u16, u16),
    /// A specific forecast zone
    Specific(ForecastZone),
    /// A list of zones
    List(Vec<ZoneSetEnum>),
}
impl ZoneSetEnum {
    fn contains(&self, zone: ForecastZone) -> bool {
        match self {
            ZoneSetEnum::All(st) => zone.details().state == st,
            ZoneSetEnum::Range(st, lo, hi) => {
                let d = zone.details();
                d.state == st && d.zone_numeric >= *lo && d.zone_numeric <= *hi
            }
            ZoneSetEnum::Specific(a) => *a == zone,
            ZoneSetEnum::List(l) => {
                for sub in l {
                    if sub.contains(zone) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

/// A set of one or more forecast zones
#[cfg_attr(test, derive(Debug))]
pub struct ZoneSet {
    inner: ZoneSetEnum,
}

impl ZoneSet {
    /// Is a specific forecast zone in this zone set?

    pub fn contains(&self, zone: ForecastZone) -> bool {
        self.inner.contains(zone)
    }
}

/// Error type for [parse_zoneset]
#[derive(Debug)]
pub enum ZoneSetError {
    /// We needed to read another character, but we were out!
    OutOfChars,
    /// We encountered some unexpected character
    UnexpectedChar(char),

    ParsingError,
}

impl From<ParseIntError> for ZoneSetError {
    fn from(_: ParseIntError) -> Self {
        ZoneSetError::ParsingError
    }
}

/// Parse a range of forecast zones
///
/// Often NWS forecasts will list a ranges of zones that apply to a given forecast.  This range might look something
/// like this:
///
/// ```test
/// MAZ017>019-RIZ001>003-140815-
/// ```
///
/// This range includes MA zones 017 through 019 (inclusive) and RI zones 001 through 003
///
/// This function will parse a string of this form into a [ZoneSet], which can be used to query
/// if a specific zone is in the parsed set.
pub fn parse_zoneset(mut range: &str) -> Result<ZoneSet, ZoneSetError> {
    macro_rules! read1 {
        ($chars:expr) => {{
            $chars.next().ok_or(ZoneSetError::OutOfChars)?
        }};
    }
    macro_rules! read3 {
        ($chars:expr) => {{
            let mut n = String::with_capacity(3);
            for _ in 0..3 {
                n.push($chars.next().ok_or(ZoneSetError::OutOfChars)?);
            }
            n
        }};
    }

    #[derive(Eq, PartialEq, Debug)]
    enum State {
        /// We want to read 3 chars, which we expect to be state in the form "__Z"
        ExpectingStateZ,
        /// We want 3 chars, which are either 'ALL' or 3 digits
        ExpectingTricode(String),

        /// We expect either a '-' or '>'
        ///
        /// We've already parsed a numeric zone, and now we expect either '-' or '>'
        ExpectingListOrRange(String, u16),

        ExpectingEndOfRange(String, u16),

        /// We are expecting either a new 3-char state, or a 3-digit numeric code
        ExpectingStateOrCode(String),
    }

    // if the last 8 chars look to be of the form "-123456-", then remove them
    let l = range.len();
    if l > 8
        && range[l - 8..]
            .chars()
            .all(|c| c == '-' || c.is_ascii_digit())
    {
        range = &range[..l - 8];
    }

    // let mut cur = Cursor::new(range);
    let mut chars = range.chars().peekable();

    let mut list = Vec::new();

    let mut parsing_state = State::ExpectingStateZ;

    // let mut state = String::new();
    // let mut numeric = 0;

    loop {
        parsing_state = match parsing_state {
            State::ExpectingStateZ => {
                let t = read3!(chars);
                assert!(t.ends_with('Z'));
                State::ExpectingTricode(t)
            }
            State::ExpectingTricode(state) => {
                let n = read3!(chars);
                if n == "ALL" {
                    list.push(ZoneSetEnum::All(state[0..2].to_string()));
                    if chars.peek().is_none() {
                        break;
                    }
                    let x = read1!(chars);
                    if x != '-' {
                        return Err(ZoneSetError::UnexpectedChar(x));
                    }
                    State::ExpectingStateZ
                } else if n.chars().all(|c| c.is_ascii_digit()) {
                    let numeric = n.parse()?;
                    // we now need to read 1 more character, which should either be a '-' or '>'

                    State::ExpectingListOrRange(state, numeric)
                } else {
                    todo!()
                }
            }
            State::ExpectingListOrRange(state, numeric) => {
                match chars.next() {
                    None => {
                        // end of data, so we know we're not the start of a range.  thus we are a specific zone
                        list.push(ZoneSetEnum::Specific(
                            ForecastZone::from_str(&format!("{}{:03}", &state[..2], numeric))
                                .unwrap(),
                        ));
                        break;
                    }
                    Some('>') => {
                        // we've already read the first part of the range (in 'numeric'), now
                        // we expect to read another 3 digits, which indicate the ending zone of a range
                        State::ExpectingEndOfRange(state, numeric)
                    }
                    Some('-') => {
                        // we've already read the first part of the range (in 'numeric') and we
                        // know we're not the start of a range (nor the end).  so we have a specific zone

                        list.push(ZoneSetEnum::Specific(
                            ForecastZone::from_str(&format!("{}{:03}", &state[..2], numeric))
                                .unwrap(),
                        ));

                        State::ExpectingStateOrCode(state)
                    }
                    Some(x) => {
                        return Err(ZoneSetError::UnexpectedChar(x));
                    }
                }
            }
            State::ExpectingEndOfRange(state, numeric) => {
                let code = read3!(chars);
                let end_numeric = code.parse()?;
                list.push(ZoneSetEnum::Range(
                    state[0..2].to_string(),
                    numeric,
                    end_numeric,
                ));

                match chars.next() {
                    None => break,
                    Some('-') => State::ExpectingStateOrCode(state),
                    Some(x) => return Err(ZoneSetError::UnexpectedChar(x)),
                }
            }
            State::ExpectingStateOrCode(state) => {
                let t = read3!(chars);
                if t.ends_with('Z') {
                    State::ExpectingTricode(t)
                } else if t.chars().all(|c| c.is_ascii_digit()) {
                    let numeric = t.parse()?;
                    State::ExpectingListOrRange(state, numeric)
                } else {
                    todo!("{:?}", t)
                }
            }
        };
    }

    let inner = if list.len() == 1 {
        list.pop().unwrap()
    } else {
        ZoneSetEnum::List(list)
    };

    Ok(ZoneSet { inner })
}

#[cfg(test)]
mod tests {

    use crate::parse_zoneset;
    use crate::ForecastZone;

    #[test]
    fn test_parsing() {
        let a = parse_zoneset("RIZALL").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::RI004));
        assert!(!a.contains(ForecastZone::AK017));

        let a = parse_zoneset("MEZALL-NHZALL-142200-").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::ME001));
        assert!(a.contains(ForecastZone::NH001));
        assert!(!a.contains(ForecastZone::RI001));

        let a = parse_zoneset("RIZ001-002").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::RI001));
        assert!(a.contains(ForecastZone::RI002));
        assert!(!a.contains(ForecastZone::RI003));

        let a = parse_zoneset("RIZ001-002-MAZ003").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::RI001));
        assert!(a.contains(ForecastZone::RI002));
        assert!(!a.contains(ForecastZone::RI003));
        assert!(a.contains(ForecastZone::MA003));

        let a = parse_zoneset("RIZ001>002").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::RI001));
        assert!(a.contains(ForecastZone::RI002));
        assert!(!a.contains(ForecastZone::RI003));

        let a = parse_zoneset("MAZ017>019-RIZ001>003-140815-").unwrap();
        println!("{a:?}");
        assert!(!a.contains(ForecastZone::MA016));
        assert!(a.contains(ForecastZone::MA017));
        assert!(a.contains(ForecastZone::MA018));
        assert!(a.contains(ForecastZone::MA019));
        assert!(!a.contains(ForecastZone::MA020));

        assert!(a.contains(ForecastZone::RI001));
        assert!(a.contains(ForecastZone::RI002));
        assert!(a.contains(ForecastZone::RI003));
        assert!(!a.contains(ForecastZone::RI004));

        let a =
            parse_zoneset("CTZ002>004-MAZ002>006-008>014-017-020-026-RIZ001>007-110000-").unwrap();
        println!("{a:?}");
        assert!(a.contains(ForecastZone::CT002));
        assert!(a.contains(ForecastZone::MA006));
        assert!(!a.contains(ForecastZone::MA007));
        assert!(a.contains(ForecastZone::MA008));
        assert!(a.contains(ForecastZone::MA009));
        assert!(a.contains(ForecastZone::MA014));
        assert!(!a.contains(ForecastZone::MA015));
        assert!(!a.contains(ForecastZone::MA016));
        assert!(a.contains(ForecastZone::MA017));
        assert!(a.contains(ForecastZone::RI001));
        assert!(a.contains(ForecastZone::RI007));
        assert!(!a.contains(ForecastZone::RI008));

        let a = parse_zoneset("ILZ095>097-MOZ018-019-026-027-034>036-142200-").unwrap();
        println!("{a:?}");
    }
}

// Test docs
//  RIZALL-131000-
//  MEZ002-015-021-024-NHZ003-005-008-011-012-014-VTZ005-008-100300-
//  MAZ017>019-RIZ001>003-140815-
//  CTZ002-004>006-009-010-012-RIZ004-006-007-MAZ004-011-015-142100-
//  CTZ002>004-MAZ002>006-008>014-017-020-026-RIZ001>007-110000-
//  MEZALL-NHZALL-142200-
