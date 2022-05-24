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
//! And also 632 [coastal](CoastalMarineZone) and [offshore](OffshoreMarineZone) marine zones
//! (last updated March 22, 2022, downloaded from [here](https://www.weather.gov/gis/MarineZones))
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

use std::num::ParseIntError;

mod gen;
mod gen_fz;
mod gen_mz;
mod gen_oz;

pub use gen::ForecastZone;
pub use gen_fz::FireZone;
pub use gen_mz::CoastalMarineZone;
pub use gen_oz::OffshoreMarineZone;

/// An onshore, offshore, or coastal marine zone
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Zone {
    /// An onshore forecast zone
    ///
    /// See <https://www.weather.gov/gis/PublicZones>
    Forecast(ForecastZone),
    /// A coastal marine forecast zone
    ///
    /// See <https://www.weather.gov/gis/MarineZones>
    CoastalMarine(CoastalMarineZone),
    /// An offshore marine forecast zone
    ///
    /// See <https://www.weather.gov/gis/MarineZones>
    Offshore(OffshoreMarineZone),

    /// A fire weather zone
    ///
    /// See <https://www.weather.gov/gis/FireZones>
    FireZone(FireZone),
}

impl Zone {
    /// Get details about this particular zone
    pub fn details(&self) -> ZoneDetails {
        match self {
            Zone::Forecast(z) => z.details(),
            Zone::CoastalMarine(z) => z.details(),
            Zone::Offshore(z) => z.details(),
            Zone::FireZone(z) => z.details(),
        }
    }
    /// Try to create a new zone
    ///
    /// If the given zone isn't a known onshore, offshore, or coastal zone, `None` is returned
    pub fn new(two: &str, numeric: u16) -> Option<Self> {
        if let Some(z) = ForecastZone::new(two, numeric) {
            Some(Zone::Forecast(z))
        } else if let Some(z) = CoastalMarineZone::new(two, numeric) {
            Some(Zone::CoastalMarine(z))
        } else if let Some(z) = OffshoreMarineZone::new(two, numeric) {
            Some(Zone::Offshore(z))
        } else if let Some(z) = FireZone::new(two, numeric) {
            Some(Zone::FireZone(z))
        } else {
            None
        }
    }
}
impl From<ForecastZone> for Zone {
    fn from(z: ForecastZone) -> Self {
        Zone::Forecast(z)
    }
}

impl From<CoastalMarineZone> for Zone {
    fn from(z: CoastalMarineZone) -> Self {
        Zone::CoastalMarine(z)
    }
}

impl From<OffshoreMarineZone> for Zone {
    fn from(z: OffshoreMarineZone) -> Self {
        Zone::Offshore(z)
    }
}

impl From<FireZone> for Zone {
    fn from(z: FireZone) -> Self {
        Zone::FireZone(z)
    }
}

/// Details about an NWS forecast zone
#[derive(Debug, Copy, Clone)]
pub struct ZoneDetails {
    /// Two-letter state abbreviation, or a two-letter code for marine forecast zones
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

#[cfg_attr(test, derive(Debug))]
enum ZoneSetEnum {
    /// All zones in a specific state
    ///
    /// Only applies for land-based forecast zones (i.e. not Coastal or Offshore zones)
    All(String),
    /// A inclusive range of zones for a specific state
    Range(String, u16, u16),
    /// A specific forecast zone
    Specific(Zone),
    /// A specifc zone, but that is unknown
    UnknownZone(String, u16),
    /// A list of zones
    List(Vec<ZoneSetEnum>),
}
impl ZoneSetEnum {
    fn contains(&self, zone: Zone) -> bool {
        match self {
            ZoneSetEnum::All(st) => {
                if let Zone::Forecast(zone) = zone {
                    zone.details().state == st
                } else {
                    false
                }
            }
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
            ZoneSetEnum::UnknownZone(..) => false,
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

    pub fn contains(&self, zone: impl Into<Zone>) -> bool {
        self.inner.contains(zone.into())
    }
}

/// Error type for [parse_zoneset]
#[derive(Debug)]
pub enum ZoneSetError {
    /// We needed to read another character, but we were out!
    OutOfChars,
    /// We encountered some unexpected character
    UnexpectedChar(char),

    /// We encountered an unexpected string
    UnexpectedString(String),

    ParsingError,

    NoSuchZone(String, u16),
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
                    return Err(ZoneSetError::UnexpectedString(n));
                }
            }
            State::ExpectingListOrRange(state, numeric) => {
                match chars.next() {
                    None => {
                        // end of data, so we know we're not the start of a range.  thus we are a specific zone
                        if let Some(z) = Zone::new(&state[..2], numeric) {
                            list.push(ZoneSetEnum::Specific(z));
                        } else {
                            list.push(ZoneSetEnum::UnknownZone(state, numeric));
                            // return Err(ZoneSetError::NoSuchZone(state, numeric));
                        }
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
                        if let Some(z) = Zone::new(&state[..2], numeric) {
                            list.push(ZoneSetEnum::Specific(z));
                        } else {
                            list.push(ZoneSetEnum::UnknownZone(state.clone(), numeric));
                            // return Err(ZoneSetError::NoSuchZone(state, numeric));
                        }

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
                    return Err(ZoneSetError::UnexpectedString(t));
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
    use crate::CoastalMarineZone;
    use crate::ForecastZone;
    use crate::OffshoreMarineZone;
    use crate::Zone;

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

    #[test]
    fn test_parsing_marine() {
        let a = parse_zoneset("GMZ634>636-650-655-675-222130-").unwrap();
        println!("{a:?}");
        assert!(a.contains(CoastalMarineZone::GMZ634));
        assert!(a.contains(Zone::CoastalMarine(CoastalMarineZone::GMZ675)));
        assert!(a.contains(CoastalMarineZone::GMZ650));
        assert!(!a.contains(ForecastZone::MA001));
        assert!(!a.contains(OffshoreMarineZone::AMZ040));

        let a =
            parse_zoneset("NYZ176-178-MAZ016-IL014-TNZ027-NJZ106-GAZ044-MD014-230500-").unwrap();
        println!("{a:?}");
    }

    #[test]
    fn test_nopanic() {
        let a = parse_zoneset("AAZ000-123456-").unwrap();
        println!("{a:?}");

        let a = parse_zoneset("RIZ999-123456-").unwrap();
        println!("{a:?}");

        let a =
            parse_zoneset("ANZ835-935-AMZ111-154-330-350-352-354-370-450-454-472-230500-").unwrap();
        println!("{a:?}");

        let a = parse_zoneset("ANZ600-241415-").unwrap();
        println!("{a:?}");

        parse_zoneset("LEZ161-240830-").unwrap();
        parse_zoneset("LSZ261-241000-").unwrap();
        parse_zoneset("LMZ867-665-643>646-240300-").unwrap();
    }
}

// Test docs
//  RIZALL-131000-
//  MEZ002-015-021-024-NHZ003-005-008-011-012-014-VTZ005-008-100300-
//  MAZ017>019-RIZ001>003-140815-
//  CTZ002-004>006-009-010-012-RIZ004-006-007-MAZ004-011-015-142100-
//  CTZ002>004-MAZ002>006-008>014-017-020-026-RIZ001>007-110000-
//  MEZALL-NHZALL-142200-
