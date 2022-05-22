//! Data structures for parsing WMO abbreviated headings, in particular data from attachment II-5 of
//! [WMO manual 386](https://library.wmo.int/doc_num.php?explnum_id=10469).
//!
//! These abbreviated headings are often referenced by their fields TTAAii, and can be seen in
//! NWS bulletings and EMWIN filenames.  For example, consider the first two lines of an
//! [NWS forecast](https://web.archive.org/web/20220521191347/https://forecast.weather.gov/product.php?site=NWS&issuedby=BOX&product=ZFP&format=CI&version=1&glossary=0):
//!
//! ```text
//! FPUS51 KBOX 211708
//! ZFPBOX
//! ```
//!
//! The `FPUS51` part is the WMO abbreviated heading and can be decoded using tables A through D of WMO manual 386.
//! In this particular example, the `F` indicates a Forecast, the `P` indicates a public forecast, the `US` part
//! indicates "United States", and the `51` part is an NWS specific code that indicates it was issues from the Northeast US.
//!
//! The `NNN` part is generally a product category, and for NWS products you can use the the
//! [nws-product-list](https://crates.io/crates/nws-product-list) crate to look up what
//! [ZFP](https://docs.rs/nws-product-list/latest/nws_product_list/enum.NWSProduct.html#variant.ZFP) means.
//!
//! # References:
//!
//! * <https://library.wmo.int/doc_num.php?explnum_id=10469>
//! * <https://www.weather.gov/tg/awips>
//! * <https://www.weather.gov/tg/head>
//! 
//! This crate uses data published by the World Meteorological Organization, but is otherwise unaffiliated with the
//! WMO, and is not an offical WMO library.

/// Parse an abbreviated heading into its two primary data types and area information
pub fn parse_wmo_abbreviated_heading(
    t1: char,
    t2: char,
    aa: &str,
) -> (WMODataTypeT1, WMODataTypeT2, Area) {
    // The first character (T1) indicates data type
    let data_type = WMODataTypeT1::from(t1);

    // The next two characters (T2) depend on the data type
    let data_type_2 = match data_type {
        WMODataTypeT1::Analyses
        | WMODataTypeT1::ClimaticData
        | WMODataTypeT1::Forecasts
        | WMODataTypeT1::Notices
        | WMODataTypeT1::SurfaceData
        | WMODataTypeT1::SatalliteData
        | WMODataTypeT1::UpperAirData
        | WMODataTypeT1::Warnings => {
            // table B1 is used to look up the next data type
            lookup_table_b1(data_type, t2)
        }
        WMODataTypeT1::GridD
        | WMODataTypeT1::GridPointInformationGrid
        | WMODataTypeT1::GridPointInformationGrib
        | WMODataTypeT1::GribRegionalUse => lookup_table_b2(t2),
        WMODataTypeT1::SatelliteImg => lookup_table_b5(t2),
        WMODataTypeT1::Pictoral | WMODataTypeT1::PictoralRegional => lookup_table_b6(t2),

        x => panic!("Unknow {:?}", x),
    };

    // next is A1 and A2.  This is nominally an area designator, but T1 can adjust
    // this meaning slightly documented in table C1 of WMO manual 386

    let area = match data_type {
        WMODataTypeT1::Analyses
        | WMODataTypeT1::ClimaticData
        | WMODataTypeT1::SatelliteImg
        | WMODataTypeT1::Forecasts
        | WMODataTypeT1::Notices
        | WMODataTypeT1::Warnings => {
            // these types ues table c1 to look up area designator
            let a = AreaDesignator::from_c1(aa).unwrap_or_else(|| {
                panic!(
                    "Unknown area designator: {} for {}{}: {:?} {:?}",
                    aa, t1, t2, data_type, data_type_2
                )
            });
            Area::Area(a)
        }
        WMODataTypeT1::SurfaceData | WMODataTypeT1::UpperAirData => {
            let mut c = aa.chars();
            let a1 = c.next().unwrap();
            let a2 = c.next().unwrap();

            if let Some((a, b)) = lookup_nature_and_area(a1, a2) {
                Area::ReportArea(a, b)
            } else {
                // fall back to table c1
                let a = AreaDesignator::from_c1(aa).unwrap_or_else(|| {
                    panic!(
                        "Unknown area designator: {} for {}{}: {:?} {:?}",
                        aa, t1, t2, data_type, data_type_2
                    )
                });
                Area::Area(a)
            }
        }
        WMODataTypeT1::PictoralRegional | WMODataTypeT1::SatalliteData => {
            let mut c = aa.chars();
            let a1 = c.next().unwrap();
            let a2 = c.next().unwrap();
            let a = GeographicalAreaDesignator::from_c3(a1).unwrap();

            let t = if data_type == WMODataTypeT1::SatalliteData {
                TimeDesignator::from_c4(a2).unwrap()
            } else {
                TimeDesignator::from_c5(a2).unwrap()
            };

            Area::GeoArea(a, t)
        }
        _ => panic!(
            "Unable to lookup area designator for {:?} {}",
            data_type, t2
        ),
    };

    (data_type, data_type_2, area)
}

fn lookup_nature_and_area(a1: char, a2: char) -> Option<(ReportAreaDesignator, ReportNature)> {
    let nature = match a1 {
        'W' => ReportNature::OceanWeatherStation,
        'V' => ReportNature::MobileShipOrStation,
        'F' => ReportNature::Floats,
        _ => return None,
    };

    let area = match a2 {
        'A' => ReportAreaDesignator::A,
        'B' => ReportAreaDesignator::B,
        'C' => ReportAreaDesignator::C,
        'D' => ReportAreaDesignator::D,
        'E' => ReportAreaDesignator::E,
        'F' => ReportAreaDesignator::F,
        'J' => ReportAreaDesignator::J,
        'X' => ReportAreaDesignator::X,
        _ => return None,
    };

    Some((area, nature))
}

/// T1 datatype designator
///
/// This is defined in table A, and represents the general data type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WMODataTypeT1 {
    /// Analyses
    ///
    /// T1 Code A
    Analyses,
    /// Addressed message
    ///
    /// T1 Code B
    AddressedMessage,
    /// Climatic data
    ///
    /// T1 Code C
    ClimaticData,
    /// Grid point information (GRID)
    ///
    /// T1 Code D
    GridD,

    /// Satellite imagery
    ///
    /// T1 Code E
    SatelliteImg,

    /// Forecasts
    ///
    /// T1 Code F
    Forecasts,

    /// Grid point information (GRID format)
    ///
    /// T1 code G
    GridPointInformationGrid,

    /// Grid point information (GRIB format)
    ///
    /// T1 code H
    GridPointInformationGrib,

    /// Observational data (Binary coded) - BUFR
    ///
    /// T1 code I
    ObservationalData,

    /// Forecast information (Binary coded) - BUFR
    ///
    /// T1 code J
    ForecastInformation,

    /// CREX
    ///
    /// T1 code K
    Crex,

    /// Aviation inforamtion (XML)
    ///
    /// T1 code L
    AviationInformation,

    // Note: code M is not used
    /// Notices
    ///
    /// T1 Code N
    Notices,

    /// Oceanographic information (GRIB format)
    ///
    /// T1 code O
    OceanographicInformation,

    /// Pictoral information
    ///
    /// T1 code P
    Pictoral,
    /// Picture information regional
    ///
    /// T1 code Q
    PictoralRegional,

    // Note: code R is not used
    /// Surface data
    ///
    /// T1 Code S
    SurfaceData,

    /// Satellite data
    ///
    /// T1 Code T
    SatalliteData,

    /// Upper-air data
    ///
    /// T1 Code U
    UpperAirData,

    /// National data
    ///
    /// T1 code V
    NationalData,

    /// Warning
    ///
    /// T1 Code W
    Warnings,

    /// Common Alert Protocol (CAP) messages
    ///
    /// T1 Code X
    CAP,

    /// GRIB regional use
    ///
    /// T1 Code Y
    GribRegionalUse,

    /// Unused
    ///
    /// T1 codes M, R, and Z are unused, and if parsed, will return this value
    Unused,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
/// WMODataTypeT2
///
/// This enum captures the possible meaning for the T2 field.  This field describes the data that is
/// associated with a WMO abbreviated heading.  In some enum variants, a format/code is specifed, which
/// describes the expected format of the data assocated with this heading.  More details on these formats/codes
/// can be found in the [Manual on Codes (Volume I.1)](https://library.wmo.int/doc_num.php?explnum_id=10235)
///
/// Reference: Table B2 of WMO manual 386
pub enum WMODataTypeT2 {
    // Start of T1=A analysis reference
    /// Temperature Precipitation Table
    ///
    /// Code B
    ///
    /// This is not documented by WMO, but NWS uses this for its TPT products
    TemperaturePrecipitationTableAnalysis,

    /// Cyclone
    ///
    /// T2 Code C
    CycloneAnalysis,

    /// Air Quality Alert
    ///
    /// T2 Code E
    ///
    /// This is not documented by WMO, but NWS uses this for its AQA products
    AirQualityAlertAnalysis,

    /// Hydrological/marine
    ///
    /// T2 Code G
    HydrologicalMarineAnalysis,

    /// Thickness
    ///
    /// Code H
    ThicknessAnalysis,

    /// Ice
    ///
    /// Data format is FM 44 (ICEAN)
    ///
    /// T2 Code I
    IceAnalysis,

    /// Ozone layer
    ///
    /// T2 Code O
    OzoneAnalysis,

    /// Radar
    ///
    /// T2 code Code R
    RadarAnalysis,

