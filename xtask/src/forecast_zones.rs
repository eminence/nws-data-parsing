use std::{collections::HashMap, fs::File, io::Write, path::Path, process::Command};

use anyhow::Context;
use dbase::FieldValue;

// Reference: https://www.weather.gov/gis/PublicZones

#[derive(Debug)]
struct Zone {
    /// 2-letter state abbreviation (or some other 2-letter code for marine zones)
    state: String,
    /// 3-number zone identifier
    zone: String,
    /// Long description of the zone
    name: String,

    wfo: String,
}

pub fn main() -> Result<(), anyhow::Error> {
    // let me_x = latlon::parse_lng("71° 23' 8\" W").unwrap();
    // let me_y = latlon::parse_lat("41° 45' 4\" N").unwrap();
    // let me = Coordinate { x: me_x, y: me_y };

    enum ZoneType {
        /// Regular land zone
        Zone,
        CoastalMarine,
        OffshoreMarine,
        /// River forecast centers
        Rivers,
        WarningZone,
        FireZone,
        /// River basins
        ///
        /// ba12my15
        ///
        ///
        RiverBasin,
        // HighSeasMarine,
    }

    let to_parse = [
        (ZoneType::Zone, "z_22mr22/z_22mr22.shp", "gen.rs"),
        (
            ZoneType::CoastalMarine,
            "mz22mr22/mz22mr22.shp",
            "gen_mz.rs",
        ),
        (
            ZoneType::OffshoreMarine,
            "oz22mr22/oz22mr22.shp",
            "gen_oz.rs",
        ),
        (ZoneType::FireZone, "fz22mr22/fz22mr22.shp", "gen_fz.rs"),
        // (ZoneType::Rivers, "rf12ja05/rf12ja05.shp", "gen_rz.rs"),
        // (ZoneType::RiverBasin, "ba12my15/ba12my15.shp", "gen_rz.rs"),

        // The High Seas Marine data doesn't have an "ID" field, so we can't really do much with these files
        // (
        //     ZoneType::HighSeasMarine,
        //     "hz30jn17/hz30jn17.shp",
        //     "gen_hz.rs",
        // ),
    ];

    for (zone, src, dst) in to_parse {
        let gen_rs = Path::new("nws-forecast-zones").join("src").join(dst);
        let input_shp = src;

        let mut f = File::options()
            .create(false)
            .append(false)
            .write(true)
            .truncate(true)
            .open(&gen_rs)
            .context("Opening output gen rs file")?;

        let mut zones = HashMap::new();

        let mut reader = shapefile::Reader::from_path(Path::new("xtask/data").join(input_shp))?;
        for result in reader.iter_shapes_and_records() {
            let (_shape, mut record) = result?;

            if matches!(zone, ZoneType::RiverBasin) {
                println!("{:?}", record);
            }

            let zoneid = if let Some(st) = record.remove("STATE_ZONE") {
                st.try_into().unwrap()
            } else {
                let r = record.remove("ID").or_else(|| record.remove("id"));

                if let Some(FieldValue::Character(Some(c))) = r {
                    c
                } else {
                    panic!("{:?}", record);
                    continue;
                }
            };

            let state: String = record
                .remove("STATE")
                .and_then(|f| f.try_into().ok())
                .unwrap_or_else(|| zoneid[..2].to_string());
            // RI

            let zone_num: String = if let Some(st) = record.remove("ZONE") {
                st.try_into().unwrap() // 004
            } else {
                // extract the last 3 digits from ID
                zoneid[3..6].to_string()
            };

            let name: String = record.remove("NAME").unwrap().try_into().unwrap(); // Eastern Kent
            let wfo: String = record
                .remove("CWA")
                .or_else(|| record.remove("WFO"))
                .unwrap()
                .try_into()
                .unwrap();

            let zone = Zone {
                state,
                zone: zone_num,
                name,
                wfo,
            };
            zones.insert(zoneid, zone);
        }

        let mut all_zones: Vec<_> = zones.keys().collect();
        all_zones.sort();

        // also build a map between the 2-letter state and a list of numerics (to be used when generating a 'new'
        // method for each enum)
        let mut all_zone_map: HashMap<_, Vec<_>> = HashMap::new();
        for zonename in &all_zones {
            let zone = zones.get(*zonename).unwrap();
            let two = zonename[0..2].to_string();
            all_zone_map.entry(two).or_default().push(zone.zone.clone());
        }

        let mut enum_buf = Vec::new();
        let enum_name = match zone {
            ZoneType::Zone => "ForecastZone",
            ZoneType::CoastalMarine => "CoastalMarineZone",
            ZoneType::OffshoreMarine => "OffshoreMarineZone",
            ZoneType::Rivers => "RiverZone",
            ZoneType::WarningZone => "WarningZone",
            ZoneType::FireZone => "FireZone",
            ZoneType::RiverBasin => "RiverBasin",
        };
        writeln!(
            enum_buf,
            "#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] pub enum {enum_name} {{",
        )?;
        for zonename in &all_zones {
            let zone = zones.get(*zonename).unwrap();
            writeln!(enum_buf, "/// {}, {}", zone.name, zone.state,)?;
            writeln!(enum_buf, "#[doc(hidden)]")?;
            writeln!(enum_buf, "{zonename},")?;
        }

        writeln!(enum_buf, "}}")?;

        // FromStr impl
        writeln!(enum_buf, "impl ::std::str::FromStr for {enum_name} {{")?;
        writeln!(enum_buf, "type Err = ();")?;
        writeln!(
            enum_buf,
            "fn from_str(s: &str) -> Result<Self, ()> {{ match s {{"
        )?;

        for zonename in &all_zones {
            // let zone = zones.get(zonename).unwrap();
            writeln!(enum_buf, "\"{zonename}\" => Ok({enum_name}::{zonename}),")?;
        }
        writeln!(enum_buf, "_ => Err(()),")?;
        writeln!(enum_buf, "}} }} }}")?;

        // As str impl
        writeln!(enum_buf, "impl {enum_name} {{")?;
        writeln!(
            enum_buf,
            "pub fn details(&self) -> crate::ZoneDetails {{ match self {{"
        )?;

        for zonename in &all_zones {
            let zone = zones.get(*zonename).unwrap();
            writeln!(
                enum_buf,
                "{enum_name}::{zonename} => crate::ZoneDetails {{state: \"{}\", zone: \"{}\",\
                zone_numeric: {},\
                name: \"{}\", wfo: \"{}\" }},",
                zone.state,
                zone.zone,
                zone.zone.trim_start_matches('0'),
                zone.name,
                zone.wfo
            )?;
        }
        writeln!(enum_buf, "}} }}")?;

        // new impl
        writeln!(
            enum_buf,
            "pub fn new(two: &str, numeric: u16) -> Option<Self> {{ "
        )?;
        writeln!(enum_buf, "match two {{")?;

        let mut all_zone_map_keys: Vec<String> = all_zone_map.keys().cloned().collect();
        all_zone_map_keys.sort();
        for two in all_zone_map_keys {
            let mut numerics = all_zone_map.remove(&two).unwrap();
            writeln!(enum_buf, "\"{two}\" => match numeric {{ ")?;

            numerics.sort();

            for num in numerics {
                if matches!(zone, ZoneType::Zone | ZoneType::FireZone) {
                    writeln!(enum_buf, "{num} => Some({enum_name}::{two}{num}),")?;
                } else {
                    writeln!(enum_buf, "{num} => Some({enum_name}::{two}Z{num}),")?;
                }
            }
            writeln!(enum_buf, "_ => None,")?;

            writeln!(enum_buf, "}}")?; // end match numeric
        }
        writeln!(enum_buf, "_ => None,")?;

        writeln!(enum_buf, "}} }}")?; // end match two and fn new

        writeln!(enum_buf, "}}")?;

        f.write_all(&enum_buf)?;
        f.flush()?;
        drop(f);

        let mut cargo = Command::new("rustfmt")
            .arg("--config")
            .arg("error_on_line_overflow=true,error_on_unformatted=true,max_width=200")
            .arg("-v")
            .arg(gen_rs)
            .spawn()?;

        cargo.wait()?;

        println!("Generated {} zones for {dst}", all_zones.len());
    }

    Ok(())
}
