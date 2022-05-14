use std::{collections::HashMap, fs::File, io::Write, path::Path, process::Command};

// Reference: https://www.weather.gov/gis/CWABounds

#[derive(Debug)]
struct Zone {
    /// 2-letter state abbreviation
    state: String,
    /// 3-number zone identifier
    zone: String,
    /// Long description of the zone
    name: String,

    wfo: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let me_x = latlon::parse_lng("71° 23' 8\" W").unwrap();
    // let me_y = latlon::parse_lat("41° 45' 4\" N").unwrap();
    // let me = Coordinate { x: me_x, y: me_y };
    let gen_rs = Path::new("nws-forecast-zones").join("src").join("gen.rs");

    let mut f = File::options()
        .create(false)
        .append(false)
        .write(true)
        .truncate(true)
        .open(&gen_rs)?;

    let mut zones = HashMap::new();

    let mut reader = shapefile::Reader::from_path("z_22mr22/z_22mr22.shp")?;
    for result in reader.iter_shapes_and_records() {
        let (_shape, mut record) = result?;

        let state: String = record.remove("STATE").unwrap().try_into().unwrap(); // RI
        let zone: String = record.remove("ZONE").unwrap().try_into().unwrap(); // 004
        let name: String = record.remove("NAME").unwrap().try_into().unwrap(); // Eastern Kent
        let wfo: String = record.remove("CWA").unwrap().try_into().unwrap(); // BOX
        let zonename: String = record.remove("STATE_ZONE").unwrap().try_into().unwrap(); // RI004

        let zone = Zone {
            state,
            zone,
            name,
            wfo,
        };
        zones.insert(zonename, zone);
    }

    let mut all_zones: Vec<_> = zones.keys().collect();
    all_zones.sort();

    let mut enum_buf = Vec::new();
    writeln!(
        enum_buf,
        "#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] pub enum ForecastZone {{"
    )?;
    for zonename in &all_zones {
        let zone = zones.get(*zonename).unwrap();
        writeln!(enum_buf, "/// {}, {}", zone.name, zone.state)?;
        writeln!(enum_buf, "#[doc(hidden)]")?;
        writeln!(enum_buf, "{zonename},")?;
    }

    writeln!(enum_buf, "}}")?;

    // FromStr impl
    writeln!(enum_buf, "impl ::std::str::FromStr for ForecastZone {{")?;
    writeln!(enum_buf, "type Err = ();")?;
    writeln!(
        enum_buf,
        "fn from_str(s: &str) -> Result<Self, ()> {{ match s {{"
    )?;

    for zonename in &all_zones {
        // let zone = zones.get(zonename).unwrap();
        writeln!(enum_buf, "\"{zonename}\" => Ok(ForecastZone::{zonename}),")?;
    }
    writeln!(enum_buf, "_ => Err(()),")?;
    writeln!(enum_buf, "}} }} }}")?;

    // As str impl
    writeln!(enum_buf, "impl ForecastZone {{")?;
    writeln!(
        enum_buf,
        "pub fn details(&self) -> crate::ZoneDetails {{ match self {{"
    )?;

    for zonename in &all_zones {
        let zone = zones.get(*zonename).unwrap();
        writeln!(enum_buf, "ForecastZone::{zonename} => crate::ZoneDetails {{state: \"{}\", zone: \"{}\", zone_numeric: {}, name: \"{}\", wfo: \"{}\" }},",
            zone.state,
            zone.zone,
            zone.zone,
            zone.name,
            zone.wfo
    )?;
    }
    writeln!(enum_buf, "}} }} }}")?;

    f.write_all(&enum_buf)?;

    let cargo_bin = std::env::var("CARGO").unwrap();

    let mut cargo = Command::new(cargo_bin)
        .arg("fmt")
        .arg("--")
        .arg(gen_rs)
        .spawn()?;

    cargo.wait()?;

    println!("Generated {} zones", all_zones.len());

    Ok(())
}