    /// Surface
    ///
    /// Data format is FM 45 (ICAN) or FM 45 (IAC FLEET)
    ///
    /// T2 Code S
    SurfaceAnalysis,

    /// Upper air
    ///
    /// Data format is FM 45 (IAC)
    ///
    /// T2 Code U
    UpperAirAnalysis,

    /// Weather summary
    ///
    /// Code W
    WeatherSummaryAnalysis,

    /// Miscellaneous
    ///
    /// Code X
    MiscellaneousAnalysis,

    // =======================================================================
    // Start of T1=C (climate) reference
    /// Climate anomalies
    ///
    /// T2 Code A
    ClimateAnomalies,

    /// Climatological Report (Daily)
    ///
    /// T2 Code D
    ///
    /// Not documented by WMO, but NWS uses this
    ClimatologicalReportDaily,

    /// A non-daily climatological report
    ///
    /// T2 Code X
    ///
    /// Not documented by WMO, but NWS uses this
    ///
    /// See also: <https://www.weather.gov/gyx/f6-key.html>
    ClimatologicalReport,

    /// Monthly means (upper air)
    ///
    /// Report of monthly aerological means from an ocean weather stationum
    ///
    /// Format is FM 76 (SHIP)
    ///
    /// T2 Code E
    MonthlyMeansUpperAir,

    /// Monthly means (surface)
    ///
    /// Report of monthly means and totals from an ocean weather station
    ///
    /// Format is FM 72 (CLIMAT SHIP)
    ///
    /// T2 Code H
    MonthlyMeansSurfaceShip,

    /// Monthly means (ocean areas)
    ///
    /// Report of monthly means for an oceanic area
    ///
    /// Format is FM 73 (CACLI, CLINP, SPCLI, CLISA, INCLI)
    ///
    /// T2 Code O
    MonthlyMeansOceanAreasClimate,

    /// Monthly means (surface)
    ///
    /// Report of monthly values from a land station
    ///
    /// Data format is FM 71 (CLIMAT)
    ///
    /// T2 Code S
    MonthlyMeansSurfaceClimate,

    // =======================================================================
    // Start of T1=F (forecasts) reference
    /// Aviation Area/GAMET/advisories
    ///
    /// Area forecast for aviation
    ///
    /// Dat aformat is FM 53 (ARFOR) or text
    ///
    /// T2 Code A
    AviationAreaAdvisoriesForecast,

    /// Upper winds and temperatures
    ///
    /// Forecast upper wind and temperature for aviation
    ///
    /// Data format is FM 50 (WINTEM)
    ///
    /// T2 Code B
    UpperWindsAndTemperaturesForecast,

    /// Aerodrome (VT < 12 hours)
    ///
    /// Aerodrome forecast
    ///
    /// Data form is FM 51 (TAF)
    ///
    /// T2 Code C
    AerodromeForecast,

    /// Radiological Trajectory Dose
    ///
    /// Radiological trajectory dose forecast (defined time of arrival and location)
    ///
    /// Data format is FM 57
    ///
    /// T2 Code D
    RadiologicalTrajectoryDoseForecast,

    /// Extended
    ///
    /// T2 Code E
    ExtendedForecast,

    /// Shipping
    ///
    /// Analysis in abbreviated form
    ///
    /// Data format is FM 46 (IAC FLEET)
    ///
    /// T2 Code F
    ShippingForecast,

    /// Hydrological
    ///
    /// Hydrological forecast
    ///
    /// Data format is FM 68 (HYFOR)
    ///
    /// T2 Code G
    HydrologicalForecast,

    /// Upper air thickness
    ///
    /// T2 Code H
    UpperAirThicknessForecast,

    /// Iceberg
    ///
    /// T2 Code I
    IcebergForecast,

    /// Radio warning service (including IUWDS data)
    ///
    /// T2 code Code J
    RadioWarningServiceForecast,

    /// Tropical cylone advisories
    ///
    /// T2 Code K
    TropicalCycloneAdvisoriesForecast,

    /// Local/area
    ///
    /// T2 Code L
    LocalAreaForecast,

    /// Temperature extremes
    ///
    /// T2 Code M
    TemperatureExtremesForecast,

    /// Space weather advisories
    ///
    /// T2 Code N
    SpaceWeatherAdvisoriesForecast,

    /// Guidance
    ///
    /// T2 Code O
    GuidanceForecast,

    /// Public
    ///
    /// T2 Code P
    PublicForecast,

    /// Other shipping
    /// T2 Code Q
    OtherShippingForecast,

    /// Aviation route
    ///
    /// Route forecast for aviation
    ///
    /// Data format is FM 54 (ROFOR)
    ///
    /// T2 Code R
    AviationRouteForecast,

    /// Surface
    ///
    /// Analysis in full or abbreviated form
    ///
    /// Data format is FM 45 (IAC) or FM 46 (IAC Fleet)
    ///
    /// T2 Code S
    SurfaceForecast,

    /// Aerodrome (VT >= 12 hours)
    ///
    /// Aerodrome forecast
    ///
    /// Data format is FM 51 (TAF)
    ///
    /// T2 Code T
    Aerodrome12Forecast,

    /// Upper air
    ///
    /// Analysis in full form
    ///
    /// Data format is FM 45 (IAC)
    ///
    /// T2 Code U
    UpperAirForecast,

    /// Volcanic ash advisories
    ///
    /// T2 Code V
    VolcanicAshAdvisoriesForecast,

    /// Winter sports
    ///
    /// T2 Code W
    WinterSportsForecast,

    /// Miscellaneous
    ///
    /// T2 Code X
    MiscellaneousForecast,

    /// Shipping area
    ///
    /// Forecast for shipping
    ///
    /// T2 Code Z
    ShippingAreaForecast,

    // #######################################################################
    // Start of T=N notice reference
    /// Hydrological
    ///
    /// T2 Code G
    HydrologicalNotice,

    /// Marine
    ///
    /// T2 Code H
    MarineNotice,

    /// Nuclear emergency response
    ///
    /// T2 Code N
    NuclearEmergencyResponseNotice,

    /// METNO/WIFMA
    ///
    /// See section 5.2.1 of WMO manual 386 for into on METNO and WIFMA messages    
    ///
    /// T2 Code O
    METNOWIFMANotice,

    /// Product Generation delay
    ///
    /// T2 Code P
    ProductGenerationDelayNotice,

    /// Test Msg
    ///
    /// T2 Code T
    TestMsgNotice,

    /// Warning related and/or cancellation
    ///
    /// T2 Code W
    WarningRelatedCancellationWarning,

    /// Regional Weather Roundup
    ///
    /// t2 Code Z
    ///
    /// This is not documented by WMO, but NWS uses this for its RWR (and, strangly, RWT) products
    RegionalWeatherRoundupNotice,

    // =======================================================================
    // Start of T=S surface data reference
    /// Aviation routine reports
    ///
    /// Aerodrome routine meteorological report (with or without trend forecast)
    ///
    /// Data format is FM 15 (METAR)
    ///
    /// T2 Code A
    AviationRoutineReports,

    /// Radar reports (part A)
    ///
    /// Report of ground radar weather observation
    ///
    /// Data format is FM 20 (RADOB)
    ///
    /// T2 Code B
    RadarReportsPartA,

    /// Radar reports (part B)
    ///
    /// Report of ground radar weather observation
    ///
    /// Data format is FM 20 (RADOB)
    ///
    /// T2 Code C
    RadarReportsPartB,

    /// Radar reports (parts A & B)
    ///
    /// Report of ground radar weather observation
    ///
    /// Data format is FM 20 (RADOB)
    ///
    /// T2 Code D
    RadarReportsPartsAB,

    /// Seismic data
    ///
    /// T2 Code E
    SeismicData,

    /// Atmospherics reports
    ///
    /// Possible data formats:
    /// * FM 81 (SFAZI) Synoptic report of bearings of sources of atmospherics
    /// * FM 82 (SFLOC) Synoptic report of the geographical location of sources of atmospherics
    /// * FM 83 (SFAZU) Detailed report of the distribution of sources of atmospherics by bearings for any period up to and including 24 hours
    ///
    /// T2 Code F
    AtmosphericsReports,

    /// Radiological data report
    ///
    /// Data format is FM 22 (RADREP)
    ///
    /// Radiological data report (monitored on a routine basis and/or in case of accident)
    ///
    /// T2 Code G
    RadiologicalDataReport,

    /// Reports from DCP stations
    ///
    /// T2 Code H
    ReportsFromDCPStations,

    /// Intermediate synoptic hour
    ///
    /// Report of surface observation from a fixed land station or a sea station
    ///
    /// Data format is FM 12 (SYNOP) or FM 13 (SHIP)
    ///
    /// T2 Code I
    IntermediateSynopticHour,

    /// (not used)
    ///
    /// T2 Code L
    NotUsed,

    /// Main synoptic hour
    ///
    /// Report of surface observation from a fixed land station or a sea station
    ///
    /// Data format is FM 12 (SYNOP) or FM 13 (SHIP)
    ///
    /// T2 Code M
    MainSynopticHour,

    /// Non-standard synoptic hour
    ///
    /// Report of surface observation from a fixed land station or a sea station
    ///
    /// Data format is FM 12 (SYNOP) or FM 13 (SHIP)
    ///
    /// T2 Code N
    NonStandardSynopticHour,

    /// Oceanographic data
    ///
    /// Possible data formats:
    /// * FM 63 (BATHY) Report of bathythermal observation
    /// * FM 64 (TESAC) Temperature, salinity and current report from a sea station
    /// * FM 62 (TRACKOB) Report of marine surface observation along a ship’s track
    ///
    /// T2 Code O
    OceanographicData,

