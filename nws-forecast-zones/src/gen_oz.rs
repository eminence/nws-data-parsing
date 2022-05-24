#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OffshoreMarineZone {
    /// Caribbean Nof 18N W of 85W including Yucatan Basin, AM
    #[doc(hidden)]
    AMZ040,
    /// Caribbean N of 20N E of 85W, AM
    #[doc(hidden)]
    AMZ041,
    /// Caribbean from 18N-20N between 80W-85W including Cayman Basin, AM
    #[doc(hidden)]
    AMZ042,
    /// Caribbean from 18N-20N between 76W-80W, AM
    #[doc(hidden)]
    AMZ043,
    /// Caribbean approaches to the Windward Passage, AM
    #[doc(hidden)]
    AMZ044,
    /// Gulf of Honduras, AM
    #[doc(hidden)]
    AMZ045,
    /// Caribbean from 15N to 18N between 80W and 85W, AM
    #[doc(hidden)]
    AMZ046,
    /// Caribbean from 15N to 18N between 76W and 80W, AM
    #[doc(hidden)]
    AMZ047,
    /// Caribbean from 15N to 18N between 72W and 76W, AM
    #[doc(hidden)]
    AMZ048,
    /// Caribbean N of 15N between 68W and 72W, AM
    #[doc(hidden)]
    AMZ049,
    /// Caribbean N of 15N between 64W and 68W, AM
    #[doc(hidden)]
    AMZ050,
    /// Offshore Waters Leeward Islands, AM
    #[doc(hidden)]
    AMZ051,
    /// Tropical N Atlantic from 15N to 19N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ052,
    /// W Central Caribbean from 11N to 15N W of 80W, AM
    #[doc(hidden)]
    AMZ053,
    /// Caribbean from 11N to 15N between 76W and 80W including Colombia Basin, AM
    #[doc(hidden)]
    AMZ054,
    /// Caribbean from 11N to 15N between 72W and 76W, AM
    #[doc(hidden)]
    AMZ055,
    /// Caribbean S of 15N between 68W and 72W, AM
    #[doc(hidden)]
    AMZ056,
    /// Caribbean S of 15N between 64W and 68W including Venezuela Basin, AM
    #[doc(hidden)]
    AMZ057,
    /// Offshore Waters Windward Islands including Trinidad and Tobago, AM
    #[doc(hidden)]
    AMZ058,
    /// Tropical N Atlantic from 11N and 15N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ059,
    /// SW Caribbean S of 11N W of 80W, AM
    #[doc(hidden)]
    AMZ060,
    /// SW Caribbean S of 11N E of 80W including the approaches to the Panama Canal, AM
    #[doc(hidden)]
    AMZ061,
    /// Tropical N Atlantic from 7N and 11N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ062,
    /// Atlantic from 29N to 31N W of 77W, AM
    #[doc(hidden)]
    AMZ063,
    /// Atlantic from 29N to 31N between 74W and 77W, AM
    #[doc(hidden)]
    AMZ064,
    /// Atlantic from 29N to 31N between 70W and 74W, AM
    #[doc(hidden)]
    AMZ065,
    /// Atlantic from 29N to 31N between 65W and 70W, AM
    #[doc(hidden)]
    AMZ066,
    /// Atlantic from 29N to 31N between 60W and 65W, AM
    #[doc(hidden)]
    AMZ067,
    /// Atlantic from 29N to 31N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ068,
    /// Atlantic from 27N to 29N W of 77W, AM
    #[doc(hidden)]
    AMZ069,
    /// Atlantic from 27N to 29N between 74W and 77W, AM
    #[doc(hidden)]
    AMZ070,
    /// Atlantic from 27N to 29N between 70W and 74W, AM
    #[doc(hidden)]
    AMZ071,
    /// Atlantic from 27N to 29N between 65W and 70W, AM
    #[doc(hidden)]
    AMZ072,
    /// Atlantic from 27N to 29N between 60W and 65W, AM
    #[doc(hidden)]
    AMZ073,
    /// Atlantic from 27N to 29N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ074,
    /// Northern Bahamas from 24N to 27N, AM
    #[doc(hidden)]
    AMZ075,
    /// Atlantic from 22N to 27N E of Bahamas to 70W, AM
    #[doc(hidden)]
    AMZ076,
    /// Atlantic from 22N to 27N between 65W and 70W, AM
    #[doc(hidden)]
    AMZ077,
    /// Atlantic from 25N to 27N between 60W and 65W, AM
    #[doc(hidden)]
    AMZ078,
    /// Atlantic from 25N to 27N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ079,
    /// Central Bahamas including Cay Sal Bank, AM
    #[doc(hidden)]
    AMZ080,
    /// Atlantic from 22N to 25N E of Bahamas to 70W, AM
    #[doc(hidden)]
    AMZ081,
    /// Atlantic from 22N to 25N between 65W and 70W, AM
    #[doc(hidden)]
    AMZ082,
    /// Atlantic from 22N to 25N between 60W and 65W, AM
    #[doc(hidden)]
    AMZ083,
    /// Atlantic from 22N to 25N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ084,
    /// Atlantic S of 22N W of 70W including approaches to the Windward Passage, AM
    #[doc(hidden)]
    AMZ085,
    /// Atlantic S of 22N between 65W and 70W including Puerto Rico Trench, AM
    #[doc(hidden)]
    AMZ086,
    /// Atlantic from 19N to 22N between 60W and 65W, AM
    #[doc(hidden)]
    AMZ087,
    /// Atlantic from 19N to 22N between 55W and 60W, AM
    #[doc(hidden)]
    AMZ088,
    /// Gulf of Maine to the Hague Line, AN
    #[doc(hidden)]
    ANZ800,
    /// Georges Bank between Cape Cod and 68W north of 1000 FM, AN
    #[doc(hidden)]
    ANZ805,
    /// South of New England between the Great South Channel and Montauk Point to 1000 FM, AN
    #[doc(hidden)]
    ANZ810,
    /// South of Long Island between Montauk Point and Sandy Hook to 1000 FM, AN
    #[doc(hidden)]
    ANZ815,
    /// Hudson Canyon to Baltimore Canyon to 1000 FM, AN
    #[doc(hidden)]
    ANZ820,
    /// Baltimore Canyon to Cape Charles Light to 100 NM offshore, AN
    #[doc(hidden)]
    ANZ825,
    /// Cape Charles Light to Currituck Beach Light to 100 NM offshore, AN
    #[doc(hidden)]
    ANZ828,
    /// Currituck Beach Light to Cape Hatteras to 100 NM offshore, AN
    #[doc(hidden)]
    ANZ830,
    /// Cape Hatteras to Cape Fear to 100 NM Offshore., AN
    #[doc(hidden)]
    ANZ833,
    /// Cape Fear to 31N to 1000 FM, AN
    #[doc(hidden)]
    ANZ835,
    /// Georges Bank between 68W and the Hague Line, AN
    #[doc(hidden)]
    ANZ900,
    /// East of 69W to the Hague Line between 1000 FM and 39N, AN
    #[doc(hidden)]
    ANZ905,
    /// East of 69W and south of 39N to 250 NM offshore, AN
    #[doc(hidden)]
    ANZ910,
    /// Between 1000FM and 38.5 N west of 69 W, AN
    #[doc(hidden)]
    ANZ915,
    /// Baltimore Canyon to 69W east of 1000 FM and south of 38.5N to 250 NM offshore, AN
    #[doc(hidden)]
    ANZ920,
    /// Baltimore Canyon to Hatteras Canyon between 100 NM and 250 NM offshore, AN
    #[doc(hidden)]
    ANZ925,
    /// Hatteras Canyon to Cape Fear between 100 NM and 250 NM offshore, AN
    #[doc(hidden)]
    ANZ930,
    /// Cape Fear to 31N east of 1000 FM to 250 NM offshore, AN
    #[doc(hidden)]
    ANZ935,
    /// NW Gulf including Stetson Bank, GM
    #[doc(hidden)]
    GMZ040,
    /// SW Louisiana Offshore Waters including Flower Garden Bank Marine Sanctuary, GM
    #[doc(hidden)]
    GMZ041,
    /// W Central Gulf from 22N to 26N between 91W and 94W, GM
    #[doc(hidden)]
    GMZ045,
    /// Central Gulf from 22N to 26N between 87W and 91W, GM
    #[doc(hidden)]
    GMZ046,
    /// SE Gulf from 22N to 26N E of 87W including Straits of Florida, GM
    #[doc(hidden)]
    GMZ047,
    /// SW Gulf S of 22N W of 94W, GM
    #[doc(hidden)]
    GMZ048,
    /// Central Bay of Campeche, GM
    #[doc(hidden)]
    GMZ049,
    /// E Bay of Campeche including Campeche Bank, GM
    #[doc(hidden)]
    GMZ050,
    /// N Central Gulf Offshore Waters, GM
    #[doc(hidden)]
    GMZ056,
    /// NE Gulf N of 26N E of 87W, GM
    #[doc(hidden)]
    GMZ057,
    /// W Central Gulf from 22N to 26N W of 94W, GM
    #[doc(hidden)]
    GMZ058,
    /// Hawaiian Offshore Waters, PH
    #[doc(hidden)]
    PHZ180,
    /// Gulf of Alaska North of 55 Degrees North and East of 144 W, PK
    #[doc(hidden)]
    PKZ310,
    /// Gulf of Alaska Offshore North of 57N and West of 144W, PK
    #[doc(hidden)]
    PKZ351,
    /// Gulf of Alaska Offshore South of 57N North of 55N and West of 144W, PK
    #[doc(hidden)]
    PKZ352,
    /// Bering Sea Offshore  West of 180 and East of the International Date Line, PK
    #[doc(hidden)]
    PKZ411,
    /// Bering Sea Offshore 171W to 180 and North of 56N, PK
    #[doc(hidden)]
    PKZ412,
    /// Bering Sea Offshore 171W to 180 and South of 56N, PK
    #[doc(hidden)]
    PKZ413,
    /// Bering Sea Offshore East of 171W, PK
    #[doc(hidden)]
    PKZ414,
    /// Western US Arctic Offshore, PK
    #[doc(hidden)]
    PKZ500,
    /// Central US Arctic Offshore, PK
    #[doc(hidden)]
    PKZ505,
    /// Eastern US Arctic Offshore, PK
    #[doc(hidden)]
    PKZ510,
    /// Mexico Border S to 29N to 60 NM offshore, PM
    #[doc(hidden)]
    PMZ009,
    /// Mexico S of 29N to Punta Eugenia to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ011,
    /// Punta Eugenia to Cabo San Lazaro to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ013,
    /// Cabo San Lazaro to Cabo San Lucas to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ015,
    /// Northern Gulf of California, PM
    #[doc(hidden)]
    PMZ017,
    /// Central Gulf of California, PM
    #[doc(hidden)]
    PMZ019,
    /// Southern Gulf of California, PM
    #[doc(hidden)]
    PMZ021,
    /// Entrance to the Gulf of California Including Cabo Corrientes, PM
    #[doc(hidden)]
    PMZ023,
    /// Mexico - Michoachan and Guerrero to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ025,
    /// Mexico - Oaxaca and Chiapas including the Gulf of Tehuantepec, PM
    #[doc(hidden)]
    PMZ027,
    /// Guatemala and El Salvador to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ111,
    /// El Salvador to North Costa Rica Including the Gulfs of Fonseca and Papagayo, PM
    #[doc(hidden)]
    PMZ113,
    /// North Costa Rica to West Panama to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ115,
    /// East Panama and Colombia Including Gulf of Panama, PM
    #[doc(hidden)]
    PMZ117,
    /// Ecuador Including Gulf of Guayaquil to 250 NM offshore, PM
    #[doc(hidden)]
    PMZ119,
    /// Ecuador between 250 and 500 NM Offshore, PM
    #[doc(hidden)]
    PMZ121,
    /// Offshore Galapagos Islands, PM
    #[doc(hidden)]
    PMZ123,
    /// Cape Flattery to Cape Shoalwater between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ800,
    /// Cape Shoalwater to Cape Lookout between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ805,
    /// Cape Lookout to Florence, OR between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ810,
    /// Florence, OR to Point St. George between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ815,
    /// Point St. George to Point Arena between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ820,
    /// Point Arena to Pigeon Point between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ825,
    /// Pigeon Point to Point Piedras Blancas between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ830,
    /// Point Piedras Blancas to Santa Cruz Island, CA between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ835,
    /// Santa Cruz Island, CA to San Clemente Island, CA between 60 NM and 150 NM offshore, PZ
    #[doc(hidden)]
    PZZ840,
    /// Cape Flattery to Cape Shoalwater between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ900,
    /// Cape Shoalwater to Cape Lookout between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ905,
    /// Cape Lookout to Florence, OR between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ910,
    /// Florence, OR to Point St. George between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ915,
    /// Point St. George to Point Arena between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ920,
    /// Point Arena to Pigeon Point between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ925,
    /// Pigeon Point to Point Piedras Blancas between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ930,
    /// Point Piedras Blancas to Santa Cruz Island, CA between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ935,
    /// Santa Cruz Island, CA to 120W between 150 NM and 250 NM offshore, PZ
    #[doc(hidden)]
    PZZ940,
    /// San Clemente Island, CA to Guadalupe Island from 60 NM offshore west to 120W, PZ
    #[doc(hidden)]
    PZZ945,
}
impl ::std::str::FromStr for OffshoreMarineZone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "AMZ040" => Ok(OffshoreMarineZone::AMZ040),
            "AMZ041" => Ok(OffshoreMarineZone::AMZ041),
            "AMZ042" => Ok(OffshoreMarineZone::AMZ042),
            "AMZ043" => Ok(OffshoreMarineZone::AMZ043),
            "AMZ044" => Ok(OffshoreMarineZone::AMZ044),
            "AMZ045" => Ok(OffshoreMarineZone::AMZ045),
            "AMZ046" => Ok(OffshoreMarineZone::AMZ046),
            "AMZ047" => Ok(OffshoreMarineZone::AMZ047),
            "AMZ048" => Ok(OffshoreMarineZone::AMZ048),
            "AMZ049" => Ok(OffshoreMarineZone::AMZ049),
            "AMZ050" => Ok(OffshoreMarineZone::AMZ050),
            "AMZ051" => Ok(OffshoreMarineZone::AMZ051),
            "AMZ052" => Ok(OffshoreMarineZone::AMZ052),
            "AMZ053" => Ok(OffshoreMarineZone::AMZ053),
            "AMZ054" => Ok(OffshoreMarineZone::AMZ054),
            "AMZ055" => Ok(OffshoreMarineZone::AMZ055),
            "AMZ056" => Ok(OffshoreMarineZone::AMZ056),
            "AMZ057" => Ok(OffshoreMarineZone::AMZ057),
            "AMZ058" => Ok(OffshoreMarineZone::AMZ058),
            "AMZ059" => Ok(OffshoreMarineZone::AMZ059),
            "AMZ060" => Ok(OffshoreMarineZone::AMZ060),
            "AMZ061" => Ok(OffshoreMarineZone::AMZ061),
            "AMZ062" => Ok(OffshoreMarineZone::AMZ062),
            "AMZ063" => Ok(OffshoreMarineZone::AMZ063),
            "AMZ064" => Ok(OffshoreMarineZone::AMZ064),
            "AMZ065" => Ok(OffshoreMarineZone::AMZ065),
            "AMZ066" => Ok(OffshoreMarineZone::AMZ066),
            "AMZ067" => Ok(OffshoreMarineZone::AMZ067),
            "AMZ068" => Ok(OffshoreMarineZone::AMZ068),
            "AMZ069" => Ok(OffshoreMarineZone::AMZ069),
            "AMZ070" => Ok(OffshoreMarineZone::AMZ070),
            "AMZ071" => Ok(OffshoreMarineZone::AMZ071),
            "AMZ072" => Ok(OffshoreMarineZone::AMZ072),
            "AMZ073" => Ok(OffshoreMarineZone::AMZ073),
            "AMZ074" => Ok(OffshoreMarineZone::AMZ074),
            "AMZ075" => Ok(OffshoreMarineZone::AMZ075),
            "AMZ076" => Ok(OffshoreMarineZone::AMZ076),
            "AMZ077" => Ok(OffshoreMarineZone::AMZ077),
            "AMZ078" => Ok(OffshoreMarineZone::AMZ078),
            "AMZ079" => Ok(OffshoreMarineZone::AMZ079),
            "AMZ080" => Ok(OffshoreMarineZone::AMZ080),
            "AMZ081" => Ok(OffshoreMarineZone::AMZ081),
            "AMZ082" => Ok(OffshoreMarineZone::AMZ082),
            "AMZ083" => Ok(OffshoreMarineZone::AMZ083),
            "AMZ084" => Ok(OffshoreMarineZone::AMZ084),
            "AMZ085" => Ok(OffshoreMarineZone::AMZ085),
            "AMZ086" => Ok(OffshoreMarineZone::AMZ086),
            "AMZ087" => Ok(OffshoreMarineZone::AMZ087),
            "AMZ088" => Ok(OffshoreMarineZone::AMZ088),
            "ANZ800" => Ok(OffshoreMarineZone::ANZ800),
            "ANZ805" => Ok(OffshoreMarineZone::ANZ805),
            "ANZ810" => Ok(OffshoreMarineZone::ANZ810),
            "ANZ815" => Ok(OffshoreMarineZone::ANZ815),
            "ANZ820" => Ok(OffshoreMarineZone::ANZ820),
            "ANZ825" => Ok(OffshoreMarineZone::ANZ825),
            "ANZ828" => Ok(OffshoreMarineZone::ANZ828),
            "ANZ830" => Ok(OffshoreMarineZone::ANZ830),
            "ANZ833" => Ok(OffshoreMarineZone::ANZ833),
            "ANZ835" => Ok(OffshoreMarineZone::ANZ835),
            "ANZ900" => Ok(OffshoreMarineZone::ANZ900),
            "ANZ905" => Ok(OffshoreMarineZone::ANZ905),
            "ANZ910" => Ok(OffshoreMarineZone::ANZ910),
            "ANZ915" => Ok(OffshoreMarineZone::ANZ915),
            "ANZ920" => Ok(OffshoreMarineZone::ANZ920),
            "ANZ925" => Ok(OffshoreMarineZone::ANZ925),
            "ANZ930" => Ok(OffshoreMarineZone::ANZ930),
            "ANZ935" => Ok(OffshoreMarineZone::ANZ935),
            "GMZ040" => Ok(OffshoreMarineZone::GMZ040),
            "GMZ041" => Ok(OffshoreMarineZone::GMZ041),
            "GMZ045" => Ok(OffshoreMarineZone::GMZ045),
            "GMZ046" => Ok(OffshoreMarineZone::GMZ046),
            "GMZ047" => Ok(OffshoreMarineZone::GMZ047),
            "GMZ048" => Ok(OffshoreMarineZone::GMZ048),
            "GMZ049" => Ok(OffshoreMarineZone::GMZ049),
            "GMZ050" => Ok(OffshoreMarineZone::GMZ050),
            "GMZ056" => Ok(OffshoreMarineZone::GMZ056),
            "GMZ057" => Ok(OffshoreMarineZone::GMZ057),
            "GMZ058" => Ok(OffshoreMarineZone::GMZ058),
            "PHZ180" => Ok(OffshoreMarineZone::PHZ180),
            "PKZ310" => Ok(OffshoreMarineZone::PKZ310),
            "PKZ351" => Ok(OffshoreMarineZone::PKZ351),
            "PKZ352" => Ok(OffshoreMarineZone::PKZ352),
            "PKZ411" => Ok(OffshoreMarineZone::PKZ411),
            "PKZ412" => Ok(OffshoreMarineZone::PKZ412),
            "PKZ413" => Ok(OffshoreMarineZone::PKZ413),
            "PKZ414" => Ok(OffshoreMarineZone::PKZ414),
            "PKZ500" => Ok(OffshoreMarineZone::PKZ500),
            "PKZ505" => Ok(OffshoreMarineZone::PKZ505),
            "PKZ510" => Ok(OffshoreMarineZone::PKZ510),
            "PMZ009" => Ok(OffshoreMarineZone::PMZ009),
            "PMZ011" => Ok(OffshoreMarineZone::PMZ011),
            "PMZ013" => Ok(OffshoreMarineZone::PMZ013),
            "PMZ015" => Ok(OffshoreMarineZone::PMZ015),
            "PMZ017" => Ok(OffshoreMarineZone::PMZ017),
            "PMZ019" => Ok(OffshoreMarineZone::PMZ019),
            "PMZ021" => Ok(OffshoreMarineZone::PMZ021),
            "PMZ023" => Ok(OffshoreMarineZone::PMZ023),
            "PMZ025" => Ok(OffshoreMarineZone::PMZ025),
            "PMZ027" => Ok(OffshoreMarineZone::PMZ027),
            "PMZ111" => Ok(OffshoreMarineZone::PMZ111),
            "PMZ113" => Ok(OffshoreMarineZone::PMZ113),
            "PMZ115" => Ok(OffshoreMarineZone::PMZ115),
            "PMZ117" => Ok(OffshoreMarineZone::PMZ117),
            "PMZ119" => Ok(OffshoreMarineZone::PMZ119),
            "PMZ121" => Ok(OffshoreMarineZone::PMZ121),
            "PMZ123" => Ok(OffshoreMarineZone::PMZ123),
            "PZZ800" => Ok(OffshoreMarineZone::PZZ800),
            "PZZ805" => Ok(OffshoreMarineZone::PZZ805),
            "PZZ810" => Ok(OffshoreMarineZone::PZZ810),
            "PZZ815" => Ok(OffshoreMarineZone::PZZ815),
            "PZZ820" => Ok(OffshoreMarineZone::PZZ820),
            "PZZ825" => Ok(OffshoreMarineZone::PZZ825),
            "PZZ830" => Ok(OffshoreMarineZone::PZZ830),
            "PZZ835" => Ok(OffshoreMarineZone::PZZ835),
            "PZZ840" => Ok(OffshoreMarineZone::PZZ840),
            "PZZ900" => Ok(OffshoreMarineZone::PZZ900),
            "PZZ905" => Ok(OffshoreMarineZone::PZZ905),
            "PZZ910" => Ok(OffshoreMarineZone::PZZ910),
            "PZZ915" => Ok(OffshoreMarineZone::PZZ915),
            "PZZ920" => Ok(OffshoreMarineZone::PZZ920),
            "PZZ925" => Ok(OffshoreMarineZone::PZZ925),
            "PZZ930" => Ok(OffshoreMarineZone::PZZ930),
            "PZZ935" => Ok(OffshoreMarineZone::PZZ935),
            "PZZ940" => Ok(OffshoreMarineZone::PZZ940),
            "PZZ945" => Ok(OffshoreMarineZone::PZZ945),
            _ => Err(()),
        }
    }
}
impl OffshoreMarineZone {
    pub fn details(&self) -> crate::ZoneDetails {
        match self {
            OffshoreMarineZone::AMZ040 => crate::ZoneDetails {
                state: "AM",
                zone: "040",
                zone_numeric: 40,
                name: "Caribbean Nof 18N W of 85W including Yucatan Basin",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ041 => crate::ZoneDetails {
                state: "AM",
                zone: "041",
                zone_numeric: 41,
                name: "Caribbean N of 20N E of 85W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ042 => crate::ZoneDetails {
                state: "AM",
                zone: "042",
                zone_numeric: 42,
                name: "Caribbean from 18N-20N between 80W-85W including Cayman Basin",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ043 => crate::ZoneDetails {
                state: "AM",
                zone: "043",
                zone_numeric: 43,
                name: "Caribbean from 18N-20N between 76W-80W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ044 => crate::ZoneDetails {
                state: "AM",
                zone: "044",
                zone_numeric: 44,
                name: "Caribbean approaches to the Windward Passage",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ045 => crate::ZoneDetails {
                state: "AM",
                zone: "045",
                zone_numeric: 45,
                name: "Gulf of Honduras",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ046 => crate::ZoneDetails {
                state: "AM",
                zone: "046",
                zone_numeric: 46,
                name: "Caribbean from 15N to 18N between 80W and 85W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ047 => crate::ZoneDetails {
                state: "AM",
                zone: "047",
                zone_numeric: 47,
                name: "Caribbean from 15N to 18N between 76W and 80W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ048 => crate::ZoneDetails {
                state: "AM",
                zone: "048",
                zone_numeric: 48,
                name: "Caribbean from 15N to 18N between 72W and 76W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ049 => crate::ZoneDetails {
                state: "AM",
                zone: "049",
                zone_numeric: 49,
                name: "Caribbean N of 15N between 68W and 72W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ050 => crate::ZoneDetails {
                state: "AM",
                zone: "050",
                zone_numeric: 50,
                name: "Caribbean N of 15N between 64W and 68W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ051 => crate::ZoneDetails {
                state: "AM",
                zone: "051",
                zone_numeric: 51,
                name: "Offshore Waters Leeward Islands",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ052 => crate::ZoneDetails {
                state: "AM",
                zone: "052",
                zone_numeric: 52,
                name: "Tropical N Atlantic from 15N to 19N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ053 => crate::ZoneDetails {
                state: "AM",
                zone: "053",
                zone_numeric: 53,
                name: "W Central Caribbean from 11N to 15N W of 80W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ054 => crate::ZoneDetails {
                state: "AM",
                zone: "054",
                zone_numeric: 54,
                name: "Caribbean from 11N to 15N between 76W and 80W including Colombia Basin",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ055 => crate::ZoneDetails {
                state: "AM",
                zone: "055",
                zone_numeric: 55,
                name: "Caribbean from 11N to 15N between 72W and 76W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ056 => crate::ZoneDetails {
                state: "AM",
                zone: "056",
                zone_numeric: 56,
                name: "Caribbean S of 15N between 68W and 72W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ057 => crate::ZoneDetails {
                state: "AM",
                zone: "057",
                zone_numeric: 57,
                name: "Caribbean S of 15N between 64W and 68W including Venezuela Basin",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ058 => crate::ZoneDetails {
                state: "AM",
                zone: "058",
                zone_numeric: 58,
                name: "Offshore Waters Windward Islands including Trinidad and Tobago",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ059 => crate::ZoneDetails {
                state: "AM",
                zone: "059",
                zone_numeric: 59,
                name: "Tropical N Atlantic from 11N and 15N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ060 => crate::ZoneDetails {
                state: "AM",
                zone: "060",
                zone_numeric: 60,
                name: "SW Caribbean S of 11N W of 80W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ061 => crate::ZoneDetails {
                state: "AM",
                zone: "061",
                zone_numeric: 61,
                name: "SW Caribbean S of 11N E of 80W including the approaches to the Panama Canal",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ062 => crate::ZoneDetails {
                state: "AM",
                zone: "062",
                zone_numeric: 62,
                name: "Tropical N Atlantic from 7N and 11N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ063 => crate::ZoneDetails {
                state: "AM",
                zone: "063",
                zone_numeric: 63,
                name: "Atlantic from 29N to 31N W of 77W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ064 => crate::ZoneDetails {
                state: "AM",
                zone: "064",
                zone_numeric: 64,
                name: "Atlantic from 29N to 31N between 74W and 77W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ065 => crate::ZoneDetails {
                state: "AM",
                zone: "065",
                zone_numeric: 65,
                name: "Atlantic from 29N to 31N between 70W and 74W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ066 => crate::ZoneDetails {
                state: "AM",
                zone: "066",
                zone_numeric: 66,
                name: "Atlantic from 29N to 31N between 65W and 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ067 => crate::ZoneDetails {
                state: "AM",
                zone: "067",
                zone_numeric: 67,
                name: "Atlantic from 29N to 31N between 60W and 65W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ068 => crate::ZoneDetails {
                state: "AM",
                zone: "068",
                zone_numeric: 68,
                name: "Atlantic from 29N to 31N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ069 => crate::ZoneDetails {
                state: "AM",
                zone: "069",
                zone_numeric: 69,
                name: "Atlantic from 27N to 29N W of 77W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ070 => crate::ZoneDetails {
                state: "AM",
                zone: "070",
                zone_numeric: 70,
                name: "Atlantic from 27N to 29N between 74W and 77W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ071 => crate::ZoneDetails {
                state: "AM",
                zone: "071",
                zone_numeric: 71,
                name: "Atlantic from 27N to 29N between 70W and 74W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ072 => crate::ZoneDetails {
                state: "AM",
                zone: "072",
                zone_numeric: 72,
                name: "Atlantic from 27N to 29N between 65W and 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ073 => crate::ZoneDetails {
                state: "AM",
                zone: "073",
                zone_numeric: 73,
                name: "Atlantic from 27N to 29N between 60W and 65W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ074 => crate::ZoneDetails {
                state: "AM",
                zone: "074",
                zone_numeric: 74,
                name: "Atlantic from 27N to 29N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ075 => crate::ZoneDetails {
                state: "AM",
                zone: "075",
                zone_numeric: 75,
                name: "Northern Bahamas from 24N to 27N",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ076 => crate::ZoneDetails {
                state: "AM",
                zone: "076",
                zone_numeric: 76,
                name: "Atlantic from 22N to 27N E of Bahamas to 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ077 => crate::ZoneDetails {
                state: "AM",
                zone: "077",
                zone_numeric: 77,
                name: "Atlantic from 22N to 27N between 65W and 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ078 => crate::ZoneDetails {
                state: "AM",
                zone: "078",
                zone_numeric: 78,
                name: "Atlantic from 25N to 27N between 60W and 65W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ079 => crate::ZoneDetails {
                state: "AM",
                zone: "079",
                zone_numeric: 79,
                name: "Atlantic from 25N to 27N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ080 => crate::ZoneDetails {
                state: "AM",
                zone: "080",
                zone_numeric: 80,
                name: "Central Bahamas including Cay Sal Bank",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ081 => crate::ZoneDetails {
                state: "AM",
                zone: "081",
                zone_numeric: 81,
                name: "Atlantic from 22N to 25N E of Bahamas to 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ082 => crate::ZoneDetails {
                state: "AM",
                zone: "082",
                zone_numeric: 82,
                name: "Atlantic from 22N to 25N between 65W and 70W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ083 => crate::ZoneDetails {
                state: "AM",
                zone: "083",
                zone_numeric: 83,
                name: "Atlantic from 22N to 25N between 60W and 65W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ084 => crate::ZoneDetails {
                state: "AM",
                zone: "084",
                zone_numeric: 84,
                name: "Atlantic from 22N to 25N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ085 => crate::ZoneDetails {
                state: "AM",
                zone: "085",
                zone_numeric: 85,
                name: "Atlantic S of 22N W of 70W including approaches to the Windward Passage",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ086 => crate::ZoneDetails {
                state: "AM",
                zone: "086",
                zone_numeric: 86,
                name: "Atlantic S of 22N between 65W and 70W including Puerto Rico Trench",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ087 => crate::ZoneDetails {
                state: "AM",
                zone: "087",
                zone_numeric: 87,
                name: "Atlantic from 19N to 22N between 60W and 65W",
                wfo: "NH2",
            },
            OffshoreMarineZone::AMZ088 => crate::ZoneDetails {
                state: "AM",
                zone: "088",
                zone_numeric: 88,
                name: "Atlantic from 19N to 22N between 55W and 60W",
                wfo: "NH2",
            },
            OffshoreMarineZone::ANZ800 => crate::ZoneDetails {
                state: "AN",
                zone: "800",
                zone_numeric: 800,
                name: "Gulf of Maine to the Hague Line",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ805 => crate::ZoneDetails {
                state: "AN",
                zone: "805",
                zone_numeric: 805,
                name: "Georges Bank between Cape Cod and 68W north of 1000 FM",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ810 => crate::ZoneDetails {
                state: "AN",
                zone: "810",
                zone_numeric: 810,
                name: "South of New England between the Great South Channel and Montauk Point to 1000 FM",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ815 => crate::ZoneDetails {
                state: "AN",
                zone: "815",
                zone_numeric: 815,
                name: "South of Long Island between Montauk Point and Sandy Hook to 1000 FM",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ820 => crate::ZoneDetails {
                state: "AN",
                zone: "820",
                zone_numeric: 820,
                name: "Hudson Canyon to Baltimore Canyon to 1000 FM",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ825 => crate::ZoneDetails {
                state: "AN",
                zone: "825",
                zone_numeric: 825,
                name: "Baltimore Canyon to Cape Charles Light to 100 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ828 => crate::ZoneDetails {
                state: "AN",
                zone: "828",
                zone_numeric: 828,
                name: "Cape Charles Light to Currituck Beach Light to 100 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ830 => crate::ZoneDetails {
                state: "AN",
                zone: "830",
                zone_numeric: 830,
                name: "Currituck Beach Light to Cape Hatteras to 100 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ833 => crate::ZoneDetails {
                state: "AN",
                zone: "833",
                zone_numeric: 833,
                name: "Cape Hatteras to Cape Fear to 100 NM Offshore.",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ835 => crate::ZoneDetails {
                state: "AN",
                zone: "835",
                zone_numeric: 835,
                name: "Cape Fear to 31N to 1000 FM",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ900 => crate::ZoneDetails {
                state: "AN",
                zone: "900",
                zone_numeric: 900,
                name: "Georges Bank between 68W and the Hague Line",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ905 => crate::ZoneDetails {
                state: "AN",
                zone: "905",
                zone_numeric: 905,
                name: "East of 69W to the Hague Line between 1000 FM and 39N",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ910 => crate::ZoneDetails {
                state: "AN",
                zone: "910",
                zone_numeric: 910,
                name: "East of 69W and south of 39N to 250 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ915 => crate::ZoneDetails {
                state: "AN",
                zone: "915",
                zone_numeric: 915,
                name: "Between 1000FM and 38.5 N west of 69 W",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ920 => crate::ZoneDetails {
                state: "AN",
                zone: "920",
                zone_numeric: 920,
                name: "Baltimore Canyon to 69W east of 1000 FM and south of 38.5N to 250 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ925 => crate::ZoneDetails {
                state: "AN",
                zone: "925",
                zone_numeric: 925,
                name: "Baltimore Canyon to Hatteras Canyon between 100 NM and 250 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ930 => crate::ZoneDetails {
                state: "AN",
                zone: "930",
                zone_numeric: 930,
                name: "Hatteras Canyon to Cape Fear between 100 NM and 250 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::ANZ935 => crate::ZoneDetails {
                state: "AN",
                zone: "935",
                zone_numeric: 935,
                name: "Cape Fear to 31N east of 1000 FM to 250 NM offshore",
                wfo: "ONA",
            },
            OffshoreMarineZone::GMZ040 => crate::ZoneDetails {
                state: "GM",
                zone: "040",
                zone_numeric: 40,
                name: "NW Gulf including Stetson Bank",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ041 => crate::ZoneDetails {
                state: "GM",
                zone: "041",
                zone_numeric: 41,
                name: "SW Louisiana Offshore Waters including Flower Garden Bank Marine Sanctuary",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ045 => crate::ZoneDetails {
                state: "GM",
                zone: "045",
                zone_numeric: 45,
                name: "W Central Gulf from 22N to 26N between 91W and 94W",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ046 => crate::ZoneDetails {
                state: "GM",
                zone: "046",
                zone_numeric: 46,
                name: "Central Gulf from 22N to 26N between 87W and 91W",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ047 => crate::ZoneDetails {
                state: "GM",
                zone: "047",
                zone_numeric: 47,
                name: "SE Gulf from 22N to 26N E of 87W including Straits of Florida",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ048 => crate::ZoneDetails {
                state: "GM",
                zone: "048",
                zone_numeric: 48,
                name: "SW Gulf S of 22N W of 94W",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ049 => crate::ZoneDetails {
                state: "GM",
                zone: "049",
                zone_numeric: 49,
                name: "Central Bay of Campeche",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ050 => crate::ZoneDetails {
                state: "GM",
                zone: "050",
                zone_numeric: 50,
                name: "E Bay of Campeche including Campeche Bank",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ056 => crate::ZoneDetails {
                state: "GM",
                zone: "056",
                zone_numeric: 56,
                name: "N Central Gulf Offshore Waters",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ057 => crate::ZoneDetails {
                state: "GM",
                zone: "057",
                zone_numeric: 57,
                name: "NE Gulf N of 26N E of 87W",
                wfo: "NH2",
            },
            OffshoreMarineZone::GMZ058 => crate::ZoneDetails {
                state: "GM",
                zone: "058",
                zone_numeric: 58,
                name: "W Central Gulf from 22N to 26N W of 94W",
                wfo: "NH2",
            },
            OffshoreMarineZone::PHZ180 => crate::ZoneDetails {
                state: "PH",
                zone: "180",
                zone_numeric: 180,
                name: "Hawaiian Offshore Waters",
                wfo: "HPA",
            },
            OffshoreMarineZone::PKZ310 => crate::ZoneDetails {
                state: "PK",
                zone: "310",
                zone_numeric: 310,
                name: "Gulf of Alaska North of 55 Degrees North and East of 144 W",
                wfo: "AJK",
            },
            OffshoreMarineZone::PKZ351 => crate::ZoneDetails {
                state: "PK",
                zone: "351",
                zone_numeric: 351,
                name: "Gulf of Alaska Offshore North of 57N and West of 144W",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ352 => crate::ZoneDetails {
                state: "PK",
                zone: "352",
                zone_numeric: 352,
                name: "Gulf of Alaska Offshore South of 57N North of 55N and West of 144W",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ411 => crate::ZoneDetails {
                state: "PK",
                zone: "411",
                zone_numeric: 411,
                name: "Bering Sea Offshore  West of 180 and East of the International Date Line",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ412 => crate::ZoneDetails {
                state: "PK",
                zone: "412",
                zone_numeric: 412,
                name: "Bering Sea Offshore 171W to 180 and North of 56N",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ413 => crate::ZoneDetails {
                state: "PK",
                zone: "413",
                zone_numeric: 413,
                name: "Bering Sea Offshore 171W to 180 and South of 56N",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ414 => crate::ZoneDetails {
                state: "PK",
                zone: "414",
                zone_numeric: 414,
                name: "Bering Sea Offshore East of 171W",
                wfo: "AFC",
            },
            OffshoreMarineZone::PKZ500 => crate::ZoneDetails {
                state: "PK",
                zone: "500",
                zone_numeric: 500,
                name: "Western US Arctic Offshore",
                wfo: "AFG",
            },
            OffshoreMarineZone::PKZ505 => crate::ZoneDetails {
                state: "PK",
                zone: "505",
                zone_numeric: 505,
                name: "Central US Arctic Offshore",
                wfo: "AFG",
            },
            OffshoreMarineZone::PKZ510 => crate::ZoneDetails {
                state: "PK",
                zone: "510",
                zone_numeric: 510,
                name: "Eastern US Arctic Offshore",
                wfo: "AFG",
            },
            OffshoreMarineZone::PMZ009 => crate::ZoneDetails {
                state: "PM",
                zone: "009",
                zone_numeric: 9,
                name: "Mexico Border S to 29N to 60 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ011 => crate::ZoneDetails {
                state: "PM",
                zone: "011",
                zone_numeric: 11,
                name: "Mexico S of 29N to Punta Eugenia to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ013 => crate::ZoneDetails {
                state: "PM",
                zone: "013",
                zone_numeric: 13,
                name: "Punta Eugenia to Cabo San Lazaro to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ015 => crate::ZoneDetails {
                state: "PM",
                zone: "015",
                zone_numeric: 15,
                name: "Cabo San Lazaro to Cabo San Lucas to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ017 => crate::ZoneDetails {
                state: "PM",
                zone: "017",
                zone_numeric: 17,
                name: "Northern Gulf of California",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ019 => crate::ZoneDetails {
                state: "PM",
                zone: "019",
                zone_numeric: 19,
                name: "Central Gulf of California",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ021 => crate::ZoneDetails {
                state: "PM",
                zone: "021",
                zone_numeric: 21,
                name: "Southern Gulf of California",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ023 => crate::ZoneDetails {
                state: "PM",
                zone: "023",
                zone_numeric: 23,
                name: "Entrance to the Gulf of California Including Cabo Corrientes",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ025 => crate::ZoneDetails {
                state: "PM",
                zone: "025",
                zone_numeric: 25,
                name: "Mexico - Michoachan and Guerrero to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ027 => crate::ZoneDetails {
                state: "PM",
                zone: "027",
                zone_numeric: 27,
                name: "Mexico - Oaxaca and Chiapas including the Gulf of Tehuantepec",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ111 => crate::ZoneDetails {
                state: "PM",
                zone: "111",
                zone_numeric: 111,
                name: "Guatemala and El Salvador to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ113 => crate::ZoneDetails {
                state: "PM",
                zone: "113",
                zone_numeric: 113,
                name: "El Salvador to North Costa Rica Including the Gulfs of Fonseca and Papagayo",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ115 => crate::ZoneDetails {
                state: "PM",
                zone: "115",
                zone_numeric: 115,
                name: "North Costa Rica to West Panama to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ117 => crate::ZoneDetails {
                state: "PM",
                zone: "117",
                zone_numeric: 117,
                name: "East Panama and Colombia Including Gulf of Panama",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ119 => crate::ZoneDetails {
                state: "PM",
                zone: "119",
                zone_numeric: 119,
                name: "Ecuador Including Gulf of Guayaquil to 250 NM offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ121 => crate::ZoneDetails {
                state: "PM",
                zone: "121",
                zone_numeric: 121,
                name: "Ecuador between 250 and 500 NM Offshore",
                wfo: "NH1",
            },
            OffshoreMarineZone::PMZ123 => crate::ZoneDetails {
                state: "PM",
                zone: "123",
                zone_numeric: 123,
                name: "Offshore Galapagos Islands",
                wfo: "NH1",
            },
            OffshoreMarineZone::PZZ800 => crate::ZoneDetails {
                state: "PZ",
                zone: "800",
                zone_numeric: 800,
                name: "Cape Flattery to Cape Shoalwater between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ805 => crate::ZoneDetails {
                state: "PZ",
                zone: "805",
                zone_numeric: 805,
                name: "Cape Shoalwater to Cape Lookout between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ810 => crate::ZoneDetails {
                state: "PZ",
                zone: "810",
                zone_numeric: 810,
                name: "Cape Lookout to Florence, OR between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ815 => crate::ZoneDetails {
                state: "PZ",
                zone: "815",
                zone_numeric: 815,
                name: "Florence, OR to Point St. George between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ820 => crate::ZoneDetails {
                state: "PZ",
                zone: "820",
                zone_numeric: 820,
                name: "Point St. George to Point Arena between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ825 => crate::ZoneDetails {
                state: "PZ",
                zone: "825",
                zone_numeric: 825,
                name: "Point Arena to Pigeon Point between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ830 => crate::ZoneDetails {
                state: "PZ",
                zone: "830",
                zone_numeric: 830,
                name: "Pigeon Point to Point Piedras Blancas between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ835 => crate::ZoneDetails {
                state: "PZ",
                zone: "835",
                zone_numeric: 835,
                name: "Point Piedras Blancas to Santa Cruz Island, CA between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ840 => crate::ZoneDetails {
                state: "PZ",
                zone: "840",
                zone_numeric: 840,
                name: "Santa Cruz Island, CA to San Clemente Island, CA between 60 NM and 150 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ900 => crate::ZoneDetails {
                state: "PZ",
                zone: "900",
                zone_numeric: 900,
                name: "Cape Flattery to Cape Shoalwater between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ905 => crate::ZoneDetails {
                state: "PZ",
                zone: "905",
                zone_numeric: 905,
                name: "Cape Shoalwater to Cape Lookout between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ910 => crate::ZoneDetails {
                state: "PZ",
                zone: "910",
                zone_numeric: 910,
                name: "Cape Lookout to Florence, OR between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ915 => crate::ZoneDetails {
                state: "PZ",
                zone: "915",
                zone_numeric: 915,
                name: "Florence, OR to Point St. George between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ920 => crate::ZoneDetails {
                state: "PZ",
                zone: "920",
                zone_numeric: 920,
                name: "Point St. George to Point Arena between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ925 => crate::ZoneDetails {
                state: "PZ",
                zone: "925",
                zone_numeric: 925,
                name: "Point Arena to Pigeon Point between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ930 => crate::ZoneDetails {
                state: "PZ",
                zone: "930",
                zone_numeric: 930,
                name: "Pigeon Point to Point Piedras Blancas between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ935 => crate::ZoneDetails {
                state: "PZ",
                zone: "935",
                zone_numeric: 935,
                name: "Point Piedras Blancas to Santa Cruz Island, CA between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ940 => crate::ZoneDetails {
                state: "PZ",
                zone: "940",
                zone_numeric: 940,
                name: "Santa Cruz Island, CA to 120W between 150 NM and 250 NM offshore",
                wfo: "ONP",
            },
            OffshoreMarineZone::PZZ945 => crate::ZoneDetails {
                state: "PZ",
                zone: "945",
                zone_numeric: 945,
                name: "San Clemente Island, CA to Guadalupe Island from 60 NM offshore west to 120W",
                wfo: "ONP",
            },
        }
    }
    pub fn new(two: &str, numeric: u16) -> Option<Self> {
        match two {
            "AM" => match numeric {
                040 => Some(OffshoreMarineZone::AMZ040),
                041 => Some(OffshoreMarineZone::AMZ041),
                042 => Some(OffshoreMarineZone::AMZ042),
                043 => Some(OffshoreMarineZone::AMZ043),
                044 => Some(OffshoreMarineZone::AMZ044),
                045 => Some(OffshoreMarineZone::AMZ045),
                046 => Some(OffshoreMarineZone::AMZ046),
                047 => Some(OffshoreMarineZone::AMZ047),
                048 => Some(OffshoreMarineZone::AMZ048),
                049 => Some(OffshoreMarineZone::AMZ049),
                050 => Some(OffshoreMarineZone::AMZ050),
                051 => Some(OffshoreMarineZone::AMZ051),
                052 => Some(OffshoreMarineZone::AMZ052),
                053 => Some(OffshoreMarineZone::AMZ053),
                054 => Some(OffshoreMarineZone::AMZ054),
                055 => Some(OffshoreMarineZone::AMZ055),
                056 => Some(OffshoreMarineZone::AMZ056),
                057 => Some(OffshoreMarineZone::AMZ057),
                058 => Some(OffshoreMarineZone::AMZ058),
                059 => Some(OffshoreMarineZone::AMZ059),
                060 => Some(OffshoreMarineZone::AMZ060),
                061 => Some(OffshoreMarineZone::AMZ061),
                062 => Some(OffshoreMarineZone::AMZ062),
                063 => Some(OffshoreMarineZone::AMZ063),
                064 => Some(OffshoreMarineZone::AMZ064),
                065 => Some(OffshoreMarineZone::AMZ065),
                066 => Some(OffshoreMarineZone::AMZ066),
                067 => Some(OffshoreMarineZone::AMZ067),
                068 => Some(OffshoreMarineZone::AMZ068),
                069 => Some(OffshoreMarineZone::AMZ069),
                070 => Some(OffshoreMarineZone::AMZ070),
                071 => Some(OffshoreMarineZone::AMZ071),
                072 => Some(OffshoreMarineZone::AMZ072),
                073 => Some(OffshoreMarineZone::AMZ073),
                074 => Some(OffshoreMarineZone::AMZ074),
                075 => Some(OffshoreMarineZone::AMZ075),
                076 => Some(OffshoreMarineZone::AMZ076),
                077 => Some(OffshoreMarineZone::AMZ077),
                078 => Some(OffshoreMarineZone::AMZ078),
                079 => Some(OffshoreMarineZone::AMZ079),
                080 => Some(OffshoreMarineZone::AMZ080),
                081 => Some(OffshoreMarineZone::AMZ081),
                082 => Some(OffshoreMarineZone::AMZ082),
                083 => Some(OffshoreMarineZone::AMZ083),
                084 => Some(OffshoreMarineZone::AMZ084),
                085 => Some(OffshoreMarineZone::AMZ085),
                086 => Some(OffshoreMarineZone::AMZ086),
                087 => Some(OffshoreMarineZone::AMZ087),
                088 => Some(OffshoreMarineZone::AMZ088),
                _ => None,
            },
            "AN" => match numeric {
                800 => Some(OffshoreMarineZone::ANZ800),
                805 => Some(OffshoreMarineZone::ANZ805),
                810 => Some(OffshoreMarineZone::ANZ810),
                815 => Some(OffshoreMarineZone::ANZ815),
                820 => Some(OffshoreMarineZone::ANZ820),
                825 => Some(OffshoreMarineZone::ANZ825),
                828 => Some(OffshoreMarineZone::ANZ828),
                830 => Some(OffshoreMarineZone::ANZ830),
                833 => Some(OffshoreMarineZone::ANZ833),
                835 => Some(OffshoreMarineZone::ANZ835),
                900 => Some(OffshoreMarineZone::ANZ900),
                905 => Some(OffshoreMarineZone::ANZ905),
                910 => Some(OffshoreMarineZone::ANZ910),
                915 => Some(OffshoreMarineZone::ANZ915),
                920 => Some(OffshoreMarineZone::ANZ920),
                925 => Some(OffshoreMarineZone::ANZ925),
                930 => Some(OffshoreMarineZone::ANZ930),
                935 => Some(OffshoreMarineZone::ANZ935),
                _ => None,
            },
            "GM" => match numeric {
                040 => Some(OffshoreMarineZone::GMZ040),
                041 => Some(OffshoreMarineZone::GMZ041),
                045 => Some(OffshoreMarineZone::GMZ045),
                046 => Some(OffshoreMarineZone::GMZ046),
                047 => Some(OffshoreMarineZone::GMZ047),
                048 => Some(OffshoreMarineZone::GMZ048),
                049 => Some(OffshoreMarineZone::GMZ049),
                050 => Some(OffshoreMarineZone::GMZ050),
                056 => Some(OffshoreMarineZone::GMZ056),
                057 => Some(OffshoreMarineZone::GMZ057),
                058 => Some(OffshoreMarineZone::GMZ058),
                _ => None,
            },
            "PH" => match numeric {
                180 => Some(OffshoreMarineZone::PHZ180),
                _ => None,
            },
            "PK" => match numeric {
                310 => Some(OffshoreMarineZone::PKZ310),
                351 => Some(OffshoreMarineZone::PKZ351),
                352 => Some(OffshoreMarineZone::PKZ352),
                411 => Some(OffshoreMarineZone::PKZ411),
                412 => Some(OffshoreMarineZone::PKZ412),
                413 => Some(OffshoreMarineZone::PKZ413),
                414 => Some(OffshoreMarineZone::PKZ414),
                500 => Some(OffshoreMarineZone::PKZ500),
                505 => Some(OffshoreMarineZone::PKZ505),
                510 => Some(OffshoreMarineZone::PKZ510),
                _ => None,
            },
            "PM" => match numeric {
                009 => Some(OffshoreMarineZone::PMZ009),
                011 => Some(OffshoreMarineZone::PMZ011),
                013 => Some(OffshoreMarineZone::PMZ013),
                015 => Some(OffshoreMarineZone::PMZ015),
                017 => Some(OffshoreMarineZone::PMZ017),
                019 => Some(OffshoreMarineZone::PMZ019),
                021 => Some(OffshoreMarineZone::PMZ021),
                023 => Some(OffshoreMarineZone::PMZ023),
                025 => Some(OffshoreMarineZone::PMZ025),
                027 => Some(OffshoreMarineZone::PMZ027),
                111 => Some(OffshoreMarineZone::PMZ111),
                113 => Some(OffshoreMarineZone::PMZ113),
                115 => Some(OffshoreMarineZone::PMZ115),
                117 => Some(OffshoreMarineZone::PMZ117),
                119 => Some(OffshoreMarineZone::PMZ119),
                121 => Some(OffshoreMarineZone::PMZ121),
                123 => Some(OffshoreMarineZone::PMZ123),
                _ => None,
            },
            "PZ" => match numeric {
                800 => Some(OffshoreMarineZone::PZZ800),
                805 => Some(OffshoreMarineZone::PZZ805),
                810 => Some(OffshoreMarineZone::PZZ810),
                815 => Some(OffshoreMarineZone::PZZ815),
                820 => Some(OffshoreMarineZone::PZZ820),
                825 => Some(OffshoreMarineZone::PZZ825),
                830 => Some(OffshoreMarineZone::PZZ830),
                835 => Some(OffshoreMarineZone::PZZ835),
                840 => Some(OffshoreMarineZone::PZZ840),
                900 => Some(OffshoreMarineZone::PZZ900),
                905 => Some(OffshoreMarineZone::PZZ905),
                910 => Some(OffshoreMarineZone::PZZ910),
                915 => Some(OffshoreMarineZone::PZZ915),
                920 => Some(OffshoreMarineZone::PZZ920),
                925 => Some(OffshoreMarineZone::PZZ925),
                930 => Some(OffshoreMarineZone::PZZ930),
                935 => Some(OffshoreMarineZone::PZZ935),
                940 => Some(OffshoreMarineZone::PZZ940),
                945 => Some(OffshoreMarineZone::PZZ945),
                _ => None,
            },
            _ => None,
        }
    }
}
