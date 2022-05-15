#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NWSProduct {
    /// Rawinsonde Data Above 100 Millibars
    ABV,
    /// Alarm/Alert Administrative Msg
    ADA,
    /// Alert Administrative Message
    ADM,
    /// NWS Administrative Message
    ADR,
    /// Generic Space Environment Advisory
    ADV,
    /// Area Forecast Discussion
    AFD,
    /// Area Forecast Matrices
    AFM,
    /// Area Forecast Product
    AFP,
    /// Fire Weather Matrix
    AFW,
    /// Agricultural Forecast
    AGF,
    /// Agricultural Observations
    AGO,
    /// Space Environment Alert
    ALT,
    /// Air Quality Alert
    AQA,
    /// Air Quality Index Statement
    AQI,
    /// Air Stagnation Advisory
    ASA,
    /// Avalanche Watch
    AVA,
    /// Avalanche Weather Guidance
    AVG,
    /// Avalanche Warning
    AVW,
    /// Area Weather Outlook
    AWO,
    /// Area Weather Summary
    AWS,
    /// Area Weather Update
    AWU,
    /// Airport Weather Warning
    AWW,
    /// Blue Alert
    BLU,
    /// Buoy Report
    BOY,
    /// Coast Guard Observations
    BRG,
    /// Hourly Roundup for Weather Radio
    BRT,
    /// Child Abduction Emergency
    CAE,
    /// Coded City Forecast
    CCF,
    /// Civil Danger Warning
    CDW,
    /// Civil Emergency Message
    CEM,
    /// WFO Monthly/Daily Climate Data
    CF6,
    /// Convective Forecast Product
    CFP,
    /// Coastal Flood Warnings/Watches/Statements
    CFW,
    /// Coast Guard Surface Report
    CGR,
    /// Computer Hurricane Guidance
    CHG,
    /// Climatological Report (Annual)
    CLA,
    /// Climatological Report (Daily)
    CLI,
    /// Climatological Report (Monthly)
    CLM,
    /// Climatological Report (Quarterly)
    CLQ,
    /// Climatological Report (Seasonal)
    CLS,
    /// Climate Report
    CLT,
    /// Coded Climatological Monthly Means
    CMM,
    /// Coded Analysis and Forecasts
    COD,
    /// Great Lakes Port Forecast
    CPF,
    /// Routine Space Environment Products
    CUR,
    /// Center (CWSU) Weather Advisory
    CWA,
    /// Coastal Waters Forecast
    CWF,
    /// Center (CWSU) Weather Statement
    CWS,
    /// Routine Space Environment Product (Daily)
    DAY,
    /// Daily Dispersion Outlook
    DDO,
    /// Drought Information Statement
    DGT,
    /// Practice/Demo Warning
    DMO,
    /// Unnumbered Depression / Suspicious Area Advisory
    DSA,
    /// ASOS Daily Summary
    DSM,
    /// Dust Storm Warning and Dust Advisory
    DSW,
    /// 3 To 5 Day Extended Forecast
    EFP,
    /// Average 6 To 10 Day Weather Outlook (Local)
    EOL,
    /// Tsunami Bulletin
    EQI,
    /// Earthquake Report
    EQR,
    /// Earthquake Warning
    EQW,
    /// Flood Potential Outlook
    ESF,
    /// Extended Streamflow Guidance
    ESG,
    /// Extended Streamflow Prediction
    ESP,
    /// Water Supply Outlook
    ESS,
    /// Evacuation Immediate
    EVI,
    /// Extreme Wind Warning
    EWW,
    /// Aviation Area Forecasts (Pacific)
    FA0,
    /// Aviation Area Forecasts (Northeast)
    FA1,
    /// Aviation Area Forecasts (Southeast)
    FA2,
    /// Aviation Area Forecasts (North Central)
    FA3,
    /// Aviation Area Forecasts (South Central)
    FA4,
    /// Aviation Area Forecasts (Rocky Mountains)
    FA5,
    /// Aviation Area Forecasts (West Coast)
    FA6,
    /// Aviation Area Forecasts (Juneau, AK)
    FA7,
    /// Aviation Area Forecasts (Anchorage, AK)
    FA8,
    /// Aviation Area Forecasts (Fairbanks, AK)
    FA9,
    /// 24 Hr Fd Winds Aloft Fcst (45,000 and 53,000 Ft)
    FD0,
    /// 6 Hour Winds Aloft Forecast
    FD1,
    /// 12 Hour Winds Aloft Forecast
    FD2,
    /// 24 Hour Winds Aloft Forecast
    FD3,
    /// Winds Aloft Forecast
    FD4,
    /// Winds Aloft Forecast
    FD5,
    /// Winds Aloft Forecast
    FD6,
    /// Winds Aloft Forecast
    FD7,
    /// 6 Hour Fd Winds Aloft Fcst (45,000 and 53,000 Ft)
    FD8,
    /// 12 Hr Fd Winds Aloft Fcst (45,000 and 53,000 Ft)
    FD9,
    /// Fire Danger Indices
    FDI,
    /// Flash Flood Watch
    FFA,
    /// Flash Flood Guidance
    FFG,
    /// Headwater Guidance
    FFH,
    /// Flash Flood Statement
    FFS,
    /// Flash Flood Warning
    FFW,
    /// National Flood Summary
    FLN,
    /// Flood Statement
    FLS,
    /// Flood Warning
    FLW,
    /// Upper Wind Fallout Forecast
    FOF,
    /// Fire Warning
    FRW,
    /// Natl Marine Fisheries Administrative Service Message
    FSH,
    /// WSR-88D Radar Outage Notification / Free Text Message
    FTM,
    /// FOUS Prog Max/Min Temp/Pop Guidance
    FTP,
    /// Fire Weather Administrative Message
    FWA,
    /// Fire Weather Outlook Discussion
    FWD,
    /// Routine Fire Wx Fcst (With/Without 6-10 Day Outlook)
    FWF,
    /// Land Management Forecasts
    FWL,
    /// Miscellaneous Fire Weather Product
    FWM,
    /// Fire Weather Notification
    FWN,
    /// Fire Weather Observation
    FWO,
    /// Suppression Forecast
    FWS,
    /// Freezing Level Data (RADAT)
    FZL,
    /// Great Lakes Forecast
    GLF,
    /// Great Lakes Storm Summary
    GLS,
    /// GREEN
    GRE,
    /// RFC Derived QPF Data Product
    HD1,
    /// RFC Derived QPF Data Product
    HD2,
    /// RFC Derived QPF Data Product
    HD3,
    /// RFC Derived QPF Data Product
    HD4,
    /// RFC Derived QPF Data Product
    HD7,
    /// RFC Derived QPF Data Product
    HD8,
    /// RFC Derived QPF Data Product
    HD9,
    /// Hurricane Local Statement
    HLS,
    /// Hydrometeorological Discussion
    HMD,
    /// AHPS XML
    HML,
    /// Hazardous Materials Warning
    HMW,
    /// RFC QPF Verification Product
    HP1,
    /// RFC QPF Verification Product
    HP2,
    /// RFC QPF Verification Product
    HP3,
    /// RFC QPF Verification Product
    HP4,
    /// RFC QPF Verification Product
    HP5,
    /// RFC QPF Verification Product
    HP6,
    /// RFC QPF Verification Product
    HP7,
    /// RFC QPF Verification Product
    HP8,
    /// Weather Roundup
    HRR,
    /// High Seas Forecast
    HSF,
    /// Hazardous Weather Outlook
    HWO,
    /// Hourly Weather Roundup
    HWR,
    /// Daily Hydrometeorological Products
    HYD,
    /// Monthly Hydrometeorological Plain Language Product
    HYM,
    /// Ice Forecast
    ICE,
    /// Ice Drift Vectors
    IDM,
    /// ADMINISTR [NOUS51 KWBC]
    INI,
    /// Ice Observation
    IOB,
    /// Keep Alive Message
    KPA,
    /// Local Area Emergency
    LAE,
    /// Preliminary Local Climatological Data
    LCD,
    /// Local Cooperative Observation
    LCO,
    /// Law Enforcement Warning
    LEW,
    /// Local Forecast
    LFP,
    /// Lake Stages
    LKE,
    /// Low-Level Sounding
    LLS,
    /// Low Temperatures
    LOW,
    /// Local Storm Report
    LSR,
    /// Lightning Data
    LTG,
    /// Rawinsonde Observation Mandatory Levels
    MAN,
    /// Mean Areal Precipitation
    MAP,
    /// Amended Marine Forecast
    MAW,
    /// Marine Forecast Matrix
    MFM,
    /// Marine Interpretation Message
    MIM,
    /// Miscellaneous Local Product
    MIS,
    /// MOB Observations
    MOB,
    /// Routine Space Environment Product Issued Monthly
    MON,
    /// Techniques Development Laboratory Marine Product
    MRP,
    /// ASOS Monthly Summary Message
    MSM,
    /// METAR Formatted Surface Weather Observation
    MTR,
    /// METAR Test Message
    MTT,
    /// Marine Verification Coded Message
    MVF,
    /// Marine Weather Statement
    MWS,
    /// Marine Weather Message
    MWW,
    /// Weather Reconnaisance Flights
    NOU,
    /// Short Term Forecast
    NOW,
    /// Data Mgt Message
    NOX,
    /// Non-Precipitation Warnings / Watches / Advisories
    NPW,
    /// Nearshore Marine Forecast
    NSH,
    /// Nuclear Power Plant Warning
    NUW,
    /// NOAA Weather Radio Forecast
    NWR,
    /// Other Aviation Products
    OAV,
    /// Observations
    OBS,
    /// Offshore Aviation Area Forecast
    OFA,
    /// Offshore Forecast
    OFF,
    /// Other Marine Products
    OMR,
    /// Other Public Products
    OPU,
    /// Other Surface Observations
    OSO,
    /// Ocean Surface Winds
    OSW,
    /// Other Upper Air Data
    OUA,
    /// Zone Forecast
    OZF,
    /// Point Forecast Matrices
    PFM,
    /// Fire Weather Point Forecast Matrices
    PFW,
    /// Plain Language Ship Report
    PLS,
    /// Prognostic Meteorological Discussion
    PMD,
    /// Public Information Statement
    PNS,
    /// Probability of Exceed
    POE,
    /// Heat Index Forecast Tables
    PRB,
    /// State Pilot Report Collective
    PRC,
    /// Preliminary Forecasts
    PRE,
    /// Post Storm Hurricane Report
    PSH,
    /// Probabilistic Outlook Points
    PTS,
    /// Public Severe Weather Outlook
    PWO,
    /// Tropical Cyclone Probabilities
    PWS,
    /// Quantitative Precipitation Forecast
    QPF,
    /// Quantitative Precipitation Statement
    QPS,
    /// Revised Digital Forecast
    RDF,
    /// Recreational Report
    REC,
    /// Record Report
    RER,
    /// EAS Activation Request
    RET,
    /// Rangeland Fire Danger Forecast
    RFD,
    /// RFI Observation
    RFI,
    /// Route Forecast
    RFR,
    /// Red Flag Warning
    RFW,
    /// Radiological Hazard Warning
    RHW,
    /// Required Monthly Test
    RMT,
    /// Rain Information Statement
    RNS,
    /// Hydro-Met Data Report Part 1
    RR1,
    /// Hydro-Met Data Report Part 2
    RR2,
    /// Hydro-Met Data Report Part 3
    RR3,
    /// Hydro-Met Data Report Part 4
    RR4,
    /// Hydro-Met Data Report Part 5
    RR5,
    /// Hydro-Met Data Report Part 6
    RR6,
    /// Hydro-Met Data Report Part 7
    RR7,
    /// Hydro-Met Data Report Part 8
    RR8,
    /// Hydro-Met Data Report Part 9
    RR9,
    /// Automated Hydrologic Observation Sta Report (AHOS)
    RRA,
    /// Miscellaneous Hydrologic Data
    RRM,
    /// HADS Data
    RRS,
    /// ASOS SHEF Hourly Routine Test Message
    RRY,
    /// Daily Snotel Data
    RSD,
    /// Monthly Snotel Data
    RSM,
    /// Regional Max/Min Temp and Precipitation Table
    RTP,
    /// River Summary
    RVA,
    /// Daily River Forecasts
    RVD,
    /// River Forecast
    RVF,
    /// River Ice Statement
    RVI,
    /// Miscellaneous River Product
    RVM,
    /// River Recreation Statement
    RVR,
    /// River Statement
    RVS,
    /// Regional Weather Roundup
    RWR,
    /// Regional Weather Summary
    RWS,
    /// Required Weekly Test
    RWT,
    /// Special Avalanche Bulletin
    SAB,
    /// Speci Agri Wx Fcst / Advisory / Flying Farmer Fcst Outlook
    SAF,
    /// Snow Avalanche Guidance
    SAG,
    /// APT Prediction
    SAT,
    /// Prelim Notice of Watch & Cancellation Msg (Aviation)
    SAW,
    /// Storm Summary
    SCC,
    /// Supplementary Climatological Data (ASOS)
    SCD,
    /// Soil Climate Analysis Network Data
    SCN,
    /// Satellite Cloud Product
    SCP,
    /// Selected Cities Summary
    SCS,
    /// Supplementary Data Observation (ASOS)
    SDO,
    /// Special Dispersion Statement
    SDS,
    /// Severe Local Storm Watch and Watch Cancellation Msg
    SEL,
    /// SPC Watch Point Information Message
    SEV,
    /// State Forecast
    SFP,
    /// Tabular State Forecast
    SFT,
    /// Rawinsonde Observation Significant Levels
    SGL,
    /// Surface Ship Report at Synoptic Time
    SHP,
    /// International Sigmet / Convective Sigmet
    SIG,
    /// Satellite Interpretation Message
    SIM,
    /// Severe Local Storm Watch and Areal Outline
    SLS,
    /// Smoke Management Weather Forecast
    SMF,
    /// Special Marine Warning
    SMW,
    /// SOO Product
    SOO,
    /// Satellite Precipitation Estimates (TXUS20 KWBC)
    SPE,
    /// Storm Strike Probability Bulletin (TPC)
    SPF,
    /// Special Weather Statement
    SPS,
    /// Shelter in Place Warning
    SPW,
    /// Snow Squall Warning
    SQW,
    /// Surf Discussion
    SRD,
    /// Surf Forecast
    SRF,
    /// Soaring Guidance
    SRG,
    /// Main Synoptic Hour Surface Observation
    SSM,
    /// Network and Severe Weather Statistical Summaries
    STA,
    /// Satellite Tropical Disturbance Summary
    STD,
    /// Road Condition Reports (State Agencies)
    STO,
    /// State Max/Min Temperature and Precipitation Table
    STP,
    /// Spot Forecast Request
    STQ,
    /// Space Weather Message
    SUM,
    /// Severe Thunderstorm Warning
    SVR,
    /// Severe Weather Statement
    SVS,
    /// Severe Storm Outlook Narrative (AC)
    SWO,
    /// State Weather Summary
    SWS,
    /// Regional Weather Synopsis
    SYN,
    /// Terminal Aerodrome Forecast
    TAF,
    /// Terminal Alerting Products
    TAP,
    /// Travelers Forecast Table
    TAV,
    /// Aviation Tropical Cyclone Advisory
    TCA,
    /// Tropical Cyclone Discussion
    TCD,
    /// Tropical Cyclone Position Estimate
    TCE,
    /// Marine/Aviation Tropical Cyclone Advisory
    TCM,
    /// Public Tropical Cyclone Advisory
    TCP,
    /// Satellite Tropical Cyclone Summary
    TCS,
    /// Tropical Cyclone Update
    TCU,
    /// Tropical Cyclone Watch/Warning Break Points
    TCV,
    /// Tsunami Bulletin
    TIB,
    /// Tide Report
    TID,
    /// Tsunami Tide/Seismic Message Acknowledgement
    TMA,
    /// 911 Telephone Outage Emergency
    TOE,
    /// Tornado Warning
    TOR,
    /// Temperature Precipitation Table (Natl and Intnl)
    TPT,
    /// Tsunami Watch/Warning
    TSU,
    /// Weather Bulletin
    TUV,
    /// Travelers Forecast
    TVL,
    /// Transcribed Weather Broadcast
    TWB,
    /// Tropical Weather Discussion
    TWD,
    /// Tropical Weather Outlook and Summary
    TWO,
    /// Tropical Weather Summary
    TWS,
    /// Aircraft Reconnaissance
    URN,
    /// Ultraviolet Index
    UVI,
    /// Volcanic Activity Advisory
    VAA,
    /// Forecast Verification Statistics
    VER,
    /// Terminal Aerodrome Forecast (TAF) Verification
    VFT,
    /// Volcano Warning
    VOW,
    /// Airmet (Pacific)
    WA0,
    /// Airmet (Northeast)
    WA1,
    /// Airmet (Southeast)
    WA2,
    /// Airmet (North Central)
    WA3,
    /// Airmet (South Central)
    WA4,
    /// Airmet (Rocky Mountains)
    WA5,
    /// Airmet (West Coast)
    WA6,
    /// Airmet (Juneau, AK)
    WA7,
    /// Airmet (Anchorage, AK)
    WA8,
    /// Airmet (Fairbanks, AK)
    WA9,
    /// Space Environment Warning
    WAR,
    /// Space Environment Watch
    WAT,
    /// Weather Watch Clearance Notification
    WCN,
    /// Weekly Weather and Crop Report
    WCR,
    /// Weekly Data for Agriculture
    WDA,
    /// Warning Decision Update
    WDU,
    /// Routine Space Environment Product Issued Weekly
    WEK,
    /// Tornado/Severe Thunderstorm Watch
    WOU,
    /// Sigmet (Northeast)
    WS1,
    /// Sigmet (Southeast)
    WS2,
    /// Sigmet (North Central)
    WS3,
    /// Sigmet (South Central)
    WS4,
    /// Sigmet (Rocky Mountains)
    WS5,
    /// Sigmet (West Coast)
    WS6,
    /// Tropical Cyclone Sigmet
    WST,
    /// Volcanic Activity Sigmet
    WSV,
    /// Winter Weather Warnings / Watches / Advisories
    WSW,
    /// Watch Status Report
    WWA,
    /// Severe Thunderstorm / Tornado Watch Probabilities
    WWP,
    /// Zone Forecast Product
    ZFP,
}
impl NWSProduct {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ABV" => Some(NWSProduct::ABV),
            "ADA" => Some(NWSProduct::ADA),
            "ADM" => Some(NWSProduct::ADM),
            "ADR" => Some(NWSProduct::ADR),
            "ADV" => Some(NWSProduct::ADV),
            "AFD" => Some(NWSProduct::AFD),
            "AFM" => Some(NWSProduct::AFM),
            "AFP" => Some(NWSProduct::AFP),
            "AFW" => Some(NWSProduct::AFW),
            "AGF" => Some(NWSProduct::AGF),
            "AGO" => Some(NWSProduct::AGO),
            "ALT" => Some(NWSProduct::ALT),
            "AQA" => Some(NWSProduct::AQA),
            "AQI" => Some(NWSProduct::AQI),
            "ASA" => Some(NWSProduct::ASA),
            "AVA" => Some(NWSProduct::AVA),
            "AVG" => Some(NWSProduct::AVG),
            "AVW" => Some(NWSProduct::AVW),
            "AWO" => Some(NWSProduct::AWO),
            "AWS" => Some(NWSProduct::AWS),
            "AWU" => Some(NWSProduct::AWU),
            "AWW" => Some(NWSProduct::AWW),
            "BLU" => Some(NWSProduct::BLU),
            "BOY" => Some(NWSProduct::BOY),
            "BRG" => Some(NWSProduct::BRG),
            "BRT" => Some(NWSProduct::BRT),
            "CAE" => Some(NWSProduct::CAE),
            "CCF" => Some(NWSProduct::CCF),
            "CDW" => Some(NWSProduct::CDW),
            "CEM" => Some(NWSProduct::CEM),
            "CF6" => Some(NWSProduct::CF6),
            "CFP" => Some(NWSProduct::CFP),
            "CFW" => Some(NWSProduct::CFW),
            "CGR" => Some(NWSProduct::CGR),
            "CHG" => Some(NWSProduct::CHG),
            "CLA" => Some(NWSProduct::CLA),
            "CLI" => Some(NWSProduct::CLI),
            "CLM" => Some(NWSProduct::CLM),
            "CLQ" => Some(NWSProduct::CLQ),
            "CLS" => Some(NWSProduct::CLS),
            "CLT" => Some(NWSProduct::CLT),
            "CMM" => Some(NWSProduct::CMM),
            "COD" => Some(NWSProduct::COD),
            "CPF" => Some(NWSProduct::CPF),
            "CUR" => Some(NWSProduct::CUR),
            "CWA" => Some(NWSProduct::CWA),
            "CWF" => Some(NWSProduct::CWF),
            "CWS" => Some(NWSProduct::CWS),
            "DAY" => Some(NWSProduct::DAY),
            "DDO" => Some(NWSProduct::DDO),
            "DGT" => Some(NWSProduct::DGT),
            "DMO" => Some(NWSProduct::DMO),
            "DSA" => Some(NWSProduct::DSA),
            "DSM" => Some(NWSProduct::DSM),
            "DSW" => Some(NWSProduct::DSW),
            "EFP" => Some(NWSProduct::EFP),
            "EOL" => Some(NWSProduct::EOL),
            "EQI" => Some(NWSProduct::EQI),
            "EQR" => Some(NWSProduct::EQR),
            "EQW" => Some(NWSProduct::EQW),
            "ESF" => Some(NWSProduct::ESF),
            "ESG" => Some(NWSProduct::ESG),
            "ESP" => Some(NWSProduct::ESP),
            "ESS" => Some(NWSProduct::ESS),
            "EVI" => Some(NWSProduct::EVI),
            "EWW" => Some(NWSProduct::EWW),
            "FA0" => Some(NWSProduct::FA0),
            "FA1" => Some(NWSProduct::FA1),
            "FA2" => Some(NWSProduct::FA2),
            "FA3" => Some(NWSProduct::FA3),
            "FA4" => Some(NWSProduct::FA4),
            "FA5" => Some(NWSProduct::FA5),
            "FA6" => Some(NWSProduct::FA6),
            "FA7" => Some(NWSProduct::FA7),
            "FA8" => Some(NWSProduct::FA8),
            "FA9" => Some(NWSProduct::FA9),
            "FD0" => Some(NWSProduct::FD0),
            "FD1" => Some(NWSProduct::FD1),
            "FD2" => Some(NWSProduct::FD2),
            "FD3" => Some(NWSProduct::FD3),
            "FD4" => Some(NWSProduct::FD4),
            "FD5" => Some(NWSProduct::FD5),
            "FD6" => Some(NWSProduct::FD6),
            "FD7" => Some(NWSProduct::FD7),
            "FD8" => Some(NWSProduct::FD8),
            "FD9" => Some(NWSProduct::FD9),
            "FDI" => Some(NWSProduct::FDI),
            "FFA" => Some(NWSProduct::FFA),
            "FFG" => Some(NWSProduct::FFG),
            "FFH" => Some(NWSProduct::FFH),
            "FFS" => Some(NWSProduct::FFS),
            "FFW" => Some(NWSProduct::FFW),
            "FLN" => Some(NWSProduct::FLN),
            "FLS" => Some(NWSProduct::FLS),
            "FLW" => Some(NWSProduct::FLW),
            "FOF" => Some(NWSProduct::FOF),
            "FRW" => Some(NWSProduct::FRW),
            "FSH" => Some(NWSProduct::FSH),
            "FTM" => Some(NWSProduct::FTM),
            "FTP" => Some(NWSProduct::FTP),
            "FWA" => Some(NWSProduct::FWA),
            "FWD" => Some(NWSProduct::FWD),
            "FWF" => Some(NWSProduct::FWF),
            "FWL" => Some(NWSProduct::FWL),
            "FWM" => Some(NWSProduct::FWM),
            "FWN" => Some(NWSProduct::FWN),
            "FWO" => Some(NWSProduct::FWO),
            "FWS" => Some(NWSProduct::FWS),
            "FZL" => Some(NWSProduct::FZL),
            "GLF" => Some(NWSProduct::GLF),
            "GLS" => Some(NWSProduct::GLS),
            "GRE" => Some(NWSProduct::GRE),
            "HD1" => Some(NWSProduct::HD1),
            "HD2" => Some(NWSProduct::HD2),
            "HD3" => Some(NWSProduct::HD3),
            "HD4" => Some(NWSProduct::HD4),
            "HD7" => Some(NWSProduct::HD7),
            "HD8" => Some(NWSProduct::HD8),
            "HD9" => Some(NWSProduct::HD9),
            "HLS" => Some(NWSProduct::HLS),
            "HMD" => Some(NWSProduct::HMD),
            "HML" => Some(NWSProduct::HML),
            "HMW" => Some(NWSProduct::HMW),
            "HP1" => Some(NWSProduct::HP1),
            "HP2" => Some(NWSProduct::HP2),
            "HP3" => Some(NWSProduct::HP3),
            "HP4" => Some(NWSProduct::HP4),
            "HP5" => Some(NWSProduct::HP5),
            "HP6" => Some(NWSProduct::HP6),
            "HP7" => Some(NWSProduct::HP7),
            "HP8" => Some(NWSProduct::HP8),
            "HRR" => Some(NWSProduct::HRR),
            "HSF" => Some(NWSProduct::HSF),
            "HWO" => Some(NWSProduct::HWO),
            "HWR" => Some(NWSProduct::HWR),
            "HYD" => Some(NWSProduct::HYD),
            "HYM" => Some(NWSProduct::HYM),
            "ICE" => Some(NWSProduct::ICE),
            "IDM" => Some(NWSProduct::IDM),
            "INI" => Some(NWSProduct::INI),
            "IOB" => Some(NWSProduct::IOB),
            "KPA" => Some(NWSProduct::KPA),
            "LAE" => Some(NWSProduct::LAE),
            "LCD" => Some(NWSProduct::LCD),
            "LCO" => Some(NWSProduct::LCO),
            "LEW" => Some(NWSProduct::LEW),
            "LFP" => Some(NWSProduct::LFP),
            "LKE" => Some(NWSProduct::LKE),
            "LLS" => Some(NWSProduct::LLS),
            "LOW" => Some(NWSProduct::LOW),
            "LSR" => Some(NWSProduct::LSR),
            "LTG" => Some(NWSProduct::LTG),
            "MAN" => Some(NWSProduct::MAN),
            "MAP" => Some(NWSProduct::MAP),
            "MAW" => Some(NWSProduct::MAW),
            "MFM" => Some(NWSProduct::MFM),
            "MIM" => Some(NWSProduct::MIM),
            "MIS" => Some(NWSProduct::MIS),
            "MOB" => Some(NWSProduct::MOB),
            "MON" => Some(NWSProduct::MON),
            "MRP" => Some(NWSProduct::MRP),
            "MSM" => Some(NWSProduct::MSM),
            "MTR" => Some(NWSProduct::MTR),
            "MTT" => Some(NWSProduct::MTT),
            "MVF" => Some(NWSProduct::MVF),
            "MWS" => Some(NWSProduct::MWS),
            "MWW" => Some(NWSProduct::MWW),
            "NOU" => Some(NWSProduct::NOU),
            "NOW" => Some(NWSProduct::NOW),
            "NOX" => Some(NWSProduct::NOX),
            "NPW" => Some(NWSProduct::NPW),
            "NSH" => Some(NWSProduct::NSH),
            "NUW" => Some(NWSProduct::NUW),
            "NWR" => Some(NWSProduct::NWR),
            "OAV" => Some(NWSProduct::OAV),
            "OBS" => Some(NWSProduct::OBS),
            "OFA" => Some(NWSProduct::OFA),
            "OFF" => Some(NWSProduct::OFF),
            "OMR" => Some(NWSProduct::OMR),
            "OPU" => Some(NWSProduct::OPU),
            "OSO" => Some(NWSProduct::OSO),
            "OSW" => Some(NWSProduct::OSW),
            "OUA" => Some(NWSProduct::OUA),
            "OZF" => Some(NWSProduct::OZF),
            "PFM" => Some(NWSProduct::PFM),
            "PFW" => Some(NWSProduct::PFW),
            "PLS" => Some(NWSProduct::PLS),
            "PMD" => Some(NWSProduct::PMD),
            "PNS" => Some(NWSProduct::PNS),
            "POE" => Some(NWSProduct::POE),
            "PRB" => Some(NWSProduct::PRB),
            "PRC" => Some(NWSProduct::PRC),
            "PRE" => Some(NWSProduct::PRE),
            "PSH" => Some(NWSProduct::PSH),
            "PTS" => Some(NWSProduct::PTS),
            "PWO" => Some(NWSProduct::PWO),
            "PWS" => Some(NWSProduct::PWS),
            "QPF" => Some(NWSProduct::QPF),
            "QPS" => Some(NWSProduct::QPS),
            "RDF" => Some(NWSProduct::RDF),
            "REC" => Some(NWSProduct::REC),
            "RER" => Some(NWSProduct::RER),
            "RET" => Some(NWSProduct::RET),
            "RFD" => Some(NWSProduct::RFD),
            "RFI" => Some(NWSProduct::RFI),
            "RFR" => Some(NWSProduct::RFR),
            "RFW" => Some(NWSProduct::RFW),
            "RHW" => Some(NWSProduct::RHW),
            "RMT" => Some(NWSProduct::RMT),
            "RNS" => Some(NWSProduct::RNS),
            "RR1" => Some(NWSProduct::RR1),
            "RR2" => Some(NWSProduct::RR2),
            "RR3" => Some(NWSProduct::RR3),
            "RR4" => Some(NWSProduct::RR4),
            "RR5" => Some(NWSProduct::RR5),
            "RR6" => Some(NWSProduct::RR6),
            "RR7" => Some(NWSProduct::RR7),
            "RR8" => Some(NWSProduct::RR8),
            "RR9" => Some(NWSProduct::RR9),
            "RRA" => Some(NWSProduct::RRA),
            "RRM" => Some(NWSProduct::RRM),
            "RRS" => Some(NWSProduct::RRS),
            "RRY" => Some(NWSProduct::RRY),
            "RSD" => Some(NWSProduct::RSD),
            "RSM" => Some(NWSProduct::RSM),
            "RTP" => Some(NWSProduct::RTP),
            "RVA" => Some(NWSProduct::RVA),
            "RVD" => Some(NWSProduct::RVD),
            "RVF" => Some(NWSProduct::RVF),
            "RVI" => Some(NWSProduct::RVI),
            "RVM" => Some(NWSProduct::RVM),
            "RVR" => Some(NWSProduct::RVR),
            "RVS" => Some(NWSProduct::RVS),
            "RWR" => Some(NWSProduct::RWR),
            "RWS" => Some(NWSProduct::RWS),
            "RWT" => Some(NWSProduct::RWT),
            "SAB" => Some(NWSProduct::SAB),
            "SAF" => Some(NWSProduct::SAF),
            "SAG" => Some(NWSProduct::SAG),
            "SAT" => Some(NWSProduct::SAT),
            "SAW" => Some(NWSProduct::SAW),
            "SCC" => Some(NWSProduct::SCC),
            "SCD" => Some(NWSProduct::SCD),
            "SCN" => Some(NWSProduct::SCN),
            "SCP" => Some(NWSProduct::SCP),
            "SCS" => Some(NWSProduct::SCS),
            "SDO" => Some(NWSProduct::SDO),
            "SDS" => Some(NWSProduct::SDS),
            "SEL" => Some(NWSProduct::SEL),
            "SEV" => Some(NWSProduct::SEV),
            "SFP" => Some(NWSProduct::SFP),
            "SFT" => Some(NWSProduct::SFT),
            "SGL" => Some(NWSProduct::SGL),
            "SHP" => Some(NWSProduct::SHP),
            "SIG" => Some(NWSProduct::SIG),
            "SIM" => Some(NWSProduct::SIM),
            "SLS" => Some(NWSProduct::SLS),
            "SMF" => Some(NWSProduct::SMF),
            "SMW" => Some(NWSProduct::SMW),
            "SOO" => Some(NWSProduct::SOO),
            "SPE" => Some(NWSProduct::SPE),
            "SPF" => Some(NWSProduct::SPF),
            "SPS" => Some(NWSProduct::SPS),
            "SPW" => Some(NWSProduct::SPW),
            "SQW" => Some(NWSProduct::SQW),
            "SRD" => Some(NWSProduct::SRD),
            "SRF" => Some(NWSProduct::SRF),
            "SRG" => Some(NWSProduct::SRG),
            "SSM" => Some(NWSProduct::SSM),
            "STA" => Some(NWSProduct::STA),
            "STD" => Some(NWSProduct::STD),
            "STO" => Some(NWSProduct::STO),
            "STP" => Some(NWSProduct::STP),
            "STQ" => Some(NWSProduct::STQ),
            "SUM" => Some(NWSProduct::SUM),
            "SVR" => Some(NWSProduct::SVR),
            "SVS" => Some(NWSProduct::SVS),
            "SWO" => Some(NWSProduct::SWO),
            "SWS" => Some(NWSProduct::SWS),
            "SYN" => Some(NWSProduct::SYN),
            "TAF" => Some(NWSProduct::TAF),
            "TAP" => Some(NWSProduct::TAP),
            "TAV" => Some(NWSProduct::TAV),
            "TCA" => Some(NWSProduct::TCA),
            "TCD" => Some(NWSProduct::TCD),
            "TCE" => Some(NWSProduct::TCE),
            "TCM" => Some(NWSProduct::TCM),
            "TCP" => Some(NWSProduct::TCP),
            "TCS" => Some(NWSProduct::TCS),
            "TCU" => Some(NWSProduct::TCU),
            "TCV" => Some(NWSProduct::TCV),
            "TIB" => Some(NWSProduct::TIB),
            "TID" => Some(NWSProduct::TID),
            "TMA" => Some(NWSProduct::TMA),
            "TOE" => Some(NWSProduct::TOE),
            "TOR" => Some(NWSProduct::TOR),
            "TPT" => Some(NWSProduct::TPT),
            "TSU" => Some(NWSProduct::TSU),
            "TUV" => Some(NWSProduct::TUV),
            "TVL" => Some(NWSProduct::TVL),
            "TWB" => Some(NWSProduct::TWB),
            "TWD" => Some(NWSProduct::TWD),
            "TWO" => Some(NWSProduct::TWO),
            "TWS" => Some(NWSProduct::TWS),
            "URN" => Some(NWSProduct::URN),
            "UVI" => Some(NWSProduct::UVI),
            "VAA" => Some(NWSProduct::VAA),
            "VER" => Some(NWSProduct::VER),
            "VFT" => Some(NWSProduct::VFT),
            "VOW" => Some(NWSProduct::VOW),
            "WA0" => Some(NWSProduct::WA0),
            "WA1" => Some(NWSProduct::WA1),
            "WA2" => Some(NWSProduct::WA2),
            "WA3" => Some(NWSProduct::WA3),
            "WA4" => Some(NWSProduct::WA4),
            "WA5" => Some(NWSProduct::WA5),
            "WA6" => Some(NWSProduct::WA6),
            "WA7" => Some(NWSProduct::WA7),
            "WA8" => Some(NWSProduct::WA8),
            "WA9" => Some(NWSProduct::WA9),
            "WAR" => Some(NWSProduct::WAR),
            "WAT" => Some(NWSProduct::WAT),
            "WCN" => Some(NWSProduct::WCN),
            "WCR" => Some(NWSProduct::WCR),
            "WDA" => Some(NWSProduct::WDA),
            "WDU" => Some(NWSProduct::WDU),
            "WEK" => Some(NWSProduct::WEK),
            "WOU" => Some(NWSProduct::WOU),
            "WS1" => Some(NWSProduct::WS1),
            "WS2" => Some(NWSProduct::WS2),
            "WS3" => Some(NWSProduct::WS3),
            "WS4" => Some(NWSProduct::WS4),
            "WS5" => Some(NWSProduct::WS5),
            "WS6" => Some(NWSProduct::WS6),
            "WST" => Some(NWSProduct::WST),
            "WSV" => Some(NWSProduct::WSV),
            "WSW" => Some(NWSProduct::WSW),
            "WWA" => Some(NWSProduct::WWA),
            "WWP" => Some(NWSProduct::WWP),
            "ZFP" => Some(NWSProduct::ZFP),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            NWSProduct::ABV => "ABV",
            NWSProduct::ADA => "ADA",
            NWSProduct::ADM => "ADM",
            NWSProduct::ADR => "ADR",
            NWSProduct::ADV => "ADV",
            NWSProduct::AFD => "AFD",
            NWSProduct::AFM => "AFM",
            NWSProduct::AFP => "AFP",
            NWSProduct::AFW => "AFW",
            NWSProduct::AGF => "AGF",
            NWSProduct::AGO => "AGO",
            NWSProduct::ALT => "ALT",
            NWSProduct::AQA => "AQA",
            NWSProduct::AQI => "AQI",
            NWSProduct::ASA => "ASA",
            NWSProduct::AVA => "AVA",
            NWSProduct::AVG => "AVG",
            NWSProduct::AVW => "AVW",
            NWSProduct::AWO => "AWO",
            NWSProduct::AWS => "AWS",
            NWSProduct::AWU => "AWU",
            NWSProduct::AWW => "AWW",
            NWSProduct::BLU => "BLU",
            NWSProduct::BOY => "BOY",
            NWSProduct::BRG => "BRG",
            NWSProduct::BRT => "BRT",
            NWSProduct::CAE => "CAE",
            NWSProduct::CCF => "CCF",
            NWSProduct::CDW => "CDW",
            NWSProduct::CEM => "CEM",
            NWSProduct::CF6 => "CF6",
            NWSProduct::CFP => "CFP",
            NWSProduct::CFW => "CFW",
            NWSProduct::CGR => "CGR",
            NWSProduct::CHG => "CHG",
            NWSProduct::CLA => "CLA",
            NWSProduct::CLI => "CLI",
            NWSProduct::CLM => "CLM",
            NWSProduct::CLQ => "CLQ",
            NWSProduct::CLS => "CLS",
            NWSProduct::CLT => "CLT",
            NWSProduct::CMM => "CMM",
            NWSProduct::COD => "COD",
            NWSProduct::CPF => "CPF",
            NWSProduct::CUR => "CUR",
            NWSProduct::CWA => "CWA",
            NWSProduct::CWF => "CWF",
            NWSProduct::CWS => "CWS",
            NWSProduct::DAY => "DAY",
            NWSProduct::DDO => "DDO",
            NWSProduct::DGT => "DGT",
            NWSProduct::DMO => "DMO",
            NWSProduct::DSA => "DSA",
            NWSProduct::DSM => "DSM",
            NWSProduct::DSW => "DSW",
            NWSProduct::EFP => "EFP",
            NWSProduct::EOL => "EOL",
            NWSProduct::EQI => "EQI",
            NWSProduct::EQR => "EQR",
            NWSProduct::EQW => "EQW",
            NWSProduct::ESF => "ESF",
            NWSProduct::ESG => "ESG",
            NWSProduct::ESP => "ESP",
            NWSProduct::ESS => "ESS",
            NWSProduct::EVI => "EVI",
            NWSProduct::EWW => "EWW",
            NWSProduct::FA0 => "FA0",
            NWSProduct::FA1 => "FA1",
            NWSProduct::FA2 => "FA2",
            NWSProduct::FA3 => "FA3",
            NWSProduct::FA4 => "FA4",
            NWSProduct::FA5 => "FA5",
            NWSProduct::FA6 => "FA6",
            NWSProduct::FA7 => "FA7",
            NWSProduct::FA8 => "FA8",
            NWSProduct::FA9 => "FA9",
            NWSProduct::FD0 => "FD0",
            NWSProduct::FD1 => "FD1",
            NWSProduct::FD2 => "FD2",
            NWSProduct::FD3 => "FD3",
            NWSProduct::FD4 => "FD4",
            NWSProduct::FD5 => "FD5",
            NWSProduct::FD6 => "FD6",
            NWSProduct::FD7 => "FD7",
            NWSProduct::FD8 => "FD8",
            NWSProduct::FD9 => "FD9",
            NWSProduct::FDI => "FDI",
            NWSProduct::FFA => "FFA",
            NWSProduct::FFG => "FFG",
            NWSProduct::FFH => "FFH",
            NWSProduct::FFS => "FFS",
            NWSProduct::FFW => "FFW",
            NWSProduct::FLN => "FLN",
            NWSProduct::FLS => "FLS",
            NWSProduct::FLW => "FLW",
            NWSProduct::FOF => "FOF",
            NWSProduct::FRW => "FRW",
            NWSProduct::FSH => "FSH",
            NWSProduct::FTM => "FTM",
            NWSProduct::FTP => "FTP",
            NWSProduct::FWA => "FWA",
            NWSProduct::FWD => "FWD",
            NWSProduct::FWF => "FWF",
            NWSProduct::FWL => "FWL",
            NWSProduct::FWM => "FWM",
            NWSProduct::FWN => "FWN",
            NWSProduct::FWO => "FWO",
            NWSProduct::FWS => "FWS",
            NWSProduct::FZL => "FZL",
            NWSProduct::GLF => "GLF",
            NWSProduct::GLS => "GLS",
            NWSProduct::GRE => "GRE",
            NWSProduct::HD1 => "HD1",
            NWSProduct::HD2 => "HD2",
            NWSProduct::HD3 => "HD3",
            NWSProduct::HD4 => "HD4",
            NWSProduct::HD7 => "HD7",
            NWSProduct::HD8 => "HD8",
            NWSProduct::HD9 => "HD9",
            NWSProduct::HLS => "HLS",
            NWSProduct::HMD => "HMD",
            NWSProduct::HML => "HML",
            NWSProduct::HMW => "HMW",
            NWSProduct::HP1 => "HP1",
            NWSProduct::HP2 => "HP2",
            NWSProduct::HP3 => "HP3",
            NWSProduct::HP4 => "HP4",
            NWSProduct::HP5 => "HP5",
            NWSProduct::HP6 => "HP6",
            NWSProduct::HP7 => "HP7",
            NWSProduct::HP8 => "HP8",
            NWSProduct::HRR => "HRR",
            NWSProduct::HSF => "HSF",
            NWSProduct::HWO => "HWO",
            NWSProduct::HWR => "HWR",
            NWSProduct::HYD => "HYD",
            NWSProduct::HYM => "HYM",
            NWSProduct::ICE => "ICE",
            NWSProduct::IDM => "IDM",
            NWSProduct::INI => "INI",
            NWSProduct::IOB => "IOB",
            NWSProduct::KPA => "KPA",
            NWSProduct::LAE => "LAE",
            NWSProduct::LCD => "LCD",
            NWSProduct::LCO => "LCO",
            NWSProduct::LEW => "LEW",
            NWSProduct::LFP => "LFP",
            NWSProduct::LKE => "LKE",
            NWSProduct::LLS => "LLS",
            NWSProduct::LOW => "LOW",
            NWSProduct::LSR => "LSR",
            NWSProduct::LTG => "LTG",
            NWSProduct::MAN => "MAN",
            NWSProduct::MAP => "MAP",
            NWSProduct::MAW => "MAW",
            NWSProduct::MFM => "MFM",
            NWSProduct::MIM => "MIM",
            NWSProduct::MIS => "MIS",
            NWSProduct::MOB => "MOB",
            NWSProduct::MON => "MON",
            NWSProduct::MRP => "MRP",
            NWSProduct::MSM => "MSM",
            NWSProduct::MTR => "MTR",
            NWSProduct::MTT => "MTT",
            NWSProduct::MVF => "MVF",
            NWSProduct::MWS => "MWS",
            NWSProduct::MWW => "MWW",
            NWSProduct::NOU => "NOU",
            NWSProduct::NOW => "NOW",
            NWSProduct::NOX => "NOX",
            NWSProduct::NPW => "NPW",
            NWSProduct::NSH => "NSH",
            NWSProduct::NUW => "NUW",
            NWSProduct::NWR => "NWR",
            NWSProduct::OAV => "OAV",
            NWSProduct::OBS => "OBS",
            NWSProduct::OFA => "OFA",
            NWSProduct::OFF => "OFF",
            NWSProduct::OMR => "OMR",
            NWSProduct::OPU => "OPU",
            NWSProduct::OSO => "OSO",
            NWSProduct::OSW => "OSW",
            NWSProduct::OUA => "OUA",
            NWSProduct::OZF => "OZF",
            NWSProduct::PFM => "PFM",
            NWSProduct::PFW => "PFW",
            NWSProduct::PLS => "PLS",
            NWSProduct::PMD => "PMD",
            NWSProduct::PNS => "PNS",
            NWSProduct::POE => "POE",
            NWSProduct::PRB => "PRB",
            NWSProduct::PRC => "PRC",
            NWSProduct::PRE => "PRE",
            NWSProduct::PSH => "PSH",
            NWSProduct::PTS => "PTS",
            NWSProduct::PWO => "PWO",
            NWSProduct::PWS => "PWS",
            NWSProduct::QPF => "QPF",
            NWSProduct::QPS => "QPS",
            NWSProduct::RDF => "RDF",
            NWSProduct::REC => "REC",
            NWSProduct::RER => "RER",
            NWSProduct::RET => "RET",
            NWSProduct::RFD => "RFD",
            NWSProduct::RFI => "RFI",
            NWSProduct::RFR => "RFR",
            NWSProduct::RFW => "RFW",
            NWSProduct::RHW => "RHW",
            NWSProduct::RMT => "RMT",
            NWSProduct::RNS => "RNS",
            NWSProduct::RR1 => "RR1",
            NWSProduct::RR2 => "RR2",
            NWSProduct::RR3 => "RR3",
            NWSProduct::RR4 => "RR4",
            NWSProduct::RR5 => "RR5",
            NWSProduct::RR6 => "RR6",
            NWSProduct::RR7 => "RR7",
            NWSProduct::RR8 => "RR8",
            NWSProduct::RR9 => "RR9",
            NWSProduct::RRA => "RRA",
            NWSProduct::RRM => "RRM",
            NWSProduct::RRS => "RRS",
            NWSProduct::RRY => "RRY",
            NWSProduct::RSD => "RSD",
            NWSProduct::RSM => "RSM",
            NWSProduct::RTP => "RTP",
            NWSProduct::RVA => "RVA",
            NWSProduct::RVD => "RVD",
            NWSProduct::RVF => "RVF",
            NWSProduct::RVI => "RVI",
            NWSProduct::RVM => "RVM",
            NWSProduct::RVR => "RVR",
            NWSProduct::RVS => "RVS",
            NWSProduct::RWR => "RWR",
            NWSProduct::RWS => "RWS",
            NWSProduct::RWT => "RWT",
            NWSProduct::SAB => "SAB",
            NWSProduct::SAF => "SAF",
            NWSProduct::SAG => "SAG",
            NWSProduct::SAT => "SAT",
            NWSProduct::SAW => "SAW",
            NWSProduct::SCC => "SCC",
            NWSProduct::SCD => "SCD",
            NWSProduct::SCN => "SCN",
            NWSProduct::SCP => "SCP",
            NWSProduct::SCS => "SCS",
            NWSProduct::SDO => "SDO",
            NWSProduct::SDS => "SDS",
            NWSProduct::SEL => "SEL",
            NWSProduct::SEV => "SEV",
            NWSProduct::SFP => "SFP",
            NWSProduct::SFT => "SFT",
            NWSProduct::SGL => "SGL",
            NWSProduct::SHP => "SHP",
            NWSProduct::SIG => "SIG",
            NWSProduct::SIM => "SIM",
            NWSProduct::SLS => "SLS",
            NWSProduct::SMF => "SMF",
            NWSProduct::SMW => "SMW",
            NWSProduct::SOO => "SOO",
            NWSProduct::SPE => "SPE",
            NWSProduct::SPF => "SPF",
            NWSProduct::SPS => "SPS",
            NWSProduct::SPW => "SPW",
            NWSProduct::SQW => "SQW",
            NWSProduct::SRD => "SRD",
            NWSProduct::SRF => "SRF",
            NWSProduct::SRG => "SRG",
            NWSProduct::SSM => "SSM",
            NWSProduct::STA => "STA",
            NWSProduct::STD => "STD",
            NWSProduct::STO => "STO",
            NWSProduct::STP => "STP",
            NWSProduct::STQ => "STQ",
            NWSProduct::SUM => "SUM",
            NWSProduct::SVR => "SVR",
            NWSProduct::SVS => "SVS",
            NWSProduct::SWO => "SWO",
            NWSProduct::SWS => "SWS",
            NWSProduct::SYN => "SYN",
            NWSProduct::TAF => "TAF",
            NWSProduct::TAP => "TAP",
            NWSProduct::TAV => "TAV",
            NWSProduct::TCA => "TCA",
            NWSProduct::TCD => "TCD",
            NWSProduct::TCE => "TCE",
            NWSProduct::TCM => "TCM",
            NWSProduct::TCP => "TCP",
            NWSProduct::TCS => "TCS",
            NWSProduct::TCU => "TCU",
            NWSProduct::TCV => "TCV",
            NWSProduct::TIB => "TIB",
            NWSProduct::TID => "TID",
            NWSProduct::TMA => "TMA",
            NWSProduct::TOE => "TOE",
            NWSProduct::TOR => "TOR",
            NWSProduct::TPT => "TPT",
            NWSProduct::TSU => "TSU",
            NWSProduct::TUV => "TUV",
            NWSProduct::TVL => "TVL",
            NWSProduct::TWB => "TWB",
            NWSProduct::TWD => "TWD",
            NWSProduct::TWO => "TWO",
            NWSProduct::TWS => "TWS",
            NWSProduct::URN => "URN",
            NWSProduct::UVI => "UVI",
            NWSProduct::VAA => "VAA",
            NWSProduct::VER => "VER",
            NWSProduct::VFT => "VFT",
            NWSProduct::VOW => "VOW",
            NWSProduct::WA0 => "WA0",
            NWSProduct::WA1 => "WA1",
            NWSProduct::WA2 => "WA2",
            NWSProduct::WA3 => "WA3",
            NWSProduct::WA4 => "WA4",
            NWSProduct::WA5 => "WA5",
            NWSProduct::WA6 => "WA6",
            NWSProduct::WA7 => "WA7",
            NWSProduct::WA8 => "WA8",
            NWSProduct::WA9 => "WA9",
            NWSProduct::WAR => "WAR",
            NWSProduct::WAT => "WAT",
            NWSProduct::WCN => "WCN",
            NWSProduct::WCR => "WCR",
            NWSProduct::WDA => "WDA",
            NWSProduct::WDU => "WDU",
            NWSProduct::WEK => "WEK",
            NWSProduct::WOU => "WOU",
            NWSProduct::WS1 => "WS1",
            NWSProduct::WS2 => "WS2",
            NWSProduct::WS3 => "WS3",
            NWSProduct::WS4 => "WS4",
            NWSProduct::WS5 => "WS5",
            NWSProduct::WS6 => "WS6",
            NWSProduct::WST => "WST",
            NWSProduct::WSV => "WSV",
            NWSProduct::WSW => "WSW",
            NWSProduct::WWA => "WWA",
            NWSProduct::WWP => "WWP",
            NWSProduct::ZFP => "ZFP",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            NWSProduct::ABV => "Rawinsonde Data Above 100 Millibars",
            NWSProduct::ADA => "Alarm/Alert Administrative Msg",
            NWSProduct::ADM => "Alert Administrative Message",
            NWSProduct::ADR => "NWS Administrative Message",
            NWSProduct::ADV => "Generic Space Environment Advisory",
            NWSProduct::AFD => "Area Forecast Discussion",
            NWSProduct::AFM => "Area Forecast Matrices",
            NWSProduct::AFP => "Area Forecast Product",
            NWSProduct::AFW => "Fire Weather Matrix",
            NWSProduct::AGF => "Agricultural Forecast",
            NWSProduct::AGO => "Agricultural Observations",
            NWSProduct::ALT => "Space Environment Alert",
            NWSProduct::AQA => "Air Quality Alert",
            NWSProduct::AQI => "Air Quality Index Statement",
            NWSProduct::ASA => "Air Stagnation Advisory",
            NWSProduct::AVA => "Avalanche Watch",
            NWSProduct::AVG => "Avalanche Weather Guidance",
            NWSProduct::AVW => "Avalanche Warning",
            NWSProduct::AWO => "Area Weather Outlook",
            NWSProduct::AWS => "Area Weather Summary",
            NWSProduct::AWU => "Area Weather Update",
            NWSProduct::AWW => "Airport Weather Warning",
            NWSProduct::BLU => "Blue Alert",
            NWSProduct::BOY => "Buoy Report",
            NWSProduct::BRG => "Coast Guard Observations",
            NWSProduct::BRT => "Hourly Roundup for Weather Radio",
            NWSProduct::CAE => "Child Abduction Emergency",
            NWSProduct::CCF => "Coded City Forecast",
            NWSProduct::CDW => "Civil Danger Warning",
            NWSProduct::CEM => "Civil Emergency Message",
            NWSProduct::CF6 => "WFO Monthly/Daily Climate Data",
            NWSProduct::CFP => "Convective Forecast Product",
            NWSProduct::CFW => "Coastal Flood Warnings/Watches/Statements",
            NWSProduct::CGR => "Coast Guard Surface Report",
            NWSProduct::CHG => "Computer Hurricane Guidance",
            NWSProduct::CLA => "Climatological Report (Annual)",
            NWSProduct::CLI => "Climatological Report (Daily)",
            NWSProduct::CLM => "Climatological Report (Monthly)",
            NWSProduct::CLQ => "Climatological Report (Quarterly)",
            NWSProduct::CLS => "Climatological Report (Seasonal)",
            NWSProduct::CLT => "Climate Report",
            NWSProduct::CMM => "Coded Climatological Monthly Means",
            NWSProduct::COD => "Coded Analysis and Forecasts",
            NWSProduct::CPF => "Great Lakes Port Forecast",
            NWSProduct::CUR => "Routine Space Environment Products",
            NWSProduct::CWA => "Center (CWSU) Weather Advisory",
            NWSProduct::CWF => "Coastal Waters Forecast",
            NWSProduct::CWS => "Center (CWSU) Weather Statement",
            NWSProduct::DAY => "Routine Space Environment Product (Daily)",
            NWSProduct::DDO => "Daily Dispersion Outlook",
            NWSProduct::DGT => "Drought Information Statement",
            NWSProduct::DMO => "Practice/Demo Warning",
            NWSProduct::DSA => "Unnumbered Depression / Suspicious Area Advisory",
            NWSProduct::DSM => "ASOS Daily Summary",
            NWSProduct::DSW => "Dust Storm Warning and Dust Advisory",
            NWSProduct::EFP => "3 To 5 Day Extended Forecast",
            NWSProduct::EOL => "Average 6 To 10 Day Weather Outlook (Local)",
            NWSProduct::EQI => "Tsunami Bulletin",
            NWSProduct::EQR => "Earthquake Report",
            NWSProduct::EQW => "Earthquake Warning",
            NWSProduct::ESF => "Flood Potential Outlook",
            NWSProduct::ESG => "Extended Streamflow Guidance",
            NWSProduct::ESP => "Extended Streamflow Prediction",
            NWSProduct::ESS => "Water Supply Outlook",
            NWSProduct::EVI => "Evacuation Immediate",
            NWSProduct::EWW => "Extreme Wind Warning",
            NWSProduct::FA0 => "Aviation Area Forecasts (Pacific)",
            NWSProduct::FA1 => "Aviation Area Forecasts (Northeast)",
            NWSProduct::FA2 => "Aviation Area Forecasts (Southeast)",
            NWSProduct::FA3 => "Aviation Area Forecasts (North Central)",
            NWSProduct::FA4 => "Aviation Area Forecasts (South Central)",
            NWSProduct::FA5 => "Aviation Area Forecasts (Rocky Mountains)",
            NWSProduct::FA6 => "Aviation Area Forecasts (West Coast)",
            NWSProduct::FA7 => "Aviation Area Forecasts (Juneau, AK)",
            NWSProduct::FA8 => "Aviation Area Forecasts (Anchorage, AK)",
            NWSProduct::FA9 => "Aviation Area Forecasts (Fairbanks, AK)",
            NWSProduct::FD0 => "24 Hr Fd Winds Aloft Fcst (45,000 and 53,000 Ft)",
            NWSProduct::FD1 => "6 Hour Winds Aloft Forecast",
            NWSProduct::FD2 => "12 Hour Winds Aloft Forecast",
            NWSProduct::FD3 => "24 Hour Winds Aloft Forecast",
            NWSProduct::FD4 => "Winds Aloft Forecast",
            NWSProduct::FD5 => "Winds Aloft Forecast",
            NWSProduct::FD6 => "Winds Aloft Forecast",
            NWSProduct::FD7 => "Winds Aloft Forecast",
            NWSProduct::FD8 => "6 Hour Fd Winds Aloft Fcst (45,000 and 53,000 Ft)",
            NWSProduct::FD9 => "12 Hr Fd Winds Aloft Fcst (45,000 and 53,000 Ft)",
            NWSProduct::FDI => "Fire Danger Indices",
            NWSProduct::FFA => "Flash Flood Watch",
            NWSProduct::FFG => "Flash Flood Guidance",
            NWSProduct::FFH => "Headwater Guidance",
            NWSProduct::FFS => "Flash Flood Statement",
            NWSProduct::FFW => "Flash Flood Warning",
            NWSProduct::FLN => "National Flood Summary",
            NWSProduct::FLS => "Flood Statement",
            NWSProduct::FLW => "Flood Warning",
            NWSProduct::FOF => "Upper Wind Fallout Forecast",
            NWSProduct::FRW => "Fire Warning",
            NWSProduct::FSH => "Natl Marine Fisheries Administrative Service Message",
            NWSProduct::FTM => "WSR-88D Radar Outage Notification / Free Text Message",
            NWSProduct::FTP => "FOUS Prog Max/Min Temp/Pop Guidance",
            NWSProduct::FWA => "Fire Weather Administrative Message",
            NWSProduct::FWD => "Fire Weather Outlook Discussion",
            NWSProduct::FWF => "Routine Fire Wx Fcst (With/Without 6-10 Day Outlook)",
            NWSProduct::FWL => "Land Management Forecasts",
            NWSProduct::FWM => "Miscellaneous Fire Weather Product",
            NWSProduct::FWN => "Fire Weather Notification",
            NWSProduct::FWO => "Fire Weather Observation",
            NWSProduct::FWS => "Suppression Forecast",
            NWSProduct::FZL => "Freezing Level Data (RADAT)",
            NWSProduct::GLF => "Great Lakes Forecast",
            NWSProduct::GLS => "Great Lakes Storm Summary",
            NWSProduct::GRE => "GREEN",
            NWSProduct::HD1 => "RFC Derived QPF Data Product",
            NWSProduct::HD2 => "RFC Derived QPF Data Product",
            NWSProduct::HD3 => "RFC Derived QPF Data Product",
            NWSProduct::HD4 => "RFC Derived QPF Data Product",
            NWSProduct::HD7 => "RFC Derived QPF Data Product",
            NWSProduct::HD8 => "RFC Derived QPF Data Product",
            NWSProduct::HD9 => "RFC Derived QPF Data Product",
            NWSProduct::HLS => "Hurricane Local Statement",
            NWSProduct::HMD => "Hydrometeorological Discussion",
            NWSProduct::HML => "AHPS XML",
            NWSProduct::HMW => "Hazardous Materials Warning",
            NWSProduct::HP1 => "RFC QPF Verification Product",
            NWSProduct::HP2 => "RFC QPF Verification Product",
            NWSProduct::HP3 => "RFC QPF Verification Product",
            NWSProduct::HP4 => "RFC QPF Verification Product",
            NWSProduct::HP5 => "RFC QPF Verification Product",
            NWSProduct::HP6 => "RFC QPF Verification Product",
            NWSProduct::HP7 => "RFC QPF Verification Product",
            NWSProduct::HP8 => "RFC QPF Verification Product",
            NWSProduct::HRR => "Weather Roundup",
            NWSProduct::HSF => "High Seas Forecast",
            NWSProduct::HWO => "Hazardous Weather Outlook",
            NWSProduct::HWR => "Hourly Weather Roundup",
            NWSProduct::HYD => "Daily Hydrometeorological Products",
            NWSProduct::HYM => "Monthly Hydrometeorological Plain Language Product",
            NWSProduct::ICE => "Ice Forecast",
            NWSProduct::IDM => "Ice Drift Vectors",
            NWSProduct::INI => "ADMINISTR [NOUS51 KWBC]",
            NWSProduct::IOB => "Ice Observation",
            NWSProduct::KPA => "Keep Alive Message",
            NWSProduct::LAE => "Local Area Emergency",
            NWSProduct::LCD => "Preliminary Local Climatological Data",
            NWSProduct::LCO => "Local Cooperative Observation",
            NWSProduct::LEW => "Law Enforcement Warning",
            NWSProduct::LFP => "Local Forecast",
            NWSProduct::LKE => "Lake Stages",
            NWSProduct::LLS => "Low-Level Sounding",
            NWSProduct::LOW => "Low Temperatures",
            NWSProduct::LSR => "Local Storm Report",
            NWSProduct::LTG => "Lightning Data",
            NWSProduct::MAN => "Rawinsonde Observation Mandatory Levels",
            NWSProduct::MAP => "Mean Areal Precipitation",
            NWSProduct::MAW => "Amended Marine Forecast",
            NWSProduct::MFM => "Marine Forecast Matrix",
            NWSProduct::MIM => "Marine Interpretation Message",
            NWSProduct::MIS => "Miscellaneous Local Product",
            NWSProduct::MOB => "MOB Observations",
            NWSProduct::MON => "Routine Space Environment Product Issued Monthly",
            NWSProduct::MRP => "Techniques Development Laboratory Marine Product",
            NWSProduct::MSM => "ASOS Monthly Summary Message",
            NWSProduct::MTR => "METAR Formatted Surface Weather Observation",
            NWSProduct::MTT => "METAR Test Message",
            NWSProduct::MVF => "Marine Verification Coded Message",
            NWSProduct::MWS => "Marine Weather Statement",
            NWSProduct::MWW => "Marine Weather Message",
            NWSProduct::NOU => "Weather Reconnaisance Flights",
            NWSProduct::NOW => "Short Term Forecast",
            NWSProduct::NOX => "Data Mgt Message",
            NWSProduct::NPW => "Non-Precipitation Warnings / Watches / Advisories",
            NWSProduct::NSH => "Nearshore Marine Forecast",
            NWSProduct::NUW => "Nuclear Power Plant Warning",
            NWSProduct::NWR => "NOAA Weather Radio Forecast",
            NWSProduct::OAV => "Other Aviation Products",
            NWSProduct::OBS => "Observations",
            NWSProduct::OFA => "Offshore Aviation Area Forecast",
            NWSProduct::OFF => "Offshore Forecast",
            NWSProduct::OMR => "Other Marine Products",
            NWSProduct::OPU => "Other Public Products",
            NWSProduct::OSO => "Other Surface Observations",
            NWSProduct::OSW => "Ocean Surface Winds",
            NWSProduct::OUA => "Other Upper Air Data",
            NWSProduct::OZF => "Zone Forecast",
            NWSProduct::PFM => "Point Forecast Matrices",
            NWSProduct::PFW => "Fire Weather Point Forecast Matrices",
            NWSProduct::PLS => "Plain Language Ship Report",
            NWSProduct::PMD => "Prognostic Meteorological Discussion",
            NWSProduct::PNS => "Public Information Statement",
            NWSProduct::POE => "Probability of Exceed",
            NWSProduct::PRB => "Heat Index Forecast Tables",
            NWSProduct::PRC => "State Pilot Report Collective",
            NWSProduct::PRE => "Preliminary Forecasts",
            NWSProduct::PSH => "Post Storm Hurricane Report",
            NWSProduct::PTS => "Probabilistic Outlook Points",
            NWSProduct::PWO => "Public Severe Weather Outlook",
            NWSProduct::PWS => "Tropical Cyclone Probabilities",
            NWSProduct::QPF => "Quantitative Precipitation Forecast",
            NWSProduct::QPS => "Quantitative Precipitation Statement",
            NWSProduct::RDF => "Revised Digital Forecast",
            NWSProduct::REC => "Recreational Report",
            NWSProduct::RER => "Record Report",
            NWSProduct::RET => "EAS Activation Request",
            NWSProduct::RFD => "Rangeland Fire Danger Forecast",
            NWSProduct::RFI => "RFI Observation",
            NWSProduct::RFR => "Route Forecast",
            NWSProduct::RFW => "Red Flag Warning",
            NWSProduct::RHW => "Radiological Hazard Warning",
            NWSProduct::RMT => "Required Monthly Test",
            NWSProduct::RNS => "Rain Information Statement",
            NWSProduct::RR1 => "Hydro-Met Data Report Part 1",
            NWSProduct::RR2 => "Hydro-Met Data Report Part 2",
            NWSProduct::RR3 => "Hydro-Met Data Report Part 3",
            NWSProduct::RR4 => "Hydro-Met Data Report Part 4",
            NWSProduct::RR5 => "Hydro-Met Data Report Part 5",
            NWSProduct::RR6 => "Hydro-Met Data Report Part 6",
            NWSProduct::RR7 => "Hydro-Met Data Report Part 7",
            NWSProduct::RR8 => "Hydro-Met Data Report Part 8",
            NWSProduct::RR9 => "Hydro-Met Data Report Part 9",
            NWSProduct::RRA => "Automated Hydrologic Observation Sta Report (AHOS)",
            NWSProduct::RRM => "Miscellaneous Hydrologic Data",
            NWSProduct::RRS => "HADS Data",
            NWSProduct::RRY => "ASOS SHEF Hourly Routine Test Message",
            NWSProduct::RSD => "Daily Snotel Data",
            NWSProduct::RSM => "Monthly Snotel Data",
            NWSProduct::RTP => "Regional Max/Min Temp and Precipitation Table",
            NWSProduct::RVA => "River Summary",
            NWSProduct::RVD => "Daily River Forecasts",
            NWSProduct::RVF => "River Forecast",
            NWSProduct::RVI => "River Ice Statement",
            NWSProduct::RVM => "Miscellaneous River Product",
            NWSProduct::RVR => "River Recreation Statement",
            NWSProduct::RVS => "River Statement",
            NWSProduct::RWR => "Regional Weather Roundup",
            NWSProduct::RWS => "Regional Weather Summary",
            NWSProduct::RWT => "Required Weekly Test",
            NWSProduct::SAB => "Special Avalanche Bulletin",
            NWSProduct::SAF => "Speci Agri Wx Fcst / Advisory / Flying Farmer Fcst Outlook",
            NWSProduct::SAG => "Snow Avalanche Guidance",
            NWSProduct::SAT => "APT Prediction",
            NWSProduct::SAW => "Prelim Notice of Watch & Cancellation Msg (Aviation)",
            NWSProduct::SCC => "Storm Summary",
            NWSProduct::SCD => "Supplementary Climatological Data (ASOS)",
            NWSProduct::SCN => "Soil Climate Analysis Network Data",
            NWSProduct::SCP => "Satellite Cloud Product",
            NWSProduct::SCS => "Selected Cities Summary",
            NWSProduct::SDO => "Supplementary Data Observation (ASOS)",
            NWSProduct::SDS => "Special Dispersion Statement",
            NWSProduct::SEL => "Severe Local Storm Watch and Watch Cancellation Msg",
            NWSProduct::SEV => "SPC Watch Point Information Message",
            NWSProduct::SFP => "State Forecast",
            NWSProduct::SFT => "Tabular State Forecast",
            NWSProduct::SGL => "Rawinsonde Observation Significant Levels",
            NWSProduct::SHP => "Surface Ship Report at Synoptic Time",
            NWSProduct::SIG => "International Sigmet / Convective Sigmet",
            NWSProduct::SIM => "Satellite Interpretation Message",
            NWSProduct::SLS => "Severe Local Storm Watch and Areal Outline",
            NWSProduct::SMF => "Smoke Management Weather Forecast",
            NWSProduct::SMW => "Special Marine Warning",
            NWSProduct::SOO => "SOO Product",
            NWSProduct::SPE => "Satellite Precipitation Estimates (TXUS20 KWBC)",
            NWSProduct::SPF => "Storm Strike Probability Bulletin (TPC)",
            NWSProduct::SPS => "Special Weather Statement",
            NWSProduct::SPW => "Shelter in Place Warning",
            NWSProduct::SQW => "Snow Squall Warning",
            NWSProduct::SRD => "Surf Discussion",
            NWSProduct::SRF => "Surf Forecast",
            NWSProduct::SRG => "Soaring Guidance",
            NWSProduct::SSM => "Main Synoptic Hour Surface Observation",
            NWSProduct::STA => "Network and Severe Weather Statistical Summaries",
            NWSProduct::STD => "Satellite Tropical Disturbance Summary",
            NWSProduct::STO => "Road Condition Reports (State Agencies)",
            NWSProduct::STP => "State Max/Min Temperature and Precipitation Table",
            NWSProduct::STQ => "Spot Forecast Request",
            NWSProduct::SUM => "Space Weather Message",
            NWSProduct::SVR => "Severe Thunderstorm Warning",
            NWSProduct::SVS => "Severe Weather Statement",
            NWSProduct::SWO => "Severe Storm Outlook Narrative (AC)",
            NWSProduct::SWS => "State Weather Summary",
            NWSProduct::SYN => "Regional Weather Synopsis",
            NWSProduct::TAF => "Terminal Aerodrome Forecast",
            NWSProduct::TAP => "Terminal Alerting Products",
            NWSProduct::TAV => "Travelers Forecast Table",
            NWSProduct::TCA => "Aviation Tropical Cyclone Advisory",
            NWSProduct::TCD => "Tropical Cyclone Discussion",
            NWSProduct::TCE => "Tropical Cyclone Position Estimate",
            NWSProduct::TCM => "Marine/Aviation Tropical Cyclone Advisory",
            NWSProduct::TCP => "Public Tropical Cyclone Advisory",
            NWSProduct::TCS => "Satellite Tropical Cyclone Summary",
            NWSProduct::TCU => "Tropical Cyclone Update",
            NWSProduct::TCV => "Tropical Cyclone Watch/Warning Break Points",
            NWSProduct::TIB => "Tsunami Bulletin",
            NWSProduct::TID => "Tide Report",
            NWSProduct::TMA => "Tsunami Tide/Seismic Message Acknowledgement",
            NWSProduct::TOE => "911 Telephone Outage Emergency",
            NWSProduct::TOR => "Tornado Warning",
            NWSProduct::TPT => "Temperature Precipitation Table (Natl and Intnl)",
            NWSProduct::TSU => "Tsunami Watch/Warning",
            NWSProduct::TUV => "Weather Bulletin",
            NWSProduct::TVL => "Travelers Forecast",
            NWSProduct::TWB => "Transcribed Weather Broadcast",
            NWSProduct::TWD => "Tropical Weather Discussion",
            NWSProduct::TWO => "Tropical Weather Outlook and Summary",
            NWSProduct::TWS => "Tropical Weather Summary",
            NWSProduct::URN => "Aircraft Reconnaissance",
            NWSProduct::UVI => "Ultraviolet Index",
            NWSProduct::VAA => "Volcanic Activity Advisory",
            NWSProduct::VER => "Forecast Verification Statistics",
            NWSProduct::VFT => "Terminal Aerodrome Forecast (TAF) Verification",
            NWSProduct::VOW => "Volcano Warning",
            NWSProduct::WA0 => "Airmet (Pacific)",
            NWSProduct::WA1 => "Airmet (Northeast)",
            NWSProduct::WA2 => "Airmet (Southeast)",
            NWSProduct::WA3 => "Airmet (North Central)",
            NWSProduct::WA4 => "Airmet (South Central)",
            NWSProduct::WA5 => "Airmet (Rocky Mountains)",
            NWSProduct::WA6 => "Airmet (West Coast)",
            NWSProduct::WA7 => "Airmet (Juneau, AK)",
            NWSProduct::WA8 => "Airmet (Anchorage, AK)",
            NWSProduct::WA9 => "Airmet (Fairbanks, AK)",
            NWSProduct::WAR => "Space Environment Warning",
            NWSProduct::WAT => "Space Environment Watch",
            NWSProduct::WCN => "Weather Watch Clearance Notification",
            NWSProduct::WCR => "Weekly Weather and Crop Report",
            NWSProduct::WDA => "Weekly Data for Agriculture",
            NWSProduct::WDU => "Warning Decision Update",
            NWSProduct::WEK => "Routine Space Environment Product Issued Weekly",
            NWSProduct::WOU => "Tornado/Severe Thunderstorm Watch",
            NWSProduct::WS1 => "Sigmet (Northeast)",
            NWSProduct::WS2 => "Sigmet (Southeast)",
            NWSProduct::WS3 => "Sigmet (North Central)",
            NWSProduct::WS4 => "Sigmet (South Central)",
            NWSProduct::WS5 => "Sigmet (Rocky Mountains)",
            NWSProduct::WS6 => "Sigmet (West Coast)",
            NWSProduct::WST => "Tropical Cyclone Sigmet",
            NWSProduct::WSV => "Volcanic Activity Sigmet",
            NWSProduct::WSW => "Winter Weather Warnings / Watches / Advisories",
            NWSProduct::WWA => "Watch Status Report",
            NWSProduct::WWP => "Severe Thunderstorm / Tornado Watch Probabilities",
            NWSProduct::ZFP => "Zone Forecast Product",
        }
    }
}