    /// Special aviation weather reports
    ///
    /// Aerodrome special meteorological report (with or without trend forecast
    ///
    /// Data format is FM 16 (SPECI)
    ///
    /// T2 Code P
    SpecialAviationWeatherReports,

    /// Hydrological (river) reports
    ///
    /// Report of hydrological observation from a hydrological station
    ///
    /// Data format is FM 67 (HYDRA)
    ///
    /// T2 Code R
    HydrologicalRiverReports,

    /// Drifting bouy reports
    ///
    /// Report of a buoy observation
    ///
    /// Data format is FM 18 (DRIFTER)
    ///
    /// T2 Code S
    DriftingBouyReports,

    /// Sea ice
    ///
    /// T2 Code T
    SeaIce,

    /// Snow depth
    ///
    /// T2 Code U
    SnowDepth,

    /// Lake ice
    ///
    /// T2 Code V
    LakeIce,

    /// Wave information
    ///
    /// Report of spectral wave information from a sea station or from a remote platform (aircraft or satellite)
    ///
    /// Data format is FM 65 (WAVEOB)
    ///
    /// T2 Code W
    WaveInformation,

    /// Miscellaneous
    ///
    /// T2 Code X
    MiscellaneousSurface,

    /// Seismic waveform data
    ///
    /// T2 Code Y
    SeismicWaveformData,

    /// Sea-level data and deep-ocean tsunami data
    ///
    /// T2 Code Z
    TsunamiData,

    // =======================================================================
    // Start of T=T satellite data reference
    /// Satellite orbit parameters
    ///
    /// T2 Code B
    SatelliteOrbitParameters,

    /// Satellite cloud interpretations
    ///
    /// Report of synoptic interpretation of cloud data obtained by a meteorological satellite
    ///
    /// Data format is FM 85 (SAREP)
    ///
    /// T2 Code C
    SatelliteCloudInterpretations,

    /// Satellite remote upper-air soundings
    ///
    /// Report of satellite remote upper-air soundings of pressure, temperature and humidity
    ///
    /// Data format is FM 86 (SATEM)
    ///
    /// T2 Code H
    SatelliteRemoteUpperAirSounding,

    /// Clear radiance observations
    ///
    /// Report of satellite clear radiance observations
    ///
    /// Data format is FM 87 (SARAD)
    ///
    /// T2 Code R
    ClearRadianceObservations,

    /// Sea surface temperatures
    ///
    /// Report of satellite observations of wind, surface temperature, cloud, humidity and radiation
    ///
    /// Data format is FM 88 (SATOB)
    ///
    /// T2 Code T
    SeaSurfaceTemperatures,

    /// Winds and cloud temperatures
    ///
    /// Report of satellite observations of wind, surface temperature, cloud, humidity and radiation
    ///
    /// Data format is FM 88 (SATOB)
    ///
    /// T2 Code W
    WindsAndCloudTemperatures,

    /// Miscellaneous Satellite data
    ///
    /// T2 Code X
    MiscellaneousSatellite,

    // =======================================================================
    // Start of T=U upper-air data reference =======
    /// Aircraft reports
    ///
    /// Data foramt is FM 41 (CODAR) or ICAO (AIREP)
    ///
    /// T2 Code A
    AircraftReports41,

    /// Aircraft reports (FM 42)
    ///
    /// T2 Code D
    AircraftReports42,

    /// Upper-level pressure, temperature, humidity and wind (Part D)
    ///
    /// T2 Code E
    UpperLevelPressureTemperatureHumidityWindPartD,

    /// Upper-level pressure, temperature, humidity and wind (Part C and D)
    ///
    /// National and bilateral option
    ///
    /// T2 Code F
    UpperLevelPressureTemperatureHumidityWindPartCD,

    /// Upper wind (part B)
    ///
    /// T2 Code G
    UpperWindPartB,

    /// Upper wind (part C)
    ///
    /// T2 Code H
    UpperWindPartC,

    /// Upper wind (parts A and B)
    ///
    /// National and bilateral option
    ///
    /// T2 Code I
    UpperWindPartsAB,

    /// Upper-level pressure, temperature, humidity and wind (Part B)
    ///
    /// T2 Code K
    UpperLevelPressureTemperatureHumidityWindPartB,

    /// Upper-level pressure, temperature, humidity and wind (part C)
    ///
    /// T2 Code L
    UpperLevelPressureTemperatureHumidityWindPartC,

    /// Upper-level pressure, temperature, humidity and wind (parts A and B)
    ///
    /// National and bilateral option
    ///
    /// T2 Code M
    UpperLevelPressureTemperatureHumidityWindPartsAB,

    /// Rocketsonde reports
    ///
    /// T2 Code N
    RocketsondeReports,

    /// Upper wind (part A)
    ///
    /// T2 Code P
    UpperWindPartA,

    /// Upper wind (part D)
    ///
    /// T2 Code Q
    UpperWindPartD,

    /// Aircraft report
    ///
    /// T2 Code R
    AircraftReport,

    /// Upper-level pressure, temperature, humidity and wind (part A)
    ///
    /// T2 Code S
    UpperLevelPressureTemperatureHumidityWindPartA,

    /// Aircraft report
    ///
    /// T2 Code T
    AircraftReport2,

    /// Miscellaneous
    ///
    /// T2 Code X
    MiscellaneousUpperAir,

    /// Upper wind (parts C and D)
    ///
    /// National and bilateral option
    ///
    /// T2 Code Y
    UpperWindPartsCD,

    /// Upper-level pressure, temperature, humidity, and wind from a sonde
    /// released by carrier balloon or aircraft
    ///
    /// (Parts A, B, C and D)
    ///
    /// T2 code Z
    PTHWFromSonde,

    // =======================================================================
    // Start of T=W warning reference ========
    /// AIRMET
    ///
    /// T2 Code A
    AIRMETWarning,

    /// Tropical cyclone (SIGMET)
    ///
    /// T2 Code C
    TropicalCycloneSigmetWarning,

    /// Tsunami
    ///
    /// T2 Code E
    TsunamiWarning,

    /// Tornado
    ///
    /// T2 Code F
    TornadoWarning,

    /// Hydrological/river flood
    ///
    /// T2 Code G
    HydrologicalRiverFloodWarning,

    /// Marine/coastal flood
    ///
    /// T2 Code H
    MarineCoastalFloodWarning,

    /// Other
    ///
    /// T2 Code O
    OtherWarning,

    /// Humanitarian activities
    ///
    /// T2 Code R
    HumanitarianActivitiesWarning,

    /// SIGMET
    ///
    /// T2 Code S
    SIGMETWarning,

    /// Troical Cyclone (typhoon/hurricane)
    ///
    /// T2 Code T
    TropicalCycloneWarning,

    /// Severe thunderstorm
    ///
    /// T2 Code U
    SevereThunderstormWarning,

    /// Volcanic ash clouds (SIGMET)
    ///
    /// T2 Code V
    VolcanicAshCloudsWarning,

    /// Warnings and weather summary
    ///
    /// T2 Code W
    WarningsAndWeatherSummaryWarning,

    // =======================================================================
    // Start of T=P and T=Q reference (table B6)
    // also overlaps quite a bit with table B2 (T1=D, G, H or Y)
    /// Radar data
    ///
    /// T2 Code A
    RadarData,

    /// Cloud
    ///
    /// T2 Code B
    CloudData,

    /// Clear air turbulence
    ///
    /// T2 Code C
    ClearAirTurbulenceData,

    /// Vorticity
    ///
    /// T2 Code C
    VorticityData,

    /// Thickness
    ///
    /// T2 Code D
    ThicknessData,

    /// Precipitation
    ///
    /// T2 Code E
    PrecipitationData,

    /// Aerological diagrams (ash cloud)
    ///
    /// T2 Code F
    AerologicalDiagramsData,

    /// Divergence
    ///
    /// T2 Code G
    DivergenceData,

    /// Significan weather
    /// Code G
    SignificantWeatherData,

    /// Height
    ///
    /// T2 Code H
    HeightData,

    /// Ice flow
    ///
    /// T2 Code I
    IceFlowData,

    /// Wave height + combinations
    ///
    /// T2 Code J
    WaveHeightCombinationsData,

    /// Swell height + combinations
    ///
    /// T2 Code K
    SwellHeightCombinationsData,

    /// Plain langauge
    ///
    /// T2 Code L
    PlainLanguageData,

    /// For national use
    ///
    /// T2 Code M
    NationalUseData,

    /// Radiation
    ///
    /// T2 Code N
    RadiationData,

    /// Vertical velocity
    ///
    /// T2 Code O
    VerticalVelocityData,

    /// Pressure
    ///
    /// T2 Code P
    PressureData,

    /// Wet bulb potential temperature
    ///
    /// T2 Code Q
    WetBulbPotentialTemperatureData,

    /// Relative humidity
    ///
    /// T2 Code R
    RelativeHumidityData,

    /// Snow cover
    ///
    /// T2 Code S
    SnowCoverData,

    /// Temperature
    ///
    /// T2 Code T
    TemperatureData,

    /// Eastward wind component
    ///
    /// T2 Code U
    EastwardWindComponentData,

    /// Northward wind component
    ///
    /// T2 Code V
    NorthwardWindComponentData,

    /// Wind
    ///
    /// T2 Code W
    WindData,

    /// Lifted index
    ///
    /// T2 Code X
    LiftedIndexData,

    /// Observational plotted chart
    ///
    /// T2 Code Y
    ObservationalPlottedChartData,

    /// Not assigned
    ///
    /// Code Z
    NotAssignedData,

    // =======================================================================
    // ======= Start of T=E reference (table B5) =======
    /// CloudTopTemperature
    /// Code C
    CloudTopTemperatureSatImg,

    /// Fog
    /// Code F
    FogSatImg,

    /// Infrared
    /// Code I
    InfraredSatImg,

    /// Surface temperature
    /// Code S
    SurfaceTemperatureSatImg,

    /// Visible
    /// Code V
    VisibleSatImg,

    /// Water vapour
    /// Code W
    WaterVaporSatImg,

    /// User specified
    /// Code Y
    UserSpecifiedSatImg,

    /// Unspecified
    /// Code Z
    UnspecifiedSatImg,

    // ## Start of unknown T2 codes ##
    // These codes seem to be used by NWS, but are not assigned in WMO manual 386
    UnknownAnalyses(char),
    UnknownForecast(char),
    UnknownClimate(char),
    UnknownNotice(char),
    UnknownSurfaceData(char),
    UnknownUpperAir(char),
    UnknownWarning(char),
    UnknownSatellite(char),
}

impl WMODataTypeT2 {
    pub fn is_unknown(&self) -> bool {
        match self {
            WMODataTypeT2::UnknownAnalyses(_)
            | WMODataTypeT2::UnknownForecast(_)
            | WMODataTypeT2::UnknownClimate(_)
            | WMODataTypeT2::UnknownNotice(_)
            | WMODataTypeT2::UnknownSurfaceData(_)
            | WMODataTypeT2::UnknownUpperAir(_)
            | WMODataTypeT2::UnknownWarning(_)
            | WMODataTypeT2::UnknownSatellite(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AreaDesignator {
    Albania,
    Argentina,
    Afghanistan,
    AscensionIsland,
    Azerbaijan,
    Alaska,
    Algeria,
    Angola,
    AntiguaAndBarbuda,
    Australia,
    Armenia,
    Azores,

    Bahamas,
    Botswana,
    BruneiDarussalam,
    Bermuda,
    Belize,
    Burundi,
    Benin,
    BanksIslands,
    Myanmar,
    Bahrain,
    Bolivia,
    Barbados,
    Bhutan,
    Bulgaria,
    BouvetIsland,
    Bangladesh,
    Belgium,
    Belarus,
    Brazil,

    Chad,
    CentralAfricanRepublic,
    Congo,
    Chili,
    China,
    Cameroon,
    Canada,
    Columbia,
    CanaryIslands,
    CostaRica,
    CantonIsland,
    Cuba,
    CaboVerde,
    Cyprus,
    Czechia,

    Bonaire,
    Djibouti,
    Germany,
    Denmark,
    Dominica,
    DominicanRepublic,

    Egypt,
    Eritrea,
    Estonia,
    Ecuador,
    UnitedArabEmirates,
    ElSalvador,
    Ethiopia,

    FaroeIslands,
    FrenchGuiana,
    Finland,
    Fiji,
    FalklandIslands,
    FederatedStatesOfMicronesia,
    SaintPierre,
    France,
    WallisAndFutuna,

    Gambia,
    CaymanIslands,
    Grenada,
    GoughIsland,
    Georgia,
    Ghana,
    Gibraltar,
    Greenland,
    Guam,
    Guinea,
    Gabon,
    EquatorialGuinea,
    Greece,
    Guatemala,
    GuineaBissau,
    Guyana,

    Haiti,
    SaintHelena,
    HongKong,
    Honduras,
    Hungary,
    BurkinaFaso,
    HawaiianIslands,

    Comoros,
    Indonesia,
    Ireland,
    Iceland,
    India,
    Iraq,
    IslamicRepublicOfIran,
    Israel,
    CotedIvoire,
    Italy,

    Jordan,
    Jamaica,
    Japan,

    CarolineIslands,
    Kiribati,
    ChristmasIsland,
    CocosIslands,
    Kenya,
    /// South Korea
    RepublicOfKorea,
    Cambodia,
    /// North Korea
    DemocraticPeoplesRepublicOfKorea,
    CookIslands,
    Kuwait,
    Kyrgyzstan,
    Kazakhstan,

    LaoPeopleDemocraticRepublic,
    Lebanon,
    SaintLucia,
    Liberia,
    Slovenia,
    SouthernLineIslands,
    Lesotho,
    Lithuania,
    Latvia,
    Libya,

    Mauritius,
    MarionIsland,
    Morocco,
    Madeira,
    SaintMartin,
    Madagascar,
    MarshallIslands,
    Mali,
    Macedonia,
    Montenegro,
    Malta,
    StMaarten,
    Mongolia,
    Martinique,
    Malaysia,
    Mauritania,
    MacaoChina,
    Maldives,
    Malawi,
    Mexico,
    MarinaIslands,
    Mozambique,

    NewCaledonia,
    Niue,
    PapuaNewGuinea,
    Nigeria,
    Nicaragua,
    Netherlands,
    Namibia,
    Norway,
    Nepal,
    Niger,
    CuraacoAndAruba,
    Vanuatu,
    Nauru,
    NewZealand,

    Oman,
    Monaco,
    SouthOrkneyIslands,
    Austria,

    FrenchPolynesia,
    Philippines,
    PhoenixIslands,
    Pakistan,
    Poland,
    Panama,
    Portugal,
    Palau,
    Peru,
    Pitcairn,
    PuertoRico,
    Paraguay,

    BosniaAndHerzegovina,
    Qatar,

    RussianFedereationEast,
    Reunion,
    Croatia,
    RepublicOfMoldova,
    Romania,
    RussianFedereationWest,
    Rwanda,

    SriLanka,
    Seychelles,
    SaudiArabia,
    Senegal,
    Somalia,
    Sarawak,
    SierraLeone,
    Suriname,
    Sweden,
    SolomonIslands,
    Spain,
    Slovakia,
    Singapore,
    Sudan,
    Swaziland,
    Switzerland,
    SantaCruzIslands,
    SyrianArabRepublic,
    SpitzbergenIslands,

    Tajikistan,
    TristanDaCunha,
    TrinidadAndTobago,
    Togo,
    Thailand,
    TurksAndCaicosIslands,
    Tokelau,
    TimorLeste,
    UnitedRepublicOfTanzania,
    Tonga,
    SaoTomeAndPrincipe,
    Turkmenistan,
    Tunisia,
    Turkey,
    Tuvalu,

    Uganda,
    UnitedKingdom,
    Ukraine,
    UnitedStates,
    Uruguay,
    Uzbekistan,

    SaintVincentAndTheGrenadines,
    VirginIslands,
    Venezuela,
    Vietnam,

    Yemen,
    Serbia,

    SouthAfrica,
    Zambia,
    Samoa,
    DemocraticRepublicOfTheCongo,
    SouthSudan,
    Zimbabwe,

    // ### Part 2
    AntarcticArea,
    ArcticArea,
    SouthEastAsiaArea,
    AfricaArea,
    CentralAfricaArea,
    WestAfricaArea,
    SouthernAfricaArea,
    AsiaArea,
    NearEastArea,
    ArabianSeaArea,

    BalticSeaArea,

    CaribbeanAndCentralAmerica,

    EastAfricaArea,
    EastChinaSeaArea,
    EasternEuropeArea,
    MiddleEuropArea,
    NorthernEuropeArea,
    EuropeArea,
    WesternEuropeArea,

    FarEastArea,

    GulfOfAlaskaArea,
    GulfOfMexicoArea,

    IndianOceanArea,

    EasternMiditerraneanArea,
    MediterraneanArea,
    CentralMediterraneanArea,
    WesternMediterraneanArea,

    NorthAmericaArea,
    NorthAtlanticArea,

    OceaniaArea,
    SeaOfOkhotskArea,

    PacificArea,
    PersianGulfArea,
    NorthPacificArea,
    WesternNorthPacificArea,
    SouthPacificArea,
    WesternPacificArea,
    EasternPacificArea,

    SouthAmericaArea,
    SouthernOceanArea,
    SeaOfJapanArea,
    SouthChinaseaArea,
    SouthAtlanticArea,

    EasternHemisphereArea,
    NorthernHemisphereArea,
    SouthernHemisphereArea,
    TropicalBeltArea,
    WesternHemisphereArea,

    // unknown areas that are not in the WMO manual but are common in NWS data
    HNUnknown,

    /// An area without a designator
    Unknown,
}

impl AreaDesignator {
    // Table C1 of WMO.386
    pub fn from_c1(s: &str) -> Option<AreaDesignator> {
        Some(match s {
            "AB" => AreaDesignator::Albania,
            "AG" => AreaDesignator::Argentina,
            "AH" => AreaDesignator::Afghanistan,
            "AI" => AreaDesignator::AscensionIsland,
            "AJ" => AreaDesignator::Azerbaijan,
            "AK" => AreaDesignator::Alaska,
            "AL" => AreaDesignator::Algeria,
            "AN" => AreaDesignator::Angola,
            "AT" => AreaDesignator::AntiguaAndBarbuda,
            "AU" => AreaDesignator::Australia,
            "AY" => AreaDesignator::Armenia,
            "AZ" => AreaDesignator::Azores,

            "BA" => AreaDesignator::Bahamas,
            "BC" => AreaDesignator::Botswana,
            "BD" => AreaDesignator::BruneiDarussalam,
            "BE" => AreaDesignator::Bermuda,
            "BH" => AreaDesignator::Belize,
            "BI" => AreaDesignator::Burundi,
            "BJ" => AreaDesignator::Benin,
            "BK" => AreaDesignator::BanksIslands,
            "BM" => AreaDesignator::Myanmar,
            "BN" => AreaDesignator::Bahrain,
            "BO" => AreaDesignator::Bolivia,
            "BR" => AreaDesignator::Barbados,
            "BT" => AreaDesignator::Bhutan,
            "BU" => AreaDesignator::Bulgaria,
            "BV" => AreaDesignator::BouvetIsland,
            "BW" => AreaDesignator::Bangladesh,
            "BX" => AreaDesignator::Belgium,
            "BY" => AreaDesignator::Belarus,
            "BZ" => AreaDesignator::Brazil,

            "CD" => AreaDesignator::Chad,
            "CE" => AreaDesignator::CentralAfricanRepublic,
            "CG" => AreaDesignator::Congo,
            "CH" => AreaDesignator::Chili,
            "CI" => AreaDesignator::China,
            "CM" => AreaDesignator::Cameroon,
            "CN" => AreaDesignator::Canada,
            "CO" => AreaDesignator::Columbia,
            "CR" => AreaDesignator::CanaryIslands,
            "CS" => AreaDesignator::CostaRica,
            "CT" => AreaDesignator::CantonIsland,
            "CU" => AreaDesignator::Cuba,
            "CV" => AreaDesignator::CaboVerde,
            "CY" => AreaDesignator::Cyprus,
            "CZ" => AreaDesignator::Czechia,

            "DC" => AreaDesignator::Bonaire,
            "DJ" => AreaDesignator::Djibouti,
            "DL" => AreaDesignator::Germany,
            "DN" => AreaDesignator::Denmark,
            "DO" => AreaDesignator::Dominica,
            "DR" => AreaDesignator::DominicanRepublic,

            "EG" => AreaDesignator::Egypt,
            "EI" => AreaDesignator::Eritrea,
            "EO" => AreaDesignator::Estonia,
            "EQ" => AreaDesignator::Ecuador,
            "ER" => AreaDesignator::UnitedArabEmirates,
            "ES" => AreaDesignator::ElSalvador,
            "ET" => AreaDesignator::Ethiopia,

            "FA" => AreaDesignator::FaroeIslands,
            "FG" => AreaDesignator::FrenchGuiana,
            "FI" => AreaDesignator::Finland,
            "FJ" => AreaDesignator::Fiji,
            "FK" => AreaDesignator::FalklandIslands,
            "FM" => AreaDesignator::FederatedStatesOfMicronesia,
            "FP" => AreaDesignator::SaintPierre,
            "FR" => AreaDesignator::France,
            "FW" => AreaDesignator::WallisAndFutuna,

            "GB" => AreaDesignator::Gambia,
            "GC" => AreaDesignator::CaymanIslands,
            "GD" => AreaDesignator::Grenada,
            "GE" => AreaDesignator::GoughIsland,
            "GG" => AreaDesignator::Georgia,
            "GH" => AreaDesignator::Ghana,
            "GI" => AreaDesignator::Gibraltar,
            "GL" => AreaDesignator::Greenland,
            "GM" => AreaDesignator::Guam,
            "GN" => AreaDesignator::Guinea,
            "GO" => AreaDesignator::Gabon,
            "GQ" => AreaDesignator::EquatorialGuinea,
            "GR" => AreaDesignator::Greece,
            "GU" => AreaDesignator::Guatemala,
            "GW" => AreaDesignator::GuineaBissau,
            "GY" => AreaDesignator::Guyana,

            "HA" => AreaDesignator::Haiti,
            "HE" => AreaDesignator::SaintHelena,
            "HK" => AreaDesignator::HongKong,
            "HO" => AreaDesignator::Honduras,
            "HU" => AreaDesignator::Hungary,
            "HV" => AreaDesignator::BurkinaFaso,
            "HW" => AreaDesignator::HawaiianIslands,

            "IC" => AreaDesignator::Comoros,
            "ID" => AreaDesignator::Indonesia,
            "IE" => AreaDesignator::Ireland,
            "IL" => AreaDesignator::Iceland,
            "IN" => AreaDesignator::India,
            "IQ" => AreaDesignator::Iraq,
            "IR" => AreaDesignator::IslamicRepublicOfIran,
            "IS" => AreaDesignator::Israel,
            "IV" => AreaDesignator::CotedIvoire,
            "IY" => AreaDesignator::Italy,

            "JD" => AreaDesignator::Jordan,
            "JM" => AreaDesignator::Jamaica,
            "JP" => AreaDesignator::Japan,

            "KA" => AreaDesignator::CarolineIslands,
            "KB" => AreaDesignator::Kiribati,
            "KI" => AreaDesignator::ChristmasIsland,
            "KK" => AreaDesignator::CocosIslands,
            "KN" => AreaDesignator::Kenya,
            "KO" => AreaDesignator::RepublicOfKorea,
            "KP" => AreaDesignator::Cambodia,
            "KR" => AreaDesignator::DemocraticPeoplesRepublicOfKorea,
            "KU" => AreaDesignator::CookIslands,
            "KW" => AreaDesignator::Kuwait,
            "KY" => AreaDesignator::Kyrgyzstan,
            "KZ" => AreaDesignator::Kazakhstan,

            "LA" => AreaDesignator::LaoPeopleDemocraticRepublic,
            "LB" => AreaDesignator::Lebanon,
            "LC" => AreaDesignator::SaintLucia,
            "LI" => AreaDesignator::Liberia,
            "LJ" => AreaDesignator::Slovenia,
            "LN" => AreaDesignator::SouthernLineIslands,
            "LS" => AreaDesignator::Lesotho,
            "LT" => AreaDesignator::Lithuania,
            "LV" => AreaDesignator::Latvia,
            "LY" => AreaDesignator::Libya,

            "MA" => AreaDesignator::Mauritius,
            "MB" => AreaDesignator::MarionIsland,
            "MC" => AreaDesignator::Morocco,
            "MD" => AreaDesignator::Madeira,
            "MF" => AreaDesignator::SaintMartin,
            "MG" => AreaDesignator::Madagascar,
            "MH" => AreaDesignator::MarshallIslands,
            "MI" => AreaDesignator::Mali,
            "MJ" => AreaDesignator::Macedonia,
            "MK" => AreaDesignator::Montenegro,
            "ML" => AreaDesignator::Malta,
            "MN" => AreaDesignator::StMaarten,
            "MO" => AreaDesignator::Mongolia,
            "MR" => AreaDesignator::Martinique,
            "MS" => AreaDesignator::Malaysia,
            "MT" => AreaDesignator::Mauritania,
            "MU" => AreaDesignator::MacaoChina,
            "MV" => AreaDesignator::Maldives,
            "MW" => AreaDesignator::Malawi,
            "MX" => AreaDesignator::Mexico,
            "MY" => AreaDesignator::MarinaIslands,
            "MZ" => AreaDesignator::Mozambique,

            "NC" => AreaDesignator::NewCaledonia,
            "NE" => AreaDesignator::Niue,
            "NG" => AreaDesignator::PapuaNewGuinea,
            "NI" => AreaDesignator::Nigeria,
            "NK" => AreaDesignator::Nicaragua,
            "NL" => AreaDesignator::Netherlands,
            "NM" => AreaDesignator::Namibia,
            "NO" => AreaDesignator::Norway,
            "NP" => AreaDesignator::Nepal,
            "NR" => AreaDesignator::Niger,
            "NU" => AreaDesignator::CuraacoAndAruba,
            "NV" => AreaDesignator::Vanuatu,
            "NW" => AreaDesignator::Nauru,
            "NZ" => AreaDesignator::NewZealand,

            "OM" => AreaDesignator::Oman,
            "OO" => AreaDesignator::Monaco,
            "OR" => AreaDesignator::SouthOrkneyIslands,
            "OS" => AreaDesignator::Austria,

            "PF" => AreaDesignator::FrenchPolynesia,
            "PH" => AreaDesignator::Philippines,
            "PI" => AreaDesignator::PhoenixIslands,
            "PK" => AreaDesignator::Pakistan,
            "PL" => AreaDesignator::Poland,
            "PM" => AreaDesignator::Panama,
            "PO" => AreaDesignator::Portugal,
            "PP" => AreaDesignator::Palau,
            "PR" => AreaDesignator::Peru,
            "PT" => AreaDesignator::Pitcairn,
            "PU" => AreaDesignator::PuertoRico,
            "PY" => AreaDesignator::Paraguay,

            "QB" => AreaDesignator::BosniaAndHerzegovina,
            "QT" => AreaDesignator::Qatar,

            "RA" => AreaDesignator::RussianFedereationEast,
            "RE" => AreaDesignator::Reunion,
            "RH" => AreaDesignator::Croatia,
            "RM" => AreaDesignator::RepublicOfMoldova,
            "RO" => AreaDesignator::Romania,
            "RS" => AreaDesignator::RussianFedereationWest,
            "RW" => AreaDesignator::Rwanda,

            "SB" => AreaDesignator::SriLanka,
            "SC" => AreaDesignator::Seychelles,
            "SD" => AreaDesignator::SaudiArabia,
            "SG" => AreaDesignator::Senegal,
            "SI" => AreaDesignator::Somalia,
            "SK" => AreaDesignator::Sarawak,
            "SL" => AreaDesignator::SierraLeone,
            "SM" => AreaDesignator::Suriname,
            "SN" => AreaDesignator::Sweden,
            "SO" => AreaDesignator::SolomonIslands,
            "SP" => AreaDesignator::Spain,
            "SQ" => AreaDesignator::Slovakia,
            "SR" => AreaDesignator::Singapore,
            "SU" => AreaDesignator::Sudan,
            "SV" => AreaDesignator::Swaziland,
            "SW" => AreaDesignator::Switzerland,
            "SX" => AreaDesignator::SantaCruzIslands,
            "SY" => AreaDesignator::SyrianArabRepublic,
            "SZ" => AreaDesignator::SpitzbergenIslands,

            "TA" => AreaDesignator::Tajikistan,
            "TC" => AreaDesignator::TristanDaCunha,
            "TD" => AreaDesignator::TrinidadAndTobago,
            "TG" => AreaDesignator::Togo,
            "TH" => AreaDesignator::Thailand,
            "TI" => AreaDesignator::TurksAndCaicosIslands,
            "TK" => AreaDesignator::Tokelau,
            "TM" => AreaDesignator::TimorLeste,
            "TN" => AreaDesignator::UnitedRepublicOfTanzania,
            "TO" => AreaDesignator::Tonga,
            "TP" => AreaDesignator::SaoTomeAndPrincipe,
            "TR" => AreaDesignator::Turkmenistan,
            "TS" => AreaDesignator::Tunisia,
            "TU" => AreaDesignator::Turkey,
            "TV" => AreaDesignator::Tuvalu,

            "UG" => AreaDesignator::Uganda,
            "UK" => AreaDesignator::UnitedKingdom,
            "UR" => AreaDesignator::Ukraine,
            "US" => AreaDesignator::UnitedStates,
            "UY" => AreaDesignator::Uruguay,
            "UZ" => AreaDesignator::Uzbekistan,

            "VG" => AreaDesignator::SaintVincentAndTheGrenadines,
            "VI" => AreaDesignator::VirginIslands,
            "VN" => AreaDesignator::Venezuela,
            "VS" => AreaDesignator::Vietnam,

            "YE" => AreaDesignator::Yemen,
            "YG" => AreaDesignator::Serbia,

            "ZA" => AreaDesignator::SouthAfrica,
            "ZB" => AreaDesignator::Zambia,
            "ZM" => AreaDesignator::Samoa,
            "ZR" => AreaDesignator::DemocraticRepublicOfTheCongo,
            "ZS" => AreaDesignator::SouthSudan,
            "ZW" => AreaDesignator::Zimbabwe,

            // ### Part 2
            "AA" => AreaDesignator::AntarcticArea,
            "AC" => AreaDesignator::ArcticArea,
            "AE" => AreaDesignator::SouthEastAsiaArea,
            "AF" => AreaDesignator::AfricaArea,
            "AM" => AreaDesignator::CentralAfricaArea,
            "AO" => AreaDesignator::WestAfricaArea,
            "AP" => AreaDesignator::SouthernAfricaArea,
            "AS" => AreaDesignator::AsiaArea,
            "AW" => AreaDesignator::NearEastArea,
            "AX" => AreaDesignator::ArabianSeaArea,

            "BQ" => AreaDesignator::BalticSeaArea,

            "CA" => AreaDesignator::CaribbeanAndCentralAmerica,

            "EA" => AreaDesignator::EastAfricaArea,
            "EC" => AreaDesignator::EastChinaSeaArea,
            "EE" => AreaDesignator::EasternEuropeArea,
            "EM" => AreaDesignator::MiddleEuropArea,
            "EN" => AreaDesignator::NorthernEuropeArea,
            "EU" => AreaDesignator::EuropeArea,
            "EW" => AreaDesignator::WesternEuropeArea,

            "FE" => AreaDesignator::FarEastArea,

            "GA" => AreaDesignator::GulfOfAlaskaArea,
            "GX" => AreaDesignator::GulfOfMexicoArea,

            "IO" => AreaDesignator::IndianOceanArea,

            "ME" => AreaDesignator::EasternMiditerraneanArea,
            "MM" => AreaDesignator::MediterraneanArea,
            "MP" => AreaDesignator::CentralMediterraneanArea,
            "MQ" => AreaDesignator::WesternMediterraneanArea,

            "NA" => AreaDesignator::NorthAmericaArea,
            "NT" => AreaDesignator::NorthAtlanticArea,

            "OC" => AreaDesignator::OceaniaArea,
            "OH" => AreaDesignator::SeaOfOkhotskArea,

            "PA" => AreaDesignator::PacificArea,
            "PE" => AreaDesignator::PersianGulfArea,
            "PN" => AreaDesignator::NorthPacificArea,
            "PQ" => AreaDesignator::WesternNorthPacificArea,
            "PS" => AreaDesignator::SouthPacificArea,
            "PW" => AreaDesignator::WesternPacificArea,
            "PZ" => AreaDesignator::EasternPacificArea,

            "SA" => AreaDesignator::SouthAmericaArea,
            "SE" => AreaDesignator::SouthernOceanArea,
            "SJ" => AreaDesignator::SeaOfJapanArea,
            "SS" => AreaDesignator::SouthChinaseaArea,
            "ST" => AreaDesignator::SouthAtlanticArea,

            "XE" => AreaDesignator::EasternHemisphereArea,
            "XN" => AreaDesignator::NorthernHemisphereArea,
            "XS" => AreaDesignator::SouthernHemisphereArea,
            "XT" => AreaDesignator::TropicalBeltArea,
            "XW" => AreaDesignator::WesternHemisphereArea,

            "HN" => AreaDesignator::HNUnknown,

            "XX" => AreaDesignator::Unknown,

            x => {
                println!("Unknown area designator: {}", x);
                return None;
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReportNature {
    OceanWeatherStation,
    MobileShipOrStation,
    Floats,
}

/// WMO.385 table C2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportAreaDesignator {
    /// Area between 30°N–60°S, 35°W–70°E
    A,
    /// Area between 90°N–05°N, 70°E–180°E
    B,
    /// Area between 05°N–60°S, 120°W–35°W
    C,
    /// Area between 90°N–05°N, 180°W–35°W
    D,
    /// Area between 05°N–60°S, 70°E–120°W
    E,
    /// Area between 90°N–30°N, 35°W–70°E
    F,
    /// Area south of 60°S
    J,
    /// More than one area
    X,
}

// Table C3 of WMO.386
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GeographicalAreaDesignator {
    /// Nothern hemisphere, 0 to 90 degrees West
    NorthernHemisphere_0_90W,

    /// Northern hemisphere, 90 to 180 degrees West
    NorthernHemisphere_90_180W,

    /// Northern hemisphere, 180 to 90 degrees East
    NorthernHemisphere_180_90E,

    /// Northern hemisphere, 90 to 0 degrees East
    NorthernHemisphere_90E_0,

    /// Southern hemisphere, 0 to 90 degrees West
    SouthernHemisphere_0_90W,

    /// Southern hemisphere, 90 to 180 degrees West
    SouthernHemisphere_90_180W,

    /// Southern hemisphere, 180 to 90 degrees East
    SouthernHemisphere_180_90E,

    /// Southern hemisphere, 90 to 0 degrees East
    SouthernHemisphere_90E_E,

    /// Tropical belt, 0 to 90 degrees West
    TropicalBelt_0_90W,

    /// Tropical belt, 90 to 180 degrees West
    TropicalBelt_90_180W,

    /// Tropical belt, 180 to 90 degrees East
    TropicalBelt_180_90E,

    /// Tropical belt, 90 to 0 degrees East
    TropicalBelt_90E_0,

    /// Northern hemispher
    NorthernHemisphere,

    /// Southern hemisphere
    SouthernHemisphere,

    /// Northern hemisphere, 45W to 180
    NorthernHemisphere_45W_180,

    /// An unknown area using code U
    UnknownU,

    /// An unknown area using code P
    UnknownP,

    /// Global area
    GlobalArea,
}

impl GeographicalAreaDesignator {
    pub fn from_c3(c: char) -> Option<GeographicalAreaDesignator> {
        match c {
            'A' => Some(GeographicalAreaDesignator::NorthernHemisphere_0_90W),
            'B' => Some(GeographicalAreaDesignator::NorthernHemisphere_90_180W),
            'C' => Some(GeographicalAreaDesignator::NorthernHemisphere_180_90E),
            'D' => Some(GeographicalAreaDesignator::NorthernHemisphere_90E_0),
            'E' => Some(GeographicalAreaDesignator::TropicalBelt_0_90W),
            'F' => Some(GeographicalAreaDesignator::TropicalBelt_90_180W),
            'G' => Some(GeographicalAreaDesignator::TropicalBelt_180_90E),
            'H' => Some(GeographicalAreaDesignator::TropicalBelt_90E_0),
            'I' => Some(GeographicalAreaDesignator::SouthernHemisphere_0_90W),
            'J' => Some(GeographicalAreaDesignator::SouthernHemisphere_90_180W),
            'K' => Some(GeographicalAreaDesignator::SouthernHemisphere_180_90E),
            'L' => Some(GeographicalAreaDesignator::SouthernHemisphere_90E_E),
            'N' => Some(GeographicalAreaDesignator::NorthernHemisphere),
            'S' => Some(GeographicalAreaDesignator::SouthernHemisphere),
            'T' => Some(GeographicalAreaDesignator::NorthernHemisphere_45W_180),
            'X' => Some(GeographicalAreaDesignator::GlobalArea),
            'U' => Some(GeographicalAreaDesignator::UnknownU),
            'P' => Some(GeographicalAreaDesignator::UnknownP),
            x => panic!("unknown c3: {}", x),
        }
    }
}

#[derive(Debug)]
pub enum TimeDesignator {
    Analysis,
    Forecast3Hours,
    Forecast6Hours,
    Forecast9Hours,
    Forecast12Hours,
    Forecast15Hours,
    Forecast18Hours,
    Forecast21Hours,
    Forecast24Hours,
    Forecast27Hours,
    Forecast30Hours,
    Forecast33Hours,
    Forecast36Hours,
    Forecast39Hours,
    Forecast42Hours,
    Forecast45Hours,
    Forecast48Hours,
    Forecast60Hours,
    Forecast72Hours,
    Forecast84Hours,
    Forecast96Hours,
    Forecast108Hours,
    Forecast120Hours,
    Forecast132Hours,
    Forecast144Hours,
    Forecast156Hours,
    Forecast168Hours,
    Forecast10Days,
    Forecast15Days,
    Forecast30Days,
}

impl TimeDesignator {
    pub fn from_c4(c: char) -> Option<TimeDesignator> {
        match c {
            'A' => Some(TimeDesignator::Analysis),
            'B' => Some(TimeDesignator::Forecast6Hours),
            'C' => Some(TimeDesignator::Forecast12Hours),
            'D' => Some(TimeDesignator::Forecast18Hours),
            'E' => Some(TimeDesignator::Forecast24Hours),
            'F' => Some(TimeDesignator::Forecast30Hours),
            'G' => Some(TimeDesignator::Forecast36Hours),
            'H' => Some(TimeDesignator::Forecast42Hours),
            'I' => Some(TimeDesignator::Forecast48Hours),
            'J' => Some(TimeDesignator::Forecast60Hours),
            'K' => Some(TimeDesignator::Forecast72Hours),
            'L' => Some(TimeDesignator::Forecast84Hours),
            'M' => Some(TimeDesignator::Forecast96Hours),
            'N' => Some(TimeDesignator::Forecast108Hours),
            'O' => Some(TimeDesignator::Forecast120Hours),
            'P' => Some(TimeDesignator::Forecast132Hours),
            'Q' => Some(TimeDesignator::Forecast144Hours),
            'R' => Some(TimeDesignator::Forecast156Hours),
            'S' => Some(TimeDesignator::Forecast168Hours),
            'T' => Some(TimeDesignator::Forecast10Days),
            'U' => Some(TimeDesignator::Forecast15Days),
            'V' => Some(TimeDesignator::Forecast30Days),
            x => panic!("unknown C4 time designator: {}", x),
        }
    }
    pub fn from_c5(c: char) -> Option<TimeDesignator> {
        match c {
            'A' => Some(TimeDesignator::Analysis),
            'B' => Some(TimeDesignator::Forecast3Hours),
            'C' => Some(TimeDesignator::Forecast6Hours),
            'D' => Some(TimeDesignator::Forecast9Hours),
            'E' => Some(TimeDesignator::Forecast12Hours),
            'F' => Some(TimeDesignator::Forecast15Hours),
            'G' => Some(TimeDesignator::Forecast18Hours),
            'H' => Some(TimeDesignator::Forecast21Hours),
            'I' => Some(TimeDesignator::Forecast24Hours),
            'J' => Some(TimeDesignator::Forecast27Hours),
            'K' => Some(TimeDesignator::Forecast30Hours),
            'L' => Some(TimeDesignator::Forecast33Hours),
            'M' => Some(TimeDesignator::Forecast36Hours),
            'N' => Some(TimeDesignator::Forecast39Hours),
            'O' => Some(TimeDesignator::Forecast42Hours),
            'P' => Some(TimeDesignator::Forecast45Hours),
            'Q' => Some(TimeDesignator::Forecast48Hours),
            x => panic!("unknown C5 time designator: {}", x),
        }
    }
}

fn lookup_table_b1(dt: WMODataTypeT1, t2: char) -> WMODataTypeT2 {
    match dt {
        WMODataTypeT1::Analyses => match t2 {
            'B' => WMODataTypeT2::TemperaturePrecipitationTableAnalysis,
            'C' => WMODataTypeT2::CycloneAnalysis,
            'E' => WMODataTypeT2::AirQualityAlertAnalysis,
            'G' => WMODataTypeT2::HydrologicalMarineAnalysis,
            'H' => WMODataTypeT2::ThicknessAnalysis,
            'I' => WMODataTypeT2::IceAnalysis,
            'O' => WMODataTypeT2::OzoneAnalysis,
            'R' => WMODataTypeT2::RadarAnalysis,
            'S' => WMODataTypeT2::SurfaceAnalysis,
            'U' => WMODataTypeT2::UpperAirAnalysis,
            'W' => WMODataTypeT2::WeatherSummaryAnalysis,
            'X' => WMODataTypeT2::MiscellaneousAnalysis,
            x => WMODataTypeT2::UnknownAnalyses(x),
        },
        WMODataTypeT1::ClimaticData => match t2 {
            'A' => WMODataTypeT2::ClimateAnomalies,
            'D' => WMODataTypeT2::ClimatologicalReportDaily,
            'X' => WMODataTypeT2::ClimatologicalReport,
            'E' => WMODataTypeT2::MonthlyMeansUpperAir,
            'H' => WMODataTypeT2::MonthlyMeansSurfaceShip,
            'O' => WMODataTypeT2::MonthlyMeansOceanAreasClimate,
            'S' => WMODataTypeT2::MonthlyMeansSurfaceClimate,
            x => WMODataTypeT2::UnknownClimate(x),
        },
        WMODataTypeT1::Forecasts => match t2 {
            'A' => WMODataTypeT2::AviationAreaAdvisoriesForecast,
            'B' => WMODataTypeT2::UpperWindsAndTemperaturesForecast,
            'C' => WMODataTypeT2::Aerodrome12Forecast,
            'D' => WMODataTypeT2::RadiologicalTrajectoryDoseForecast,
            'E' => WMODataTypeT2::ExtendedForecast,
            'F' => WMODataTypeT2::ShippingAreaForecast,
            'G' => WMODataTypeT2::HydrologicalForecast,
            'H' => WMODataTypeT2::UpperAirThicknessForecast,
            'I' => WMODataTypeT2::IcebergForecast,
            'J' => WMODataTypeT2::RadioWarningServiceForecast,
            'K' => WMODataTypeT2::TropicalCycloneAdvisoriesForecast,
            'L' => WMODataTypeT2::LocalAreaForecast,
            'M' => WMODataTypeT2::TemperatureExtremesForecast,
            'N' => WMODataTypeT2::SpaceWeatherAdvisoriesForecast,
            'O' => WMODataTypeT2::GuidanceForecast,
            'P' => WMODataTypeT2::PublicForecast,
            'Q' => WMODataTypeT2::OtherShippingForecast,
            'R' => WMODataTypeT2::AviationRouteForecast,
            'S' => WMODataTypeT2::SurfaceForecast,
            'T' => WMODataTypeT2::Aerodrome12Forecast,
            'U' => WMODataTypeT2::UpperAirForecast,
            'V' => WMODataTypeT2::VolcanicAshAdvisoriesForecast,
            'W' => WMODataTypeT2::WinterSportsForecast,
            'X' => WMODataTypeT2::MiscellaneousForecast,
            'Z' => WMODataTypeT2::ShippingAreaForecast,
            x => WMODataTypeT2::UnknownForecast(x),
        },
        WMODataTypeT1::Notices => match t2 {
            'G' => WMODataTypeT2::HydrologicalNotice,
            'H' => WMODataTypeT2::MarineNotice,
            'N' => WMODataTypeT2::NuclearEmergencyResponseNotice,
            'O' => WMODataTypeT2::METNOWIFMANotice,
            'P' => WMODataTypeT2::ProductGenerationDelayNotice,
            'T' => WMODataTypeT2::TestMsgNotice,
            'W' => WMODataTypeT2::WarningRelatedCancellationWarning,
            'Z' => WMODataTypeT2::RegionalWeatherRoundupNotice,
            x => WMODataTypeT2::UnknownNotice(x),
        },
        WMODataTypeT1::SurfaceData => match t2 {
            'A' => WMODataTypeT2::AviationRoutineReports,
            'B' => WMODataTypeT2::RadarReportsPartA,
            'C' => WMODataTypeT2::RadarReportsPartB,
            'D' => WMODataTypeT2::RadarReportsPartsAB,
            'E' => WMODataTypeT2::SeismicData,
            'F' => WMODataTypeT2::AtmosphericsReports,
            'G' => WMODataTypeT2::RadiologicalDataReport,
            'H' => WMODataTypeT2::ReportsFromDCPStations,
            'I' => WMODataTypeT2::IntermediateSynopticHour,
            'L' => WMODataTypeT2::NotUsed,
            'M' => WMODataTypeT2::MainSynopticHour,
            'N' => WMODataTypeT2::NonStandardSynopticHour,
            'O' => WMODataTypeT2::OceanographicData,
            'P' => WMODataTypeT2::SpecialAviationWeatherReports,
            'R' => WMODataTypeT2::HydrologicalRiverReports,
            'S' => WMODataTypeT2::DriftingBouyReports,
            'T' => WMODataTypeT2::SeaIce,
            'U' => WMODataTypeT2::SnowDepth,
            'V' => WMODataTypeT2::LakeIce,
            'W' => WMODataTypeT2::WaveInformation,
            'X' => WMODataTypeT2::MiscellaneousSurface,
            'Y' => WMODataTypeT2::SeismicWaveformData,
            'Z' => WMODataTypeT2::TsunamiData,
            x => WMODataTypeT2::UnknownSurfaceData(x),
        },
        WMODataTypeT1::SatalliteData => match t2 {
            'B' => WMODataTypeT2::SatelliteOrbitParameters,
            'C' => WMODataTypeT2::SatelliteCloudInterpretations,
            'H' => WMODataTypeT2::SatelliteRemoteUpperAirSounding,
            'R' => WMODataTypeT2::ClearRadianceObservations,
            'T' => WMODataTypeT2::SeaSurfaceTemperatures,
            'W' => WMODataTypeT2::WindsAndCloudTemperatures,
            'X' => WMODataTypeT2::MiscellaneousSatellite,
            x => WMODataTypeT2::UnknownSatellite(x),
        },
        WMODataTypeT1::UpperAirData => match t2 {
            'A' => WMODataTypeT2::AircraftReports41,
            'D' => WMODataTypeT2::AircraftReports42,
            'E' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartD,
            'F' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartCD,
            'G' => WMODataTypeT2::UpperWindPartB,
            'H' => WMODataTypeT2::UpperWindPartC,
            'I' => WMODataTypeT2::UpperWindPartsAB,
            'K' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartB,
            'L' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartC,
            'M' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartsAB,
            'N' => WMODataTypeT2::RocketsondeReports,
            'P' => WMODataTypeT2::UpperWindPartA,
            'Q' => WMODataTypeT2::UpperWindPartD,
            'R' => WMODataTypeT2::AircraftReport,
            'S' => WMODataTypeT2::UpperLevelPressureTemperatureHumidityWindPartA,
            'T' => WMODataTypeT2::AircraftReport2,
            'X' => WMODataTypeT2::MiscellaneousUpperAir,
            'Y' => WMODataTypeT2::UpperWindPartsCD,
            'Z' => WMODataTypeT2::PTHWFromSonde,
            x => WMODataTypeT2::UnknownUpperAir(x),
        },
        WMODataTypeT1::Warnings => match t2 {
            'A' => WMODataTypeT2::AIRMETWarning,
            'C' => WMODataTypeT2::TropicalCycloneSigmetWarning,
            'E' => WMODataTypeT2::TsunamiWarning,
            'F' => WMODataTypeT2::TornadoWarning,
            'G' => WMODataTypeT2::HydrologicalRiverFloodWarning,
            'H' => WMODataTypeT2::MarineCoastalFloodWarning,
            'O' => WMODataTypeT2::OtherWarning,
            'R' => WMODataTypeT2::HumanitarianActivitiesWarning,
            'S' => WMODataTypeT2::SIGMETWarning,
            'T' => WMODataTypeT2::TropicalCycloneWarning,
            'U' => WMODataTypeT2::SevereThunderstormWarning,
            'V' => WMODataTypeT2::VolcanicAshCloudsWarning,
            'W' => WMODataTypeT2::WarningRelatedCancellationWarning,
            x => WMODataTypeT2::UnknownWarning(x),
        },
        _ => panic!("Unknown t2 data type {} for t1 {:?}", t2, dt),
    }
}

// Very similar to B6
fn lookup_table_b2(t2: char) -> WMODataTypeT2 {
    match t2 {
        'A' => WMODataTypeT2::RadarData,
        'B' => WMODataTypeT2::CloudData,
        'C' => WMODataTypeT2::VorticityData,
        'D' => WMODataTypeT2::ThicknessData,
        'E' => WMODataTypeT2::PrecipitationData,
        'G' => WMODataTypeT2::DivergenceData,
        'H' => WMODataTypeT2::HeightData,
        'J' => WMODataTypeT2::WaveHeightCombinationsData,
        'K' => WMODataTypeT2::SwellHeightCombinationsData,
        'M' => WMODataTypeT2::NationalUseData,
        'N' => WMODataTypeT2::RadiationData,
        'O' => WMODataTypeT2::VerticalVelocityData,
        'P' => WMODataTypeT2::PressureData,
        'Q' => WMODataTypeT2::WetBulbPotentialTemperatureData,
        'R' => WMODataTypeT2::RelativeHumidityData,
        'T' => WMODataTypeT2::TemperatureData,
        'U' => WMODataTypeT2::EastwardWindComponentData,
        'V' => WMODataTypeT2::NorthwardWindComponentData,
        'W' => WMODataTypeT2::WindData,
        'Z' => WMODataTypeT2::NotAssignedData,
        _ => panic!("Unknown table b5 t2 data type {}", t2),
    }
}

fn lookup_table_b6(t2: char) -> WMODataTypeT2 {
    match t2 {
        'A' => WMODataTypeT2::RadarData,
        'B' => WMODataTypeT2::CloudData,
        'C' => WMODataTypeT2::ClearAirTurbulenceData,
        'D' => WMODataTypeT2::ThicknessData,
        'E' => WMODataTypeT2::PrecipitationData,
        'F' => WMODataTypeT2::AerologicalDiagramsData,
        'G' => WMODataTypeT2::SignificantWeatherData,
        'H' => WMODataTypeT2::HeightData,
        'I' => WMODataTypeT2::IceFlowData,
        'J' => WMODataTypeT2::WaveHeightCombinationsData,
        'K' => WMODataTypeT2::SwellHeightCombinationsData,
        'L' => WMODataTypeT2::PlainLanguageData,
        'M' => WMODataTypeT2::NationalUseData,
        'N' => WMODataTypeT2::RadiationData,
        'O' => WMODataTypeT2::VerticalVelocityData,
        'P' => WMODataTypeT2::PressureData,
        'Q' => WMODataTypeT2::WetBulbPotentialTemperatureData,
        'R' => WMODataTypeT2::RelativeHumidityData,
        'S' => WMODataTypeT2::SnowCoverData,
        'T' => WMODataTypeT2::TemperatureData,
        'U' => WMODataTypeT2::EastwardWindComponentData,
        'V' => WMODataTypeT2::NorthwardWindComponentData,
        'W' => WMODataTypeT2::WindData,
        'X' => WMODataTypeT2::LiftedIndexData,
        'Y' => WMODataTypeT2::ObservationalPlottedChartData,
        'Z' => WMODataTypeT2::NotAssignedData,
        _ => panic!("Unknown table b5 t2 data type {}", t2),
    }
}

fn lookup_table_b5(t2: char) -> WMODataTypeT2 {
    match t2 {
        'C' => WMODataTypeT2::CloudTopTemperatureSatImg,
        'F' => WMODataTypeT2::FogSatImg,
        'I' => WMODataTypeT2::InfraredSatImg,
        'S' => WMODataTypeT2::SurfaceTemperatureSatImg,
        'V' => WMODataTypeT2::VisibleSatImg,
        'W' => WMODataTypeT2::WaterVaporSatImg,
        'Y' => WMODataTypeT2::UserSpecifiedSatImg,
        'Z' => WMODataTypeT2::UnspecifiedSatImg,
        _ => panic!("Unknown table b5 t2 data type {}", t2),
    }
}

impl From<char> for WMODataTypeT1 {
    /// Parse a WMODataTypeT1 value from a single upper-case ASCII character (in the range A to Z)
    fn from(c: char) -> Self {
        match c {
            'A' => WMODataTypeT1::Analyses,
            'B' => WMODataTypeT1::AddressedMessage,
            'C' => WMODataTypeT1::ClimaticData,
            'D' => WMODataTypeT1::GridD,
            'E' => WMODataTypeT1::SatelliteImg,
            'F' => WMODataTypeT1::Forecasts,
            'G' => WMODataTypeT1::GridPointInformationGrid,
            'H' => WMODataTypeT1::GridPointInformationGrib,
            'I' => WMODataTypeT1::ObservationalData,
            'J' => WMODataTypeT1::ForecastInformation,
            'K' => WMODataTypeT1::Crex,
            'L' => WMODataTypeT1::AviationInformation,
            // M is unused
            'N' => WMODataTypeT1::Notices,
            'O' => WMODataTypeT1::OceanographicInformation,
            'P' => WMODataTypeT1::Pictoral,
            'Q' => WMODataTypeT1::PictoralRegional,
            // R is unused
            'S' => WMODataTypeT1::SurfaceData,
            'T' => WMODataTypeT1::SatalliteData,
            'U' => WMODataTypeT1::UpperAirData,
            'V' => WMODataTypeT1::NationalData,
            'W' => WMODataTypeT1::Warnings,
            'X' => WMODataTypeT1::CAP,
            'Y' => WMODataTypeT1::GribRegionalUse,
            // Z is unused
            _ => WMODataTypeT1::Unused,
        }
    }
}

#[derive(Debug)]
pub enum Area {
    Area(AreaDesignator),
    GeoArea(GeographicalAreaDesignator, TimeDesignator),
    /// The area from which a report was generated
    ///
    /// Used for bulletins containing ship's weather reports and oceanographic data including reports from
    /// automatic marine stations.
    ReportArea(ReportAreaDesignator, ReportNature),
}

#[cfg(test)]
mod tests {
    use crate::parse_wmo_abbreviated_heading;

    #[test]
    fn parse_specific() {
        let p = parse_wmo_abbreviated_heading('A', 'B', "PW");
        println!("{p:?}");
    }

    #[test]
    #[ignore]
    fn parse_all() {
        // This test parses all data in my local repo if files, and makes sure the they can all be parsed
        for entry in std::fs::read_dir("h:/tmp/49/debug").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if (filename.starts_with("A_") || filename.starts_with("Z_"))
                && filename.ends_with(".debug")
            {
                // parse the first WMO abbrev
                let mut chars = filename.chars().skip(2);

                let (_0, _1, _2) = parse_wmo_abbreviated_heading(
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                    &filename[4..6],
                );
                if _1.is_unknown() {
                    println!("{_0:?} {_1:?} {_2:?} {filename}");
                }
            }
        }
    }
}
