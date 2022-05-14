#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ForecastZone {
    /// Cape Fairweather to Cape Suckling Coastal Area, AK
    #[doc(hidden)]
    AK017,
    /// Taiya Inlet and Klondike Highway, AK
    #[doc(hidden)]
    AK018,
    /// Haines Borough and Lynn Canal, AK
    #[doc(hidden)]
    AK019,
    /// Glacier Bay, AK
    #[doc(hidden)]
    AK020,
    /// Eastern Chichagof Island, AK
    #[doc(hidden)]
    AK021,
    /// Salisbury Sound to Cape Fairweather Coastal Area, AK
    #[doc(hidden)]
    AK022,
    /// Cape Decision to Salisbury Sound Coastal Area, AK
    #[doc(hidden)]
    AK023,
    /// Eastern Baranof Island and Southern Admiralty Island, AK
    #[doc(hidden)]
    AK024,
    /// Juneau Borough and Northern Admiralty Island, AK
    #[doc(hidden)]
    AK025,
    /// Inner Channels from Kupreanof Island to Etolin Island, AK
    #[doc(hidden)]
    AK026,
    /// Dixon Entrance to Cape Decision Coastal Area, AK
    #[doc(hidden)]
    AK027,
    /// Southern Inner Channels, AK
    #[doc(hidden)]
    AK028,
    /// Misty Fjords, AK
    #[doc(hidden)]
    AK029,
    /// Anchorage, AK
    #[doc(hidden)]
    AK101,
    /// Matanuska Valley, AK
    #[doc(hidden)]
    AK111,
    /// Western Kenai Peninsula, AK
    #[doc(hidden)]
    AK121,
    /// Western Prince William Sound, AK
    #[doc(hidden)]
    AK125,
    /// Northeast Prince William Sound, AK
    #[doc(hidden)]
    AK131,
    /// Southeast Prince William Sound, AK
    #[doc(hidden)]
    AK135,
    /// Copper River Basin, AK
    #[doc(hidden)]
    AK141,
    /// Susitna Valley, AK
    #[doc(hidden)]
    AK145,
    /// Lower Kuskokwim Valley, AK
    #[doc(hidden)]
    AK152,
    /// Kuskokwim Delta, AK
    #[doc(hidden)]
    AK155,
    /// Bristol Bay, AK
    #[doc(hidden)]
    AK161,
    /// Kodiak Island, AK
    #[doc(hidden)]
    AK171,
    /// Alaska Peninsula, AK
    #[doc(hidden)]
    AK181,
    /// Eastern Aleutians, AK
    #[doc(hidden)]
    AK185,
    /// Central Aleutians, AK
    #[doc(hidden)]
    AK187,
    /// Western Aleutians, AK
    #[doc(hidden)]
    AK191,
    /// Pribilof Islands, AK
    #[doc(hidden)]
    AK195,
    /// Western Arctic Coast, AK
    #[doc(hidden)]
    AK201,
    /// Northern Arctic Coast, AK
    #[doc(hidden)]
    AK202,
    /// Central Beaufort Sea Coast, AK
    #[doc(hidden)]
    AK203,
    /// Eastern Beaufort Sea Coast, AK
    #[doc(hidden)]
    AK204,
    /// Northwestern Brooks Range, AK
    #[doc(hidden)]
    AK205,
    /// Northeastern Brooks Range, AK
    #[doc(hidden)]
    AK206,
    /// Chukchi Sea Coast, AK
    #[doc(hidden)]
    AK207,
    /// Lower Kobuk and Noatak Valleys, AK
    #[doc(hidden)]
    AK208,
    /// Baldwin Peninsula and Selawik Valley, AK
    #[doc(hidden)]
    AK209,
    /// Northern and Interior Seward Peninsula, AK
    #[doc(hidden)]
    AK210,
    /// Southern Seward Peninsula Coast, AK
    #[doc(hidden)]
    AK211,
    /// Eastern Norton Sound and Nulato Hills, AK
    #[doc(hidden)]
    AK212,
    /// St Lawrence Island and Bering Strait Coast, AK
    #[doc(hidden)]
    AK213,
    /// Yukon Delta, AK
    #[doc(hidden)]
    AK214,
    /// Lower Yukon Valley, AK
    #[doc(hidden)]
    AK215,
    /// Lower Koyukuk and Middle Yukon Valleys, AK
    #[doc(hidden)]
    AK216,
    /// Upper Kobuk and Noatak Valleys, AK
    #[doc(hidden)]
    AK217,
    /// Southeastern Brooks Range, AK
    #[doc(hidden)]
    AK218,
    /// Upper Koyukuk Valley, AK
    #[doc(hidden)]
    AK219,
    /// Yukon Flats and Surrounding Uplands, AK
    #[doc(hidden)]
    AK220,
    /// Central Interior, AK
    #[doc(hidden)]
    AK221,
    /// Middle Tanana Valley, AK
    #[doc(hidden)]
    AK222,
    /// Deltana and Tanana Flats, AK
    #[doc(hidden)]
    AK223,
    /// Upper Tanana Valley and the Fortymile Country, AK
    #[doc(hidden)]
    AK224,
    /// Denali, AK
    #[doc(hidden)]
    AK225,
    /// Eastern Alaska Range, AK
    #[doc(hidden)]
    AK226,
    /// Upper Kuskokwim Valley, AK
    #[doc(hidden)]
    AK227,
    /// Lauderdale, AL
    #[doc(hidden)]
    AL001,
    /// Colbert, AL
    #[doc(hidden)]
    AL002,
    /// Franklin, AL
    #[doc(hidden)]
    AL003,
    /// Lawrence, AL
    #[doc(hidden)]
    AL004,
    /// Limestone, AL
    #[doc(hidden)]
    AL005,
    /// Madison, AL
    #[doc(hidden)]
    AL006,
    /// Morgan, AL
    #[doc(hidden)]
    AL007,
    /// Marshall, AL
    #[doc(hidden)]
    AL008,
    /// Jackson, AL
    #[doc(hidden)]
    AL009,
    /// DeKalb, AL
    #[doc(hidden)]
    AL010,
    /// Marion, AL
    #[doc(hidden)]
    AL011,
    /// Lamar, AL
    #[doc(hidden)]
    AL012,
    /// Fayette, AL
    #[doc(hidden)]
    AL013,
    /// Winston, AL
    #[doc(hidden)]
    AL014,
    /// Walker, AL
    #[doc(hidden)]
    AL015,
    /// Cullman, AL
    #[doc(hidden)]
    AL016,
    /// Blount, AL
    #[doc(hidden)]
    AL017,
    /// Etowah, AL
    #[doc(hidden)]
    AL018,
    /// Calhoun, AL
    #[doc(hidden)]
    AL019,
    /// Cherokee, AL
    #[doc(hidden)]
    AL020,
    /// Cleburne, AL
    #[doc(hidden)]
    AL021,
    /// Pickens, AL
    #[doc(hidden)]
    AL022,
    /// Tuscaloosa, AL
    #[doc(hidden)]
    AL023,
    /// Jefferson, AL
    #[doc(hidden)]
    AL024,
    /// Shelby, AL
    #[doc(hidden)]
    AL025,
    /// St. Clair, AL
    #[doc(hidden)]
    AL026,
    /// Talladega, AL
    #[doc(hidden)]
    AL027,
    /// Clay, AL
    #[doc(hidden)]
    AL028,
    /// Randolph, AL
    #[doc(hidden)]
    AL029,
    /// Sumter, AL
    #[doc(hidden)]
    AL030,
    /// Greene, AL
    #[doc(hidden)]
    AL031,
    /// Hale, AL
    #[doc(hidden)]
    AL032,
    /// Perry, AL
    #[doc(hidden)]
    AL033,
    /// Bibb, AL
    #[doc(hidden)]
    AL034,
    /// Chilton, AL
    #[doc(hidden)]
    AL035,
    /// Coosa, AL
    #[doc(hidden)]
    AL036,
    /// Tallapoosa, AL
    #[doc(hidden)]
    AL037,
    /// Chambers, AL
    #[doc(hidden)]
    AL038,
    /// Marengo, AL
    #[doc(hidden)]
    AL039,
    /// Dallas, AL
    #[doc(hidden)]
    AL040,
    /// Autauga, AL
    #[doc(hidden)]
    AL041,
    /// Lowndes, AL
    #[doc(hidden)]
    AL042,
    /// Elmore, AL
    #[doc(hidden)]
    AL043,
    /// Montgomery, AL
    #[doc(hidden)]
    AL044,
    /// Macon, AL
    #[doc(hidden)]
    AL045,
    /// Bullock, AL
    #[doc(hidden)]
    AL046,
    /// Lee, AL
    #[doc(hidden)]
    AL047,
    /// Russell, AL
    #[doc(hidden)]
    AL048,
    /// Pike, AL
    #[doc(hidden)]
    AL049,
    /// Barbour, AL
    #[doc(hidden)]
    AL050,
    /// Choctaw, AL
    #[doc(hidden)]
    AL051,
    /// Washington, AL
    #[doc(hidden)]
    AL052,
    /// Clarke, AL
    #[doc(hidden)]
    AL053,
    /// Wilcox, AL
    #[doc(hidden)]
    AL054,
    /// Monroe, AL
    #[doc(hidden)]
    AL055,
    /// Conecuh, AL
    #[doc(hidden)]
    AL056,
    /// Butler, AL
    #[doc(hidden)]
    AL057,
    /// Crenshaw, AL
    #[doc(hidden)]
    AL058,
    /// Escambia, AL
    #[doc(hidden)]
    AL059,
    /// Covington, AL
    #[doc(hidden)]
    AL060,
    /// Coffee, AL
    #[doc(hidden)]
    AL065,
    /// Dale, AL
    #[doc(hidden)]
    AL066,
    /// Henry, AL
    #[doc(hidden)]
    AL067,
    /// Geneva, AL
    #[doc(hidden)]
    AL068,
    /// Houston, AL
    #[doc(hidden)]
    AL069,
    /// Mobile Inland, AL
    #[doc(hidden)]
    AL261,
    /// Baldwin Inland, AL
    #[doc(hidden)]
    AL262,
    /// Mobile Central, AL
    #[doc(hidden)]
    AL263,
    /// Baldwin Central, AL
    #[doc(hidden)]
    AL264,
    /// Mobile Coastal, AL
    #[doc(hidden)]
    AL265,
    /// Baldwin Coastal, AL
    #[doc(hidden)]
    AL266,
    /// Benton, AR
    #[doc(hidden)]
    AR001,
    /// Carroll, AR
    #[doc(hidden)]
    AR002,
    /// Marion, AR
    #[doc(hidden)]
    AR004,
    /// Baxter, AR
    #[doc(hidden)]
    AR005,
    /// Fulton, AR
    #[doc(hidden)]
    AR006,
    /// Sharp, AR
    #[doc(hidden)]
    AR007,
    /// Randolph, AR
    #[doc(hidden)]
    AR008,
    /// Clay, AR
    #[doc(hidden)]
    AR009,
    /// Washington, AR
    #[doc(hidden)]
    AR010,
    /// Madison, AR
    #[doc(hidden)]
    AR011,
    /// Stone, AR
    #[doc(hidden)]
    AR014,
    /// Izard, AR
    #[doc(hidden)]
    AR015,
    /// Independence, AR
    #[doc(hidden)]
    AR016,
    /// Lawrence, AR
    #[doc(hidden)]
    AR017,
    /// Greene, AR
    #[doc(hidden)]
    AR018,
    /// Crawford, AR
    #[doc(hidden)]
    AR019,
    /// Franklin, AR
    #[doc(hidden)]
    AR020,
    /// Cleburne, AR
    #[doc(hidden)]
    AR024,
    /// Jackson, AR
    #[doc(hidden)]
    AR025,
    /// Craighead, AR
    #[doc(hidden)]
    AR026,
    /// Poinsett, AR
    #[doc(hidden)]
    AR027,
    /// Mississippi, AR
    #[doc(hidden)]
    AR028,
    /// Sebastian, AR
    #[doc(hidden)]
    AR029,
    /// Conway, AR
    #[doc(hidden)]
    AR031,
    /// Faulkner, AR
    #[doc(hidden)]
    AR032,
    /// White, AR
    #[doc(hidden)]
    AR033,
    /// Woodruff, AR
    #[doc(hidden)]
    AR034,
    /// Cross, AR
    #[doc(hidden)]
    AR035,
    /// Crittenden, AR
    #[doc(hidden)]
    AR036,
    /// Perry, AR
    #[doc(hidden)]
    AR039,
    /// Garland, AR
    #[doc(hidden)]
    AR042,
    /// Saline, AR
    #[doc(hidden)]
    AR043,
    /// Pulaski, AR
    #[doc(hidden)]
    AR044,
    /// Lonoke, AR
    #[doc(hidden)]
    AR045,
    /// Prairie, AR
    #[doc(hidden)]
    AR046,
    /// Monroe, AR
    #[doc(hidden)]
    AR047,
    /// St. Francis, AR
    #[doc(hidden)]
    AR048,
    /// Lee, AR
    #[doc(hidden)]
    AR049,
    /// Sevier, AR
    #[doc(hidden)]
    AR050,
    /// Howard, AR
    #[doc(hidden)]
    AR051,
    /// Pike, AR
    #[doc(hidden)]
    AR052,
    /// Clark, AR
    #[doc(hidden)]
    AR053,
    /// Hot Spring, AR
    #[doc(hidden)]
    AR054,
    /// Grant, AR
    #[doc(hidden)]
    AR055,
    /// Jefferson, AR
    #[doc(hidden)]
    AR056,
    /// Arkansas, AR
    #[doc(hidden)]
    AR057,
    /// Phillips, AR
    #[doc(hidden)]
    AR058,
    /// Little River, AR
    #[doc(hidden)]
    AR059,
    /// Hempstead, AR
    #[doc(hidden)]
    AR060,
    /// Nevada, AR
    #[doc(hidden)]
    AR061,
    /// Dallas, AR
    #[doc(hidden)]
    AR062,
    /// Cleveland, AR
    #[doc(hidden)]
    AR063,
    /// Lincoln, AR
    #[doc(hidden)]
    AR064,
    /// Desha, AR
    #[doc(hidden)]
    AR065,
    /// Ouachita, AR
    #[doc(hidden)]
    AR066,
    /// Calhoun, AR
    #[doc(hidden)]
    AR067,
    /// Bradley, AR
    #[doc(hidden)]
    AR068,
    /// Drew, AR
    #[doc(hidden)]
    AR069,
    /// Miller, AR
    #[doc(hidden)]
    AR070,
    /// Lafayette, AR
    #[doc(hidden)]
    AR071,
    /// Columbia, AR
    #[doc(hidden)]
    AR072,
    /// Union, AR
    #[doc(hidden)]
    AR073,
    /// Ashley, AR
    #[doc(hidden)]
    AR074,
    /// Chicot, AR
    #[doc(hidden)]
    AR075,
    /// Boone County Except Southwest, AR
    #[doc(hidden)]
    AR103,
    /// Newton County Higher Elevations, AR
    #[doc(hidden)]
    AR112,
    /// Searcy County Lower Elevations, AR
    #[doc(hidden)]
    AR113,
    /// Southern Johnson County, AR
    #[doc(hidden)]
    AR121,
    /// Southern Pope County, AR
    #[doc(hidden)]
    AR122,
    /// Southeast Van Buren County, AR
    #[doc(hidden)]
    AR123,
    /// Western and Northern Logan County, AR
    #[doc(hidden)]
    AR130,
    /// Northern Scott County, AR
    #[doc(hidden)]
    AR137,
    /// Northwest Yell County, AR
    #[doc(hidden)]
    AR138,
    /// Polk County Lower Elevations, AR
    #[doc(hidden)]
    AR140,
    /// Central and Eastern Montgomery County, AR
    #[doc(hidden)]
    AR141,
    /// Boone County Higher Elevations, AR
    #[doc(hidden)]
    AR203,
    /// Newton County Lower Elevations, AR
    #[doc(hidden)]
    AR212,
    /// Northwest Searcy County Higher Elevations, AR
    #[doc(hidden)]
    AR213,
    /// Johnson County Higher Elevations, AR
    #[doc(hidden)]
    AR221,
    /// Pope County Higher Elevations, AR
    #[doc(hidden)]
    AR222,
    /// Van Buren County Higher Elevations, AR
    #[doc(hidden)]
    AR223,
    /// Southern and Eastern Logan County, AR
    #[doc(hidden)]
    AR230,
    /// Central and Southern Scott County, AR
    #[doc(hidden)]
    AR237,
    /// Yell Excluding Northwest, AR
    #[doc(hidden)]
    AR238,
    /// Northern Polk County Higher Elevations, AR
    #[doc(hidden)]
    AR240,
    /// Northern Montgomery County Higher Elevations, AR
    #[doc(hidden)]
    AR241,
    /// Eastern, Central, and Southern Searcy County Higher Elevations, AR
    #[doc(hidden)]
    AR313,
    /// Southeast Polk County Higher Elevations, AR
    #[doc(hidden)]
    AR340,
    /// Southwest Montgomery County Higher Elevations, AR
    #[doc(hidden)]
    AR341,
    /// Tutuila and Aunuu, AS
    #[doc(hidden)]
    AS001,
    /// Manua, AS
    #[doc(hidden)]
    AS002,
    /// Swains Island, AS
    #[doc(hidden)]
    AS003,
    /// Northwest Plateau, AZ
    #[doc(hidden)]
    AZ001,
    /// Lake Havasu and Fort Mohave, AZ
    #[doc(hidden)]
    AZ002,
    /// Northwest Deserts, AZ
    #[doc(hidden)]
    AZ003,
    /// Kaibab Plateau, AZ
    #[doc(hidden)]
    AZ004,
    /// Marble and Glen Canyons, AZ
    #[doc(hidden)]
    AZ005,
    /// Grand Canyon Country, AZ
    #[doc(hidden)]
    AZ006,
    /// Coconino Plateau, AZ
    #[doc(hidden)]
    AZ007,
    /// Yavapai County  Mountains, AZ
    #[doc(hidden)]
    AZ008,
    /// Northeast Plateaus and Mesas Hwy 264 Northward, AZ
    #[doc(hidden)]
    AZ009,
    /// Chinle Valley, AZ
    #[doc(hidden)]
    AZ010,
    /// Chuska Mountains and Defiance Plateau, AZ
    #[doc(hidden)]
    AZ011,
    /// Little Colorado River Valley in Coconino County, AZ
    #[doc(hidden)]
    AZ012,
    /// Little Colorado River Valley in Navajo County, AZ
    #[doc(hidden)]
    AZ013,
    /// Little Colorado River Valley in Apache County, AZ
    #[doc(hidden)]
    AZ014,
    /// Western Mogollon Rim, AZ
    #[doc(hidden)]
    AZ015,
    /// Eastern Mogollon Rim, AZ
    #[doc(hidden)]
    AZ016,
    /// White Mountains, AZ
    #[doc(hidden)]
    AZ017,
    /// Northern Gila County, AZ
    #[doc(hidden)]
    AZ018,
    /// Lake Mead National Recreation Area, AZ
    #[doc(hidden)]
    AZ036,
    /// Yavapai County Valleys and Basins, AZ
    #[doc(hidden)]
    AZ037,
    /// Oak Creek and Sycamore Canyons, AZ
    #[doc(hidden)]
    AZ038,
    /// Black Mesa Area, AZ
    #[doc(hidden)]
    AZ039,
    /// Northeast Plateaus and Mesas South of Hwy 264, AZ
    #[doc(hidden)]
    AZ040,
    /// Western Pima County Including Ajo/Organ Pipe Cactus National Monument, AZ
    #[doc(hidden)]
    AZ501,
    /// Tohono O'odham Nation including Sells, AZ
    #[doc(hidden)]
    AZ502,
    /// Upper Santa Cruz River and Altar Valleys including Nogales, AZ
    #[doc(hidden)]
    AZ503,
    /// Tucson Metro Area including Tucson/Green Valley/Marana/Vail, AZ
    #[doc(hidden)]
    AZ504,
    /// South Central Pinal County including Eloy/Picacho Peak State Park, AZ
    #[doc(hidden)]
    AZ505,
    /// Southeast Pinal County including Kearny/Mammoth/Oracle, AZ
    #[doc(hidden)]
    AZ506,
    /// Upper San Pedro River Valley including Sierra Vista/Benson, AZ
    #[doc(hidden)]
    AZ507,
    /// Eastern Cochise County Below 5000 Feet including Douglas/Willcox, AZ
    #[doc(hidden)]
    AZ508,
    /// Upper Gila River and Aravaipa Valleys including Clifton/Safford, AZ
    #[doc(hidden)]
    AZ509,
    /// White Mountains of Graham and Greenlee Counties including Hannagan Meadow, AZ
    #[doc(hidden)]
    AZ510,
    /// Galiuro and Pinaleno Mountains including Mount Graham, AZ
    #[doc(hidden)]
    AZ511,
    /// Chiricahua Mountains including Chiricahua National Monument, AZ
    #[doc(hidden)]
    AZ512,
    /// Dragoon/Mule/Huachuca and Santa Rita Mountains including Bisbee/Canelo Hills/Madera Canyon, AZ
    #[doc(hidden)]
    AZ513,
    /// Santa Catalina and Rincon Mountains including Mount Lemmon/Summerhaven, AZ
    #[doc(hidden)]
    AZ514,
    /// Baboquivari Mountains including Kitt Peak, AZ
    #[doc(hidden)]
    AZ515,
    /// Parker Valley, AZ
    #[doc(hidden)]
    AZ530,
    /// Kofa, AZ
    #[doc(hidden)]
    AZ531,
    /// Yuma, AZ
    #[doc(hidden)]
    AZ532,
    /// Central La Paz, AZ
    #[doc(hidden)]
    AZ533,
    /// Aguila Valley, AZ
    #[doc(hidden)]
    AZ534,
    /// Southeast Yuma County, AZ
    #[doc(hidden)]
    AZ535,
    /// Gila River Valley, AZ
    #[doc(hidden)]
    AZ536,
    /// Northwest Valley, AZ
    #[doc(hidden)]
    AZ537,
    /// Tonopah Desert, AZ
    #[doc(hidden)]
    AZ538,
    /// Gila Bend, AZ
    #[doc(hidden)]
    AZ539,
    /// Buckeye/Avondale, AZ
    #[doc(hidden)]
    AZ540,
    /// Cave Creek/New River, AZ
    #[doc(hidden)]
    AZ541,
    /// Deer Valley, AZ
    #[doc(hidden)]
    AZ542,
    /// Central Phoenix, AZ
    #[doc(hidden)]
    AZ543,
    /// North Phoenix/Glendale, AZ
    #[doc(hidden)]
    AZ544,
    /// New River Mesa, AZ
    #[doc(hidden)]
    AZ545,
    /// Scottsdale/Paradise Valley, AZ
    #[doc(hidden)]
    AZ546,
    /// Rio Verde/Salt River, AZ
    #[doc(hidden)]
    AZ547,
    /// East Valley, AZ
    #[doc(hidden)]
    AZ548,
    /// Fountain Hills/East Mesa, AZ
    #[doc(hidden)]
    AZ549,
    /// South Mountain/Ahwatukee, AZ
    #[doc(hidden)]
    AZ550,
    /// Southeast Valley/Queen Creek, AZ
    #[doc(hidden)]
    AZ551,
    /// Superior, AZ
    #[doc(hidden)]
    AZ552,
    /// Northwest Pinal County, AZ
    #[doc(hidden)]
    AZ553,
    /// West Pinal County, AZ
    #[doc(hidden)]
    AZ554,
    /// Apache Junction/Gold Canyon, AZ
    #[doc(hidden)]
    AZ555,
    /// Tonto Basin, AZ
    #[doc(hidden)]
    AZ556,
    /// Mazatzal Mountains, AZ
    #[doc(hidden)]
    AZ557,
    /// Pinal/Superstition Mountains, AZ
    #[doc(hidden)]
    AZ558,
    /// Sonoran Desert Natl Monument, AZ
    #[doc(hidden)]
    AZ559,
    /// San Carlos, AZ
    #[doc(hidden)]
    AZ560,
    /// Dripping Springs, AZ
    #[doc(hidden)]
    AZ561,
    /// Globe/Miami, AZ
    #[doc(hidden)]
    AZ562,
    /// Southeast Gila County, AZ
    #[doc(hidden)]
    AZ563,
    /// San Francisco, CA
    #[doc(hidden)]
    CA006,
    /// Shasta Lake Area / Northern Shasta County, CA
    #[doc(hidden)]
    CA013,
    /// Burney Basin / Eastern Shasta County, CA
    #[doc(hidden)]
    CA014,
    /// Northern Sacramento Valley, CA
    #[doc(hidden)]
    CA015,
    /// Central Sacramento Valley, CA
    #[doc(hidden)]
    CA016,
    /// Southern Sacramento Valley, CA
    #[doc(hidden)]
    CA017,
    /// Carquinez Strait and Delta, CA
    #[doc(hidden)]
    CA018,
    /// Northern San Joaquin Valley, CA
    #[doc(hidden)]
    CA019,
    /// Cuyama Valley, CA
    #[doc(hidden)]
    CA038,
    /// San Diego County Coastal Areas, CA
    #[doc(hidden)]
    CA043,
    /// San Bernardino and Riverside County Valleys-The Inland Empire, CA
    #[doc(hidden)]
    CA048,
    /// San Diego County Inland Valleys, CA
    #[doc(hidden)]
    CA050,
    /// Ventura County Mountains, CA
    #[doc(hidden)]
    CA053,
    /// Los Angeles County Mountains, CA
    #[doc(hidden)]
    CA054,
    /// San Bernardino County Mountains, CA
    #[doc(hidden)]
    CA055,
    /// Riverside County Mountains, CA
    #[doc(hidden)]
    CA056,
    /// Santa Ana Mountains and Foothills, CA
    #[doc(hidden)]
    CA057,
    /// San Diego County Mountains, CA
    #[doc(hidden)]
    CA058,
    /// Antelope Valley, CA
    #[doc(hidden)]
    CA059,
    /// Apple and Lucerne Valleys, CA
    #[doc(hidden)]
    CA060,
    /// Coachella Valley, CA
    #[doc(hidden)]
    CA061,
    /// San Diego County Deserts, CA
    #[doc(hidden)]
    CA062,
    /// Mountains Southwestern Shasta County to Western Colusa County, CA
    #[doc(hidden)]
    CA063,
    /// San Gorgonio Pass Near Banning, CA
    #[doc(hidden)]
    CA065,
    /// Northeast Foothills/Sacramento Valley, CA
    #[doc(hidden)]
    CA066,
    /// Motherlode, CA
    #[doc(hidden)]
    CA067,
    /// Western Plumas County/Lassen Park, CA
    #[doc(hidden)]
    CA068,
    /// West Slope Northern Sierra Nevada, CA
    #[doc(hidden)]
    CA069,
    /// Surprise Valley California, CA
    #[doc(hidden)]
    CA070,
    /// Lassen-Eastern Plumas-Eastern Sierra Counties, CA
    #[doc(hidden)]
    CA071,
    /// Greater Lake Tahoe Area, CA
    #[doc(hidden)]
    CA072,
    /// Mono, CA
    #[doc(hidden)]
    CA073,
    /// Western Siskiyou County, CA
    #[doc(hidden)]
    CA080,
    /// Central Siskiyou County, CA
    #[doc(hidden)]
    CA081,
    /// South Central Siskiyou County, CA
    #[doc(hidden)]
    CA082,
    /// North Central and Southeast Siskiyou County, CA
    #[doc(hidden)]
    CA083,
    /// Northeast Siskiyou and Northwest Modoc Counties, CA
    #[doc(hidden)]
    CA084,
    /// Modoc County, CA
    #[doc(hidden)]
    CA085,
    /// Catalina and Santa Barbara Islands, CA
    #[doc(hidden)]
    CA087,
    /// Santa Clarita Valley, CA
    #[doc(hidden)]
    CA088,
    /// Coastal Del Norte, CA
    #[doc(hidden)]
    CA101,
    /// Del Norte Interior, CA
    #[doc(hidden)]
    CA102,
    /// Northern Humboldt Coast, CA
    #[doc(hidden)]
    CA103,
    /// Southwestern Humboldt, CA
    #[doc(hidden)]
    CA104,
    /// Northern Humboldt Interior, CA
    #[doc(hidden)]
    CA105,
    /// Southern Humboldt Interior, CA
    #[doc(hidden)]
    CA106,
    /// Northern Trinity, CA
    #[doc(hidden)]
    CA107,
    /// Southern Trinity, CA
    #[doc(hidden)]
    CA108,
    /// Mendocino Coast, CA
    #[doc(hidden)]
    CA109,
    /// Northwestern Mendocino Interior, CA
    #[doc(hidden)]
    CA110,
    /// Northeastern Mendocino Interior, CA
    #[doc(hidden)]
    CA111,
    /// Southwestern Mendocino Interior, CA
    #[doc(hidden)]
    CA112,
    /// Southeastern Mendocino Interior, CA
    #[doc(hidden)]
    CA113,
    /// Northern Lake County, CA
    #[doc(hidden)]
    CA114,
    /// Southern Lake County, CA
    #[doc(hidden)]
    CA115,
    /// West Side Mountains north of 198, CA
    #[doc(hidden)]
    CA300,
    /// Los Banos - Dos Palos, CA
    #[doc(hidden)]
    CA301,
    /// Merced - Madera - Mendota, CA
    #[doc(hidden)]
    CA302,
    /// Planada - Le Grand - Snelling, CA
    #[doc(hidden)]
    CA303,
    /// Coalinga - Avenal, CA
    #[doc(hidden)]
    CA304,
    /// West Side of Fresno and Kings Counties, CA
    #[doc(hidden)]
    CA305,
    /// Caruthers - San Joaquin - Selma, CA
    #[doc(hidden)]
    CA306,
    /// Fresno-Clovis, CA
    #[doc(hidden)]
    CA307,
    /// West Side Mountains South of 198, CA
    #[doc(hidden)]
    CA308,
    /// Buttonwillow - Lost Hills - I5, CA
    #[doc(hidden)]
    CA309,
    /// Delano-Wasco-Shafter, CA
    #[doc(hidden)]
    CA310,
    /// Hanford - Corcoran - Lemoore, CA
    #[doc(hidden)]
    CA311,
    /// Visalia - Porterville - Reedley, CA
    #[doc(hidden)]
    CA312,
    /// Buena Vista, CA
    #[doc(hidden)]
    CA313,
    /// Bakersfield, CA
    #[doc(hidden)]
    CA314,
    /// Southeast San Joaquin Valley, CA
    #[doc(hidden)]
    CA315,
    /// South End San Joaquin Valley, CA
    #[doc(hidden)]
    CA316,
    /// Mariposa Madera Foothills, CA
    #[doc(hidden)]
    CA317,
    /// Mariposa-Madera Lower Sierra, CA
    #[doc(hidden)]
    CA318,
    /// Fresno-Tulare Foothills, CA
    #[doc(hidden)]
    CA319,
    /// Fresno-Tulare Lower Sierra, CA
    #[doc(hidden)]
    CA320,
    /// South End Sierra Foothills, CA
    #[doc(hidden)]
    CA321,
    /// South End of the Lower Sierra, CA
    #[doc(hidden)]
    CA322,
    /// Yosemite NP outside of the valley, CA
    #[doc(hidden)]
    CA323,
    /// Yosemite Valley, CA
    #[doc(hidden)]
    CA324,
    /// San Joaquin River Canyon, CA
    #[doc(hidden)]
    CA325,
    /// Upper San Joaquin River, CA
    #[doc(hidden)]
    CA326,
    /// Kaiser to Rodgers Ridge, CA
    #[doc(hidden)]
    CA327,
    /// Kings Canyon NP, CA
    #[doc(hidden)]
    CA328,
    /// Grant Grove Area, CA
    #[doc(hidden)]
    CA329,
    /// Sequoia NP, CA
    #[doc(hidden)]
    CA330,
    /// South End of the Upper Sierra, CA
    #[doc(hidden)]
    CA331,
    /// Kern River Valley, CA
    #[doc(hidden)]
    CA332,
    /// Piute Walker Basin, CA
    #[doc(hidden)]
    CA333,
    /// Tehachapi, CA
    #[doc(hidden)]
    CA334,
    /// Grapevine, CA
    #[doc(hidden)]
    CA335,
    /// Frazier Mountain Communities, CA
    #[doc(hidden)]
    CA336,
    /// Indian Wells Valley, CA
    #[doc(hidden)]
    CA337,
    /// Mojave Desert Slopes, CA
    #[doc(hidden)]
    CA338,
    /// Mojave Desert, CA
    #[doc(hidden)]
    CA339,
    /// San Luis Obispo County Beaches, CA
    #[doc(hidden)]
    CA340,
    /// San Luis Obispo County Inland Central Coast, CA
    #[doc(hidden)]
    CA341,
    /// Santa Lucia Mountains, CA
    #[doc(hidden)]
    CA342,
    /// Southern Salinas Valley, CA
    #[doc(hidden)]
    CA343,
    /// San Luis Obispo County Interior Valleys, CA
    #[doc(hidden)]
    CA344,
    /// San Luis Obispo County Mountains, CA
    #[doc(hidden)]
    CA345,
    /// Santa Barbara County Central Coast Beaches, CA
    #[doc(hidden)]
    CA346,
    /// Santa Barbara County Inland Central Coast, CA
    #[doc(hidden)]
    CA347,
    /// Santa Ynez Valley, CA
    #[doc(hidden)]
    CA348,
    /// Santa Barbara County Southwestern Coast, CA
    #[doc(hidden)]
    CA349,
    /// Santa Barbara County Southeastern Coast, CA
    #[doc(hidden)]
    CA350,
    /// Santa Ynez Mountains Western Range, CA
    #[doc(hidden)]
    CA351,
    /// Santa Ynez Mountains Eastern Range, CA
    #[doc(hidden)]
    CA352,
    /// Santa Barbara County Interior Mountains, CA
    #[doc(hidden)]
    CA353,
    /// Ventura County Beaches, CA
    #[doc(hidden)]
    CA354,
    /// Ventura County Inland Coast, CA
    #[doc(hidden)]
    CA355,
    /// Lake Casitas, CA
    #[doc(hidden)]
    CA356,
    /// Ojai Valley, CA
    #[doc(hidden)]
    CA357,
    /// Central Ventura County Valleys, CA
    #[doc(hidden)]
    CA358,
    /// Southeastern Ventura County Valleys, CA
    #[doc(hidden)]
    CA359,
    /// Malibu Coast, CA
    #[doc(hidden)]
    CA362,
    /// Santa Monica Mountains, CA
    #[doc(hidden)]
    CA363,
    /// Los Angeles County Beaches, CA
    #[doc(hidden)]
    CA364,
    /// Los Angeles County Inland Coast including Downtown Los Angeles, CA
    #[doc(hidden)]
    CA365,
    /// Marin Coastal Range, CA
    #[doc(hidden)]
    CA502,
    /// Sonoma Coastal Range, CA
    #[doc(hidden)]
    CA503,
    /// North Bay Interior Mountains, CA
    #[doc(hidden)]
    CA504,
    /// Coastal North Bay Including Point Reyes National Seashore, CA
    #[doc(hidden)]
    CA505,
    /// North Bay Interior Valleys, CA
    #[doc(hidden)]
    CA506,
    /// San Francisco Bay Shoreline, CA
    #[doc(hidden)]
    CA508,
    /// San Fransisco Peninsula Coast, CA
    #[doc(hidden)]
    CA509,
    /// East Bay Interior Valleys, CA
    #[doc(hidden)]
    CA510,
    /// Santa Cruz Mountains, CA
    #[doc(hidden)]
    CA512,
    /// Santa Clara Valley Including San Jose, CA
    #[doc(hidden)]
    CA513,
    /// Eastern Santa Clara Hills, CA
    #[doc(hidden)]
    CA514,
    /// East Bay Hills, CA
    #[doc(hidden)]
    CA515,
    /// Southern Salinas Valley/Arroyo Seco and Lake San Antonio, CA
    #[doc(hidden)]
    CA516,
    /// Santa Lucia Mountains and Los Padres National Forest, CA
    #[doc(hidden)]
    CA517,
    /// Mountains Of San Benito County And Interior Monterey County Including Pinnacles National Monument, CA
    #[doc(hidden)]
    CA518,
    /// Eastern Sierra Slopes of Inyo County, CA
    #[doc(hidden)]
    CA519,
    /// Owens Valley, CA
    #[doc(hidden)]
    CA520,
    /// White Mountains of Inyo County, CA
    #[doc(hidden)]
    CA521,
    /// Death Valley National Park, CA
    #[doc(hidden)]
    CA522,
    /// Western Mojave Desert, CA
    #[doc(hidden)]
    CA523,
    /// Eastern Mojave Desert, Including the Mojave National Preserve, CA
    #[doc(hidden)]
    CA524,
    /// Morongo Basin, CA
    #[doc(hidden)]
    CA525,
    /// Cadiz Basin, CA
    #[doc(hidden)]
    CA526,
    /// San Bernardino County-Upper Colorado River Valley, CA
    #[doc(hidden)]
    CA527,
    /// Northern Salinas Valley/Hollister Valley and Carmel Valley, CA
    #[doc(hidden)]
    CA528,
    /// Northern Monterey Bay, CA
    #[doc(hidden)]
    CA529,
    /// Southern Monterey Bay and Big Sur Coast, CA
    #[doc(hidden)]
    CA530,
    /// Los Angeles County San Fernando Valley, CA
    #[doc(hidden)]
    CA547,
    /// Los Angeles County San Gabriel Valley, CA
    #[doc(hidden)]
    CA548,
    /// San Miguel and Santa Rosa Islands, CA
    #[doc(hidden)]
    CA549,
    /// Santa Cruz and Anacapa Islands, CA
    #[doc(hidden)]
    CA550,
    /// Orange County Coastal, CA
    #[doc(hidden)]
    CA552,
    /// Orange County Inland, CA
    #[doc(hidden)]
    CA554,
    /// Joshua Tree NP West, CA
    #[doc(hidden)]
    CA560,
    /// Joshua Tree NP East, CA
    #[doc(hidden)]
    CA561,
    /// Imperial County Southwest, CA
    #[doc(hidden)]
    CA562,
    /// Salton Sea, CA
    #[doc(hidden)]
    CA563,
    /// Chuckwalla Mountains, CA
    #[doc(hidden)]
    CA564,
    /// Imperial County Southeast, CA
    #[doc(hidden)]
    CA565,
    /// Imperial County West, CA
    #[doc(hidden)]
    CA566,
    /// Imperial Valley, CA
    #[doc(hidden)]
    CA567,
    /// Chiriaco Summit, CA
    #[doc(hidden)]
    CA568,
    /// Palo Verde Valley, CA
    #[doc(hidden)]
    CA569,
    /// Chuckwalla Valley, CA
    #[doc(hidden)]
    CA570,
    /// Lower Yampa River Basin, CO
    #[doc(hidden)]
    CO001,
    /// Central Yampa River Basin, CO
    #[doc(hidden)]
    CO002,
    /// Roan and Tavaputs Plateaus, CO
    #[doc(hidden)]
    CO003,
    /// Elkhead and Park Mountains, CO
    #[doc(hidden)]
    CO004,
    /// Upper Yampa River Basin, CO
    #[doc(hidden)]
    CO005,
    /// Grand Valley, CO
    #[doc(hidden)]
    CO006,
    /// Debeque to Silt Corridor, CO
    #[doc(hidden)]
    CO007,
    /// Central Colorado River Basin, CO
    #[doc(hidden)]
    CO008,
    /// Grand and Battlement Mesas, CO
    #[doc(hidden)]
    CO009,
    /// Gore and Elk Mountains/Central Mountain Valleys, CO
    #[doc(hidden)]
    CO010,
    /// Central Gunnison and Uncompahgre River Basin, CO
    #[doc(hidden)]
    CO011,
    /// West Elk and Sawatch Mountains, CO
    #[doc(hidden)]
    CO012,
    /// Flat Tops, CO
    #[doc(hidden)]
    CO013,
    /// Upper Gunnison River Valley, CO
    #[doc(hidden)]
    CO014,
    /// Uncompahgre Plateau/Dallas Divide, CO
    #[doc(hidden)]
    CO017,
    /// Northwestern San Juan Mountains, CO
    #[doc(hidden)]
    CO018,
    /// Southwest San Juan Mountains, CO
    #[doc(hidden)]
    CO019,
    /// Paradox Valley/Lower Dolores River, CO
    #[doc(hidden)]
    CO020,
    /// Four Corners/Upper Dolores River, CO
    #[doc(hidden)]
    CO021,
    /// Animas River Basin, CO
    #[doc(hidden)]
    CO022,
    /// San Juan River Basin, CO
    #[doc(hidden)]
    CO023,
    /// Jackson County Below 9000 Feet, CO
    #[doc(hidden)]
    CO030,
    /// West Jackson and West Grand Counties Above 9000 Feet, CO
    #[doc(hidden)]
    CO031,
    /// Grand and Summit Counties Below 9000 Feet, CO
    #[doc(hidden)]
    CO032,
    /// South and East Jackson/Larimer/North and Northeast Grand/Northwest Boulder Counties Above 9000 Feet, CO
    #[doc(hidden)]
    CO033,
    /// South and Southeast Grand/West Central and Southwest Boulder/Gilpin/Clear Creek/Summit/North and West Park Counties Above 9000 Feet, CO
    #[doc(hidden)]
    CO034,
    /// Larimer and Boulder Counties Between 6000 and 9000 Feet, CO
    #[doc(hidden)]
    CO035,
    /// Jefferson and West Douglas Counties Above  6000 Feet/Gilpin/Clear Creek/Northeast Park Counties Below 9000 Feet, CO
    #[doc(hidden)]
    CO036,
    /// Central and Southeast Park County, CO
    #[doc(hidden)]
    CO037,
    /// Larimer County Below 6000 Feet/Northwest Weld County, CO
    #[doc(hidden)]
    CO038,
    /// Boulder And Jefferson Counties Below 6000 Feet/West Broomfield County, CO
    #[doc(hidden)]
    CO039,
    /// North Douglas County Below 6000 Feet/Denver/West Adams and Arapahoe Counties/East Broomfield County, CO
    #[doc(hidden)]
    CO040,
    /// Elbert/Central and East Douglas Counties Above 6000 Feet, CO
    #[doc(hidden)]
    CO041,
    /// Northeast Weld County, CO
    #[doc(hidden)]
    CO042,
    /// Central and South Weld County, CO
    #[doc(hidden)]
    CO043,
    /// Morgan County, CO
    #[doc(hidden)]
    CO044,
    /// Central and East Adams and Arapahoe Counties, CO
    #[doc(hidden)]
    CO045,
    /// North and Northeast Elbert County Below 6000 Feet/North Lincoln County, CO
    #[doc(hidden)]
    CO046,
    /// Southeast Elbert County Below 6000 Feet/South Lincoln County, CO
    #[doc(hidden)]
    CO047,
    /// Logan County, CO
    #[doc(hidden)]
    CO048,
    /// Washington County, CO
    #[doc(hidden)]
    CO049,
    /// Sedgwick County, CO
    #[doc(hidden)]
    CO050,
    /// Phillips County, CO
    #[doc(hidden)]
    CO051,
    /// Western Mosquito Range/East Lake County Above 11000 Ft, CO
    #[doc(hidden)]
    CO058,
    /// Leadville Vicinity/Lake County Below 11000 Ft, CO
    #[doc(hidden)]
    CO059,
    /// Eastern Sawatch Mountains above 11000 Ft, CO
    #[doc(hidden)]
    CO060,
    /// Western Chaffee County Between 9000 and 11000 Ft, CO
    #[doc(hidden)]
    CO061,
    /// Central Chaffee County Below 9000 Ft, CO
    #[doc(hidden)]
    CO062,
    /// Western Mosquito Range/East Chaffee County above 9000Ft, CO
    #[doc(hidden)]
    CO063,
    /// Saguache County West of Continental Divide Below 10000 Ft, CO
    #[doc(hidden)]
    CO064,
    /// Saguache County East of Continental Divide below 10000 Ft, CO
    #[doc(hidden)]
    CO065,
    /// La Garita Mountains Above 10000 Ft, CO
    #[doc(hidden)]
    CO066,
    /// Upper Rio Grande Valley/Eastern San Juan Mountains Below 10000 Ft, CO
    #[doc(hidden)]
    CO067,
    /// Eastern San Juan Mountains Above 10000 Ft, CO
    #[doc(hidden)]
    CO068,
    /// Del Norte Vicinity/Northern San Luis Valley Below 8500 Ft, CO
    #[doc(hidden)]
    CO069,
    /// Alamosa  Vicinity/Central San Luis Valley Below 8500 Ft, CO
    #[doc(hidden)]
    CO070,
    /// Southern San Luis Valley, CO
    #[doc(hidden)]
    CO071,
    /// Northern Sangre de Cristo Mountains Between 8500 And 11000 Ft, CO
    #[doc(hidden)]
    CO072,
    /// Northern Sangre de Cristo Mountains above 11000 Ft, CO
    #[doc(hidden)]
    CO073,
    /// Southern Sangre De Cristo Mountains Between 7500 and 11000 Ft, CO
    #[doc(hidden)]
    CO074,
    /// Southern Sangre De Cristo Mountains Above 11000 Ft, CO
    #[doc(hidden)]
    CO075,
    /// Northwestern Fremont County  Above 8500Ft, CO
    #[doc(hidden)]
    CO076,
    /// Western/Central Fremont County Below 8500 Ft, CO
    #[doc(hidden)]
    CO077,
    /// Wet Mountain Valley Below 8500 Ft, CO
    #[doc(hidden)]
    CO078,
    /// Wet Mountains between 6300 and 10000Ft, CO
    #[doc(hidden)]
    CO079,
    /// Wet Mountains above 10000 Ft, CO
    #[doc(hidden)]
    CO080,
    /// Teller County/Rampart Range above 7500fT/Pike's Peak Between 7500 And 11000 Ft, CO
    #[doc(hidden)]
    CO081,
    /// Pikes Peak above 11000 Ft, CO
    #[doc(hidden)]
    CO082,
    /// Canon City Vicinity/Eastern Fremont County, CO
    #[doc(hidden)]
    CO083,
    /// Northern El Paso County/Monument Ridge/Rampart Range Below 7500 Ft, CO
    #[doc(hidden)]
    CO084,
    /// Colorado Springs Vicinity/Southern El Paso County/Rampart Range Below 7400 Ft, CO
    #[doc(hidden)]
    CO085,
    /// Pueblo Vicinity/Pueblo County Below 6300 Feet, CO
    #[doc(hidden)]
    CO086,
    /// Walsenburg Vicinity/Upper Huerfano River Basin Below 7500 Ft, CO
    #[doc(hidden)]
    CO087,
    /// Trinidad Vicinity/Western Las Animas County Below 7500 Ft, CO
    #[doc(hidden)]
    CO088,
    /// Crowley County, CO
    #[doc(hidden)]
    CO089,
    /// Yuma County, CO
    #[doc(hidden)]
    CO090,
    /// Kit Carson County, CO
    #[doc(hidden)]
    CO091,
    /// Cheyenne County, CO
    #[doc(hidden)]
    CO092,
    /// La Junta Vicinity/Otero County, CO
    #[doc(hidden)]
    CO093,
    /// Eastern Las Animas County, CO
    #[doc(hidden)]
    CO094,
    /// Western Kiowa County, CO
    #[doc(hidden)]
    CO095,
    /// Eastern Kiowa County, CO
    #[doc(hidden)]
    CO096,
    /// Las Animas Vicinity/Bent County, CO
    #[doc(hidden)]
    CO097,
    /// Lamar Vicinity/Prowers County, CO
    #[doc(hidden)]
    CO098,
    /// Springfield Vicinity/Baca County, CO
    #[doc(hidden)]
    CO099,
    /// Northern Litchfield, CT
    #[doc(hidden)]
    CT001,
    /// Hartford, CT
    #[doc(hidden)]
    CT002,
    /// Tolland, CT
    #[doc(hidden)]
    CT003,
    /// Windham, CT
    #[doc(hidden)]
    CT004,
    /// Northern Fairfield, CT
    #[doc(hidden)]
    CT005,
    /// Northern New Haven, CT
    #[doc(hidden)]
    CT006,
    /// Northern Middlesex, CT
    #[doc(hidden)]
    CT007,
    /// Northern New London, CT
    #[doc(hidden)]
    CT008,
    /// Southern Fairfield, CT
    #[doc(hidden)]
    CT009,
    /// Southern New Haven, CT
    #[doc(hidden)]
    CT010,
    /// Southern Middlesex, CT
    #[doc(hidden)]
    CT011,
    /// Southern New London, CT
    #[doc(hidden)]
    CT012,
    /// Southern Litchfield, CT
    #[doc(hidden)]
    CT013,
    /// District of Columbia, DC
    #[doc(hidden)]
    DC001,
    /// New Castle, DE
    #[doc(hidden)]
    DE001,
    /// Kent, DE
    #[doc(hidden)]
    DE002,
    /// Inland Sussex, DE
    #[doc(hidden)]
    DE003,
    /// Delaware Beaches, DE
    #[doc(hidden)]
    DE004,
    /// North Walton, FL
    #[doc(hidden)]
    FL007,
    /// Central Walton, FL
    #[doc(hidden)]
    FL008,
    /// Holmes, FL
    #[doc(hidden)]
    FL009,
    /// Washington, FL
    #[doc(hidden)]
    FL010,
    /// Jackson, FL
    #[doc(hidden)]
    FL011,
    /// Inland Bay, FL
    #[doc(hidden)]
    FL012,
    /// Calhoun, FL
    #[doc(hidden)]
    FL013,
    /// Inland Gulf, FL
    #[doc(hidden)]
    FL014,
    /// Inland Franklin, FL
    #[doc(hidden)]
    FL015,
    /// Gadsden, FL
    #[doc(hidden)]
    FL016,
    /// Leon, FL
    #[doc(hidden)]
    FL017,
    /// Inland Jefferson, FL
    #[doc(hidden)]
    FL018,
    /// Madison, FL
    #[doc(hidden)]
    FL019,
    /// Hamilton, FL
    #[doc(hidden)]
    FL020,
    /// Suwannee, FL
    #[doc(hidden)]
    FL021,
    /// Baker, FL
    #[doc(hidden)]
    FL023,
    /// Inland Nassau, FL
    #[doc(hidden)]
    FL024,
    /// Liberty, FL
    #[doc(hidden)]
    FL026,
    /// Inland Wakulla, FL
    #[doc(hidden)]
    FL027,
    /// Inland Taylor, FL
    #[doc(hidden)]
    FL028,
    /// Lafayette, FL
    #[doc(hidden)]
    FL029,
    /// Union, FL
    #[doc(hidden)]
    FL030,
    /// Bradford, FL
    #[doc(hidden)]
    FL031,
    /// Inland St. Johns, FL
    #[doc(hidden)]
    FL033,
    /// Inland Dixie, FL
    #[doc(hidden)]
    FL034,
    /// Gilchrist, FL
    #[doc(hidden)]
    FL035,
    /// Inland Flagler, FL
    #[doc(hidden)]
    FL038,
    /// Inland Volusia, FL
    #[doc(hidden)]
    FL041,
    /// Sumter, FL
    #[doc(hidden)]
    FL043,
    /// Northern Lake County, FL
    #[doc(hidden)]
    FL044,
    /// Orange, FL
    #[doc(hidden)]
    FL045,
    /// Seminole, FL
    #[doc(hidden)]
    FL046,
    /// Pinellas, FL
    #[doc(hidden)]
    FL050,
    /// Polk, FL
    #[doc(hidden)]
    FL052,
    /// Osceola, FL
    #[doc(hidden)]
    FL053,
    /// Hardee, FL
    #[doc(hidden)]
    FL056,
    /// Highlands, FL
    #[doc(hidden)]
    FL057,
    /// Okeechobee, FL
    #[doc(hidden)]
    FL058,
    /// DeSoto, FL
    #[doc(hidden)]
    FL061,
    /// Glades, FL
    #[doc(hidden)]
    FL063,
    /// Hendry, FL
    #[doc(hidden)]
    FL066,
    /// Inland Palm Beach County, FL
    #[doc(hidden)]
    FL067,
    /// Metro Palm Beach County, FL
    #[doc(hidden)]
    FL068,
    /// Coastal Collier County, FL
    #[doc(hidden)]
    FL069,
    /// Inland Collier County, FL
    #[doc(hidden)]
    FL070,
    /// Inland Broward County, FL
    #[doc(hidden)]
    FL071,
    /// Metro Broward County, FL
    #[doc(hidden)]
    FL072,
    /// Inland Miami-Dade County, FL
    #[doc(hidden)]
    FL073,
    /// Metropolitan Miami Dade, FL
    #[doc(hidden)]
    FL074,
    /// Mainland Monroe, FL
    #[doc(hidden)]
    FL075,
    /// Monroe Upper Keys, FL
    #[doc(hidden)]
    FL076,
    /// Monroe Middle Keys, FL
    #[doc(hidden)]
    FL077,
    /// Monroe Lower Keys, FL
    #[doc(hidden)]
    FL078,
    /// South Walton, FL
    #[doc(hidden)]
    FL108,
    /// Coastal Bay, FL
    #[doc(hidden)]
    FL112,
    /// Coastal Gulf, FL
    #[doc(hidden)]
    FL114,
    /// Coastal Franklin, FL
    #[doc(hidden)]
    FL115,
    /// Coastal Jefferson, FL
    #[doc(hidden)]
    FL118,
    /// Northern Columbia, FL
    #[doc(hidden)]
    FL122,
    /// Coastal Nassau, FL
    #[doc(hidden)]
    FL124,
    /// Coastal Duval, FL
    #[doc(hidden)]
    FL125,
    /// Coastal Wakulla, FL
    #[doc(hidden)]
    FL127,
    /// Coastal Taylor, FL
    #[doc(hidden)]
    FL128,
    /// Eastern Clay, FL
    #[doc(hidden)]
    FL132,
    /// Coastal St. Johns, FL
    #[doc(hidden)]
    FL133,
    /// Coastal Dixie, FL
    #[doc(hidden)]
    FL134,
    /// Eastern Alachua, FL
    #[doc(hidden)]
    FL136,
    /// Eastern Putnam, FL
    #[doc(hidden)]
    FL137,
    /// Coastal Flagler, FL
    #[doc(hidden)]
    FL138,
    /// Coastal Levy, FL
    #[doc(hidden)]
    FL139,
    /// Eastern Marion, FL
    #[doc(hidden)]
    FL140,
    /// Coastal Volusia, FL
    #[doc(hidden)]
    FL141,
    /// Coastal Citrus, FL
    #[doc(hidden)]
    FL142,
    /// Southern Lake County, FL
    #[doc(hidden)]
    FL144,
    /// Coastal Hernando, FL
    #[doc(hidden)]
    FL148,
    /// Coastal Pasco, FL
    #[doc(hidden)]
    FL149,
    /// Coastal Hillsborough, FL
    #[doc(hidden)]
    FL151,
    /// Coastal Indian River, FL
    #[doc(hidden)]
    FL154,
    /// Coastal Manatee, FL
    #[doc(hidden)]
    FL155,
    /// Coastal St. Lucie, FL
    #[doc(hidden)]
    FL159,
    /// Coastal Sarasota, FL
    #[doc(hidden)]
    FL160,
    /// Coastal Charlotte, FL
    #[doc(hidden)]
    FL162,
    /// Coastal Martin, FL
    #[doc(hidden)]
    FL164,
    /// Coastal Lee, FL
    #[doc(hidden)]
    FL165,
    /// Coastal Palm Beach County, FL
    #[doc(hidden)]
    FL168,
    /// Coastal Broward County, FL
    #[doc(hidden)]
    FL172,
    /// Coastal Miami Dade County, FL
    #[doc(hidden)]
    FL173,
    /// Far South Miami-Dade County, FL
    #[doc(hidden)]
    FL174,
    /// Escambia Inland, FL
    #[doc(hidden)]
    FL201,
    /// Escambia Coastal, FL
    #[doc(hidden)]
    FL202,
    /// Santa Rosa Inland, FL
    #[doc(hidden)]
    FL203,
    /// Santa Rosa Coastal, FL
    #[doc(hidden)]
    FL204,
    /// Okaloosa Inland, FL
    #[doc(hidden)]
    FL205,
    /// Okaloosa Coastal, FL
    #[doc(hidden)]
    FL206,
    /// Southern Columbia, FL
    #[doc(hidden)]
    FL222,
    /// Trout River, FL
    #[doc(hidden)]
    FL225,
    /// Western Clay, FL
    #[doc(hidden)]
    FL232,
    /// Western Alachua, FL
    #[doc(hidden)]
    FL236,
    /// Western Putnam, FL
    #[doc(hidden)]
    FL237,
    /// Inland Levy, FL
    #[doc(hidden)]
    FL239,
    /// Central Marion, FL
    #[doc(hidden)]
    FL240,
    /// Inland Citrus, FL
    #[doc(hidden)]
    FL242,
    /// Inland Northern Brevard, FL
    #[doc(hidden)]
    FL247,
    /// Inland Hernando, FL
    #[doc(hidden)]
    FL248,
    /// Inland Pasco, FL
    #[doc(hidden)]
    FL249,
    /// Inland Hillsborough, FL
    #[doc(hidden)]
    FL251,
    /// Inland Indian River, FL
    #[doc(hidden)]
    FL254,
    /// Inland Manatee, FL
    #[doc(hidden)]
    FL255,
    /// Inland St. Lucie, FL
    #[doc(hidden)]
    FL259,
    /// Inland Sarasota, FL
    #[doc(hidden)]
    FL260,
    /// Inland Charlotte, FL
    #[doc(hidden)]
    FL262,
    /// Inland Martin, FL
    #[doc(hidden)]
    FL264,
    /// Inland Lee, FL
    #[doc(hidden)]
    FL265,
    /// South Central Duval, FL
    #[doc(hidden)]
    FL325,
    /// Western Marion, FL
    #[doc(hidden)]
    FL340,
    /// Mainland Northern Brevard, FL
    #[doc(hidden)]
    FL347,
    /// Western Duval, FL
    #[doc(hidden)]
    FL425,
    /// Northern Brevard Barrier Islands, FL
    #[doc(hidden)]
    FL447,
    /// Inland Southern Brevard, FL
    #[doc(hidden)]
    FL547,
    /// Mainland Southern Brevard, FL
    #[doc(hidden)]
    FL647,
    /// Southern Brevard Barrier Islands, FL
    #[doc(hidden)]
    FL747,
    /// Kosrae, FM
    #[doc(hidden)]
    FM001,
    /// Pingelap, FM
    #[doc(hidden)]
    FM011,
    /// Mwoakilloa, FM
    #[doc(hidden)]
    FM012,
    /// Pohnpei, FM
    #[doc(hidden)]
    FM013,
    /// Pakin, FM
    #[doc(hidden)]
    FM014,
    /// Sapwuahfik, FM
    #[doc(hidden)]
    FM015,
    /// Oroluk, FM
    #[doc(hidden)]
    FM016,
    /// Nukuoro, FM
    #[doc(hidden)]
    FM017,
    /// Kapingamarangi, FM
    #[doc(hidden)]
    FM018,
    /// Lukunoch, FM
    #[doc(hidden)]
    FM021,
    /// Losap, FM
    #[doc(hidden)]
    FM022,
    /// Chuuk Lagoon, FM
    #[doc(hidden)]
    FM023,
    /// Fananu, FM
    #[doc(hidden)]
    FM024,
    /// Onoun, FM
    #[doc(hidden)]
    FM025,
    /// Polowat, FM
    #[doc(hidden)]
    FM026,
    /// Satawal, FM
    #[doc(hidden)]
    FM031,
    /// Woleai, FM
    #[doc(hidden)]
    FM032,
    /// Faraulep, FM
    #[doc(hidden)]
    FM033,
    /// Eauripik, FM
    #[doc(hidden)]
    FM034,
    /// Fais, FM
    #[doc(hidden)]
    FM035,
    /// Ulithi, FM
    #[doc(hidden)]
    FM036,
    /// Yap, FM
    #[doc(hidden)]
    FM037,
    /// Ngulu, FM
    #[doc(hidden)]
    FM038,
    /// Dade, GA
    #[doc(hidden)]
    GA001,
    /// Walker, GA
    #[doc(hidden)]
    GA002,
    /// Catoosa, GA
    #[doc(hidden)]
    GA003,
    /// Whitfield, GA
    #[doc(hidden)]
    GA004,
    /// Murray, GA
    #[doc(hidden)]
    GA005,
    /// Fannin, GA
    #[doc(hidden)]
    GA006,
    /// Gilmer, GA
    #[doc(hidden)]
    GA007,
    /// Union, GA
    #[doc(hidden)]
    GA008,
    /// Towns, GA
    #[doc(hidden)]
    GA009,
    /// Rabun, GA
    #[doc(hidden)]
    GA010,
    /// Chattooga, GA
    #[doc(hidden)]
    GA011,
    /// Gordon, GA
    #[doc(hidden)]
    GA012,
    /// Pickens, GA
    #[doc(hidden)]
    GA013,
    /// Dawson, GA
    #[doc(hidden)]
    GA014,
    /// Lumpkin, GA
    #[doc(hidden)]
    GA015,
    /// White, GA
    #[doc(hidden)]
    GA016,
    /// Habersham, GA
    #[doc(hidden)]
    GA017,
    /// Stephens, GA
    #[doc(hidden)]
    GA018,
    /// Floyd, GA
    #[doc(hidden)]
    GA019,
    /// Bartow, GA
    #[doc(hidden)]
    GA020,
    /// Cherokee, GA
    #[doc(hidden)]
    GA021,
    /// Forsyth, GA
    #[doc(hidden)]
    GA022,
    /// Hall, GA
    #[doc(hidden)]
    GA023,
    /// Banks, GA
    #[doc(hidden)]
    GA024,
    /// Jackson, GA
    #[doc(hidden)]
    GA025,
    /// Franklin, GA
    #[doc(hidden)]
    GA026,
    /// Madison, GA
    #[doc(hidden)]
    GA027,
    /// Hart, GA
    #[doc(hidden)]
    GA028,
    /// Elbert, GA
    #[doc(hidden)]
    GA029,
    /// Polk, GA
    #[doc(hidden)]
    GA030,
    /// Paulding, GA
    #[doc(hidden)]
    GA031,
    /// Cobb, GA
    #[doc(hidden)]
    GA032,
    /// North Fulton, GA
    #[doc(hidden)]
    GA033,
    /// Gwinnett, GA
    #[doc(hidden)]
    GA034,
    /// Barrow, GA
    #[doc(hidden)]
    GA035,
    /// Clarke, GA
    #[doc(hidden)]
    GA036,
    /// Oconee, GA
    #[doc(hidden)]
    GA037,
    /// Oglethorpe, GA
    #[doc(hidden)]
    GA038,
    /// Wilkes, GA
    #[doc(hidden)]
    GA039,
    /// Lincoln, GA
    #[doc(hidden)]
    GA040,
    /// Haralson, GA
    #[doc(hidden)]
    GA041,
    /// Carroll, GA
    #[doc(hidden)]
    GA042,
    /// Douglas, GA
    #[doc(hidden)]
    GA043,
    /// South Fulton, GA
    #[doc(hidden)]
    GA044,
    /// DeKalb, GA
    #[doc(hidden)]
    GA045,
    /// Rockdale, GA
    #[doc(hidden)]
    GA046,
    /// Walton, GA
    #[doc(hidden)]
    GA047,
    /// Newton, GA
    #[doc(hidden)]
    GA048,
    /// Morgan, GA
    #[doc(hidden)]
    GA049,
    /// Greene, GA
    #[doc(hidden)]
    GA050,
    /// Taliaferro, GA
    #[doc(hidden)]
    GA051,
    /// Heard, GA
    #[doc(hidden)]
    GA052,
    /// Coweta, GA
    #[doc(hidden)]
    GA053,
    /// Fayette, GA
    #[doc(hidden)]
    GA054,
    /// Clayton, GA
    #[doc(hidden)]
    GA055,
    /// Spalding, GA
    #[doc(hidden)]
    GA056,
    /// Henry, GA
    #[doc(hidden)]
    GA057,
    /// Butts, GA
    #[doc(hidden)]
    GA058,
    /// Jasper, GA
    #[doc(hidden)]
    GA059,
    /// Putnam, GA
    #[doc(hidden)]
    GA060,
    /// Hancock, GA
    #[doc(hidden)]
    GA061,
    /// Warren, GA
    #[doc(hidden)]
    GA062,
    /// McDuffie, GA
    #[doc(hidden)]
    GA063,
    /// Columbia, GA
    #[doc(hidden)]
    GA064,
    /// Richmond, GA
    #[doc(hidden)]
    GA065,
    /// Troup, GA
    #[doc(hidden)]
    GA066,
    /// Meriwether, GA
    #[doc(hidden)]
    GA067,
    /// Pike, GA
    #[doc(hidden)]
    GA068,
    /// Upson, GA
    #[doc(hidden)]
    GA069,
    /// Lamar, GA
    #[doc(hidden)]
    GA070,
    /// Monroe, GA
    #[doc(hidden)]
    GA071,
    /// Jones, GA
    #[doc(hidden)]
    GA072,
    /// Baldwin, GA
    #[doc(hidden)]
    GA073,
    /// Washington, GA
    #[doc(hidden)]
    GA074,
    /// Glascock, GA
    #[doc(hidden)]
    GA075,
    /// Jefferson, GA
    #[doc(hidden)]
    GA076,
    /// Burke, GA
    #[doc(hidden)]
    GA077,
    /// Harris, GA
    #[doc(hidden)]
    GA078,
    /// Talbot, GA
    #[doc(hidden)]
    GA079,
    /// Taylor, GA
    #[doc(hidden)]
    GA080,
    /// Crawford, GA
    #[doc(hidden)]
    GA081,
    /// Bibb, GA
    #[doc(hidden)]
    GA082,
    /// Twiggs, GA
    #[doc(hidden)]
    GA083,
    /// Wilkinson, GA
    #[doc(hidden)]
    GA084,
    /// Johnson, GA
    #[doc(hidden)]
    GA085,
    /// Emanuel, GA
    #[doc(hidden)]
    GA086,
    /// Jenkins, GA
    #[doc(hidden)]
    GA087,
    /// Screven, GA
    #[doc(hidden)]
    GA088,
    /// Muscogee, GA
    #[doc(hidden)]
    GA089,
    /// Chattahoochee, GA
    #[doc(hidden)]
    GA090,
    /// Marion, GA
    #[doc(hidden)]
    GA091,
    /// Schley, GA
    #[doc(hidden)]
    GA092,
    /// Macon, GA
    #[doc(hidden)]
    GA093,
    /// Peach, GA
    #[doc(hidden)]
    GA094,
    /// Houston, GA
    #[doc(hidden)]
    GA095,
    /// Bleckley, GA
    #[doc(hidden)]
    GA096,
    /// Laurens, GA
    #[doc(hidden)]
    GA097,
    /// Treutlen, GA
    #[doc(hidden)]
    GA098,
    /// Candler, GA
    #[doc(hidden)]
    GA099,
    /// Bulloch, GA
    #[doc(hidden)]
    GA100,
    /// Effingham, GA
    #[doc(hidden)]
    GA101,
    /// Stewart, GA
    #[doc(hidden)]
    GA102,
    /// Webster, GA
    #[doc(hidden)]
    GA103,
    /// Sumter, GA
    #[doc(hidden)]
    GA104,
    /// Dooly, GA
    #[doc(hidden)]
    GA105,
    /// Crisp, GA
    #[doc(hidden)]
    GA106,
    /// Pulaski, GA
    #[doc(hidden)]
    GA107,
    /// Wilcox, GA
    #[doc(hidden)]
    GA108,
    /// Dodge, GA
    #[doc(hidden)]
    GA109,
    /// Telfair, GA
    #[doc(hidden)]
    GA110,
    /// Wheeler, GA
    #[doc(hidden)]
    GA111,
    /// Montgomery, GA
    #[doc(hidden)]
    GA112,
    /// Toombs, GA
    #[doc(hidden)]
    GA113,
    /// Tattnall, GA
    #[doc(hidden)]
    GA114,
    /// Evans, GA
    #[doc(hidden)]
    GA115,
    /// Inland Bryan, GA
    #[doc(hidden)]
    GA116,
    /// Coastal Bryan, GA
    #[doc(hidden)]
    GA117,
    /// Inland Chatham, GA
    #[doc(hidden)]
    GA118,
    /// Coastal Chatham, GA
    #[doc(hidden)]
    GA119,
    /// Quitman, GA
    #[doc(hidden)]
    GA120,
    /// Clay, GA
    #[doc(hidden)]
    GA121,
    /// Randolph, GA
    #[doc(hidden)]
    GA122,
    /// Calhoun, GA
    #[doc(hidden)]
    GA123,
    /// Terrell, GA
    #[doc(hidden)]
    GA124,
    /// Dougherty, GA
    #[doc(hidden)]
    GA125,
    /// Lee, GA
    #[doc(hidden)]
    GA126,
    /// Worth, GA
    #[doc(hidden)]
    GA127,
    /// Turner, GA
    #[doc(hidden)]
    GA128,
    /// Tift, GA
    #[doc(hidden)]
    GA129,
    /// Ben Hill, GA
    #[doc(hidden)]
    GA130,
    /// Irwin, GA
    #[doc(hidden)]
    GA131,
    /// Coffee, GA
    #[doc(hidden)]
    GA132,
    /// Jeff Davis, GA
    #[doc(hidden)]
    GA133,
    /// Bacon, GA
    #[doc(hidden)]
    GA134,
    /// Appling, GA
    #[doc(hidden)]
    GA135,
    /// Wayne, GA
    #[doc(hidden)]
    GA136,
    /// Long, GA
    #[doc(hidden)]
    GA137,
    /// Inland Liberty, GA
    #[doc(hidden)]
    GA138,
    /// Coastal Liberty, GA
    #[doc(hidden)]
    GA139,
    /// Inland McIntosh, GA
    #[doc(hidden)]
    GA140,
    /// Coastal McIntosh, GA
    #[doc(hidden)]
    GA141,
    /// Early, GA
    #[doc(hidden)]
    GA142,
    /// Miller, GA
    #[doc(hidden)]
    GA143,
    /// Baker, GA
    #[doc(hidden)]
    GA144,
    /// Mitchell, GA
    #[doc(hidden)]
    GA145,
    /// Colquitt, GA
    #[doc(hidden)]
    GA146,
    /// Cook, GA
    #[doc(hidden)]
    GA147,
    /// Berrien, GA
    #[doc(hidden)]
    GA148,
    /// Atkinson, GA
    #[doc(hidden)]
    GA149,
    /// Pierce, GA
    #[doc(hidden)]
    GA151,
    /// Brantley, GA
    #[doc(hidden)]
    GA152,
    /// Inland Glynn, GA
    #[doc(hidden)]
    GA153,
    /// Coastal Glynn, GA
    #[doc(hidden)]
    GA154,
    /// Seminole, GA
    #[doc(hidden)]
    GA155,
    /// Decatur, GA
    #[doc(hidden)]
    GA156,
    /// Grady, GA
    #[doc(hidden)]
    GA157,
    /// Thomas, GA
    #[doc(hidden)]
    GA158,
    /// Brooks, GA
    #[doc(hidden)]
    GA159,
    /// Lowndes, GA
    #[doc(hidden)]
    GA160,
    /// Lanier, GA
    #[doc(hidden)]
    GA161,
    /// Echols, GA
    #[doc(hidden)]
    GA162,
    /// Clinch, GA
    #[doc(hidden)]
    GA163,
    /// Inland Camden, GA
    #[doc(hidden)]
    GA165,
    /// Coastal Camden, GA
    #[doc(hidden)]
    GA166,
    /// Northern Ware, GA
    #[doc(hidden)]
    GA250,
    /// Northeastern Charlton, GA
    #[doc(hidden)]
    GA264,
    /// Southern Ware, GA
    #[doc(hidden)]
    GA350,
    /// Western Charlton, GA
    #[doc(hidden)]
    GA364,
    /// Guam, GU
    #[doc(hidden)]
    GU001,
    /// Niihau, HI
    #[doc(hidden)]
    HI001,
    /// Kauai Southwest, HI
    #[doc(hidden)]
    HI003,
    /// Kauai Mountains, HI
    #[doc(hidden)]
    HI004,
    /// Waianae Coast, HI
    #[doc(hidden)]
    HI006,
    /// Oahu North Shore, HI
    #[doc(hidden)]
    HI007,
    /// Olomana, HI
    #[doc(hidden)]
    HI009,
    /// Central Oahu, HI
    #[doc(hidden)]
    HI010,
    /// Waianae Mountains, HI
    #[doc(hidden)]
    HI011,
    /// Lanai Mauka, HI
    #[doc(hidden)]
    HI015,
    /// Kahoolawe, HI
    #[doc(hidden)]
    HI016,
    /// Maui Windward West, HI
    #[doc(hidden)]
    HI017,
    /// Maui Leeward West, HI
    #[doc(hidden)]
    HI018,
    /// Haleakala Summit, HI
    #[doc(hidden)]
    HI022,
    /// Kona, HI
    #[doc(hidden)]
    HI023,
    /// Kohala, HI
    #[doc(hidden)]
    HI026,
    /// Big Island Interior, HI
    #[doc(hidden)]
    HI027,
    /// Big Island Summit, HI
    #[doc(hidden)]
    HI028,
    /// Kauai North, HI
    #[doc(hidden)]
    HI029,
    /// Kauai East, HI
    #[doc(hidden)]
    HI030,
    /// Kauai South, HI
    #[doc(hidden)]
    HI031,
    /// East Honolulu, HI
    #[doc(hidden)]
    HI032,
    /// Honolulu Metro, HI
    #[doc(hidden)]
    HI033,
    /// Ewa Plain, HI
    #[doc(hidden)]
    HI034,
    /// Koolau Windward, HI
    #[doc(hidden)]
    HI035,
    /// Koolau Leeward, HI
    #[doc(hidden)]
    HI036,
    /// Molokai Windward, HI
    #[doc(hidden)]
    HI037,
    /// Molokai Southeast, HI
    #[doc(hidden)]
    HI038,
    /// Molokai North, HI
    #[doc(hidden)]
    HI039,
    /// Molokai West, HI
    #[doc(hidden)]
    HI040,
    /// Molokai Leeward South, HI
    #[doc(hidden)]
    HI041,
    /// Lanai Windward, HI
    #[doc(hidden)]
    HI042,
    /// Lanai Leeward, HI
    #[doc(hidden)]
    HI043,
    /// Lanai South, HI
    #[doc(hidden)]
    HI044,
    /// Maui Central Valley North, HI
    #[doc(hidden)]
    HI045,
    /// Maui Central Valley South, HI
    #[doc(hidden)]
    HI046,
    /// Windward Haleakala, HI
    #[doc(hidden)]
    HI047,
    /// Kipahulu, HI
    #[doc(hidden)]
    HI048,
    /// South Maui/Upcountry, HI
    #[doc(hidden)]
    HI049,
    /// South Haleakala, HI
    #[doc(hidden)]
    HI050,
    /// Big Island South, HI
    #[doc(hidden)]
    HI051,
    /// Big Island Southeast, HI
    #[doc(hidden)]
    HI052,
    /// Big Island East, HI
    #[doc(hidden)]
    HI053,
    /// Big Island North, HI
    #[doc(hidden)]
    HI054,
    /// Lyon, IA
    #[doc(hidden)]
    IA001,
    /// Osceola, IA
    #[doc(hidden)]
    IA002,
    /// Dickinson, IA
    #[doc(hidden)]
    IA003,
    /// Emmet, IA
    #[doc(hidden)]
    IA004,
    /// Kossuth, IA
    #[doc(hidden)]
    IA005,
    /// Winnebago, IA
    #[doc(hidden)]
    IA006,
    /// Worth, IA
    #[doc(hidden)]
    IA007,
    /// Mitchell, IA
    #[doc(hidden)]
    IA008,
    /// Howard, IA
    #[doc(hidden)]
    IA009,
    /// Winneshiek, IA
    #[doc(hidden)]
    IA010,
    /// Allamakee, IA
    #[doc(hidden)]
    IA011,
    /// Sioux, IA
    #[doc(hidden)]
    IA012,
    /// O'Brien, IA
    #[doc(hidden)]
    IA013,
    /// Clay, IA
    #[doc(hidden)]
    IA014,
    /// Palo Alto, IA
    #[doc(hidden)]
    IA015,
    /// Hancock, IA
    #[doc(hidden)]
    IA016,
    /// Cerro Gordo, IA
    #[doc(hidden)]
    IA017,
    /// Floyd, IA
    #[doc(hidden)]
    IA018,
    /// Chickasaw, IA
    #[doc(hidden)]
    IA019,
    /// Plymouth, IA
    #[doc(hidden)]
    IA020,
    /// Cherokee, IA
    #[doc(hidden)]
    IA021,
    /// Buena Vista, IA
    #[doc(hidden)]
    IA022,
    /// Pocahontas, IA
    #[doc(hidden)]
    IA023,
    /// Humboldt, IA
    #[doc(hidden)]
    IA024,
    /// Wright, IA
    #[doc(hidden)]
    IA025,
    /// Franklin, IA
    #[doc(hidden)]
    IA026,
    /// Butler, IA
    #[doc(hidden)]
    IA027,
    /// Bremer, IA
    #[doc(hidden)]
    IA028,
    /// Fayette, IA
    #[doc(hidden)]
    IA029,
    /// Clayton, IA
    #[doc(hidden)]
    IA030,
    /// Woodbury, IA
    #[doc(hidden)]
    IA031,
    /// Ida, IA
    #[doc(hidden)]
    IA032,
    /// Sac, IA
    #[doc(hidden)]
    IA033,
    /// Calhoun, IA
    #[doc(hidden)]
    IA034,
    /// Webster, IA
    #[doc(hidden)]
    IA035,
    /// Hamilton, IA
    #[doc(hidden)]
    IA036,
    /// Hardin, IA
    #[doc(hidden)]
    IA037,
    /// Grundy, IA
    #[doc(hidden)]
    IA038,
    /// Black Hawk, IA
    #[doc(hidden)]
    IA039,
    /// Buchanan, IA
    #[doc(hidden)]
    IA040,
    /// Delaware, IA
    #[doc(hidden)]
    IA041,
    /// Dubuque, IA
    #[doc(hidden)]
    IA042,
    /// Monona, IA
    #[doc(hidden)]
    IA043,
    /// Crawford, IA
    #[doc(hidden)]
    IA044,
    /// Carroll, IA
    #[doc(hidden)]
    IA045,
    /// Greene, IA
    #[doc(hidden)]
    IA046,
    /// Boone, IA
    #[doc(hidden)]
    IA047,
    /// Story, IA
    #[doc(hidden)]
    IA048,
    /// Marshall, IA
    #[doc(hidden)]
    IA049,
    /// Tama, IA
    #[doc(hidden)]
    IA050,
    /// Benton, IA
    #[doc(hidden)]
    IA051,
    /// Linn, IA
    #[doc(hidden)]
    IA052,
    /// Jones, IA
    #[doc(hidden)]
    IA053,
    /// Jackson, IA
    #[doc(hidden)]
    IA054,
    /// Harrison, IA
    #[doc(hidden)]
    IA055,
    /// Shelby, IA
    #[doc(hidden)]
    IA056,
    /// Audubon, IA
    #[doc(hidden)]
    IA057,
    /// Guthrie, IA
    #[doc(hidden)]
    IA058,
    /// Dallas, IA
    #[doc(hidden)]
    IA059,
    /// Polk, IA
    #[doc(hidden)]
    IA060,
    /// Jasper, IA
    #[doc(hidden)]
    IA061,
    /// Poweshiek, IA
    #[doc(hidden)]
    IA062,
    /// Iowa, IA
    #[doc(hidden)]
    IA063,
    /// Johnson, IA
    #[doc(hidden)]
    IA064,
    /// Cedar, IA
    #[doc(hidden)]
    IA065,
    /// Clinton, IA
    #[doc(hidden)]
    IA066,
    /// Muscatine, IA
    #[doc(hidden)]
    IA067,
    /// Scott, IA
    #[doc(hidden)]
    IA068,
    /// Pottawattamie, IA
    #[doc(hidden)]
    IA069,
    /// Cass, IA
    #[doc(hidden)]
    IA070,
    /// Adair, IA
    #[doc(hidden)]
    IA071,
    /// Madison, IA
    #[doc(hidden)]
    IA072,
    /// Warren, IA
    #[doc(hidden)]
    IA073,
    /// Marion, IA
    #[doc(hidden)]
    IA074,
    /// Mahaska, IA
    #[doc(hidden)]
    IA075,
    /// Keokuk, IA
    #[doc(hidden)]
    IA076,
    /// Washington, IA
    #[doc(hidden)]
    IA077,
    /// Louisa, IA
    #[doc(hidden)]
    IA078,
    /// Mills, IA
    #[doc(hidden)]
    IA079,
    /// Montgomery, IA
    #[doc(hidden)]
    IA080,
    /// Adams, IA
    #[doc(hidden)]
    IA081,
    /// Union, IA
    #[doc(hidden)]
    IA082,
    /// Clarke, IA
    #[doc(hidden)]
    IA083,
    /// Lucas, IA
    #[doc(hidden)]
    IA084,
    /// Monroe, IA
    #[doc(hidden)]
    IA085,
    /// Wapello, IA
    #[doc(hidden)]
    IA086,
    /// Jefferson, IA
    #[doc(hidden)]
    IA087,
    /// Henry, IA
    #[doc(hidden)]
    IA088,
    /// Des Moines, IA
    #[doc(hidden)]
    IA089,
    /// Fremont, IA
    #[doc(hidden)]
    IA090,
    /// Page, IA
    #[doc(hidden)]
    IA091,
    /// Taylor, IA
    #[doc(hidden)]
    IA092,
    /// Ringgold, IA
    #[doc(hidden)]
    IA093,
    /// Decatur, IA
    #[doc(hidden)]
    IA094,
    /// Wayne, IA
    #[doc(hidden)]
    IA095,
    /// Appanoose, IA
    #[doc(hidden)]
    IA096,
    /// Davis, IA
    #[doc(hidden)]
    IA097,
    /// Van Buren, IA
    #[doc(hidden)]
    IA098,
    /// Lee, IA
    #[doc(hidden)]
    IA099,
    /// Northern Panhandle, ID
    #[doc(hidden)]
    ID001,
    /// Coeur d'Alene Area, ID
    #[doc(hidden)]
    ID002,
    /// Idaho Palouse, ID
    #[doc(hidden)]
    ID003,
    /// Central Panhandle Mountains, ID
    #[doc(hidden)]
    ID004,
    /// Northern Clearwater Mountains, ID
    #[doc(hidden)]
    ID005,
    /// Southern Clearwater Mountains, ID
    #[doc(hidden)]
    ID006,
    /// Orofino/Grangeville Region, ID
    #[doc(hidden)]
    ID007,
    /// Lower Hells Canyon/Salmon River Region, ID
    #[doc(hidden)]
    ID008,
    /// Western Lemhi County, ID
    #[doc(hidden)]
    ID009,
    /// Eastern Lemhi County, ID
    #[doc(hidden)]
    ID010,
    /// West Central Mountains, ID
    #[doc(hidden)]
    ID011,
    /// Lower Treasure Valley, ID
    #[doc(hidden)]
    ID012,
    /// Boise Mountains, ID
    #[doc(hidden)]
    ID013,
    /// Upper Treasure Valley, ID
    #[doc(hidden)]
    ID014,
    /// Southwest Highlands, ID
    #[doc(hidden)]
    ID015,
    /// Western Magic Valley, ID
    #[doc(hidden)]
    ID016,
    /// Lewiston Area, ID
    #[doc(hidden)]
    ID026,
    /// Lewis and Southern Nez Perce Counties, ID
    #[doc(hidden)]
    ID027,
    /// Camas Prairie, ID
    #[doc(hidden)]
    ID028,
    /// Owyhee Mountains, ID
    #[doc(hidden)]
    ID029,
    /// Southern Twin  Falls County, ID
    #[doc(hidden)]
    ID030,
    /// Upper Weiser River, ID
    #[doc(hidden)]
    ID033,
    /// Shoshone/Lava Beds, ID
    #[doc(hidden)]
    ID051,
    /// Arco/Mud Lake Desert, ID
    #[doc(hidden)]
    ID052,
    /// Upper Snake River Plain, ID
    #[doc(hidden)]
    ID053,
    /// Lower Snake River Plain, ID
    #[doc(hidden)]
    ID054,
    /// Eastern Magic Valley, ID
    #[doc(hidden)]
    ID055,
    /// Southern Hills/Albion Mountains, ID
    #[doc(hidden)]
    ID056,
    /// Raft River Region, ID
    #[doc(hidden)]
    ID057,
    /// Marsh and Arbon Highlands, ID
    #[doc(hidden)]
    ID058,
    /// Franklin/Eastern Oneida Region, ID
    #[doc(hidden)]
    ID059,
    /// Bear River Range, ID
    #[doc(hidden)]
    ID060,
    /// Bear Lake Valley, ID
    #[doc(hidden)]
    ID061,
    /// Blackfoot Mountains, ID
    #[doc(hidden)]
    ID062,
    /// Caribou Range, ID
    #[doc(hidden)]
    ID063,
    /// Big Hole Mountains, ID
    #[doc(hidden)]
    ID064,
    /// Teton Valley, ID
    #[doc(hidden)]
    ID065,
    /// Centennial Mountains/Island Park, ID
    #[doc(hidden)]
    ID066,
    /// Beaverhead/Lemhi Highlands, ID
    #[doc(hidden)]
    ID067,
    /// Lost River Valleys, ID
    #[doc(hidden)]
    ID068,
    /// Lost River Range, ID
    #[doc(hidden)]
    ID069,
    /// Challis/Pahsimeroi Valleys, ID
    #[doc(hidden)]
    ID070,
    /// Frank Church Wilderness, ID
    #[doc(hidden)]
    ID071,
    /// Sawtooth/Stanley Basin, ID
    #[doc(hidden)]
    ID072,
    /// Sun Valley Region, ID
    #[doc(hidden)]
    ID073,
    /// Big Lost Highlands/Copper Basin, ID
    #[doc(hidden)]
    ID074,
    /// Wood River Foothills, ID
    #[doc(hidden)]
    ID075,
    /// Jo Daviess, IL
    #[doc(hidden)]
    IL001,
    /// Stephenson, IL
    #[doc(hidden)]
    IL002,
    /// Winnebago, IL
    #[doc(hidden)]
    IL003,
    /// Boone, IL
    #[doc(hidden)]
    IL004,
    /// McHenry, IL
    #[doc(hidden)]
    IL005,
    /// Lake, IL
    #[doc(hidden)]
    IL006,
    /// Carroll, IL
    #[doc(hidden)]
    IL007,
    /// Ogle, IL
    #[doc(hidden)]
    IL008,
    /// Whiteside, IL
    #[doc(hidden)]
    IL009,
    /// Lee, IL
    #[doc(hidden)]
    IL010,
    /// De Kalb, IL
    #[doc(hidden)]
    IL011,
    /// Kane, IL
    #[doc(hidden)]
    IL012,
    /// DuPage, IL
    #[doc(hidden)]
    IL013,
    /// Rock Island, IL
    #[doc(hidden)]
    IL015,
    /// Henry, IL
    #[doc(hidden)]
    IL016,
    /// Bureau, IL
    #[doc(hidden)]
    IL017,
    /// Putnam, IL
    #[doc(hidden)]
    IL018,
    /// La Salle, IL
    #[doc(hidden)]
    IL019,
    /// Kendall, IL
    #[doc(hidden)]
    IL020,
    /// Grundy, IL
    #[doc(hidden)]
    IL021,
    /// Kankakee, IL
    #[doc(hidden)]
    IL023,
    /// Mercer, IL
    #[doc(hidden)]
    IL024,
    /// Henderson, IL
    #[doc(hidden)]
    IL025,
    /// Warren, IL
    #[doc(hidden)]
    IL026,
    /// Knox, IL
    #[doc(hidden)]
    IL027,
    /// Stark, IL
    #[doc(hidden)]
    IL028,
    /// Peoria, IL
    #[doc(hidden)]
    IL029,
    /// Marshall, IL
    #[doc(hidden)]
    IL030,
    /// Woodford, IL
    #[doc(hidden)]
    IL031,
    /// Livingston, IL
    #[doc(hidden)]
    IL032,
    /// Iroquois, IL
    #[doc(hidden)]
    IL033,
    /// Hancock, IL
    #[doc(hidden)]
    IL034,
    /// McDonough, IL
    #[doc(hidden)]
    IL035,
    /// Fulton, IL
    #[doc(hidden)]
    IL036,
    /// Tazewell, IL
    #[doc(hidden)]
    IL037,
    /// McLean, IL
    #[doc(hidden)]
    IL038,
    /// Ford, IL
    #[doc(hidden)]
    IL039,
    /// Schuyler, IL
    #[doc(hidden)]
    IL040,
    /// Mason, IL
    #[doc(hidden)]
    IL041,
    /// Logan, IL
    #[doc(hidden)]
    IL042,
    /// De Witt, IL
    #[doc(hidden)]
    IL043,
    /// Piatt, IL
    #[doc(hidden)]
    IL044,
    /// Champaign, IL
    #[doc(hidden)]
    IL045,
    /// Vermilion, IL
    #[doc(hidden)]
    IL046,
    /// Cass, IL
    #[doc(hidden)]
    IL047,
    /// Menard, IL
    #[doc(hidden)]
    IL048,
    /// Scott, IL
    #[doc(hidden)]
    IL049,
    /// Morgan, IL
    #[doc(hidden)]
    IL050,
    /// Sangamon, IL
    #[doc(hidden)]
    IL051,
    /// Christian, IL
    #[doc(hidden)]
    IL052,
    /// Macon, IL
    #[doc(hidden)]
    IL053,
    /// Moultrie, IL
    #[doc(hidden)]
    IL054,
    /// Douglas, IL
    #[doc(hidden)]
    IL055,
    /// Coles, IL
    #[doc(hidden)]
    IL056,
    /// Edgar, IL
    #[doc(hidden)]
    IL057,
    /// Greene, IL
    #[doc(hidden)]
    IL058,
    /// Macoupin, IL
    #[doc(hidden)]
    IL059,
    /// Montgomery, IL
    #[doc(hidden)]
    IL060,
    /// Shelby, IL
    #[doc(hidden)]
    IL061,
    /// Cumberland, IL
    #[doc(hidden)]
    IL062,
    /// Clark, IL
    #[doc(hidden)]
    IL063,
    /// Bond, IL
    #[doc(hidden)]
    IL064,
    /// Fayette, IL
    #[doc(hidden)]
    IL065,
    /// Effingham, IL
    #[doc(hidden)]
    IL066,
    /// Jasper, IL
    #[doc(hidden)]
    IL067,
    /// Crawford, IL
    #[doc(hidden)]
    IL068,
    /// Clinton, IL
    #[doc(hidden)]
    IL069,
    /// Marion, IL
    #[doc(hidden)]
    IL070,
    /// Clay, IL
    #[doc(hidden)]
    IL071,
    /// Richland, IL
    #[doc(hidden)]
    IL072,
    /// Lawrence, IL
    #[doc(hidden)]
    IL073,
    /// Washington, IL
    #[doc(hidden)]
    IL074,
    /// Jefferson, IL
    #[doc(hidden)]
    IL075,
    /// Wayne, IL
    #[doc(hidden)]
    IL076,
    /// Edwards, IL
    #[doc(hidden)]
    IL077,
    /// Wabash, IL
    #[doc(hidden)]
    IL078,
    /// Randolph, IL
    #[doc(hidden)]
    IL079,
    /// Perry, IL
    #[doc(hidden)]
    IL080,
    /// Franklin, IL
    #[doc(hidden)]
    IL081,
    /// Hamilton, IL
    #[doc(hidden)]
    IL082,
    /// White, IL
    #[doc(hidden)]
    IL083,
    /// Jackson, IL
    #[doc(hidden)]
    IL084,
    /// Williamson, IL
    #[doc(hidden)]
    IL085,
    /// Saline, IL
    #[doc(hidden)]
    IL086,
    /// Gallatin, IL
    #[doc(hidden)]
    IL087,
    /// Union, IL
    #[doc(hidden)]
    IL088,
    /// Johnson, IL
    #[doc(hidden)]
    IL089,
    /// Pope, IL
    #[doc(hidden)]
    IL090,
    /// Hardin, IL
    #[doc(hidden)]
    IL091,
    /// Alexander, IL
    #[doc(hidden)]
    IL092,
    /// Pulaski, IL
    #[doc(hidden)]
    IL093,
    /// Massac, IL
    #[doc(hidden)]
    IL094,
    /// Adams, IL
    #[doc(hidden)]
    IL095,
    /// Brown, IL
    #[doc(hidden)]
    IL096,
    /// Pike, IL
    #[doc(hidden)]
    IL097,
    /// Calhoun, IL
    #[doc(hidden)]
    IL098,
    /// Jersey, IL
    #[doc(hidden)]
    IL099,
    /// Madison, IL
    #[doc(hidden)]
    IL100,
    /// St. Clair, IL
    #[doc(hidden)]
    IL101,
    /// Monroe, IL
    #[doc(hidden)]
    IL102,
    /// Northern Cook, IL
    #[doc(hidden)]
    IL103,
    /// Central Cook, IL
    #[doc(hidden)]
    IL104,
    /// Southern Cook, IL
    #[doc(hidden)]
    IL105,
    /// Northern Will, IL
    #[doc(hidden)]
    IL106,
    /// Southern Will, IL
    #[doc(hidden)]
    IL107,
    /// Eastern Will, IL
    #[doc(hidden)]
    IL108,
    /// Lake, IN
    #[doc(hidden)]
    IN001,
    /// Porter, IN
    #[doc(hidden)]
    IN002,
    /// La Porte, IN
    #[doc(hidden)]
    IN003,
    /// St. Joseph, IN
    #[doc(hidden)]
    IN004,
    /// Elkhart, IN
    #[doc(hidden)]
    IN005,
    /// Lagrange, IN
    #[doc(hidden)]
    IN006,
    /// Steuben, IN
    #[doc(hidden)]
    IN007,
    /// Noble, IN
    #[doc(hidden)]
    IN008,
    /// De Kalb, IN
    #[doc(hidden)]
    IN009,
    /// Newton, IN
    #[doc(hidden)]
    IN010,
    /// Jasper, IN
    #[doc(hidden)]
    IN011,
    /// Starke, IN
    #[doc(hidden)]
    IN012,
    /// Pulaski, IN
    #[doc(hidden)]
    IN013,
    /// Marshall, IN
    #[doc(hidden)]
    IN014,
    /// Fulton, IN
    #[doc(hidden)]
    IN015,
    /// Kosciusko, IN
    #[doc(hidden)]
    IN016,
    /// Whitley, IN
    #[doc(hidden)]
    IN017,
    /// Allen, IN
    #[doc(hidden)]
    IN018,
    /// Benton, IN
    #[doc(hidden)]
    IN019,
    /// White, IN
    #[doc(hidden)]
    IN020,
    /// Carroll, IN
    #[doc(hidden)]
    IN021,
    /// Cass, IN
    #[doc(hidden)]
    IN022,
    /// Miami, IN
    #[doc(hidden)]
    IN023,
    /// Wabash, IN
    #[doc(hidden)]
    IN024,
    /// Huntington, IN
    #[doc(hidden)]
    IN025,
    /// Wells, IN
    #[doc(hidden)]
    IN026,
    /// Adams, IN
    #[doc(hidden)]
    IN027,
    /// Warren, IN
    #[doc(hidden)]
    IN028,
    /// Tippecanoe, IN
    #[doc(hidden)]
    IN029,
    /// Clinton, IN
    #[doc(hidden)]
    IN030,
    /// Howard, IN
    #[doc(hidden)]
    IN031,
    /// Grant, IN
    #[doc(hidden)]
    IN032,
    /// Blackford, IN
    #[doc(hidden)]
    IN033,
    /// Jay, IN
    #[doc(hidden)]
    IN034,
    /// Fountain, IN
    #[doc(hidden)]
    IN035,
    /// Montgomery, IN
    #[doc(hidden)]
    IN036,
    /// Boone, IN
    #[doc(hidden)]
    IN037,
    /// Tipton, IN
    #[doc(hidden)]
    IN038,
    /// Hamilton, IN
    #[doc(hidden)]
    IN039,
    /// Madison, IN
    #[doc(hidden)]
    IN040,
    /// Delaware, IN
    #[doc(hidden)]
    IN041,
    /// Randolph, IN
    #[doc(hidden)]
    IN042,
    /// Vermillion, IN
    #[doc(hidden)]
    IN043,
    /// Parke, IN
    #[doc(hidden)]
    IN044,
    /// Putnam, IN
    #[doc(hidden)]
    IN045,
    /// Hendricks, IN
    #[doc(hidden)]
    IN046,
    /// Marion, IN
    #[doc(hidden)]
    IN047,
    /// Hancock, IN
    #[doc(hidden)]
    IN048,
    /// Henry, IN
    #[doc(hidden)]
    IN049,
    /// Wayne, IN
    #[doc(hidden)]
    IN050,
    /// Vigo, IN
    #[doc(hidden)]
    IN051,
    /// Clay, IN
    #[doc(hidden)]
    IN052,
    /// Owen, IN
    #[doc(hidden)]
    IN053,
    /// Morgan, IN
    #[doc(hidden)]
    IN054,
    /// Johnson, IN
    #[doc(hidden)]
    IN055,
    /// Shelby, IN
    #[doc(hidden)]
    IN056,
    /// Rush, IN
    #[doc(hidden)]
    IN057,
    /// Fayette, IN
    #[doc(hidden)]
    IN058,
    /// Union, IN
    #[doc(hidden)]
    IN059,
    /// Sullivan, IN
    #[doc(hidden)]
    IN060,
    /// Greene, IN
    #[doc(hidden)]
    IN061,
    /// Monroe, IN
    #[doc(hidden)]
    IN062,
    /// Brown, IN
    #[doc(hidden)]
    IN063,
    /// Bartholomew, IN
    #[doc(hidden)]
    IN064,
    /// Decatur, IN
    #[doc(hidden)]
    IN065,
    /// Franklin, IN
    #[doc(hidden)]
    IN066,
    /// Knox, IN
    #[doc(hidden)]
    IN067,
    /// Daviess, IN
    #[doc(hidden)]
    IN068,
    /// Martin, IN
    #[doc(hidden)]
    IN069,
    /// Lawrence, IN
    #[doc(hidden)]
    IN070,
    /// Jackson, IN
    #[doc(hidden)]
    IN071,
    /// Jennings, IN
    #[doc(hidden)]
    IN072,
    /// Ripley, IN
    #[doc(hidden)]
    IN073,
    /// Dearborn, IN
    #[doc(hidden)]
    IN074,
    /// Ohio, IN
    #[doc(hidden)]
    IN075,
    /// Orange, IN
    #[doc(hidden)]
    IN076,
    /// Washington, IN
    #[doc(hidden)]
    IN077,
    /// Scott, IN
    #[doc(hidden)]
    IN078,
    /// Jefferson, IN
    #[doc(hidden)]
    IN079,
    /// Switzerland, IN
    #[doc(hidden)]
    IN080,
    /// Gibson, IN
    #[doc(hidden)]
    IN081,
    /// Pike, IN
    #[doc(hidden)]
    IN082,
    /// Dubois, IN
    #[doc(hidden)]
    IN083,
    /// Crawford, IN
    #[doc(hidden)]
    IN084,
    /// Posey, IN
    #[doc(hidden)]
    IN085,
    /// Vanderburgh, IN
    #[doc(hidden)]
    IN086,
    /// Warrick, IN
    #[doc(hidden)]
    IN087,
    /// Spencer, IN
    #[doc(hidden)]
    IN088,
    /// Perry, IN
    #[doc(hidden)]
    IN089,
    /// Harrison, IN
    #[doc(hidden)]
    IN090,
    /// Floyd, IN
    #[doc(hidden)]
    IN091,
    /// Clark, IN
    #[doc(hidden)]
    IN092,
    /// Cheyenne, KS
    #[doc(hidden)]
    KS001,
    /// Rawlins, KS
    #[doc(hidden)]
    KS002,
    /// Decatur, KS
    #[doc(hidden)]
    KS003,
    /// Norton, KS
    #[doc(hidden)]
    KS004,
    /// Phillips, KS
    #[doc(hidden)]
    KS005,
    /// Smith, KS
    #[doc(hidden)]
    KS006,
    /// Jewell, KS
    #[doc(hidden)]
    KS007,
    /// Republic, KS
    #[doc(hidden)]
    KS008,
    /// Washington, KS
    #[doc(hidden)]
    KS009,
    /// Marshall, KS
    #[doc(hidden)]
    KS010,
    /// Nemaha, KS
    #[doc(hidden)]
    KS011,
    /// Brown, KS
    #[doc(hidden)]
    KS012,
    /// Sherman, KS
    #[doc(hidden)]
    KS013,
    /// Thomas, KS
    #[doc(hidden)]
    KS014,
    /// Sheridan, KS
    #[doc(hidden)]
    KS015,
    /// Graham, KS
    #[doc(hidden)]
    KS016,
    /// Rooks, KS
    #[doc(hidden)]
    KS017,
    /// Osborne, KS
    #[doc(hidden)]
    KS018,
    /// Mitchell, KS
    #[doc(hidden)]
    KS019,
    /// Cloud, KS
    #[doc(hidden)]
    KS020,
    /// Clay, KS
    #[doc(hidden)]
    KS021,
    /// Riley, KS
    #[doc(hidden)]
    KS022,
    /// Pottawatomie, KS
    #[doc(hidden)]
    KS023,
    /// Jackson, KS
    #[doc(hidden)]
    KS024,
    /// Atchison, KS
    #[doc(hidden)]
    KS025,
    /// Jefferson, KS
    #[doc(hidden)]
    KS026,
    /// Wallace, KS
    #[doc(hidden)]
    KS027,
    /// Logan, KS
    #[doc(hidden)]
    KS028,
    /// Gove, KS
    #[doc(hidden)]
    KS029,
    /// Trego, KS
    #[doc(hidden)]
    KS030,
    /// Ellis, KS
    #[doc(hidden)]
    KS031,
    /// Russell, KS
    #[doc(hidden)]
    KS032,
    /// Lincoln, KS
    #[doc(hidden)]
    KS033,
    /// Ottawa, KS
    #[doc(hidden)]
    KS034,
    /// Dickinson, KS
    #[doc(hidden)]
    KS035,
    /// Geary, KS
    #[doc(hidden)]
    KS036,
    /// Morris, KS
    #[doc(hidden)]
    KS037,
    /// Wabaunsee, KS
    #[doc(hidden)]
    KS038,
    /// Shawnee, KS
    #[doc(hidden)]
    KS039,
    /// Douglas, KS
    #[doc(hidden)]
    KS040,
    /// Greeley, KS
    #[doc(hidden)]
    KS041,
    /// Wichita, KS
    #[doc(hidden)]
    KS042,
    /// Scott, KS
    #[doc(hidden)]
    KS043,
    /// Lane, KS
    #[doc(hidden)]
    KS044,
    /// Ness, KS
    #[doc(hidden)]
    KS045,
    /// Rush, KS
    #[doc(hidden)]
    KS046,
    /// Barton, KS
    #[doc(hidden)]
    KS047,
    /// Ellsworth, KS
    #[doc(hidden)]
    KS048,
    /// Saline, KS
    #[doc(hidden)]
    KS049,
    /// Rice, KS
    #[doc(hidden)]
    KS050,
    /// McPherson, KS
    #[doc(hidden)]
    KS051,
    /// Marion, KS
    #[doc(hidden)]
    KS052,
    /// Chase, KS
    #[doc(hidden)]
    KS053,
    /// Lyon, KS
    #[doc(hidden)]
    KS054,
    /// Osage, KS
    #[doc(hidden)]
    KS055,
    /// Franklin, KS
    #[doc(hidden)]
    KS056,
    /// Miami, KS
    #[doc(hidden)]
    KS057,
    /// Coffey, KS
    #[doc(hidden)]
    KS058,
    /// Anderson, KS
    #[doc(hidden)]
    KS059,
    /// Linn, KS
    #[doc(hidden)]
    KS060,
    /// Hamilton, KS
    #[doc(hidden)]
    KS061,
    /// Kearny, KS
    #[doc(hidden)]
    KS062,
    /// Finney, KS
    #[doc(hidden)]
    KS063,
    /// Hodgeman, KS
    #[doc(hidden)]
    KS064,
    /// Pawnee, KS
    #[doc(hidden)]
    KS065,
    /// Stafford, KS
    #[doc(hidden)]
    KS066,
    /// Reno, KS
    #[doc(hidden)]
    KS067,
    /// Harvey, KS
    #[doc(hidden)]
    KS068,
    /// Butler, KS
    #[doc(hidden)]
    KS069,
    /// Greenwood, KS
    #[doc(hidden)]
    KS070,
    /// Woodson, KS
    #[doc(hidden)]
    KS071,
    /// Allen, KS
    #[doc(hidden)]
    KS072,
    /// Bourbon, KS
    #[doc(hidden)]
    KS073,
    /// Stanton, KS
    #[doc(hidden)]
    KS074,
    /// Grant, KS
    #[doc(hidden)]
    KS075,
    /// Haskell, KS
    #[doc(hidden)]
    KS076,
    /// Gray, KS
    #[doc(hidden)]
    KS077,
    /// Ford, KS
    #[doc(hidden)]
    KS078,
    /// Edwards, KS
    #[doc(hidden)]
    KS079,
    /// Kiowa, KS
    #[doc(hidden)]
    KS080,
    /// Pratt, KS
    #[doc(hidden)]
    KS081,
    /// Kingman, KS
    #[doc(hidden)]
    KS082,
    /// Sedgwick, KS
    #[doc(hidden)]
    KS083,
    /// Morton, KS
    #[doc(hidden)]
    KS084,
    /// Stevens, KS
    #[doc(hidden)]
    KS085,
    /// Seward, KS
    #[doc(hidden)]
    KS086,
    /// Meade, KS
    #[doc(hidden)]
    KS087,
    /// Clark, KS
    #[doc(hidden)]
    KS088,
    /// Comanche, KS
    #[doc(hidden)]
    KS089,
    /// Barber, KS
    #[doc(hidden)]
    KS090,
    /// Harper, KS
    #[doc(hidden)]
    KS091,
    /// Sumner, KS
    #[doc(hidden)]
    KS092,
    /// Cowley, KS
    #[doc(hidden)]
    KS093,
    /// Elk, KS
    #[doc(hidden)]
    KS094,
    /// Wilson, KS
    #[doc(hidden)]
    KS095,
    /// Neosho, KS
    #[doc(hidden)]
    KS096,
    /// Crawford, KS
    #[doc(hidden)]
    KS097,
    /// Chautauqua, KS
    #[doc(hidden)]
    KS098,
    /// Montgomery, KS
    #[doc(hidden)]
    KS099,
    /// Labette, KS
    #[doc(hidden)]
    KS100,
    /// Cherokee, KS
    #[doc(hidden)]
    KS101,
    /// Doniphan, KS
    #[doc(hidden)]
    KS102,
    /// Leavenworth, KS
    #[doc(hidden)]
    KS103,
    /// Wyandotte, KS
    #[doc(hidden)]
    KS104,
    /// Johnson, KS
    #[doc(hidden)]
    KS105,
    /// Fulton, KY
    #[doc(hidden)]
    KY001,
    /// Hickman, KY
    #[doc(hidden)]
    KY002,
    /// Carlisle, KY
    #[doc(hidden)]
    KY003,
    /// Ballard, KY
    #[doc(hidden)]
    KY004,
    /// McCracken, KY
    #[doc(hidden)]
    KY005,
    /// Graves, KY
    #[doc(hidden)]
    KY006,
    /// Livingston, KY
    #[doc(hidden)]
    KY007,
    /// Marshall, KY
    #[doc(hidden)]
    KY008,
    /// Calloway, KY
    #[doc(hidden)]
    KY009,
    /// Crittenden, KY
    #[doc(hidden)]
    KY010,
    /// Lyon, KY
    #[doc(hidden)]
    KY011,
    /// Trigg, KY
    #[doc(hidden)]
    KY012,
    /// Caldwell, KY
    #[doc(hidden)]
    KY013,
    /// Union, KY
    #[doc(hidden)]
    KY014,
    /// Webster, KY
    #[doc(hidden)]
    KY015,
    /// Hopkins, KY
    #[doc(hidden)]
    KY016,
    /// Christian, KY
    #[doc(hidden)]
    KY017,
    /// Henderson, KY
    #[doc(hidden)]
    KY018,
    /// Daviess, KY
    #[doc(hidden)]
    KY019,
    /// McLean, KY
    #[doc(hidden)]
    KY020,
    /// Muhlenberg, KY
    #[doc(hidden)]
    KY021,
    /// Todd, KY
    #[doc(hidden)]
    KY022,
    /// Hancock, KY
    #[doc(hidden)]
    KY023,
    /// Breckinridge, KY
    #[doc(hidden)]
    KY024,
    /// Meade, KY
    #[doc(hidden)]
    KY025,
    /// Ohio, KY
    #[doc(hidden)]
    KY026,
    /// Grayson, KY
    #[doc(hidden)]
    KY027,
    /// Hardin, KY
    #[doc(hidden)]
    KY028,
    /// Bullitt, KY
    #[doc(hidden)]
    KY029,
    /// Jefferson, KY
    #[doc(hidden)]
    KY030,
    /// Oldham, KY
    #[doc(hidden)]
    KY031,
    /// Trimble, KY
    #[doc(hidden)]
    KY032,
    /// Henry, KY
    #[doc(hidden)]
    KY033,
    /// Shelby, KY
    #[doc(hidden)]
    KY034,
    /// Franklin, KY
    #[doc(hidden)]
    KY035,
    /// Scott, KY
    #[doc(hidden)]
    KY036,
    /// Harrison, KY
    #[doc(hidden)]
    KY037,
    /// Spencer, KY
    #[doc(hidden)]
    KY038,
    /// Anderson, KY
    #[doc(hidden)]
    KY039,
    /// Woodford, KY
    #[doc(hidden)]
    KY040,
    /// Fayette, KY
    #[doc(hidden)]
    KY041,
    /// Bourbon, KY
    #[doc(hidden)]
    KY042,
    /// Nicholas, KY
    #[doc(hidden)]
    KY043,
    /// Fleming, KY
    #[doc(hidden)]
    KY044,
    /// Nelson, KY
    #[doc(hidden)]
    KY045,
    /// Washington, KY
    #[doc(hidden)]
    KY046,
    /// Mercer, KY
    #[doc(hidden)]
    KY047,
    /// Jessamine, KY
    #[doc(hidden)]
    KY048,
    /// Clark, KY
    #[doc(hidden)]
    KY049,
    /// Montgomery, KY
    #[doc(hidden)]
    KY050,
    /// Bath, KY
    #[doc(hidden)]
    KY051,
    /// Rowan, KY
    #[doc(hidden)]
    KY052,
    /// Larue, KY
    #[doc(hidden)]
    KY053,
    /// Marion, KY
    #[doc(hidden)]
    KY054,
    /// Boyle, KY
    #[doc(hidden)]
    KY055,
    /// Garrard, KY
    #[doc(hidden)]
    KY056,
    /// Madison, KY
    #[doc(hidden)]
    KY057,
    /// Estill, KY
    #[doc(hidden)]
    KY058,
    /// Powell, KY
    #[doc(hidden)]
    KY059,
    /// Menifee, KY
    #[doc(hidden)]
    KY060,
    /// Butler, KY
    #[doc(hidden)]
    KY061,
    /// Edmonson, KY
    #[doc(hidden)]
    KY062,
    /// Hart, KY
    #[doc(hidden)]
    KY063,
    /// Green, KY
    #[doc(hidden)]
    KY064,
    /// Taylor, KY
    #[doc(hidden)]
    KY065,
    /// Casey, KY
    #[doc(hidden)]
    KY066,
    /// Lincoln, KY
    #[doc(hidden)]
    KY067,
    /// Rockcastle, KY
    #[doc(hidden)]
    KY068,
    /// Jackson, KY
    #[doc(hidden)]
    KY069,
    /// Logan, KY
    #[doc(hidden)]
    KY070,
    /// Warren, KY
    #[doc(hidden)]
    KY071,
    /// Simpson, KY
    #[doc(hidden)]
    KY072,
    /// Allen, KY
    #[doc(hidden)]
    KY073,
    /// Barren, KY
    #[doc(hidden)]
    KY074,
    /// Monroe, KY
    #[doc(hidden)]
    KY075,
    /// Metcalfe, KY
    #[doc(hidden)]
    KY076,
    /// Adair, KY
    #[doc(hidden)]
    KY077,
    /// Russell, KY
    #[doc(hidden)]
    KY078,
    /// Pulaski, KY
    #[doc(hidden)]
    KY079,
    /// Laurel, KY
    #[doc(hidden)]
    KY080,
    /// Cumberland, KY
    #[doc(hidden)]
    KY081,
    /// Clinton, KY
    #[doc(hidden)]
    KY082,
    /// Wayne, KY
    #[doc(hidden)]
    KY083,
    /// McCreary, KY
    #[doc(hidden)]
    KY084,
    /// Whitley, KY
    #[doc(hidden)]
    KY085,
    /// Knox, KY
    #[doc(hidden)]
    KY086,
    /// Bell, KY
    #[doc(hidden)]
    KY087,
    /// Harlan, KY
    #[doc(hidden)]
    KY088,
    /// Carroll, KY
    #[doc(hidden)]
    KY089,
    /// Gallatin, KY
    #[doc(hidden)]
    KY090,
    /// Boone, KY
    #[doc(hidden)]
    KY091,
    /// Kenton, KY
    #[doc(hidden)]
    KY092,
    /// Campbell, KY
    #[doc(hidden)]
    KY093,
    /// Owen, KY
    #[doc(hidden)]
    KY094,
    /// Grant, KY
    #[doc(hidden)]
    KY095,
    /// Pendleton, KY
    #[doc(hidden)]
    KY096,
    /// Bracken, KY
    #[doc(hidden)]
    KY097,
    /// Robertson, KY
    #[doc(hidden)]
    KY098,
    /// Mason, KY
    #[doc(hidden)]
    KY099,
    /// Lewis, KY
    #[doc(hidden)]
    KY100,
    /// Greenup, KY
    #[doc(hidden)]
    KY101,
    /// Carter, KY
    #[doc(hidden)]
    KY102,
    /// Boyd, KY
    #[doc(hidden)]
    KY103,
    /// Elliott, KY
    #[doc(hidden)]
    KY104,
    /// Lawrence, KY
    #[doc(hidden)]
    KY105,
    /// Morgan, KY
    #[doc(hidden)]
    KY106,
    /// Johnson, KY
    #[doc(hidden)]
    KY107,
    /// Wolfe, KY
    #[doc(hidden)]
    KY108,
    /// Magoffin, KY
    #[doc(hidden)]
    KY109,
    /// Floyd, KY
    #[doc(hidden)]
    KY110,
    /// Lee, KY
    #[doc(hidden)]
    KY111,
    /// Breathitt, KY
    #[doc(hidden)]
    KY112,
    /// Knott, KY
    #[doc(hidden)]
    KY113,
    /// Owsley, KY
    #[doc(hidden)]
    KY114,
    /// Perry, KY
    #[doc(hidden)]
    KY115,
    /// Clay, KY
    #[doc(hidden)]
    KY116,
    /// Leslie, KY
    #[doc(hidden)]
    KY117,
    /// Letcher, KY
    #[doc(hidden)]
    KY118,
    /// Martin, KY
    #[doc(hidden)]
    KY119,
    /// Pike, KY
    #[doc(hidden)]
    KY120,
    /// Caddo, LA
    #[doc(hidden)]
    LA001,
    /// Bossier, LA
    #[doc(hidden)]
    LA002,
    /// Webster, LA
    #[doc(hidden)]
    LA003,
    /// Claiborne, LA
    #[doc(hidden)]
    LA004,
    /// Lincoln, LA
    #[doc(hidden)]
    LA005,
    /// Union, LA
    #[doc(hidden)]
    LA006,
    /// Morehouse, LA
    #[doc(hidden)]
    LA007,
    /// West Carroll, LA
    #[doc(hidden)]
    LA008,
    /// East Carroll, LA
    #[doc(hidden)]
    LA009,
    /// De Soto, LA
    #[doc(hidden)]
    LA010,
    /// Red River, LA
    #[doc(hidden)]
    LA011,
    /// Bienville, LA
    #[doc(hidden)]
    LA012,
    /// Jackson, LA
    #[doc(hidden)]
    LA013,
    /// Ouachita, LA
    #[doc(hidden)]
    LA014,
    /// Richland, LA
    #[doc(hidden)]
    LA015,
    /// Madison, LA
    #[doc(hidden)]
    LA016,
    /// Sabine, LA
    #[doc(hidden)]
    LA017,
    /// Natchitoches, LA
    #[doc(hidden)]
    LA018,
    /// Winn, LA
    #[doc(hidden)]
    LA019,
    /// Grant, LA
    #[doc(hidden)]
    LA020,
    /// Caldwell, LA
    #[doc(hidden)]
    LA021,
    /// La Salle, LA
    #[doc(hidden)]
    LA022,
    /// Franklin, LA
    #[doc(hidden)]
    LA023,
    /// Catahoula, LA
    #[doc(hidden)]
    LA024,
    /// Tensas, LA
    #[doc(hidden)]
    LA025,
    /// Concordia, LA
    #[doc(hidden)]
    LA026,
    /// Vernon, LA
    #[doc(hidden)]
    LA027,
    /// Rapides, LA
    #[doc(hidden)]
    LA028,
    /// Avoyelles, LA
    #[doc(hidden)]
    LA029,
    /// Beauregard, LA
    #[doc(hidden)]
    LA030,
    /// Allen, LA
    #[doc(hidden)]
    LA031,
    /// Evangeline, LA
    #[doc(hidden)]
    LA032,
    /// St. Landry, LA
    #[doc(hidden)]
    LA033,
    /// Pointe Coupee, LA
    #[doc(hidden)]
    LA034,
    /// West Feliciana, LA
    #[doc(hidden)]
    LA035,
    /// East Feliciana, LA
    #[doc(hidden)]
    LA036,
    /// St. Helena, LA
    #[doc(hidden)]
    LA037,
    /// Washington, LA
    #[doc(hidden)]
    LA039,
    /// Calcasieu, LA
    #[doc(hidden)]
    LA041,
    /// Jefferson Davis, LA
    #[doc(hidden)]
    LA042,
    /// Acadia, LA
    #[doc(hidden)]
    LA043,
    /// Lafayette, LA
    #[doc(hidden)]
    LA044,
    /// Upper St. Martin, LA
    #[doc(hidden)]
    LA045,
    /// Iberville, LA
    #[doc(hidden)]
    LA046,
    /// West Baton Rouge, LA
    #[doc(hidden)]
    LA047,
    /// East Baton Rouge, LA
    #[doc(hidden)]
    LA048,
    /// Vermilion, LA
    #[doc(hidden)]
    LA052,
    /// Iberia, LA
    #[doc(hidden)]
    LA053,
    /// St. Mary, LA
    #[doc(hidden)]
    LA054,
    /// Lower St. Martin, LA
    #[doc(hidden)]
    LA055,
    /// Assumption, LA
    #[doc(hidden)]
    LA056,
    /// St. James, LA
    #[doc(hidden)]
    LA057,
    /// St. John The Baptist, LA
    #[doc(hidden)]
    LA058,
    /// Upper Lafourche, LA
    #[doc(hidden)]
    LA059,
    /// St. Charles, LA
    #[doc(hidden)]
    LA060,
    /// Upper St. Bernard, LA
    #[doc(hidden)]
    LA064,
    /// Upper Terrebonne, LA
    #[doc(hidden)]
    LA065,
    /// Lower Terrebonne, LA
    #[doc(hidden)]
    LA066,
    /// Lower Lafourche, LA
    #[doc(hidden)]
    LA067,
    /// Coastal Jefferson, LA
    #[doc(hidden)]
    LA068,
    /// Lower Plaquemines, LA
    #[doc(hidden)]
    LA069,
    /// Lower St. Bernard, LA
    #[doc(hidden)]
    LA070,
    /// Northern Tangipahoa, LA
    #[doc(hidden)]
    LA071,
    /// West Cameron, LA
    #[doc(hidden)]
    LA073,
    /// East Cameron, LA
    #[doc(hidden)]
    LA074,
    /// Southeast St. Tammany, LA
    #[doc(hidden)]
    LA076,
    /// Western Orleans, LA
    #[doc(hidden)]
    LA077,
    /// Eastern Orleans, LA
    #[doc(hidden)]
    LA078,
    /// Northern St. Tammany, LA
    #[doc(hidden)]
    LA079,
    /// Southwestern St. Tammany, LA
    #[doc(hidden)]
    LA080,
    /// Central Tangipahoa, LA
    #[doc(hidden)]
    LA081,
    /// Lower Tangipahoa, LA
    #[doc(hidden)]
    LA082,
    /// Northern Livingston, LA
    #[doc(hidden)]
    LA083,
    /// Southern Livingston, LA
    #[doc(hidden)]
    LA084,
    /// Western Ascension, LA
    #[doc(hidden)]
    LA085,
    /// Eastern Ascension, LA
    #[doc(hidden)]
    LA086,
    /// Upper Jefferson, LA
    #[doc(hidden)]
    LA087,
    /// Lower Jefferson, LA
    #[doc(hidden)]
    LA088,
    /// Upper Plaquemines, LA
    #[doc(hidden)]
    LA089,
    /// Central Plaquemines, LA
    #[doc(hidden)]
    LA090,
    /// Northern Berkshire, MA
    #[doc(hidden)]
    MA001,
    /// Western Franklin, MA
    #[doc(hidden)]
    MA002,
    /// Eastern Franklin, MA
    #[doc(hidden)]
    MA003,
    /// Northern Worcester, MA
    #[doc(hidden)]
    MA004,
    /// Central Middlesex County, MA
    #[doc(hidden)]
    MA005,
    /// Western Essex, MA
    #[doc(hidden)]
    MA006,
    /// Eastern Essex, MA
    #[doc(hidden)]
    MA007,
    /// Western Hampshire, MA
    #[doc(hidden)]
    MA008,
    /// Western Hampden, MA
    #[doc(hidden)]
    MA009,
    /// Eastern Hampshire, MA
    #[doc(hidden)]
    MA010,
    /// Eastern Hampden, MA
    #[doc(hidden)]
    MA011,
    /// Southern Worcester, MA
    #[doc(hidden)]
    MA012,
    /// Western Norfolk, MA
    #[doc(hidden)]
    MA013,
    /// Southeast Middlesex, MA
    #[doc(hidden)]
    MA014,
    /// Suffolk, MA
    #[doc(hidden)]
    MA015,
    /// Eastern Norfolk, MA
    #[doc(hidden)]
    MA016,
    /// Northern Bristol, MA
    #[doc(hidden)]
    MA017,
    /// Western Plymouth, MA
    #[doc(hidden)]
    MA018,
    /// Eastern Plymouth, MA
    #[doc(hidden)]
    MA019,
    /// Southern Bristol, MA
    #[doc(hidden)]
    MA020,
    /// Southern Plymouth, MA
    #[doc(hidden)]
    MA021,
    /// Barnstable, MA
    #[doc(hidden)]
    MA022,
    /// Dukes, MA
    #[doc(hidden)]
    MA023,
    /// Nantucket, MA
    #[doc(hidden)]
    MA024,
    /// Southern Berkshire, MA
    #[doc(hidden)]
    MA025,
    /// Northwest Middlesex County, MA
    #[doc(hidden)]
    MA026,
    /// Garrett, MD
    #[doc(hidden)]
    MD001,
    /// Washington, MD
    #[doc(hidden)]
    MD003,
    /// Frederick, MD
    #[doc(hidden)]
    MD004,
    /// Carroll, MD
    #[doc(hidden)]
    MD005,
    /// Northern Baltimore, MD
    #[doc(hidden)]
    MD006,
    /// Cecil, MD
    #[doc(hidden)]
    MD008,
    /// Southern Baltimore, MD
    #[doc(hidden)]
    MD011,
    /// Kent, MD
    #[doc(hidden)]
    MD012,
    /// Prince Georges, MD
    #[doc(hidden)]
    MD013,
    /// Anne Arundel, MD
    #[doc(hidden)]
    MD014,
    /// Queen Anne's, MD
    #[doc(hidden)]
    MD015,
    /// Charles, MD
    #[doc(hidden)]
    MD016,
    /// St. Marys, MD
    #[doc(hidden)]
    MD017,
    /// Calvert, MD
    #[doc(hidden)]
    MD018,
    /// Talbot, MD
    #[doc(hidden)]
    MD019,
    /// Caroline, MD
    #[doc(hidden)]
    MD020,
    /// Dorchester, MD
    #[doc(hidden)]
    MD021,
    /// Wicomico, MD
    #[doc(hidden)]
    MD022,
    /// Somerset, MD
    #[doc(hidden)]
    MD023,
    /// Inland Worcester, MD
    #[doc(hidden)]
    MD024,
    /// Maryland Beaches, MD
    #[doc(hidden)]
    MD025,
    /// Extreme Western Allegany, MD
    #[doc(hidden)]
    MD501,
    /// Central and Eastern Allegany, MD
    #[doc(hidden)]
    MD502,
    /// Northwest Montgomery, MD
    #[doc(hidden)]
    MD503,
    /// Central and Southeast Montgomery, MD
    #[doc(hidden)]
    MD504,
    /// Northwest Howard, MD
    #[doc(hidden)]
    MD505,
    /// Central and Southeast Howard, MD
    #[doc(hidden)]
    MD506,
    /// Northwest Harford, MD
    #[doc(hidden)]
    MD507,
    /// Southeast Harford, MD
    #[doc(hidden)]
    MD508,
    /// Northwest Aroostook, ME
    #[doc(hidden)]
    ME001,
    /// Northeast Aroostook, ME
    #[doc(hidden)]
    ME002,
    /// Northern Somerset, ME
    #[doc(hidden)]
    ME003,
    /// Northern Piscataquis, ME
    #[doc(hidden)]
    ME004,
    /// Northern Penobscot, ME
    #[doc(hidden)]
    ME005,
    /// Southeast Aroostook, ME
    #[doc(hidden)]
    ME006,
    /// Northern Oxford, ME
    #[doc(hidden)]
    ME007,
    /// Northern Franklin, ME
    #[doc(hidden)]
    ME008,
    /// Central Somerset, ME
    #[doc(hidden)]
    ME009,
    /// Central Piscataquis, ME
    #[doc(hidden)]
    ME010,
    /// Central Penobscot, ME
    #[doc(hidden)]
    ME011,
    /// Southern Oxford, ME
    #[doc(hidden)]
    ME012,
    /// Southern Franklin, ME
    #[doc(hidden)]
    ME013,
    /// Southern Somerset, ME
    #[doc(hidden)]
    ME014,
    /// Southern Penobscot, ME
    #[doc(hidden)]
    ME015,
    /// Interior Hancock, ME
    #[doc(hidden)]
    ME016,
    /// Central Washington, ME
    #[doc(hidden)]
    ME017,
    /// Interior York, ME
    #[doc(hidden)]
    ME018,
    /// Central Interior Cumberland, ME
    #[doc(hidden)]
    ME019,
    /// Androscoggin, ME
    #[doc(hidden)]
    ME020,
    /// Kennebec, ME
    #[doc(hidden)]
    ME021,
    /// Interior Waldo, ME
    #[doc(hidden)]
    ME022,
    /// Coastal York, ME
    #[doc(hidden)]
    ME023,
    /// Coastal Cumberland, ME
    #[doc(hidden)]
    ME024,
    /// Sagadahoc, ME
    #[doc(hidden)]
    ME025,
    /// Lincoln, ME
    #[doc(hidden)]
    ME026,
    /// Knox, ME
    #[doc(hidden)]
    ME027,
    /// Coastal Waldo, ME
    #[doc(hidden)]
    ME028,
    /// Coastal Hancock, ME
    #[doc(hidden)]
    ME029,
    /// Coastal Washington, ME
    #[doc(hidden)]
    ME030,
    /// Southern Piscataquis, ME
    #[doc(hidden)]
    ME031,
    /// Northern Washington, ME
    #[doc(hidden)]
    ME032,
    /// Interior Cumberland Highlands, ME
    #[doc(hidden)]
    ME033,
    /// Arno, MH
    #[doc(hidden)]
    MH001,
    /// Majuro, MH
    #[doc(hidden)]
    MH002,
    /// Wotje, MH
    #[doc(hidden)]
    MH003,
    /// Mili, MH
    #[doc(hidden)]
    MH004,
    /// Utrok, MH
    #[doc(hidden)]
    MH005,
    /// Jaluit, MH
    #[doc(hidden)]
    MH006,
    /// Ailinglaplap, MH
    #[doc(hidden)]
    MH007,
    /// Kwajalein, MH
    #[doc(hidden)]
    MH008,
    /// Ujae, MH
    #[doc(hidden)]
    MH009,
    /// Enewetak, MH
    #[doc(hidden)]
    MH010,
    /// Keweenaw, MI
    #[doc(hidden)]
    MI001,
    /// Ontonagon, MI
    #[doc(hidden)]
    MI002,
    /// Houghton, MI
    #[doc(hidden)]
    MI003,
    /// Baraga, MI
    #[doc(hidden)]
    MI004,
    /// Marquette, MI
    #[doc(hidden)]
    MI005,
    /// Alger, MI
    #[doc(hidden)]
    MI006,
    /// Luce, MI
    #[doc(hidden)]
    MI007,
    /// Gogebic, MI
    #[doc(hidden)]
    MI009,
    /// Iron, MI
    #[doc(hidden)]
    MI010,
    /// Dickinson, MI
    #[doc(hidden)]
    MI011,
    /// Menominee, MI
    #[doc(hidden)]
    MI012,
    /// Delta, MI
    #[doc(hidden)]
    MI013,
    /// Southern Schoolcraft, MI
    #[doc(hidden)]
    MI014,
    /// Emmet, MI
    #[doc(hidden)]
    MI016,
    /// Cheboygan, MI
    #[doc(hidden)]
    MI017,
    /// Presque Isle, MI
    #[doc(hidden)]
    MI018,
    /// Leelanau, MI
    #[doc(hidden)]
    MI020,
    /// Antrim, MI
    #[doc(hidden)]
    MI021,
    /// Otsego, MI
    #[doc(hidden)]
    MI022,
    /// Montmorency, MI
    #[doc(hidden)]
    MI023,
    /// Alpena, MI
    #[doc(hidden)]
    MI024,
    /// Benzie, MI
    #[doc(hidden)]
    MI025,
    /// Grand Traverse, MI
    #[doc(hidden)]
    MI026,
    /// Kalkaska, MI
    #[doc(hidden)]
    MI027,
    /// Crawford, MI
    #[doc(hidden)]
    MI028,
    /// Oscoda, MI
    #[doc(hidden)]
    MI029,
    /// Alcona, MI
    #[doc(hidden)]
    MI030,
    /// Manistee, MI
    #[doc(hidden)]
    MI031,
    /// Wexford, MI
    #[doc(hidden)]
    MI032,
    /// Missaukee, MI
    #[doc(hidden)]
    MI033,
    /// Roscommon, MI
    #[doc(hidden)]
    MI034,
    /// Ogemaw, MI
    #[doc(hidden)]
    MI035,
    /// Iosco, MI
    #[doc(hidden)]
    MI036,
    /// Mason, MI
    #[doc(hidden)]
    MI037,
    /// Lake, MI
    #[doc(hidden)]
    MI038,
    /// Osceola, MI
    #[doc(hidden)]
    MI039,
    /// Clare, MI
    #[doc(hidden)]
    MI040,
    /// Gladwin, MI
    #[doc(hidden)]
    MI041,
    /// Arenac, MI
    #[doc(hidden)]
    MI042,
    /// Oceana, MI
    #[doc(hidden)]
    MI043,
    /// Newaygo, MI
    #[doc(hidden)]
    MI044,
    /// Mecosta, MI
    #[doc(hidden)]
    MI045,
    /// Isabella, MI
    #[doc(hidden)]
    MI046,
    /// Midland, MI
    #[doc(hidden)]
    MI047,
    /// Bay, MI
    #[doc(hidden)]
    MI048,
    /// Huron, MI
    #[doc(hidden)]
    MI049,
    /// Muskegon, MI
    #[doc(hidden)]
    MI050,
    /// Montcalm, MI
    #[doc(hidden)]
    MI051,
    /// Gratiot, MI
    #[doc(hidden)]
    MI052,
    /// Saginaw, MI
    #[doc(hidden)]
    MI053,
    /// Tuscola, MI
    #[doc(hidden)]
    MI054,
    /// Sanilac, MI
    #[doc(hidden)]
    MI055,
    /// Ottawa, MI
    #[doc(hidden)]
    MI056,
    /// Kent, MI
    #[doc(hidden)]
    MI057,
    /// Ionia, MI
    #[doc(hidden)]
    MI058,
    /// Clinton, MI
    #[doc(hidden)]
    MI059,
    /// Shiawassee, MI
    #[doc(hidden)]
    MI060,
    /// Genesee, MI
    #[doc(hidden)]
    MI061,
    /// Lapeer, MI
    #[doc(hidden)]
    MI062,
    /// St. Clair, MI
    #[doc(hidden)]
    MI063,
    /// Allegan, MI
    #[doc(hidden)]
    MI064,
    /// Barry, MI
    #[doc(hidden)]
    MI065,
    /// Eaton, MI
    #[doc(hidden)]
    MI066,
    /// Ingham, MI
    #[doc(hidden)]
    MI067,
    /// Livingston, MI
    #[doc(hidden)]
    MI068,
    /// Oakland, MI
    #[doc(hidden)]
    MI069,
    /// Macomb, MI
    #[doc(hidden)]
    MI070,
    /// Van Buren, MI
    #[doc(hidden)]
    MI071,
    /// Kalamazoo, MI
    #[doc(hidden)]
    MI072,
    /// Calhoun, MI
    #[doc(hidden)]
    MI073,
    /// Jackson, MI
    #[doc(hidden)]
    MI074,
    /// Washtenaw, MI
    #[doc(hidden)]
    MI075,
    /// Wayne, MI
    #[doc(hidden)]
    MI076,
    /// Berrien, MI
    #[doc(hidden)]
    MI077,
    /// Cass, MI
    #[doc(hidden)]
    MI078,
    /// St. Joseph, MI
    #[doc(hidden)]
    MI079,
    /// Branch, MI
    #[doc(hidden)]
    MI080,
    /// Hillsdale, MI
    #[doc(hidden)]
    MI081,
    /// Lenawee, MI
    #[doc(hidden)]
    MI082,
    /// Monroe, MI
    #[doc(hidden)]
    MI083,
    /// Southern Houghton, MI
    #[doc(hidden)]
    MI084,
    /// Northern Schoolcraft, MI
    #[doc(hidden)]
    MI085,
    /// Western Chippewa, MI
    #[doc(hidden)]
    MI086,
    /// Central Chippewa, MI
    #[doc(hidden)]
    MI087,
    /// Southeast Chippewa, MI
    #[doc(hidden)]
    MI088,
    /// Western Mackinac, MI
    #[doc(hidden)]
    MI095,
    /// Eastern Mackinac, MI
    #[doc(hidden)]
    MI096,
    /// Mackinac Island/Bois Blanc Island, MI
    #[doc(hidden)]
    MI097,
    /// Beaver Island and surrounding islands, MI
    #[doc(hidden)]
    MI098,
    /// Charlevoix, MI
    #[doc(hidden)]
    MI099,
    /// West Polk, MN
    #[doc(hidden)]
    MN001,
    /// Norman, MN
    #[doc(hidden)]
    MN002,
    /// Clay, MN
    #[doc(hidden)]
    MN003,
    /// Kittson, MN
    #[doc(hidden)]
    MN004,
    /// Roseau, MN
    #[doc(hidden)]
    MN005,
    /// Lake Of The Woods, MN
    #[doc(hidden)]
    MN006,
    /// West Marshall, MN
    #[doc(hidden)]
    MN007,
    /// East Marshall, MN
    #[doc(hidden)]
    MN008,
    /// North Beltrami, MN
    #[doc(hidden)]
    MN009,
    /// Koochiching, MN
    #[doc(hidden)]
    MN010,
    /// North St. Louis, MN
    #[doc(hidden)]
    MN011,
    /// Northern Cook/Northern Lake, MN
    #[doc(hidden)]
    MN012,
    /// Pennington, MN
    #[doc(hidden)]
    MN013,
    /// Red Lake, MN
    #[doc(hidden)]
    MN014,
    /// East Polk, MN
    #[doc(hidden)]
    MN015,
    /// North Clearwater, MN
    #[doc(hidden)]
    MN016,
    /// South Beltrami, MN
    #[doc(hidden)]
    MN017,
    /// North Itasca, MN
    #[doc(hidden)]
    MN018,
    /// Central St. Louis, MN
    #[doc(hidden)]
    MN019,
    /// Southern Lake/North Shore, MN
    #[doc(hidden)]
    MN020,
    /// Southern Cook/North Shore, MN
    #[doc(hidden)]
    MN021,
    /// Mahnomen, MN
    #[doc(hidden)]
    MN022,
    /// South Clearwater, MN
    #[doc(hidden)]
    MN023,
    /// Hubbard, MN
    #[doc(hidden)]
    MN024,
    /// North Cass, MN
    #[doc(hidden)]
    MN025,
    /// South Itasca, MN
    #[doc(hidden)]
    MN026,
    /// West Becker, MN
    #[doc(hidden)]
    MN027,
    /// East Becker, MN
    #[doc(hidden)]
    MN028,
    /// Wilkin, MN
    #[doc(hidden)]
    MN029,
    /// West Otter Tail, MN
    #[doc(hidden)]
    MN030,
    /// East Otter Tail, MN
    #[doc(hidden)]
    MN031,
    /// Wadena, MN
    #[doc(hidden)]
    MN032,
    /// South Cass, MN
    #[doc(hidden)]
    MN033,
    /// Crow Wing, MN
    #[doc(hidden)]
    MN034,
    /// Northern Aitkin, MN
    #[doc(hidden)]
    MN035,
    /// South Aitkin, MN
    #[doc(hidden)]
    MN036,
    /// Carlton/South St. Louis, MN
    #[doc(hidden)]
    MN037,
    /// Pine, MN
    #[doc(hidden)]
    MN038,
    /// Traverse, MN
    #[doc(hidden)]
    MN039,
    /// Grant, MN
    #[doc(hidden)]
    MN040,
    /// Douglas, MN
    #[doc(hidden)]
    MN041,
    /// Todd, MN
    #[doc(hidden)]
    MN042,
    /// Morrison, MN
    #[doc(hidden)]
    MN043,
    /// Mille Lacs, MN
    #[doc(hidden)]
    MN044,
    /// Kanabec, MN
    #[doc(hidden)]
    MN045,
    /// Big Stone, MN
    #[doc(hidden)]
    MN046,
    /// Stevens, MN
    #[doc(hidden)]
    MN047,
    /// Pope, MN
    #[doc(hidden)]
    MN048,
    /// Stearns, MN
    #[doc(hidden)]
    MN049,
    /// Benton, MN
    #[doc(hidden)]
    MN050,
    /// Sherburne, MN
    #[doc(hidden)]
    MN051,
    /// Isanti, MN
    #[doc(hidden)]
    MN052,
    /// Chisago, MN
    #[doc(hidden)]
    MN053,
    /// Lac Qui Parle, MN
    #[doc(hidden)]
    MN054,
    /// Swift, MN
    #[doc(hidden)]
    MN055,
    /// Chippewa, MN
    #[doc(hidden)]
    MN056,
    /// Kandiyohi, MN
    #[doc(hidden)]
    MN057,
    /// Meeker, MN
    #[doc(hidden)]
    MN058,
    /// Wright, MN
    #[doc(hidden)]
    MN059,
    /// Hennepin, MN
    #[doc(hidden)]
    MN060,
    /// Anoka, MN
    #[doc(hidden)]
    MN061,
    /// Ramsey, MN
    #[doc(hidden)]
    MN062,
    /// Washington, MN
    #[doc(hidden)]
    MN063,
    /// Yellow Medicine, MN
    #[doc(hidden)]
    MN064,
    /// Renville, MN
    #[doc(hidden)]
    MN065,
    /// McLeod, MN
    #[doc(hidden)]
    MN066,
    /// Sibley, MN
    #[doc(hidden)]
    MN067,
    /// Carver, MN
    #[doc(hidden)]
    MN068,
    /// Scott, MN
    #[doc(hidden)]
    MN069,
    /// Dakota, MN
    #[doc(hidden)]
    MN070,
    /// Lincoln, MN
    #[doc(hidden)]
    MN071,
    /// Lyon, MN
    #[doc(hidden)]
    MN072,
    /// Redwood, MN
    #[doc(hidden)]
    MN073,
    /// Brown, MN
    #[doc(hidden)]
    MN074,
    /// Nicollet, MN
    #[doc(hidden)]
    MN075,
    /// Le Sueur, MN
    #[doc(hidden)]
    MN076,
    /// Rice, MN
    #[doc(hidden)]
    MN077,
    /// Goodhue, MN
    #[doc(hidden)]
    MN078,
    /// Wabasha, MN
    #[doc(hidden)]
    MN079,
    /// Murray, MN
    #[doc(hidden)]
    MN080,
    /// Cottonwood, MN
    #[doc(hidden)]
    MN081,
    /// Watonwan, MN
    #[doc(hidden)]
    MN082,
    /// Blue Earth, MN
    #[doc(hidden)]
    MN083,
    /// Waseca, MN
    #[doc(hidden)]
    MN084,
    /// Steele, MN
    #[doc(hidden)]
    MN085,
    /// Dodge, MN
    #[doc(hidden)]
    MN086,
    /// Olmsted, MN
    #[doc(hidden)]
    MN087,
    /// Winona, MN
    #[doc(hidden)]
    MN088,
    /// Nobles, MN
    #[doc(hidden)]
    MN089,
    /// Jackson, MN
    #[doc(hidden)]
    MN090,
    /// Martin, MN
    #[doc(hidden)]
    MN091,
    /// Faribault, MN
    #[doc(hidden)]
    MN092,
    /// Freeborn, MN
    #[doc(hidden)]
    MN093,
    /// Mower, MN
    #[doc(hidden)]
    MN094,
    /// Fillmore, MN
    #[doc(hidden)]
    MN095,
    /// Houston, MN
    #[doc(hidden)]
    MN096,
    /// Pipestone, MN
    #[doc(hidden)]
    MN097,
    /// Rock, MN
    #[doc(hidden)]
    MN098,
    /// Atchison, MO
    #[doc(hidden)]
    MO001,
    /// Nodaway, MO
    #[doc(hidden)]
    MO002,
    /// Worth, MO
    #[doc(hidden)]
    MO003,
    /// Gentry, MO
    #[doc(hidden)]
    MO004,
    /// Harrison, MO
    #[doc(hidden)]
    MO005,
    /// Mercer, MO
    #[doc(hidden)]
    MO006,
    /// Putnam, MO
    #[doc(hidden)]
    MO007,
    /// Schuyler, MO
    #[doc(hidden)]
    MO008,
    /// Scotland, MO
    #[doc(hidden)]
    MO009,
    /// Clark, MO
    #[doc(hidden)]
    MO010,
    /// Holt, MO
    #[doc(hidden)]
    MO011,
    /// Andrew, MO
    #[doc(hidden)]
    MO012,
    /// De Kalb, MO
    #[doc(hidden)]
    MO013,
    /// Daviess, MO
    #[doc(hidden)]
    MO014,
    /// Grundy, MO
    #[doc(hidden)]
    MO015,
    /// Sullivan, MO
    #[doc(hidden)]
    MO016,
    /// Adair, MO
    #[doc(hidden)]
    MO017,
    /// Knox, MO
    #[doc(hidden)]
    MO018,
    /// Lewis, MO
    #[doc(hidden)]
    MO019,
    /// Buchanan, MO
    #[doc(hidden)]
    MO020,
    /// Clinton, MO
    #[doc(hidden)]
    MO021,
    /// Caldwell, MO
    #[doc(hidden)]
    MO022,
    /// Livingston, MO
    #[doc(hidden)]
    MO023,
    /// Linn, MO
    #[doc(hidden)]
    MO024,
    /// Macon, MO
    #[doc(hidden)]
    MO025,
    /// Shelby, MO
    #[doc(hidden)]
    MO026,
    /// Marion, MO
    #[doc(hidden)]
    MO027,
    /// Platte, MO
    #[doc(hidden)]
    MO028,
    /// Clay, MO
    #[doc(hidden)]
    MO029,
    /// Ray, MO
    #[doc(hidden)]
    MO030,
    /// Carroll, MO
    #[doc(hidden)]
    MO031,
    /// Chariton, MO
    #[doc(hidden)]
    MO032,
    /// Randolph, MO
    #[doc(hidden)]
    MO033,
    /// Monroe, MO
    #[doc(hidden)]
    MO034,
    /// Ralls, MO
    #[doc(hidden)]
    MO035,
    /// Pike, MO
    #[doc(hidden)]
    MO036,
    /// Jackson, MO
    #[doc(hidden)]
    MO037,
    /// Lafayette, MO
    #[doc(hidden)]
    MO038,
    /// Saline, MO
    #[doc(hidden)]
    MO039,
    /// Howard, MO
    #[doc(hidden)]
    MO040,
    /// Boone, MO
    #[doc(hidden)]
    MO041,
    /// Audrain, MO
    #[doc(hidden)]
    MO042,
    /// Cass, MO
    #[doc(hidden)]
    MO043,
    /// Johnson, MO
    #[doc(hidden)]
    MO044,
    /// Pettis, MO
    #[doc(hidden)]
    MO045,
    /// Cooper, MO
    #[doc(hidden)]
    MO046,
    /// Moniteau, MO
    #[doc(hidden)]
    MO047,
    /// Cole, MO
    #[doc(hidden)]
    MO048,
    /// Osage, MO
    #[doc(hidden)]
    MO049,
    /// Callaway, MO
    #[doc(hidden)]
    MO050,
    /// Montgomery, MO
    #[doc(hidden)]
    MO051,
    /// Lincoln, MO
    #[doc(hidden)]
    MO052,
    /// Bates, MO
    #[doc(hidden)]
    MO053,
    /// Henry, MO
    #[doc(hidden)]
    MO054,
    /// Benton, MO
    #[doc(hidden)]
    MO055,
    /// Morgan, MO
    #[doc(hidden)]
    MO056,
    /// Miller, MO
    #[doc(hidden)]
    MO057,
    /// Maries, MO
    #[doc(hidden)]
    MO058,
    /// Gasconade, MO
    #[doc(hidden)]
    MO059,
    /// Warren, MO
    #[doc(hidden)]
    MO060,
    /// St. Charles, MO
    #[doc(hidden)]
    MO061,
    /// Franklin, MO
    #[doc(hidden)]
    MO062,
    /// St. Louis, MO
    #[doc(hidden)]
    MO063,
    /// St. Louis City, MO
    #[doc(hidden)]
    MO064,
    /// Jefferson, MO
    #[doc(hidden)]
    MO065,
    /// Vernon, MO
    #[doc(hidden)]
    MO066,
    /// St. Clair, MO
    #[doc(hidden)]
    MO067,
    /// Hickory, MO
    #[doc(hidden)]
    MO068,
    /// Camden, MO
    #[doc(hidden)]
    MO069,
    /// Pulaski, MO
    #[doc(hidden)]
    MO070,
    /// Phelps, MO
    #[doc(hidden)]
    MO071,
    /// Crawford, MO
    #[doc(hidden)]
    MO072,
    /// Washington, MO
    #[doc(hidden)]
    MO073,
    /// St. Francois, MO
    #[doc(hidden)]
    MO074,
    /// Ste. Genevieve, MO
    #[doc(hidden)]
    MO075,
    /// Perry, MO
    #[doc(hidden)]
    MO076,
    /// Barton, MO
    #[doc(hidden)]
    MO077,
    /// Cedar, MO
    #[doc(hidden)]
    MO078,
    /// Polk, MO
    #[doc(hidden)]
    MO079,
    /// Dallas, MO
    #[doc(hidden)]
    MO080,
    /// Laclede, MO
    #[doc(hidden)]
    MO081,
    /// Texas, MO
    #[doc(hidden)]
    MO082,
    /// Dent, MO
    #[doc(hidden)]
    MO083,
    /// Iron, MO
    #[doc(hidden)]
    MO084,
    /// Madison, MO
    #[doc(hidden)]
    MO085,
    /// Bollinger, MO
    #[doc(hidden)]
    MO086,
    /// Cape Girardeau, MO
    #[doc(hidden)]
    MO087,
    /// Jasper, MO
    #[doc(hidden)]
    MO088,
    /// Dade, MO
    #[doc(hidden)]
    MO089,
    /// Greene, MO
    #[doc(hidden)]
    MO090,
    /// Webster, MO
    #[doc(hidden)]
    MO091,
    /// Wright, MO
    #[doc(hidden)]
    MO092,
    /// Newton, MO
    #[doc(hidden)]
    MO093,
    /// Lawrence, MO
    #[doc(hidden)]
    MO094,
    /// Christian, MO
    #[doc(hidden)]
    MO095,
    /// Douglas, MO
    #[doc(hidden)]
    MO096,
    /// Howell, MO
    #[doc(hidden)]
    MO097,
    /// Shannon, MO
    #[doc(hidden)]
    MO098,
    /// Reynolds, MO
    #[doc(hidden)]
    MO099,
    /// Wayne, MO
    #[doc(hidden)]
    MO100,
    /// McDonald, MO
    #[doc(hidden)]
    MO101,
    /// Barry, MO
    #[doc(hidden)]
    MO102,
    /// Stone, MO
    #[doc(hidden)]
    MO103,
    /// Taney, MO
    #[doc(hidden)]
    MO104,
    /// Ozark, MO
    #[doc(hidden)]
    MO105,
    /// Oregon, MO
    #[doc(hidden)]
    MO106,
    /// Carter, MO
    #[doc(hidden)]
    MO107,
    /// Ripley, MO
    #[doc(hidden)]
    MO108,
    /// Butler, MO
    #[doc(hidden)]
    MO109,
    /// Stoddard, MO
    #[doc(hidden)]
    MO110,
    /// Scott, MO
    #[doc(hidden)]
    MO111,
    /// Mississippi, MO
    #[doc(hidden)]
    MO112,
    /// Dunklin, MO
    #[doc(hidden)]
    MO113,
    /// New Madrid, MO
    #[doc(hidden)]
    MO114,
    /// Pemiscot, MO
    #[doc(hidden)]
    MO115,
    /// Rota, MP
    #[doc(hidden)]
    MP001,
    /// Tinian, MP
    #[doc(hidden)]
    MP002,
    /// Saipan, MP
    #[doc(hidden)]
    MP003,
    /// Anatahan, MP
    #[doc(hidden)]
    MP004,
    /// Alamagan, MP
    #[doc(hidden)]
    MP005,
    /// Pagan, MP
    #[doc(hidden)]
    MP006,
    /// Agrihan, MP
    #[doc(hidden)]
    MP007,
    /// DeSoto, MS
    #[doc(hidden)]
    MS001,
    /// Marshall, MS
    #[doc(hidden)]
    MS002,
    /// Benton, MS
    #[doc(hidden)]
    MS003,
    /// Tippah, MS
    #[doc(hidden)]
    MS004,
    /// Alcorn, MS
    #[doc(hidden)]
    MS005,
    /// Tishomingo, MS
    #[doc(hidden)]
    MS006,
    /// Tunica, MS
    #[doc(hidden)]
    MS007,
    /// Tate, MS
    #[doc(hidden)]
    MS008,
    /// Prentiss, MS
    #[doc(hidden)]
    MS009,
    /// Coahoma, MS
    #[doc(hidden)]
    MS010,
    /// Quitman, MS
    #[doc(hidden)]
    MS011,
    /// Panola, MS
    #[doc(hidden)]
    MS012,
    /// Lafayette, MS
    #[doc(hidden)]
    MS013,
    /// Union, MS
    #[doc(hidden)]
    MS014,
    /// Pontotoc, MS
    #[doc(hidden)]
    MS015,
    /// Lee, MS
    #[doc(hidden)]
    MS016,
    /// Itawamba, MS
    #[doc(hidden)]
    MS017,
    /// Bolivar, MS
    #[doc(hidden)]
    MS018,
    /// Sunflower, MS
    #[doc(hidden)]
    MS019,
    /// Tallahatchie, MS
    #[doc(hidden)]
    MS020,
    /// Yalobusha, MS
    #[doc(hidden)]
    MS021,
    /// Calhoun, MS
    #[doc(hidden)]
    MS022,
    /// Chickasaw, MS
    #[doc(hidden)]
    MS023,
    /// Monroe, MS
    #[doc(hidden)]
    MS024,
    /// Leflore, MS
    #[doc(hidden)]
    MS025,
    /// Grenada, MS
    #[doc(hidden)]
    MS026,
    /// Carroll, MS
    #[doc(hidden)]
    MS027,
    /// Montgomery, MS
    #[doc(hidden)]
    MS028,
    /// Webster, MS
    #[doc(hidden)]
    MS029,
    /// Clay, MS
    #[doc(hidden)]
    MS030,
    /// Lowndes, MS
    #[doc(hidden)]
    MS031,
    /// Choctaw, MS
    #[doc(hidden)]
    MS032,
    /// Oktibbeha, MS
    #[doc(hidden)]
    MS033,
    /// Washington, MS
    #[doc(hidden)]
    MS034,
    /// Humphreys, MS
    #[doc(hidden)]
    MS035,
    /// Holmes, MS
    #[doc(hidden)]
    MS036,
    /// Attala, MS
    #[doc(hidden)]
    MS037,
    /// Winston, MS
    #[doc(hidden)]
    MS038,
    /// Noxubee, MS
    #[doc(hidden)]
    MS039,
    /// Issaquena, MS
    #[doc(hidden)]
    MS040,
    /// Sharkey, MS
    #[doc(hidden)]
    MS041,
    /// Yazoo, MS
    #[doc(hidden)]
    MS042,
    /// Madison, MS
    #[doc(hidden)]
    MS043,
    /// Leake, MS
    #[doc(hidden)]
    MS044,
    /// Neshoba, MS
    #[doc(hidden)]
    MS045,
    /// Kemper, MS
    #[doc(hidden)]
    MS046,
    /// Warren, MS
    #[doc(hidden)]
    MS047,
    /// Hinds, MS
    #[doc(hidden)]
    MS048,
    /// Rankin, MS
    #[doc(hidden)]
    MS049,
    /// Scott, MS
    #[doc(hidden)]
    MS050,
    /// Newton, MS
    #[doc(hidden)]
    MS051,
    /// Lauderdale, MS
    #[doc(hidden)]
    MS052,
    /// Claiborne, MS
    #[doc(hidden)]
    MS053,
    /// Copiah, MS
    #[doc(hidden)]
    MS054,
    /// Simpson, MS
    #[doc(hidden)]
    MS055,
    /// Smith, MS
    #[doc(hidden)]
    MS056,
    /// Jasper, MS
    #[doc(hidden)]
    MS057,
    /// Clarke, MS
    #[doc(hidden)]
    MS058,
    /// Jefferson, MS
    #[doc(hidden)]
    MS059,
    /// Adams, MS
    #[doc(hidden)]
    MS060,
    /// Franklin, MS
    #[doc(hidden)]
    MS061,
    /// Lincoln, MS
    #[doc(hidden)]
    MS062,
    /// Lawrence, MS
    #[doc(hidden)]
    MS063,
    /// Jefferson Davis, MS
    #[doc(hidden)]
    MS064,
    /// Covington, MS
    #[doc(hidden)]
    MS065,
    /// Jones, MS
    #[doc(hidden)]
    MS066,
    /// Wayne, MS
    #[doc(hidden)]
    MS067,
    /// Wilkinson, MS
    #[doc(hidden)]
    MS068,
    /// Amite, MS
    #[doc(hidden)]
    MS069,
    /// Pike, MS
    #[doc(hidden)]
    MS070,
    /// Walthall, MS
    #[doc(hidden)]
    MS071,
    /// Marion, MS
    #[doc(hidden)]
    MS072,
    /// Lamar, MS
    #[doc(hidden)]
    MS073,
    /// Forrest, MS
    #[doc(hidden)]
    MS074,
    /// Perry, MS
    #[doc(hidden)]
    MS075,
    /// Greene, MS
    #[doc(hidden)]
    MS076,
    /// Pearl River, MS
    #[doc(hidden)]
    MS077,
    /// Stone, MS
    #[doc(hidden)]
    MS078,
    /// George, MS
    #[doc(hidden)]
    MS079,
    /// Hancock, MS
    #[doc(hidden)]
    MS080,
    /// Harrison, MS
    #[doc(hidden)]
    MS081,
    /// Jackson, MS
    #[doc(hidden)]
    MS082,
    /// Kootenai/Cabinet Region, MT
    #[doc(hidden)]
    MT001,
    /// West Glacier Region, MT
    #[doc(hidden)]
    MT002,
    /// Flathead/Mission Valleys, MT
    #[doc(hidden)]
    MT003,
    /// Lower Clark Fork Region, MT
    #[doc(hidden)]
    MT004,
    /// Missoula/Bitterroot Valleys, MT
    #[doc(hidden)]
    MT005,
    /// Bitterroot/Sapphire Mountains, MT
    #[doc(hidden)]
    MT006,
    /// Butte/Blackfoot Region, MT
    #[doc(hidden)]
    MT007,
    /// Beaverhead, MT
    #[doc(hidden)]
    MT008,
    /// Northern Rocky Mountain Front, MT
    #[doc(hidden)]
    MT009,
    /// Eastern Glacier, MT
    #[doc(hidden)]
    MT010,
    /// Hill, MT
    #[doc(hidden)]
    MT011,
    /// Cascade, MT
    #[doc(hidden)]
    MT012,
    /// Chouteau, MT
    #[doc(hidden)]
    MT013,
    /// Central and Southern Lewis and Clark, MT
    #[doc(hidden)]
    MT014,
    /// Madison, MT
    #[doc(hidden)]
    MT015,
    /// Central and Southeast Phillips, MT
    #[doc(hidden)]
    MT016,
    /// Central and Southern Valley, MT
    #[doc(hidden)]
    MT017,
    /// Daniels, MT
    #[doc(hidden)]
    MT018,
    /// Sheridan, MT
    #[doc(hidden)]
    MT019,
    /// Western Roosevelt, MT
    #[doc(hidden)]
    MT020,
    /// Petroleum, MT
    #[doc(hidden)]
    MT021,
    /// Garfield, MT
    #[doc(hidden)]
    MT022,
    /// McCone, MT
    #[doc(hidden)]
    MT023,
    /// Richland, MT
    #[doc(hidden)]
    MT024,
    /// Dawson, MT
    #[doc(hidden)]
    MT025,
    /// Prairie, MT
    #[doc(hidden)]
    MT026,
    /// Wibaux, MT
    #[doc(hidden)]
    MT027,
    /// Musselshell, MT
    #[doc(hidden)]
    MT029,
    /// Treasure, MT
    #[doc(hidden)]
    MT030,
    /// Northern Rosebud, MT
    #[doc(hidden)]
    MT031,
    /// Custer, MT
    #[doc(hidden)]
    MT032,
    /// Fallon, MT
    #[doc(hidden)]
    MT033,
    /// Northern Stillwater, MT
    #[doc(hidden)]
    MT034,
    /// Powder River, MT
    #[doc(hidden)]
    MT036,
    /// Carter, MT
    #[doc(hidden)]
    MT037,
    /// Northern Park, MT
    #[doc(hidden)]
    MT040,
    /// Golden Valley, MT
    #[doc(hidden)]
    MT042,
    /// Potomac/Seeley Lake Region, MT
    #[doc(hidden)]
    MT043,
    /// Toole, MT
    #[doc(hidden)]
    MT044,
    /// Liberty, MT
    #[doc(hidden)]
    MT045,
    /// Eastern Pondera, MT
    #[doc(hidden)]
    MT046,
    /// Blaine, MT
    #[doc(hidden)]
    MT047,
    /// Southern Rocky Mountain Front, MT
    #[doc(hidden)]
    MT048,
    /// Eastern Teton, MT
    #[doc(hidden)]
    MT049,
    /// Judith Basin, MT
    #[doc(hidden)]
    MT050,
    /// Fergus, MT
    #[doc(hidden)]
    MT051,
    /// Jefferson, MT
    #[doc(hidden)]
    MT052,
    /// Broadwater, MT
    #[doc(hidden)]
    MT053,
    /// Meagher, MT
    #[doc(hidden)]
    MT054,
    /// Gallatin, MT
    #[doc(hidden)]
    MT055,
    /// Red Lodge Foothills, MT
    #[doc(hidden)]
    MT056,
    /// Northern Big Horn, MT
    #[doc(hidden)]
    MT057,
    /// Southern Rosebud, MT
    #[doc(hidden)]
    MT058,
    /// Northern Phillips, MT
    #[doc(hidden)]
    MT059,
    /// Southwest Phillips, MT
    #[doc(hidden)]
    MT060,
    /// Northern Valley, MT
    #[doc(hidden)]
    MT061,
    /// Eastern Roosevelt, MT
    #[doc(hidden)]
    MT062,
    /// Judith Gap, MT
    #[doc(hidden)]
    MT063,
    /// Paradise Valley, MT
    #[doc(hidden)]
    MT064,
    /// Livingston Area, MT
    #[doc(hidden)]
    MT065,
    /// Beartooth Foothills, MT
    #[doc(hidden)]
    MT066,
    /// Absaroka/Beartooth Mountains, MT
    #[doc(hidden)]
    MT067,
    /// Crazy Mountains, MT
    #[doc(hidden)]
    MT068,
    /// Southern Big Horn, MT
    #[doc(hidden)]
    MT138,
    /// Southeastern Carbon, MT
    #[doc(hidden)]
    MT139,
    /// Northern Sweet Grass, MT
    #[doc(hidden)]
    MT141,
    /// Bighorn Canyon, MT
    #[doc(hidden)]
    MT169,
    /// Northern Carbon, MT
    #[doc(hidden)]
    MT170,
    /// Pryor/Northern Bighorn Mountains, MT
    #[doc(hidden)]
    MT171,
    /// Melville Foothills, MT
    #[doc(hidden)]
    MT172,
    /// Northeastern Yellowstone, MT
    #[doc(hidden)]
    MT173,
    /// Southern Wheatland, MT
    #[doc(hidden)]
    MT228,
    /// Southwestern Yellowstone, MT
    #[doc(hidden)]
    MT235,
    /// Ashe, NC
    #[doc(hidden)]
    NC001,
    /// Alleghany, NC
    #[doc(hidden)]
    NC002,
    /// Surry, NC
    #[doc(hidden)]
    NC003,
    /// Stokes, NC
    #[doc(hidden)]
    NC004,
    /// Rockingham, NC
    #[doc(hidden)]
    NC005,
    /// Caswell, NC
    #[doc(hidden)]
    NC006,
    /// Person, NC
    #[doc(hidden)]
    NC007,
    /// Granville, NC
    #[doc(hidden)]
    NC008,
    /// Vance, NC
    #[doc(hidden)]
    NC009,
    /// Warren, NC
    #[doc(hidden)]
    NC010,
    /// Halifax, NC
    #[doc(hidden)]
    NC011,
    /// Northampton, NC
    #[doc(hidden)]
    NC012,
    /// Hertford, NC
    #[doc(hidden)]
    NC013,
    /// Gates, NC
    #[doc(hidden)]
    NC014,
    /// Pasquotank, NC
    #[doc(hidden)]
    NC015,
    /// Camden, NC
    #[doc(hidden)]
    NC016,
    /// Western Currituck, NC
    #[doc(hidden)]
    NC017,
    /// Watauga, NC
    #[doc(hidden)]
    NC018,
    /// Wilkes, NC
    #[doc(hidden)]
    NC019,
    /// Yadkin, NC
    #[doc(hidden)]
    NC020,
    /// Forsyth, NC
    #[doc(hidden)]
    NC021,
    /// Guilford, NC
    #[doc(hidden)]
    NC022,
    /// Alamance, NC
    #[doc(hidden)]
    NC023,
    /// Orange, NC
    #[doc(hidden)]
    NC024,
    /// Durham, NC
    #[doc(hidden)]
    NC025,
    /// Franklin, NC
    #[doc(hidden)]
    NC026,
    /// Nash, NC
    #[doc(hidden)]
    NC027,
    /// Edgecombe, NC
    #[doc(hidden)]
    NC028,
    /// Martin, NC
    #[doc(hidden)]
    NC029,
    /// Bertie, NC
    #[doc(hidden)]
    NC030,
    /// Chowan, NC
    #[doc(hidden)]
    NC031,
    /// Perquimans, NC
    #[doc(hidden)]
    NC032,
    /// Avery, NC
    #[doc(hidden)]
    NC033,
    /// Alexander, NC
    #[doc(hidden)]
    NC035,
    /// Iredell, NC
    #[doc(hidden)]
    NC036,
    /// Davie, NC
    #[doc(hidden)]
    NC037,
    /// Davidson, NC
    #[doc(hidden)]
    NC038,
    /// Randolph, NC
    #[doc(hidden)]
    NC039,
    /// Chatham, NC
    #[doc(hidden)]
    NC040,
    /// Wake, NC
    #[doc(hidden)]
    NC041,
    /// Johnston, NC
    #[doc(hidden)]
    NC042,
    /// Wilson, NC
    #[doc(hidden)]
    NC043,
    /// Pitt, NC
    #[doc(hidden)]
    NC044,
    /// Washington, NC
    #[doc(hidden)]
    NC045,
    /// Tyrrell, NC
    #[doc(hidden)]
    NC046,
    /// Mainland Dare, NC
    #[doc(hidden)]
    NC047,
    /// Madison, NC
    #[doc(hidden)]
    NC048,
    /// Yancey, NC
    #[doc(hidden)]
    NC049,
    /// Mitchell, NC
    #[doc(hidden)]
    NC050,
    /// Swain, NC
    #[doc(hidden)]
    NC051,
    /// Haywood, NC
    #[doc(hidden)]
    NC052,
    /// Buncombe, NC
    #[doc(hidden)]
    NC053,
    /// Catawba, NC
    #[doc(hidden)]
    NC056,
    /// Rowan, NC
    #[doc(hidden)]
    NC057,
    /// Graham, NC
    #[doc(hidden)]
    NC058,
    /// Northern Jackson, NC
    #[doc(hidden)]
    NC059,
    /// Cherokee, NC
    #[doc(hidden)]
    NC060,
    /// Clay, NC
    #[doc(hidden)]
    NC061,
    /// Macon, NC
    #[doc(hidden)]
    NC062,
    /// Southern Jackson, NC
    #[doc(hidden)]
    NC063,
    /// Transylvania, NC
    #[doc(hidden)]
    NC064,
    /// Henderson, NC
    #[doc(hidden)]
    NC065,
    /// Cleveland, NC
    #[doc(hidden)]
    NC068,
    /// Lincoln, NC
    #[doc(hidden)]
    NC069,
    /// Gaston, NC
    #[doc(hidden)]
    NC070,
    /// Mecklenburg, NC
    #[doc(hidden)]
    NC071,
    /// Cabarrus, NC
    #[doc(hidden)]
    NC072,
    /// Stanly, NC
    #[doc(hidden)]
    NC073,
    /// Montgomery, NC
    #[doc(hidden)]
    NC074,
    /// Moore, NC
    #[doc(hidden)]
    NC075,
    /// Lee, NC
    #[doc(hidden)]
    NC076,
    /// Harnett, NC
    #[doc(hidden)]
    NC077,
    /// Wayne, NC
    #[doc(hidden)]
    NC078,
    /// Greene, NC
    #[doc(hidden)]
    NC079,
    /// Beaufort, NC
    #[doc(hidden)]
    NC080,
    /// Mainland Hyde, NC
    #[doc(hidden)]
    NC081,
    /// Union, NC
    #[doc(hidden)]
    NC082,
    /// Anson, NC
    #[doc(hidden)]
    NC083,
    /// Richmond, NC
    #[doc(hidden)]
    NC084,
    /// Scotland, NC
    #[doc(hidden)]
    NC085,
    /// Hoke, NC
    #[doc(hidden)]
    NC086,
    /// Robeson, NC
    #[doc(hidden)]
    NC087,
    /// Cumberland, NC
    #[doc(hidden)]
    NC088,
    /// Sampson, NC
    #[doc(hidden)]
    NC089,
    /// Duplin, NC
    #[doc(hidden)]
    NC090,
    /// Lenoir, NC
    #[doc(hidden)]
    NC091,
    /// Jones, NC
    #[doc(hidden)]
    NC092,
    /// Pamlico, NC
    #[doc(hidden)]
    NC094,
    /// Bladen, NC
    #[doc(hidden)]
    NC096,
    /// Columbus, NC
    #[doc(hidden)]
    NC099,
    /// Eastern Currituck, NC
    #[doc(hidden)]
    NC102,
    /// Inland Pender, NC
    #[doc(hidden)]
    NC105,
    /// Coastal Pender, NC
    #[doc(hidden)]
    NC106,
    /// Inland New Hanover, NC
    #[doc(hidden)]
    NC107,
    /// Coastal New Hanover, NC
    #[doc(hidden)]
    NC108,
    /// Inland Brunswick, NC
    #[doc(hidden)]
    NC109,
    /// Coastal Brunswick, NC
    #[doc(hidden)]
    NC110,
    /// Northern Craven, NC
    #[doc(hidden)]
    NC193,
    /// Southern Craven, NC
    #[doc(hidden)]
    NC194,
    /// West Carteret, NC
    #[doc(hidden)]
    NC195,
    /// East Carteret, NC
    #[doc(hidden)]
    NC196,
    /// Inland Onslow, NC
    #[doc(hidden)]
    NC198,
    /// Coastal Onslow, NC
    #[doc(hidden)]
    NC199,
    /// Northern Outer Banks, NC
    #[doc(hidden)]
    NC203,
    /// Ocracoke Island, NC
    #[doc(hidden)]
    NC204,
    /// Hatteras Island, NC
    #[doc(hidden)]
    NC205,
    /// Caldwell Mountains, NC
    #[doc(hidden)]
    NC501,
    /// Greater Caldwell, NC
    #[doc(hidden)]
    NC502,
    /// Burke Mountains, NC
    #[doc(hidden)]
    NC503,
    /// Greater Burke, NC
    #[doc(hidden)]
    NC504,
    /// McDowell Mountains, NC
    #[doc(hidden)]
    NC505,
    /// Eastern McDowell, NC
    #[doc(hidden)]
    NC506,
    /// Rutherford Mountains, NC
    #[doc(hidden)]
    NC507,
    /// Greater Rutherford, NC
    #[doc(hidden)]
    NC508,
    /// Polk Mountains, NC
    #[doc(hidden)]
    NC509,
    /// Eastern Polk, NC
    #[doc(hidden)]
    NC510,
    /// Divide, ND
    #[doc(hidden)]
    ND001,
    /// Burke, ND
    #[doc(hidden)]
    ND002,
    /// Renville, ND
    #[doc(hidden)]
    ND003,
    /// Bottineau, ND
    #[doc(hidden)]
    ND004,
    /// Rolette, ND
    #[doc(hidden)]
    ND005,
    /// Towner, ND
    #[doc(hidden)]
    ND006,
    /// Cavalier, ND
    #[doc(hidden)]
    ND007,
    /// Pembina, ND
    #[doc(hidden)]
    ND008,
    /// Williams, ND
    #[doc(hidden)]
    ND009,
    /// Mountrail, ND
    #[doc(hidden)]
    ND010,
    /// Ward, ND
    #[doc(hidden)]
    ND011,
    /// McHenry, ND
    #[doc(hidden)]
    ND012,
    /// Pierce, ND
    #[doc(hidden)]
    ND013,
    /// Benson, ND
    #[doc(hidden)]
    ND014,
    /// Ramsey, ND
    #[doc(hidden)]
    ND015,
    /// Eastern Walsh County, ND
    #[doc(hidden)]
    ND016,
    /// McKenzie, ND
    #[doc(hidden)]
    ND017,
    /// Dunn, ND
    #[doc(hidden)]
    ND018,
    /// Mercer, ND
    #[doc(hidden)]
    ND019,
    /// Oliver, ND
    #[doc(hidden)]
    ND020,
    /// McLean, ND
    #[doc(hidden)]
    ND021,
    /// Sheridan, ND
    #[doc(hidden)]
    ND022,
    /// Wells, ND
    #[doc(hidden)]
    ND023,
    /// Eddy, ND
    #[doc(hidden)]
    ND024,
    /// Foster, ND
    #[doc(hidden)]
    ND025,
    /// Nelson, ND
    #[doc(hidden)]
    ND026,
    /// Grand Forks, ND
    #[doc(hidden)]
    ND027,
    /// Griggs, ND
    #[doc(hidden)]
    ND028,
    /// Steele, ND
    #[doc(hidden)]
    ND029,
    /// Traill, ND
    #[doc(hidden)]
    ND030,
    /// Golden Valley, ND
    #[doc(hidden)]
    ND031,
    /// Billings, ND
    #[doc(hidden)]
    ND032,
    /// Stark, ND
    #[doc(hidden)]
    ND033,
    /// Morton, ND
    #[doc(hidden)]
    ND034,
    /// Burleigh, ND
    #[doc(hidden)]
    ND035,
    /// Kidder, ND
    #[doc(hidden)]
    ND036,
    /// Stutsman, ND
    #[doc(hidden)]
    ND037,
    /// Barnes, ND
    #[doc(hidden)]
    ND038,
    /// Cass, ND
    #[doc(hidden)]
    ND039,
    /// Slope, ND
    #[doc(hidden)]
    ND040,
    /// Hettinger, ND
    #[doc(hidden)]
    ND041,
    /// Grant, ND
    #[doc(hidden)]
    ND042,
    /// Bowman, ND
    #[doc(hidden)]
    ND043,
    /// Adams, ND
    #[doc(hidden)]
    ND044,
    /// Sioux, ND
    #[doc(hidden)]
    ND045,
    /// Emmons, ND
    #[doc(hidden)]
    ND046,
    /// Logan, ND
    #[doc(hidden)]
    ND047,
    /// La Moure, ND
    #[doc(hidden)]
    ND048,
    /// Ransom, ND
    #[doc(hidden)]
    ND049,
    /// McIntosh, ND
    #[doc(hidden)]
    ND050,
    /// Dickey, ND
    #[doc(hidden)]
    ND051,
    /// Sargent, ND
    #[doc(hidden)]
    ND052,
    /// Richland, ND
    #[doc(hidden)]
    ND053,
    /// Western Walsh County, ND
    #[doc(hidden)]
    ND054,
    /// Dawes, NE
    #[doc(hidden)]
    NE002,
    /// Box Butte, NE
    #[doc(hidden)]
    NE003,
    /// Sheridan, NE
    #[doc(hidden)]
    NE004,
    /// Eastern Cherry, NE
    #[doc(hidden)]
    NE005,
    /// Keya Paha, NE
    #[doc(hidden)]
    NE006,
    /// Boyd, NE
    #[doc(hidden)]
    NE007,
    /// Brown, NE
    #[doc(hidden)]
    NE008,
    /// Rock, NE
    #[doc(hidden)]
    NE009,
    /// Holt, NE
    #[doc(hidden)]
    NE010,
    /// Knox, NE
    #[doc(hidden)]
    NE011,
    /// Cedar, NE
    #[doc(hidden)]
    NE012,
    /// Dixon, NE
    #[doc(hidden)]
    NE013,
    /// Dakota, NE
    #[doc(hidden)]
    NE014,
    /// Thurston, NE
    #[doc(hidden)]
    NE015,
    /// Antelope, NE
    #[doc(hidden)]
    NE016,
    /// Pierce, NE
    #[doc(hidden)]
    NE017,
    /// Wayne, NE
    #[doc(hidden)]
    NE018,
    /// Scotts Bluff, NE
    #[doc(hidden)]
    NE019,
    /// Banner, NE
    #[doc(hidden)]
    NE020,
    /// Morrill, NE
    #[doc(hidden)]
    NE021,
    /// Garden, NE
    #[doc(hidden)]
    NE022,
    /// Grant, NE
    #[doc(hidden)]
    NE023,
    /// Hooker, NE
    #[doc(hidden)]
    NE024,
    /// Thomas, NE
    #[doc(hidden)]
    NE025,
    /// Blaine, NE
    #[doc(hidden)]
    NE026,
    /// Loup, NE
    #[doc(hidden)]
    NE027,
    /// Garfield, NE
    #[doc(hidden)]
    NE028,
    /// Wheeler, NE
    #[doc(hidden)]
    NE029,
    /// Boone, NE
    #[doc(hidden)]
    NE030,
    /// Madison, NE
    #[doc(hidden)]
    NE031,
    /// Stanton, NE
    #[doc(hidden)]
    NE032,
    /// Cuming, NE
    #[doc(hidden)]
    NE033,
    /// Burt, NE
    #[doc(hidden)]
    NE034,
    /// Arthur, NE
    #[doc(hidden)]
    NE035,
    /// McPherson, NE
    #[doc(hidden)]
    NE036,
    /// Logan, NE
    #[doc(hidden)]
    NE037,
    /// Custer, NE
    #[doc(hidden)]
    NE038,
    /// Valley, NE
    #[doc(hidden)]
    NE039,
    /// Greeley, NE
    #[doc(hidden)]
    NE040,
    /// Nance, NE
    #[doc(hidden)]
    NE041,
    /// Platte, NE
    #[doc(hidden)]
    NE042,
    /// Colfax, NE
    #[doc(hidden)]
    NE043,
    /// Dodge, NE
    #[doc(hidden)]
    NE044,
    /// Washington, NE
    #[doc(hidden)]
    NE045,
    /// Sherman, NE
    #[doc(hidden)]
    NE046,
    /// Howard, NE
    #[doc(hidden)]
    NE047,
    /// Merrick, NE
    #[doc(hidden)]
    NE048,
    /// Polk, NE
    #[doc(hidden)]
    NE049,
    /// Butler, NE
    #[doc(hidden)]
    NE050,
    /// Saunders, NE
    #[doc(hidden)]
    NE051,
    /// Douglas, NE
    #[doc(hidden)]
    NE052,
    /// Sarpy, NE
    #[doc(hidden)]
    NE053,
    /// Kimball, NE
    #[doc(hidden)]
    NE054,
    /// Cheyenne, NE
    #[doc(hidden)]
    NE055,
    /// Deuel, NE
    #[doc(hidden)]
    NE056,
    /// Keith, NE
    #[doc(hidden)]
    NE057,
    /// Perkins, NE
    #[doc(hidden)]
    NE058,
    /// Lincoln, NE
    #[doc(hidden)]
    NE059,
    /// Dawson, NE
    #[doc(hidden)]
    NE060,
    /// Buffalo, NE
    #[doc(hidden)]
    NE061,
    /// Hall, NE
    #[doc(hidden)]
    NE062,
    /// Hamilton, NE
    #[doc(hidden)]
    NE063,
    /// York, NE
    #[doc(hidden)]
    NE064,
    /// Seward, NE
    #[doc(hidden)]
    NE065,
    /// Lancaster, NE
    #[doc(hidden)]
    NE066,
    /// Cass, NE
    #[doc(hidden)]
    NE067,
    /// Otoe, NE
    #[doc(hidden)]
    NE068,
    /// Chase, NE
    #[doc(hidden)]
    NE069,
    /// Hayes, NE
    #[doc(hidden)]
    NE070,
    /// Frontier, NE
    #[doc(hidden)]
    NE071,
    /// Gosper, NE
    #[doc(hidden)]
    NE072,
    /// Phelps, NE
    #[doc(hidden)]
    NE073,
    /// Kearney, NE
    #[doc(hidden)]
    NE074,
    /// Adams, NE
    #[doc(hidden)]
    NE075,
    /// Clay, NE
    #[doc(hidden)]
    NE076,
    /// Fillmore, NE
    #[doc(hidden)]
    NE077,
    /// Saline, NE
    #[doc(hidden)]
    NE078,
    /// Dundy, NE
    #[doc(hidden)]
    NE079,
    /// Hitchcock, NE
    #[doc(hidden)]
    NE080,
    /// Red Willow, NE
    #[doc(hidden)]
    NE081,
    /// Furnas, NE
    #[doc(hidden)]
    NE082,
    /// Harlan, NE
    #[doc(hidden)]
    NE083,
    /// Franklin, NE
    #[doc(hidden)]
    NE084,
    /// Webster, NE
    #[doc(hidden)]
    NE085,
    /// Nuckolls, NE
    #[doc(hidden)]
    NE086,
    /// Thayer, NE
    #[doc(hidden)]
    NE087,
    /// Jefferson, NE
    #[doc(hidden)]
    NE088,
    /// Gage, NE
    #[doc(hidden)]
    NE089,
    /// Johnson, NE
    #[doc(hidden)]
    NE090,
    /// Nemaha, NE
    #[doc(hidden)]
    NE091,
    /// Pawnee, NE
    #[doc(hidden)]
    NE092,
    /// Richardson, NE
    #[doc(hidden)]
    NE093,
    /// Western Cherry, NE
    #[doc(hidden)]
    NE094,
    /// North Sioux, NE
    #[doc(hidden)]
    NE095,
    /// South Sioux, NE
    #[doc(hidden)]
    NE096,
    /// Northern Coos, NH
    #[doc(hidden)]
    NH001,
    /// Southern Coos, NH
    #[doc(hidden)]
    NH002,
    /// Northern Grafton, NH
    #[doc(hidden)]
    NH003,
    /// Northern Carroll, NH
    #[doc(hidden)]
    NH004,
    /// Southern Grafton, NH
    #[doc(hidden)]
    NH005,
    /// Southern Carroll, NH
    #[doc(hidden)]
    NH006,
    /// Sullivan, NH
    #[doc(hidden)]
    NH007,
    /// Merrimack, NH
    #[doc(hidden)]
    NH008,
    /// Belknap, NH
    #[doc(hidden)]
    NH009,
    /// Strafford, NH
    #[doc(hidden)]
    NH010,
    /// Cheshire, NH
    #[doc(hidden)]
    NH011,
    /// Eastern Hillsborough, NH
    #[doc(hidden)]
    NH012,
    /// Interior Rockingham, NH
    #[doc(hidden)]
    NH013,
    /// Coastal Rockingham, NH
    #[doc(hidden)]
    NH014,
    /// Western And Central Hillsborough, NH
    #[doc(hidden)]
    NH015,
    /// Sussex, NJ
    #[doc(hidden)]
    NJ001,
    /// Western Passaic, NJ
    #[doc(hidden)]
    NJ002,
    /// Eastern Passaic, NJ
    #[doc(hidden)]
    NJ004,
    /// Hudson, NJ
    #[doc(hidden)]
    NJ006,
    /// Warren, NJ
    #[doc(hidden)]
    NJ007,
    /// Morris, NJ
    #[doc(hidden)]
    NJ008,
    /// Hunterdon, NJ
    #[doc(hidden)]
    NJ009,
    /// Somerset, NJ
    #[doc(hidden)]
    NJ010,
    /// Middlesex, NJ
    #[doc(hidden)]
    NJ012,
    /// Western Monmouth, NJ
    #[doc(hidden)]
    NJ013,
    /// Eastern Monmouth, NJ
    #[doc(hidden)]
    NJ014,
    /// Mercer, NJ
    #[doc(hidden)]
    NJ015,
    /// Salem, NJ
    #[doc(hidden)]
    NJ016,
    /// Gloucester, NJ
    #[doc(hidden)]
    NJ017,
    /// Camden, NJ
    #[doc(hidden)]
    NJ018,
    /// Northwestern Burlington, NJ
    #[doc(hidden)]
    NJ019,
    /// Ocean, NJ
    #[doc(hidden)]
    NJ020,
    /// Cumberland, NJ
    #[doc(hidden)]
    NJ021,
    /// Atlantic, NJ
    #[doc(hidden)]
    NJ022,
    /// Cape May, NJ
    #[doc(hidden)]
    NJ023,
    /// Atlantic Coastal Cape May, NJ
    #[doc(hidden)]
    NJ024,
    /// Coastal Atlantic, NJ
    #[doc(hidden)]
    NJ025,
    /// Coastal Ocean, NJ
    #[doc(hidden)]
    NJ026,
    /// Southeastern Burlington, NJ
    #[doc(hidden)]
    NJ027,
    /// Western Bergen, NJ
    #[doc(hidden)]
    NJ103,
    /// Eastern Bergen, NJ
    #[doc(hidden)]
    NJ104,
    /// Western Essex, NJ
    #[doc(hidden)]
    NJ105,
    /// Eastern Essex, NJ
    #[doc(hidden)]
    NJ106,
    /// Western Union, NJ
    #[doc(hidden)]
    NJ107,
    /// Eastern Union, NJ
    #[doc(hidden)]
    NJ108,
    /// Guadalupe Mountains of Eddy County, NM
    #[doc(hidden)]
    NM027,
    /// Eddy County Plains, NM
    #[doc(hidden)]
    NM028,
    /// Northern Lea County, NM
    #[doc(hidden)]
    NM029,
    /// Central Lea County, NM
    #[doc(hidden)]
    NM033,
    /// Southern Lea County, NM
    #[doc(hidden)]
    NM034,
    /// Northwest Plateau, NM
    #[doc(hidden)]
    NM201,
    /// Chuska Mountains, NM
    #[doc(hidden)]
    NM202,
    /// Far Northwest Highlands, NM
    #[doc(hidden)]
    NM203,
    /// Northwest Highlands, NM
    #[doc(hidden)]
    NM204,
    /// West Central Plateau, NM
    #[doc(hidden)]
    NM205,
    /// West Central Mountains, NM
    #[doc(hidden)]
    NM206,
    /// West Central Highlands, NM
    #[doc(hidden)]
    NM207,
    /// Southwest Mountains, NM
    #[doc(hidden)]
    NM208,
    /// San Francisco River Valley, NM
    #[doc(hidden)]
    NM209,
    /// Tusas Mountains Including Chama, NM
    #[doc(hidden)]
    NM210,
    /// Jemez Mountains, NM
    #[doc(hidden)]
    NM211,
    /// Glorieta Mesa Including Glorieta Pass, NM
    #[doc(hidden)]
    NM212,
    /// Northern Sangre de Cristo Mountains, NM
    #[doc(hidden)]
    NM213,
    /// Southern Sangre de Cristo Mountains, NM
    #[doc(hidden)]
    NM214,
    /// East Slopes Sangre de Cristo Mountains, NM
    #[doc(hidden)]
    NM215,
    /// Upper Rio Grande Valley, NM
    #[doc(hidden)]
    NM216,
    /// Espanola Valley, NM
    #[doc(hidden)]
    NM217,
    /// Santa Fe Metro Area, NM
    #[doc(hidden)]
    NM218,
    /// Middle Rio Grande Valley/Albuquerque Metro Area, NM
    #[doc(hidden)]
    NM219,
    /// Lower Rio Grande Valley, NM
    #[doc(hidden)]
    NM220,
    /// Sandia/Manzano Mountains Including Edgewood, NM
    #[doc(hidden)]
    NM221,
    /// Estancia Valley, NM
    #[doc(hidden)]
    NM222,
    /// Central Highlands, NM
    #[doc(hidden)]
    NM223,
    /// South Central Highlands, NM
    #[doc(hidden)]
    NM224,
    /// Upper Tularosa Valley, NM
    #[doc(hidden)]
    NM225,
    /// South Central Mountains, NM
    #[doc(hidden)]
    NM226,
    /// Johnson and Bartlett Mesas Including Raton Pass, NM
    #[doc(hidden)]
    NM227,
    /// Far Northeast Highlands, NM
    #[doc(hidden)]
    NM228,
    /// Northeast Highlands, NM
    #[doc(hidden)]
    NM229,
    /// Union County, NM
    #[doc(hidden)]
    NM230,
    /// Harding County, NM
    #[doc(hidden)]
    NM231,
    /// Eastern San Miguel County, NM
    #[doc(hidden)]
    NM232,
    /// Guadalupe County, NM
    #[doc(hidden)]
    NM233,
    /// Quay County, NM
    #[doc(hidden)]
    NM234,
    /// Curry County, NM
    #[doc(hidden)]
    NM235,
    /// Roosevelt County, NM
    #[doc(hidden)]
    NM236,
    /// De Baca County, NM
    #[doc(hidden)]
    NM237,
    /// Chaves County Plains, NM
    #[doc(hidden)]
    NM238,
    /// Eastern Lincoln County, NM
    #[doc(hidden)]
    NM239,
    /// Southwest Chaves County, NM
    #[doc(hidden)]
    NM240,
    /// San Agustin Plains and Adjacent Lowlands, NM
    #[doc(hidden)]
    NM241,
    /// Upper Gila River Valley, NM
    #[doc(hidden)]
    NM401,
    /// Southern Gila Highlands/Black Range, NM
    #[doc(hidden)]
    NM402,
    /// Southern Gila Foothills/Mimbres Valley, NM
    #[doc(hidden)]
    NM403,
    /// Southwest Desert/Lower Gila River Valley, NM
    #[doc(hidden)]
    NM404,
    /// Lowlands of the Bootheel, NM
    #[doc(hidden)]
    NM405,
    /// Uplands of the Bootheel, NM
    #[doc(hidden)]
    NM406,
    /// Southwest Desert/Mimbres Basin, NM
    #[doc(hidden)]
    NM407,
    /// Eastern Black Range Foothills, NM
    #[doc(hidden)]
    NM408,
    /// Sierra County Lakes, NM
    #[doc(hidden)]
    NM409,
    /// Northern Dona Ana County, NM
    #[doc(hidden)]
    NM410,
    /// Southern Dona Ana County/Mesilla Valley, NM
    #[doc(hidden)]
    NM411,
    /// Central Tularosa Basin, NM
    #[doc(hidden)]
    NM412,
    /// Southern Tularosa Basin, NM
    #[doc(hidden)]
    NM413,
    /// West Slopes Sacramento Mountains Below 7500 Feet, NM
    #[doc(hidden)]
    NM414,
    /// Sacramento Mountains Above 7500 Feet, NM
    #[doc(hidden)]
    NM415,
    /// East Slopes Sacramento Mountains Below 7500 Feet, NM
    #[doc(hidden)]
    NM416,
    /// Otero Mesa, NM
    #[doc(hidden)]
    NM417,
    /// Mineral and Southern Lyon Counties, NV
    #[doc(hidden)]
    NV001,
    /// Greater Lake Tahoe Area, NV
    #[doc(hidden)]
    NV002,
    /// Greater Reno-Carson City-Minden Area, NV
    #[doc(hidden)]
    NV003,
    /// Western Nevada Basin and Range including Pyramid Lake, NV
    #[doc(hidden)]
    NV004,
    /// Northern Washoe County, NV
    #[doc(hidden)]
    NV005,
    /// Esmeralda and Central Nye County, NV
    #[doc(hidden)]
    NV014,
    /// Lincoln County, NV
    #[doc(hidden)]
    NV015,
    /// Northeast Clark County, NV
    #[doc(hidden)]
    NV016,
    /// Western Clark and Southern Nye County, NV
    #[doc(hidden)]
    NV017,
    /// Sheep Range, NV
    #[doc(hidden)]
    NV018,
    /// Spring Mountains-Red Rock Canyon, NV
    #[doc(hidden)]
    NV019,
    /// Las Vegas Valley, NV
    #[doc(hidden)]
    NV020,
    /// Lake Mead National Recreation Area, NV
    #[doc(hidden)]
    NV021,
    /// Southern Clark County, NV
    #[doc(hidden)]
    NV022,
    /// Humboldt County, NV
    #[doc(hidden)]
    NV030,
    /// Northern Elko County, NV
    #[doc(hidden)]
    NV031,
    /// Southeastern Elko County, NV
    #[doc(hidden)]
    NV033,
    /// Ruby Mountains and East Humboldt Range, NV
    #[doc(hidden)]
    NV034,
    /// White Pine County, NV
    #[doc(hidden)]
    NV035,
    /// Northern Lander County and Northern Eureka County, NV
    #[doc(hidden)]
    NV036,
    /// Southern Lander County and Southern Eureka County, NV
    #[doc(hidden)]
    NV037,
    /// Southwest Elko County, NV
    #[doc(hidden)]
    NV038,
    /// South Central Elko County, NV
    #[doc(hidden)]
    NV039,
    /// Northwestern Nye County, NV
    #[doc(hidden)]
    NV040,
    /// Northeastern Nye County, NV
    #[doc(hidden)]
    NV041,
    /// Niagara, NY
    #[doc(hidden)]
    NY001,
    /// Orleans, NY
    #[doc(hidden)]
    NY002,
    /// Monroe, NY
    #[doc(hidden)]
    NY003,
    /// Wayne, NY
    #[doc(hidden)]
    NY004,
    /// Northern Cayuga, NY
    #[doc(hidden)]
    NY005,
    /// Oswego, NY
    #[doc(hidden)]
    NY006,
    /// Jefferson, NY
    #[doc(hidden)]
    NY007,
    /// Lewis, NY
    #[doc(hidden)]
    NY008,
    /// Northern Oneida, NY
    #[doc(hidden)]
    NY009,
    /// Northern Erie, NY
    #[doc(hidden)]
    NY010,
    /// Genesee, NY
    #[doc(hidden)]
    NY011,
    /// Wyoming, NY
    #[doc(hidden)]
    NY012,
    /// Livingston, NY
    #[doc(hidden)]
    NY013,
    /// Ontario, NY
    #[doc(hidden)]
    NY014,
    /// Yates, NY
    #[doc(hidden)]
    NY015,
    /// Seneca, NY
    #[doc(hidden)]
    NY016,
    /// Southern Cayuga, NY
    #[doc(hidden)]
    NY017,
    /// Onondaga, NY
    #[doc(hidden)]
    NY018,
    /// Chautauqua, NY
    #[doc(hidden)]
    NY019,
    /// Cattaraugus, NY
    #[doc(hidden)]
    NY020,
    /// Allegany, NY
    #[doc(hidden)]
    NY021,
    /// Steuben, NY
    #[doc(hidden)]
    NY022,
    /// Schuyler, NY
    #[doc(hidden)]
    NY023,
    /// Chemung, NY
    #[doc(hidden)]
    NY024,
    /// Tompkins, NY
    #[doc(hidden)]
    NY025,
    /// Northern St. Lawrence, NY
    #[doc(hidden)]
    NY026,
    /// Northern Franklin, NY
    #[doc(hidden)]
    NY027,
    /// Eastern Clinton, NY
    #[doc(hidden)]
    NY028,
    /// Southeastern St. Lawrence, NY
    #[doc(hidden)]
    NY029,
    /// Southern Franklin, NY
    #[doc(hidden)]
    NY030,
    /// Western Clinton, NY
    #[doc(hidden)]
    NY031,
    /// Northern Herkimer, NY
    #[doc(hidden)]
    NY032,
    /// Hamilton, NY
    #[doc(hidden)]
    NY033,
    /// Western Essex, NY
    #[doc(hidden)]
    NY034,
    /// Eastern Essex, NY
    #[doc(hidden)]
    NY035,
    /// Madison, NY
    #[doc(hidden)]
    NY036,
    /// Southern Oneida, NY
    #[doc(hidden)]
    NY037,
    /// Southern Herkimer, NY
    #[doc(hidden)]
    NY038,
    /// Southern Fulton, NY
    #[doc(hidden)]
    NY039,
    /// Montgomery, NY
    #[doc(hidden)]
    NY040,
    /// Northern Saratoga, NY
    #[doc(hidden)]
    NY041,
    /// Northern Warren, NY
    #[doc(hidden)]
    NY042,
    /// Northern Washington, NY
    #[doc(hidden)]
    NY043,
    /// Cortland, NY
    #[doc(hidden)]
    NY044,
    /// Chenango, NY
    #[doc(hidden)]
    NY045,
    /// Otsego, NY
    #[doc(hidden)]
    NY046,
    /// Schoharie, NY
    #[doc(hidden)]
    NY047,
    /// Western Schenectady, NY
    #[doc(hidden)]
    NY048,
    /// Eastern Schenectady, NY
    #[doc(hidden)]
    NY049,
    /// Southern Saratoga, NY
    #[doc(hidden)]
    NY050,
    /// Western Albany, NY
    #[doc(hidden)]
    NY051,
    /// Eastern Albany, NY
    #[doc(hidden)]
    NY052,
    /// Western Rensselaer, NY
    #[doc(hidden)]
    NY053,
    /// Eastern Rensselaer, NY
    #[doc(hidden)]
    NY054,
    /// Tioga, NY
    #[doc(hidden)]
    NY055,
    /// Broome, NY
    #[doc(hidden)]
    NY056,
    /// Delaware, NY
    #[doc(hidden)]
    NY057,
    /// Western Greene, NY
    #[doc(hidden)]
    NY058,
    /// Eastern Greene, NY
    #[doc(hidden)]
    NY059,
    /// Western Columbia, NY
    #[doc(hidden)]
    NY060,
    /// Eastern Columbia, NY
    #[doc(hidden)]
    NY061,
    /// Sullivan, NY
    #[doc(hidden)]
    NY062,
    /// Western Ulster, NY
    #[doc(hidden)]
    NY063,
    /// Eastern Ulster, NY
    #[doc(hidden)]
    NY064,
    /// Western Dutchess, NY
    #[doc(hidden)]
    NY065,
    /// Eastern Dutchess, NY
    #[doc(hidden)]
    NY066,
    /// Orange, NY
    #[doc(hidden)]
    NY067,
    /// Putnam, NY
    #[doc(hidden)]
    NY068,
    /// Rockland, NY
    #[doc(hidden)]
    NY069,
    /// Northern Westchester, NY
    #[doc(hidden)]
    NY070,
    /// Southern Westchester, NY
    #[doc(hidden)]
    NY071,
    /// New York (Manhattan), NY
    #[doc(hidden)]
    NY072,
    /// Bronx, NY
    #[doc(hidden)]
    NY073,
    /// Richmond (Staten Is.), NY
    #[doc(hidden)]
    NY074,
    /// Kings (Brooklyn), NY
    #[doc(hidden)]
    NY075,
    /// Northwest Suffolk, NY
    #[doc(hidden)]
    NY078,
    /// Northeast Suffolk, NY
    #[doc(hidden)]
    NY079,
    /// Southwest Suffolk, NY
    #[doc(hidden)]
    NY080,
    /// Southeast Suffolk, NY
    #[doc(hidden)]
    NY081,
    /// Northern Fulton, NY
    #[doc(hidden)]
    NY082,
    /// Southeast Warren, NY
    #[doc(hidden)]
    NY083,
    /// Southern Washington, NY
    #[doc(hidden)]
    NY084,
    /// Southern Erie, NY
    #[doc(hidden)]
    NY085,
    /// Southwestern St. Lawrence, NY
    #[doc(hidden)]
    NY087,
    /// Northern Queens, NY
    #[doc(hidden)]
    NY176,
    /// Northern Nassau, NY
    #[doc(hidden)]
    NY177,
    /// Southern Queens, NY
    #[doc(hidden)]
    NY178,
    /// Southern Nassau, NY
    #[doc(hidden)]
    NY179,
    /// Williams, OH
    #[doc(hidden)]
    OH001,
    /// Fulton, OH
    #[doc(hidden)]
    OH002,
    /// Lucas, OH
    #[doc(hidden)]
    OH003,
    /// Defiance, OH
    #[doc(hidden)]
    OH004,
    /// Henry, OH
    #[doc(hidden)]
    OH005,
    /// Wood, OH
    #[doc(hidden)]
    OH006,
    /// Ottawa, OH
    #[doc(hidden)]
    OH007,
    /// Sandusky, OH
    #[doc(hidden)]
    OH008,
    /// Erie, OH
    #[doc(hidden)]
    OH009,
    /// Lorain, OH
    #[doc(hidden)]
    OH010,
    /// Cuyahoga, OH
    #[doc(hidden)]
    OH011,
    /// Lake, OH
    #[doc(hidden)]
    OH012,
    /// Geauga, OH
    #[doc(hidden)]
    OH013,
    /// Ashtabula Inland, OH
    #[doc(hidden)]
    OH014,
    /// Paulding, OH
    #[doc(hidden)]
    OH015,
    /// Putnam, OH
    #[doc(hidden)]
    OH016,
    /// Hancock, OH
    #[doc(hidden)]
    OH017,
    /// Seneca, OH
    #[doc(hidden)]
    OH018,
    /// Huron, OH
    #[doc(hidden)]
    OH019,
    /// Medina, OH
    #[doc(hidden)]
    OH020,
    /// Summit, OH
    #[doc(hidden)]
    OH021,
    /// Portage, OH
    #[doc(hidden)]
    OH022,
    /// Trumbull, OH
    #[doc(hidden)]
    OH023,
    /// Van Wert, OH
    #[doc(hidden)]
    OH024,
    /// Allen, OH
    #[doc(hidden)]
    OH025,
    /// Hardin, OH
    #[doc(hidden)]
    OH026,
    /// Wyandot, OH
    #[doc(hidden)]
    OH027,
    /// Crawford, OH
    #[doc(hidden)]
    OH028,
    /// Richland, OH
    #[doc(hidden)]
    OH029,
    /// Ashland, OH
    #[doc(hidden)]
    OH030,
    /// Wayne, OH
    #[doc(hidden)]
    OH031,
    /// Stark, OH
    #[doc(hidden)]
    OH032,
    /// Mahoning, OH
    #[doc(hidden)]
    OH033,
    /// Mercer, OH
    #[doc(hidden)]
    OH034,
    /// Auglaize, OH
    #[doc(hidden)]
    OH035,
    /// Marion, OH
    #[doc(hidden)]
    OH036,
    /// Morrow, OH
    #[doc(hidden)]
    OH037,
    /// Holmes, OH
    #[doc(hidden)]
    OH038,
    /// Tuscarawas, OH
    #[doc(hidden)]
    OH039,
    /// Carroll, OH
    #[doc(hidden)]
    OH040,
    /// Columbiana, OH
    #[doc(hidden)]
    OH041,
    /// Darke, OH
    #[doc(hidden)]
    OH042,
    /// Shelby, OH
    #[doc(hidden)]
    OH043,
    /// Logan, OH
    #[doc(hidden)]
    OH044,
    /// Union, OH
    #[doc(hidden)]
    OH045,
    /// Delaware, OH
    #[doc(hidden)]
    OH046,
    /// Knox, OH
    #[doc(hidden)]
    OH047,
    /// Coshocton, OH
    #[doc(hidden)]
    OH048,
    /// Harrison, OH
    #[doc(hidden)]
    OH049,
    /// Jefferson, OH
    #[doc(hidden)]
    OH050,
    /// Miami, OH
    #[doc(hidden)]
    OH051,
    /// Champaign, OH
    #[doc(hidden)]
    OH052,
    /// Clark, OH
    #[doc(hidden)]
    OH053,
    /// Madison, OH
    #[doc(hidden)]
    OH054,
    /// Franklin, OH
    #[doc(hidden)]
    OH055,
    /// Licking, OH
    #[doc(hidden)]
    OH056,
    /// Muskingum, OH
    #[doc(hidden)]
    OH057,
    /// Guernsey, OH
    #[doc(hidden)]
    OH058,
    /// Belmont, OH
    #[doc(hidden)]
    OH059,
    /// Preble, OH
    #[doc(hidden)]
    OH060,
    /// Montgomery, OH
    #[doc(hidden)]
    OH061,
    /// Greene, OH
    #[doc(hidden)]
    OH062,
    /// Fayette, OH
    #[doc(hidden)]
    OH063,
    /// Pickaway, OH
    #[doc(hidden)]
    OH064,
    /// Fairfield, OH
    #[doc(hidden)]
    OH065,
    /// Perry, OH
    #[doc(hidden)]
    OH066,
    /// Morgan, OH
    #[doc(hidden)]
    OH067,
    /// Noble, OH
    #[doc(hidden)]
    OH068,
    /// Monroe, OH
    #[doc(hidden)]
    OH069,
    /// Butler, OH
    #[doc(hidden)]
    OH070,
    /// Warren, OH
    #[doc(hidden)]
    OH071,
    /// Clinton, OH
    #[doc(hidden)]
    OH072,
    /// Ross, OH
    #[doc(hidden)]
    OH073,
    /// Hocking, OH
    #[doc(hidden)]
    OH074,
    /// Athens, OH
    #[doc(hidden)]
    OH075,
    /// Washington, OH
    #[doc(hidden)]
    OH076,
    /// Hamilton, OH
    #[doc(hidden)]
    OH077,
    /// Clermont, OH
    #[doc(hidden)]
    OH078,
    /// Brown, OH
    #[doc(hidden)]
    OH079,
    /// Highland, OH
    #[doc(hidden)]
    OH080,
    /// Adams, OH
    #[doc(hidden)]
    OH081,
    /// Pike, OH
    #[doc(hidden)]
    OH082,
    /// Jackson, OH
    #[doc(hidden)]
    OH083,
    /// Vinton, OH
    #[doc(hidden)]
    OH084,
    /// Meigs, OH
    #[doc(hidden)]
    OH085,
    /// Gallia, OH
    #[doc(hidden)]
    OH086,
    /// Lawrence, OH
    #[doc(hidden)]
    OH087,
    /// Scioto, OH
    #[doc(hidden)]
    OH088,
    /// Ashtabula Lakeshore, OH
    #[doc(hidden)]
    OH089,
    /// Cimarron, OK
    #[doc(hidden)]
    OK001,
    /// Texas, OK
    #[doc(hidden)]
    OK002,
    /// Beaver, OK
    #[doc(hidden)]
    OK003,
    /// Harper, OK
    #[doc(hidden)]
    OK004,
    /// Woods, OK
    #[doc(hidden)]
    OK005,
    /// Alfalfa, OK
    #[doc(hidden)]
    OK006,
    /// Grant, OK
    #[doc(hidden)]
    OK007,
    /// Kay, OK
    #[doc(hidden)]
    OK008,
    /// Ellis, OK
    #[doc(hidden)]
    OK009,
    /// Woodward, OK
    #[doc(hidden)]
    OK010,
    /// Major, OK
    #[doc(hidden)]
    OK011,
    /// Garfield, OK
    #[doc(hidden)]
    OK012,
    /// Noble, OK
    #[doc(hidden)]
    OK013,
    /// Roger Mills, OK
    #[doc(hidden)]
    OK014,
    /// Dewey, OK
    #[doc(hidden)]
    OK015,
    /// Custer, OK
    #[doc(hidden)]
    OK016,
    /// Blaine, OK
    #[doc(hidden)]
    OK017,
    /// Kingfisher, OK
    #[doc(hidden)]
    OK018,
    /// Logan, OK
    #[doc(hidden)]
    OK019,
    /// Payne, OK
    #[doc(hidden)]
    OK020,
    /// Beckham, OK
    #[doc(hidden)]
    OK021,
    /// Washita, OK
    #[doc(hidden)]
    OK022,
    /// Caddo, OK
    #[doc(hidden)]
    OK023,
    /// Canadian, OK
    #[doc(hidden)]
    OK024,
    /// Oklahoma, OK
    #[doc(hidden)]
    OK025,
    /// Lincoln, OK
    #[doc(hidden)]
    OK026,
    /// Grady, OK
    #[doc(hidden)]
    OK027,
    /// McClain, OK
    #[doc(hidden)]
    OK028,
    /// Cleveland, OK
    #[doc(hidden)]
    OK029,
    /// Pottawatomie, OK
    #[doc(hidden)]
    OK030,
    /// Seminole, OK
    #[doc(hidden)]
    OK031,
    /// Hughes, OK
    #[doc(hidden)]
    OK032,
    /// Harmon, OK
    #[doc(hidden)]
    OK033,
    /// Greer, OK
    #[doc(hidden)]
    OK034,
    /// Kiowa, OK
    #[doc(hidden)]
    OK035,
    /// Jackson, OK
    #[doc(hidden)]
    OK036,
    /// Tillman, OK
    #[doc(hidden)]
    OK037,
    /// Comanche, OK
    #[doc(hidden)]
    OK038,
    /// Stephens, OK
    #[doc(hidden)]
    OK039,
    /// Garvin, OK
    #[doc(hidden)]
    OK040,
    /// Murray, OK
    #[doc(hidden)]
    OK041,
    /// Pontotoc, OK
    #[doc(hidden)]
    OK042,
    /// Coal, OK
    #[doc(hidden)]
    OK043,
    /// Cotton, OK
    #[doc(hidden)]
    OK044,
    /// Jefferson, OK
    #[doc(hidden)]
    OK045,
    /// Carter, OK
    #[doc(hidden)]
    OK046,
    /// Johnston, OK
    #[doc(hidden)]
    OK047,
    /// Atoka, OK
    #[doc(hidden)]
    OK048,
    /// Pushmataha, OK
    #[doc(hidden)]
    OK049,
    /// Love, OK
    #[doc(hidden)]
    OK050,
    /// Marshall, OK
    #[doc(hidden)]
    OK051,
    /// Bryan, OK
    #[doc(hidden)]
    OK052,
    /// Choctaw, OK
    #[doc(hidden)]
    OK053,
    /// Osage, OK
    #[doc(hidden)]
    OK054,
    /// Washington, OK
    #[doc(hidden)]
    OK055,
    /// Nowata, OK
    #[doc(hidden)]
    OK056,
    /// Craig, OK
    #[doc(hidden)]
    OK057,
    /// Ottawa, OK
    #[doc(hidden)]
    OK058,
    /// Pawnee, OK
    #[doc(hidden)]
    OK059,
    /// Tulsa, OK
    #[doc(hidden)]
    OK060,
    /// Rogers, OK
    #[doc(hidden)]
    OK061,
    /// Mayes, OK
    #[doc(hidden)]
    OK062,
    /// Delaware, OK
    #[doc(hidden)]
    OK063,
    /// Creek, OK
    #[doc(hidden)]
    OK064,
    /// Okfuskee, OK
    #[doc(hidden)]
    OK065,
    /// Okmulgee, OK
    #[doc(hidden)]
    OK066,
    /// Wagoner, OK
    #[doc(hidden)]
    OK067,
    /// Cherokee, OK
    #[doc(hidden)]
    OK068,
    /// Adair, OK
    #[doc(hidden)]
    OK069,
    /// Muskogee, OK
    #[doc(hidden)]
    OK070,
    /// McIntosh, OK
    #[doc(hidden)]
    OK071,
    /// Sequoyah, OK
    #[doc(hidden)]
    OK072,
    /// Pittsburg, OK
    #[doc(hidden)]
    OK073,
    /// Haskell, OK
    #[doc(hidden)]
    OK074,
    /// Latimer, OK
    #[doc(hidden)]
    OK075,
    /// Le Flore, OK
    #[doc(hidden)]
    OK076,
    /// McCurtain, OK
    #[doc(hidden)]
    OK077,
    /// North Oregon Coast, OR
    #[doc(hidden)]
    OR001,
    /// Central Oregon Coast, OR
    #[doc(hidden)]
    OR002,
    /// Coast Range of Northwest Oregon, OR
    #[doc(hidden)]
    OR003,
    /// Central Coast Range of Western Oregon, OR
    #[doc(hidden)]
    OR004,
    /// Lower Columbia, OR
    #[doc(hidden)]
    OR005,
    /// Greater Portland Metro Area, OR
    #[doc(hidden)]
    OR006,
    /// Central Willamette Valley, OR
    #[doc(hidden)]
    OR007,
    /// South  Willamette Valley, OR
    #[doc(hidden)]
    OR008,
    /// Northern Oregon Cascade Foothills, OR
    #[doc(hidden)]
    OR010,
    /// Northern Oregon Cascades, OR
    #[doc(hidden)]
    OR011,
    /// Cascade Foothills in Lane County, OR
    #[doc(hidden)]
    OR012,
    /// Cascades in Lane County, OR
    #[doc(hidden)]
    OR013,
    /// Upper Hood River Valley, OR
    #[doc(hidden)]
    OR014,
    /// Western Columbia River Gorge, OR
    #[doc(hidden)]
    OR015,
    /// Central Columbia River Gorge, OR
    #[doc(hidden)]
    OR016,
    /// South Central Oregon Coast, OR
    #[doc(hidden)]
    OR021,
    /// Curry County Coast, OR
    #[doc(hidden)]
    OR022,
    /// Central Douglas County, OR
    #[doc(hidden)]
    OR023,
    /// Eastern Curry County and Josephine County, OR
    #[doc(hidden)]
    OR024,
    /// Eastern Douglas County Foothills, OR
    #[doc(hidden)]
    OR025,
    /// Jackson County, OR
    #[doc(hidden)]
    OR026,
    /// South Central Oregon Cascades, OR
    #[doc(hidden)]
    OR027,
    /// Siskiyou Mountains and Southern Oregon Cascades, OR
    #[doc(hidden)]
    OR028,
    /// Klamath Basin, OR
    #[doc(hidden)]
    OR029,
    /// Northern and Eastern Klamath County and Western Lake County, OR
    #[doc(hidden)]
    OR030,
    /// Central and Eastern Lake County, OR
    #[doc(hidden)]
    OR031,
    /// Eastern Columbia River Gorge of Oregon, OR
    #[doc(hidden)]
    OR041,
    /// Lower Columbia Basin of Oregon, OR
    #[doc(hidden)]
    OR044,
    /// Grande Ronde Valley, OR
    #[doc(hidden)]
    OR049,
    /// Wallowa County, OR
    #[doc(hidden)]
    OR050,
    /// Harney County, OR
    #[doc(hidden)]
    OR061,
    /// Baker County, OR
    #[doc(hidden)]
    OR062,
    /// Malheur County, OR
    #[doc(hidden)]
    OR063,
    /// Oregon Lower Treasure Valley, OR
    #[doc(hidden)]
    OR064,
    /// Northern Blue Mountains of Oregon, OR
    #[doc(hidden)]
    OR502,
    /// Southern Blue Mountains of Oregon, OR
    #[doc(hidden)]
    OR503,
    /// John Day Basin, OR
    #[doc(hidden)]
    OR505,
    /// Ochoco-John Day Highlands, OR
    #[doc(hidden)]
    OR506,
    /// Foothills of the Northern Blue Mountains of Oregon, OR
    #[doc(hidden)]
    OR507,
    /// Foothills of the Southern Blue Mountains of Oregon, OR
    #[doc(hidden)]
    OR508,
    /// East Slopes of the Oregon Cascades, OR
    #[doc(hidden)]
    OR509,
    /// North Central Oregon, OR
    #[doc(hidden)]
    OR510,
    /// Central Oregon, OR
    #[doc(hidden)]
    OR511,
    /// Northern Erie, PA
    #[doc(hidden)]
    PA001,
    /// Southern Erie, PA
    #[doc(hidden)]
    PA002,
    /// Crawford, PA
    #[doc(hidden)]
    PA003,
    /// Warren, PA
    #[doc(hidden)]
    PA004,
    /// McKean, PA
    #[doc(hidden)]
    PA005,
    /// Potter, PA
    #[doc(hidden)]
    PA006,
    /// Mercer, PA
    #[doc(hidden)]
    PA007,
    /// Venango, PA
    #[doc(hidden)]
    PA008,
    /// Forest, PA
    #[doc(hidden)]
    PA009,
    /// Elk, PA
    #[doc(hidden)]
    PA010,
    /// Cameron, PA
    #[doc(hidden)]
    PA011,
    /// Northern Clinton, PA
    #[doc(hidden)]
    PA012,
    /// Lawrence, PA
    #[doc(hidden)]
    PA013,
    /// Butler, PA
    #[doc(hidden)]
    PA014,
    /// Clarion, PA
    #[doc(hidden)]
    PA015,
    /// Jefferson, PA
    #[doc(hidden)]
    PA016,
    /// Clearfield, PA
    #[doc(hidden)]
    PA017,
    /// Northern Centre, PA
    #[doc(hidden)]
    PA018,
    /// Southern Centre, PA
    #[doc(hidden)]
    PA019,
    /// Beaver, PA
    #[doc(hidden)]
    PA020,
    /// Allegheny, PA
    #[doc(hidden)]
    PA021,
    /// Armstrong, PA
    #[doc(hidden)]
    PA022,
    /// Indiana, PA
    #[doc(hidden)]
    PA023,
    /// Cambria, PA
    #[doc(hidden)]
    PA024,
    /// Blair, PA
    #[doc(hidden)]
    PA025,
    /// Huntingdon, PA
    #[doc(hidden)]
    PA026,
    /// Mifflin, PA
    #[doc(hidden)]
    PA027,
    /// Juniata, PA
    #[doc(hidden)]
    PA028,
    /// Washington, PA
    #[doc(hidden)]
    PA029,
    /// Greene, PA
    #[doc(hidden)]
    PA031,
    /// Somerset, PA
    #[doc(hidden)]
    PA033,
    /// Bedford, PA
    #[doc(hidden)]
    PA034,
    /// Fulton, PA
    #[doc(hidden)]
    PA035,
    /// Franklin, PA
    #[doc(hidden)]
    PA036,
    /// Tioga, PA
    #[doc(hidden)]
    PA037,
    /// Bradford, PA
    #[doc(hidden)]
    PA038,
    /// Susquehanna, PA
    #[doc(hidden)]
    PA039,
    /// Northern Wayne, PA
    #[doc(hidden)]
    PA040,
    /// Northern Lycoming, PA
    #[doc(hidden)]
    PA041,
    /// Sullivan, PA
    #[doc(hidden)]
    PA042,
    /// Wyoming, PA
    #[doc(hidden)]
    PA043,
    /// Lackawanna, PA
    #[doc(hidden)]
    PA044,
    /// Southern Clinton, PA
    #[doc(hidden)]
    PA045,
    /// Southern Lycoming, PA
    #[doc(hidden)]
    PA046,
    /// Luzerne, PA
    #[doc(hidden)]
    PA047,
    /// Pike, PA
    #[doc(hidden)]
    PA048,
    /// Union, PA
    #[doc(hidden)]
    PA049,
    /// Snyder, PA
    #[doc(hidden)]
    PA050,
    /// Montour, PA
    #[doc(hidden)]
    PA051,
    /// Northumberland, PA
    #[doc(hidden)]
    PA052,
    /// Columbia, PA
    #[doc(hidden)]
    PA053,
    /// Carbon, PA
    #[doc(hidden)]
    PA054,
    /// Monroe, PA
    #[doc(hidden)]
    PA055,
    /// Perry, PA
    #[doc(hidden)]
    PA056,
    /// Dauphin, PA
    #[doc(hidden)]
    PA057,
    /// Schuylkill, PA
    #[doc(hidden)]
    PA058,
    /// Lebanon, PA
    #[doc(hidden)]
    PA059,
    /// Berks, PA
    #[doc(hidden)]
    PA060,
    /// Lehigh, PA
    #[doc(hidden)]
    PA061,
    /// Northampton, PA
    #[doc(hidden)]
    PA062,
    /// Cumberland, PA
    #[doc(hidden)]
    PA063,
    /// Adams, PA
    #[doc(hidden)]
    PA064,
    /// York, PA
    #[doc(hidden)]
    PA065,
    /// Lancaster, PA
    #[doc(hidden)]
    PA066,
    /// Delaware, PA
    #[doc(hidden)]
    PA070,
    /// Philadelphia, PA
    #[doc(hidden)]
    PA071,
    /// Southern Wayne, PA
    #[doc(hidden)]
    PA072,
    /// Westmoreland, PA
    #[doc(hidden)]
    PA073,
    /// Westmoreland Ridges, PA
    #[doc(hidden)]
    PA074,
    /// Fayette, PA
    #[doc(hidden)]
    PA075,
    /// Fayette Ridges, PA
    #[doc(hidden)]
    PA076,
    /// Western Chester, PA
    #[doc(hidden)]
    PA101,
    /// Eastern Chester, PA
    #[doc(hidden)]
    PA102,
    /// Western Montgomery, PA
    #[doc(hidden)]
    PA103,
    /// Eastern Montgomery, PA
    #[doc(hidden)]
    PA104,
    /// Upper Bucks, PA
    #[doc(hidden)]
    PA105,
    /// Lower Bucks, PA
    #[doc(hidden)]
    PA106,
    /// San Juan and Vicinity, PR
    #[doc(hidden)]
    PR001,
    /// Northeast, PR
    #[doc(hidden)]
    PR002,
    /// Southeast, PR
    #[doc(hidden)]
    PR003,
    /// Eastern Interior, PR
    #[doc(hidden)]
    PR004,
    /// North Central, PR
    #[doc(hidden)]
    PR005,
    /// Central Interior, PR
    #[doc(hidden)]
    PR006,
    /// Ponce and Vicinity, PR
    #[doc(hidden)]
    PR007,
    /// Northwest, PR
    #[doc(hidden)]
    PR008,
    /// Western Interior, PR
    #[doc(hidden)]
    PR009,
    /// Mayaguez and Vicinity, PR
    #[doc(hidden)]
    PR010,
    /// Southwest, PR
    #[doc(hidden)]
    PR011,
    /// Culebra, PR
    #[doc(hidden)]
    PR012,
    /// Vieques, PR
    #[doc(hidden)]
    PR013,
    /// Kayangel, PW
    #[doc(hidden)]
    PW001,
    /// Melekeok, PW
    #[doc(hidden)]
    PW002,
    /// Airai, PW
    #[doc(hidden)]
    PW003,
    /// Koror, PW
    #[doc(hidden)]
    PW004,
    /// Peleliu, PW
    #[doc(hidden)]
    PW005,
    /// Angaur, PW
    #[doc(hidden)]
    PW006,
    /// Sonsorol, PW
    #[doc(hidden)]
    PW007,
    /// Tobi, PW
    #[doc(hidden)]
    PW008,
    /// Northwest Providence, RI
    #[doc(hidden)]
    RI001,
    /// Southeast Providence, RI
    #[doc(hidden)]
    RI002,
    /// Western Kent, RI
    #[doc(hidden)]
    RI003,
    /// Eastern Kent, RI
    #[doc(hidden)]
    RI004,
    /// Bristol, RI
    #[doc(hidden)]
    RI005,
    /// Washington, RI
    #[doc(hidden)]
    RI006,
    /// Newport, RI
    #[doc(hidden)]
    RI007,
    /// Block Island, RI
    #[doc(hidden)]
    RI008,
    /// Cherokee, SC
    #[doc(hidden)]
    SC008,
    /// York, SC
    #[doc(hidden)]
    SC009,
    /// Anderson, SC
    #[doc(hidden)]
    SC010,
    /// Abbeville, SC
    #[doc(hidden)]
    SC011,
    /// Laurens, SC
    #[doc(hidden)]
    SC012,
    /// Union, SC
    #[doc(hidden)]
    SC013,
    /// Chester, SC
    #[doc(hidden)]
    SC014,
    /// Chesterfield, SC
    #[doc(hidden)]
    SC016,
    /// Marlboro, SC
    #[doc(hidden)]
    SC017,
    /// McCormick, SC
    #[doc(hidden)]
    SC018,
    /// Greenwood, SC
    #[doc(hidden)]
    SC019,
    /// Newberry, SC
    #[doc(hidden)]
    SC020,
    /// Fairfield, SC
    #[doc(hidden)]
    SC021,
    /// Kershaw, SC
    #[doc(hidden)]
    SC022,
    /// Darlington, SC
    #[doc(hidden)]
    SC023,
    /// Dillon, SC
    #[doc(hidden)]
    SC024,
    /// Edgefield, SC
    #[doc(hidden)]
    SC025,
    /// Saluda, SC
    #[doc(hidden)]
    SC026,
    /// Lexington, SC
    #[doc(hidden)]
    SC027,
    /// Richland, SC
    #[doc(hidden)]
    SC028,
    /// Lee, SC
    #[doc(hidden)]
    SC029,
    /// Aiken, SC
    #[doc(hidden)]
    SC030,
    /// Sumter, SC
    #[doc(hidden)]
    SC031,
    /// Florence, SC
    #[doc(hidden)]
    SC032,
    /// Marion, SC
    #[doc(hidden)]
    SC033,
    /// Barnwell, SC
    #[doc(hidden)]
    SC035,
    /// Calhoun, SC
    #[doc(hidden)]
    SC037,
    /// Clarendon, SC
    #[doc(hidden)]
    SC038,
    /// Williamsburg, SC
    #[doc(hidden)]
    SC039,
    /// Allendale, SC
    #[doc(hidden)]
    SC040,
    /// Bamberg, SC
    #[doc(hidden)]
    SC041,
    /// Hampton, SC
    #[doc(hidden)]
    SC042,
    /// Inland Colleton, SC
    #[doc(hidden)]
    SC043,
    /// Dorchester, SC
    #[doc(hidden)]
    SC044,
    /// Inland Berkeley, SC
    #[doc(hidden)]
    SC045,
    /// Inland Jasper, SC
    #[doc(hidden)]
    SC047,
    /// Beaufort, SC
    #[doc(hidden)]
    SC048,
    /// Coastal Colleton, SC
    #[doc(hidden)]
    SC049,
    /// Charleston, SC
    #[doc(hidden)]
    SC050,
    /// Coastal Jasper, SC
    #[doc(hidden)]
    SC051,
    /// Tidal Berkeley, SC
    #[doc(hidden)]
    SC052,
    /// Coastal Horry, SC
    #[doc(hidden)]
    SC054,
    /// Inland Georgetown, SC
    #[doc(hidden)]
    SC055,
    /// Coastal Georgetown, SC
    #[doc(hidden)]
    SC056,
    /// Central Horry, SC
    #[doc(hidden)]
    SC058,
    /// Northern Horry, SC
    #[doc(hidden)]
    SC059,
    /// Oconee Mountains, SC
    #[doc(hidden)]
    SC101,
    /// Pickens Mountains, SC
    #[doc(hidden)]
    SC102,
    /// Greenville Mountains, SC
    #[doc(hidden)]
    SC103,
    /// Greater Oconee, SC
    #[doc(hidden)]
    SC104,
    /// Greater Pickens, SC
    #[doc(hidden)]
    SC105,
    /// Central Greenville, SC
    #[doc(hidden)]
    SC106,
    /// Southern Greenville, SC
    #[doc(hidden)]
    SC107,
    /// Northern Spartanburg, SC
    #[doc(hidden)]
    SC108,
    /// Southern Spartanburg, SC
    #[doc(hidden)]
    SC109,
    /// Northern Lancaster, SC
    #[doc(hidden)]
    SC115,
    /// Southern Lancaster, SC
    #[doc(hidden)]
    SC116,
    /// Northwestern Orangeburg, SC
    #[doc(hidden)]
    SC135,
    /// Central Orangeburg, SC
    #[doc(hidden)]
    SC136,
    /// Southeastern Orangeburg, SC
    #[doc(hidden)]
    SC137,
    /// Harding, SD
    #[doc(hidden)]
    SD001,
    /// Perkins, SD
    #[doc(hidden)]
    SD002,
    /// Corson, SD
    #[doc(hidden)]
    SD003,
    /// Campbell, SD
    #[doc(hidden)]
    SD004,
    /// McPherson, SD
    #[doc(hidden)]
    SD005,
    /// Brown, SD
    #[doc(hidden)]
    SD006,
    /// Marshall, SD
    #[doc(hidden)]
    SD007,
    /// Roberts, SD
    #[doc(hidden)]
    SD008,
    /// Walworth, SD
    #[doc(hidden)]
    SD009,
    /// Edmunds, SD
    #[doc(hidden)]
    SD010,
    /// Day, SD
    #[doc(hidden)]
    SD011,
    /// Butte, SD
    #[doc(hidden)]
    SD012,
    /// Northern Meade Co Plains, SD
    #[doc(hidden)]
    SD013,
    /// Ziebach, SD
    #[doc(hidden)]
    SD014,
    /// Dewey, SD
    #[doc(hidden)]
    SD015,
    /// Potter, SD
    #[doc(hidden)]
    SD016,
    /// Faulk, SD
    #[doc(hidden)]
    SD017,
    /// Spink, SD
    #[doc(hidden)]
    SD018,
    /// Clark, SD
    #[doc(hidden)]
    SD019,
    /// Codington, SD
    #[doc(hidden)]
    SD020,
    /// Grant, SD
    #[doc(hidden)]
    SD021,
    /// Hamlin, SD
    #[doc(hidden)]
    SD022,
    /// Deuel, SD
    #[doc(hidden)]
    SD023,
    /// Northern Black Hills, SD
    #[doc(hidden)]
    SD024,
    /// Northern Foot Hills, SD
    #[doc(hidden)]
    SD025,
    /// Rapid City, SD
    #[doc(hidden)]
    SD026,
    /// Southern Foot Hills, SD
    #[doc(hidden)]
    SD027,
    /// Central Black Hills, SD
    #[doc(hidden)]
    SD028,
    /// Southern Black Hills, SD
    #[doc(hidden)]
    SD029,
    /// Custer Co Plains, SD
    #[doc(hidden)]
    SD030,
    /// Pennington Co Plains, SD
    #[doc(hidden)]
    SD031,
    /// Haakon, SD
    #[doc(hidden)]
    SD032,
    /// Stanley, SD
    #[doc(hidden)]
    SD033,
    /// Sully, SD
    #[doc(hidden)]
    SD034,
    /// Hughes, SD
    #[doc(hidden)]
    SD035,
    /// Hyde, SD
    #[doc(hidden)]
    SD036,
    /// Hand, SD
    #[doc(hidden)]
    SD037,
    /// Beadle, SD
    #[doc(hidden)]
    SD038,
    /// Kingsbury, SD
    #[doc(hidden)]
    SD039,
    /// Brookings, SD
    #[doc(hidden)]
    SD040,
    /// Fall River, SD
    #[doc(hidden)]
    SD041,
    /// Oglala Lakota, SD
    #[doc(hidden)]
    SD042,
    /// Jackson, SD
    #[doc(hidden)]
    SD043,
    /// Bennett, SD
    #[doc(hidden)]
    SD044,
    /// Jones, SD
    #[doc(hidden)]
    SD045,
    /// Mellette, SD
    #[doc(hidden)]
    SD046,
    /// Todd, SD
    #[doc(hidden)]
    SD047,
    /// Lyman, SD
    #[doc(hidden)]
    SD048,
    /// Tripp, SD
    #[doc(hidden)]
    SD049,
    /// Gregory, SD
    #[doc(hidden)]
    SD050,
    /// Buffalo, SD
    #[doc(hidden)]
    SD051,
    /// Jerauld, SD
    #[doc(hidden)]
    SD052,
    /// Sanborn, SD
    #[doc(hidden)]
    SD053,
    /// Miner, SD
    #[doc(hidden)]
    SD054,
    /// Lake, SD
    #[doc(hidden)]
    SD055,
    /// Moody, SD
    #[doc(hidden)]
    SD056,
    /// Brule, SD
    #[doc(hidden)]
    SD057,
    /// Aurora, SD
    #[doc(hidden)]
    SD058,
    /// Davison, SD
    #[doc(hidden)]
    SD059,
    /// Hanson, SD
    #[doc(hidden)]
    SD060,
    /// McCook, SD
    #[doc(hidden)]
    SD061,
    /// Minnehaha, SD
    #[doc(hidden)]
    SD062,
    /// Charles Mix, SD
    #[doc(hidden)]
    SD063,
    /// Douglas, SD
    #[doc(hidden)]
    SD064,
    /// Hutchinson, SD
    #[doc(hidden)]
    SD065,
    /// Turner, SD
    #[doc(hidden)]
    SD066,
    /// Lincoln, SD
    #[doc(hidden)]
    SD067,
    /// Bon Homme, SD
    #[doc(hidden)]
    SD068,
    /// Yankton, SD
    #[doc(hidden)]
    SD069,
    /// Clay, SD
    #[doc(hidden)]
    SD070,
    /// Union, SD
    #[doc(hidden)]
    SD071,
    /// Sturgis/Piedmont Foot Hills, SD
    #[doc(hidden)]
    SD072,
    /// Southern Meade Co Plains, SD
    #[doc(hidden)]
    SD073,
    /// Hermosa Foot Hills, SD
    #[doc(hidden)]
    SD074,
    /// Lake, TN
    #[doc(hidden)]
    TN001,
    /// Obion, TN
    #[doc(hidden)]
    TN002,
    /// Weakley, TN
    #[doc(hidden)]
    TN003,
    /// Henry, TN
    #[doc(hidden)]
    TN004,
    /// Stewart, TN
    #[doc(hidden)]
    TN005,
    /// Montgomery, TN
    #[doc(hidden)]
    TN006,
    /// Robertson, TN
    #[doc(hidden)]
    TN007,
    /// Sumner, TN
    #[doc(hidden)]
    TN008,
    /// Macon, TN
    #[doc(hidden)]
    TN009,
    /// Clay, TN
    #[doc(hidden)]
    TN010,
    /// Pickett, TN
    #[doc(hidden)]
    TN011,
    /// Scott, TN
    #[doc(hidden)]
    TN012,
    /// Campbell, TN
    #[doc(hidden)]
    TN013,
    /// Claiborne, TN
    #[doc(hidden)]
    TN014,
    /// Hancock, TN
    #[doc(hidden)]
    TN015,
    /// Hawkins, TN
    #[doc(hidden)]
    TN016,
    /// Sullivan, TN
    #[doc(hidden)]
    TN017,
    /// Johnson, TN
    #[doc(hidden)]
    TN018,
    /// Dyer, TN
    #[doc(hidden)]
    TN019,
    /// Gibson, TN
    #[doc(hidden)]
    TN020,
    /// Carroll, TN
    #[doc(hidden)]
    TN021,
    /// Benton, TN
    #[doc(hidden)]
    TN022,
    /// Houston, TN
    #[doc(hidden)]
    TN023,
    /// Humphreys, TN
    #[doc(hidden)]
    TN024,
    /// Dickson, TN
    #[doc(hidden)]
    TN025,
    /// Cheatham, TN
    #[doc(hidden)]
    TN026,
    /// Davidson, TN
    #[doc(hidden)]
    TN027,
    /// Wilson, TN
    #[doc(hidden)]
    TN028,
    /// Trousdale, TN
    #[doc(hidden)]
    TN029,
    /// Smith, TN
    #[doc(hidden)]
    TN030,
    /// Jackson, TN
    #[doc(hidden)]
    TN031,
    /// Putnam, TN
    #[doc(hidden)]
    TN032,
    /// Overton, TN
    #[doc(hidden)]
    TN033,
    /// Fentress, TN
    #[doc(hidden)]
    TN034,
    /// Morgan, TN
    #[doc(hidden)]
    TN035,
    /// Anderson, TN
    #[doc(hidden)]
    TN036,
    /// Union, TN
    #[doc(hidden)]
    TN037,
    /// Grainger, TN
    #[doc(hidden)]
    TN038,
    /// Hamblen, TN
    #[doc(hidden)]
    TN039,
    /// Northwest Cocke, TN
    #[doc(hidden)]
    TN040,
    /// Cocke Smoky Mountains, TN
    #[doc(hidden)]
    TN041,
    /// Northwest Greene, TN
    #[doc(hidden)]
    TN042,
    /// Southeast Greene, TN
    #[doc(hidden)]
    TN043,
    /// Washington, TN
    #[doc(hidden)]
    TN044,
    /// Unicoi, TN
    #[doc(hidden)]
    TN045,
    /// Northwest Carter, TN
    #[doc(hidden)]
    TN046,
    /// Southeast Carter, TN
    #[doc(hidden)]
    TN047,
    /// Lauderdale, TN
    #[doc(hidden)]
    TN048,
    /// Tipton, TN
    #[doc(hidden)]
    TN049,
    /// Haywood, TN
    #[doc(hidden)]
    TN050,
    /// Crockett, TN
    #[doc(hidden)]
    TN051,
    /// Madison, TN
    #[doc(hidden)]
    TN052,
    /// Chester, TN
    #[doc(hidden)]
    TN053,
    /// Henderson, TN
    #[doc(hidden)]
    TN054,
    /// Decatur, TN
    #[doc(hidden)]
    TN055,
    /// Perry, TN
    #[doc(hidden)]
    TN056,
    /// Hickman, TN
    #[doc(hidden)]
    TN057,
    /// Lewis, TN
    #[doc(hidden)]
    TN058,
    /// Williamson, TN
    #[doc(hidden)]
    TN059,
    /// Maury, TN
    #[doc(hidden)]
    TN060,
    /// Marshall, TN
    #[doc(hidden)]
    TN061,
    /// Rutherford, TN
    #[doc(hidden)]
    TN062,
    /// Cannon, TN
    #[doc(hidden)]
    TN063,
    /// De Kalb, TN
    #[doc(hidden)]
    TN064,
    /// White, TN
    #[doc(hidden)]
    TN065,
    /// Cumberland, TN
    #[doc(hidden)]
    TN066,
    /// Roane, TN
    #[doc(hidden)]
    TN067,
    /// Loudon, TN
    #[doc(hidden)]
    TN068,
    /// Knox, TN
    #[doc(hidden)]
    TN069,
    /// Jefferson, TN
    #[doc(hidden)]
    TN070,
    /// NW Blount, TN
    #[doc(hidden)]
    TN071,
    /// Blount Smoky Mountains, TN
    #[doc(hidden)]
    TN072,
    /// North Sevier, TN
    #[doc(hidden)]
    TN073,
    /// Sevier Smoky Mountains, TN
    #[doc(hidden)]
    TN074,
    /// Bedford, TN
    #[doc(hidden)]
    TN075,
    /// Moore, TN
    #[doc(hidden)]
    TN076,
    /// Coffee, TN
    #[doc(hidden)]
    TN077,
    /// Warren, TN
    #[doc(hidden)]
    TN078,
    /// Grundy, TN
    #[doc(hidden)]
    TN079,
    /// Van Buren, TN
    #[doc(hidden)]
    TN080,
    /// Sequatchie, TN
    #[doc(hidden)]
    TN081,
    /// Bledsoe, TN
    #[doc(hidden)]
    TN082,
    /// Rhea, TN
    #[doc(hidden)]
    TN083,
    /// Meigs, TN
    #[doc(hidden)]
    TN084,
    /// McMinn, TN
    #[doc(hidden)]
    TN085,
    /// Northwest Monroe, TN
    #[doc(hidden)]
    TN086,
    /// Southeast Monroe, TN
    #[doc(hidden)]
    TN087,
    /// Shelby, TN
    #[doc(hidden)]
    TN088,
    /// Fayette, TN
    #[doc(hidden)]
    TN089,
    /// Hardeman, TN
    #[doc(hidden)]
    TN090,
    /// McNairy, TN
    #[doc(hidden)]
    TN091,
    /// Hardin, TN
    #[doc(hidden)]
    TN092,
    /// Wayne, TN
    #[doc(hidden)]
    TN093,
    /// Lawrence, TN
    #[doc(hidden)]
    TN094,
    /// Giles, TN
    #[doc(hidden)]
    TN095,
    /// Lincoln, TN
    #[doc(hidden)]
    TN096,
    /// Franklin, TN
    #[doc(hidden)]
    TN097,
    /// Marion, TN
    #[doc(hidden)]
    TN098,
    /// Hamilton, TN
    #[doc(hidden)]
    TN099,
    /// Bradley, TN
    #[doc(hidden)]
    TN100,
    /// West Polk, TN
    #[doc(hidden)]
    TN101,
    /// East Polk, TN
    #[doc(hidden)]
    TN102,
    /// Dallam, TX
    #[doc(hidden)]
    TX001,
    /// Sherman, TX
    #[doc(hidden)]
    TX002,
    /// Hansford, TX
    #[doc(hidden)]
    TX003,
    /// Ochiltree, TX
    #[doc(hidden)]
    TX004,
    /// Lipscomb, TX
    #[doc(hidden)]
    TX005,
    /// Hartley, TX
    #[doc(hidden)]
    TX006,
    /// Moore, TX
    #[doc(hidden)]
    TX007,
    /// Hutchinson, TX
    #[doc(hidden)]
    TX008,
    /// Roberts, TX
    #[doc(hidden)]
    TX009,
    /// Hemphill, TX
    #[doc(hidden)]
    TX010,
    /// Oldham, TX
    #[doc(hidden)]
    TX011,
    /// Potter, TX
    #[doc(hidden)]
    TX012,
    /// Carson, TX
    #[doc(hidden)]
    TX013,
    /// Gray, TX
    #[doc(hidden)]
    TX014,
    /// Wheeler, TX
    #[doc(hidden)]
    TX015,
    /// Deaf Smith, TX
    #[doc(hidden)]
    TX016,
    /// Randall, TX
    #[doc(hidden)]
    TX017,
    /// Armstrong, TX
    #[doc(hidden)]
    TX018,
    /// Donley, TX
    #[doc(hidden)]
    TX019,
    /// Collingsworth, TX
    #[doc(hidden)]
    TX020,
    /// Parmer, TX
    #[doc(hidden)]
    TX021,
    /// Castro, TX
    #[doc(hidden)]
    TX022,
    /// Swisher, TX
    #[doc(hidden)]
    TX023,
    /// Briscoe, TX
    #[doc(hidden)]
    TX024,
    /// Hall, TX
    #[doc(hidden)]
    TX025,
    /// Childress, TX
    #[doc(hidden)]
    TX026,
    /// Bailey, TX
    #[doc(hidden)]
    TX027,
    /// Lamb, TX
    #[doc(hidden)]
    TX028,
    /// Hale, TX
    #[doc(hidden)]
    TX029,
    /// Floyd, TX
    #[doc(hidden)]
    TX030,
    /// Motley, TX
    #[doc(hidden)]
    TX031,
    /// Cottle, TX
    #[doc(hidden)]
    TX032,
    /// Cochran, TX
    #[doc(hidden)]
    TX033,
    /// Hockley, TX
    #[doc(hidden)]
    TX034,
    /// Lubbock, TX
    #[doc(hidden)]
    TX035,
    /// Crosby, TX
    #[doc(hidden)]
    TX036,
    /// Dickens, TX
    #[doc(hidden)]
    TX037,
    /// King, TX
    #[doc(hidden)]
    TX038,
    /// Yoakum, TX
    #[doc(hidden)]
    TX039,
    /// Terry, TX
    #[doc(hidden)]
    TX040,
    /// Lynn, TX
    #[doc(hidden)]
    TX041,
    /// Garza, TX
    #[doc(hidden)]
    TX042,
    /// Kent, TX
    #[doc(hidden)]
    TX043,
    /// Stonewall, TX
    #[doc(hidden)]
    TX044,
    /// Gaines, TX
    #[doc(hidden)]
    TX045,
    /// Dawson, TX
    #[doc(hidden)]
    TX046,
    /// Borden, TX
    #[doc(hidden)]
    TX047,
    /// Scurry, TX
    #[doc(hidden)]
    TX048,
    /// Fisher, TX
    #[doc(hidden)]
    TX049,
    /// Andrews, TX
    #[doc(hidden)]
    TX050,
    /// Martin, TX
    #[doc(hidden)]
    TX051,
    /// Howard, TX
    #[doc(hidden)]
    TX052,
    /// Mitchell, TX
    #[doc(hidden)]
    TX053,
    /// Nolan, TX
    #[doc(hidden)]
    TX054,
    /// Loving, TX
    #[doc(hidden)]
    TX059,
    /// Winkler, TX
    #[doc(hidden)]
    TX060,
    /// Ector, TX
    #[doc(hidden)]
    TX061,
    /// Midland, TX
    #[doc(hidden)]
    TX062,
    /// Glasscock, TX
    #[doc(hidden)]
    TX063,
    /// Sterling, TX
    #[doc(hidden)]
    TX064,
    /// Coke, TX
    #[doc(hidden)]
    TX065,
    /// Runnels, TX
    #[doc(hidden)]
    TX066,
    /// Ward, TX
    #[doc(hidden)]
    TX067,
    /// Crane, TX
    #[doc(hidden)]
    TX068,
    /// Upton, TX
    #[doc(hidden)]
    TX069,
    /// Reagan, TX
    #[doc(hidden)]
    TX070,
    /// Irion, TX
    #[doc(hidden)]
    TX071,
    /// Tom Green, TX
    #[doc(hidden)]
    TX072,
    /// Concho, TX
    #[doc(hidden)]
    TX073,
    /// Pecos, TX
    #[doc(hidden)]
    TX075,
    /// Crockett, TX
    #[doc(hidden)]
    TX076,
    /// Schleicher, TX
    #[doc(hidden)]
    TX077,
    /// Sutton, TX
    #[doc(hidden)]
    TX078,
    /// Terrell, TX
    #[doc(hidden)]
    TX082,
    /// Hardeman, TX
    #[doc(hidden)]
    TX083,
    /// Foard, TX
    #[doc(hidden)]
    TX084,
    /// Wilbarger, TX
    #[doc(hidden)]
    TX085,
    /// Wichita, TX
    #[doc(hidden)]
    TX086,
    /// Knox, TX
    #[doc(hidden)]
    TX087,
    /// Baylor, TX
    #[doc(hidden)]
    TX088,
    /// Archer, TX
    #[doc(hidden)]
    TX089,
    /// Clay, TX
    #[doc(hidden)]
    TX090,
    /// Montague, TX
    #[doc(hidden)]
    TX091,
    /// Cooke, TX
    #[doc(hidden)]
    TX092,
    /// Grayson, TX
    #[doc(hidden)]
    TX093,
    /// Fannin, TX
    #[doc(hidden)]
    TX094,
    /// Lamar, TX
    #[doc(hidden)]
    TX095,
    /// Red River, TX
    #[doc(hidden)]
    TX096,
    /// Bowie, TX
    #[doc(hidden)]
    TX097,
    /// Haskell, TX
    #[doc(hidden)]
    TX098,
    /// Throckmorton, TX
    #[doc(hidden)]
    TX099,
    /// Young, TX
    #[doc(hidden)]
    TX100,
    /// Jack, TX
    #[doc(hidden)]
    TX101,
    /// Wise, TX
    #[doc(hidden)]
    TX102,
    /// Denton, TX
    #[doc(hidden)]
    TX103,
    /// Collin, TX
    #[doc(hidden)]
    TX104,
    /// Hunt, TX
    #[doc(hidden)]
    TX105,
    /// Delta, TX
    #[doc(hidden)]
    TX106,
    /// Hopkins, TX
    #[doc(hidden)]
    TX107,
    /// Franklin, TX
    #[doc(hidden)]
    TX108,
    /// Titus, TX
    #[doc(hidden)]
    TX109,
    /// Camp, TX
    #[doc(hidden)]
    TX110,
    /// Morris, TX
    #[doc(hidden)]
    TX111,
    /// Cass, TX
    #[doc(hidden)]
    TX112,
    /// Jones, TX
    #[doc(hidden)]
    TX113,
    /// Shackelford, TX
    #[doc(hidden)]
    TX114,
    /// Stephens, TX
    #[doc(hidden)]
    TX115,
    /// Palo Pinto, TX
    #[doc(hidden)]
    TX116,
    /// Parker, TX
    #[doc(hidden)]
    TX117,
    /// Tarrant, TX
    #[doc(hidden)]
    TX118,
    /// Dallas, TX
    #[doc(hidden)]
    TX119,
    /// Rockwall, TX
    #[doc(hidden)]
    TX120,
    /// Kaufman, TX
    #[doc(hidden)]
    TX121,
    /// Van Zandt, TX
    #[doc(hidden)]
    TX122,
    /// Rains, TX
    #[doc(hidden)]
    TX123,
    /// Wood, TX
    #[doc(hidden)]
    TX124,
    /// Upshur, TX
    #[doc(hidden)]
    TX125,
    /// Marion, TX
    #[doc(hidden)]
    TX126,
    /// Taylor, TX
    #[doc(hidden)]
    TX127,
    /// Callahan, TX
    #[doc(hidden)]
    TX128,
    /// Eastland, TX
    #[doc(hidden)]
    TX129,
    /// Erath, TX
    #[doc(hidden)]
    TX130,
    /// Hood, TX
    #[doc(hidden)]
    TX131,
    /// Somervell, TX
    #[doc(hidden)]
    TX132,
    /// Johnson, TX
    #[doc(hidden)]
    TX133,
    /// Ellis, TX
    #[doc(hidden)]
    TX134,
    /// Henderson, TX
    #[doc(hidden)]
    TX135,
    /// Smith, TX
    #[doc(hidden)]
    TX136,
    /// Gregg, TX
    #[doc(hidden)]
    TX137,
    /// Harrison, TX
    #[doc(hidden)]
    TX138,
    /// Coleman, TX
    #[doc(hidden)]
    TX139,
    /// Brown, TX
    #[doc(hidden)]
    TX140,
    /// Comanche, TX
    #[doc(hidden)]
    TX141,
    /// Mills, TX
    #[doc(hidden)]
    TX142,
    /// Hamilton, TX
    #[doc(hidden)]
    TX143,
    /// Bosque, TX
    #[doc(hidden)]
    TX144,
    /// Hill, TX
    #[doc(hidden)]
    TX145,
    /// Navarro, TX
    #[doc(hidden)]
    TX146,
    /// Freestone, TX
    #[doc(hidden)]
    TX147,
    /// Anderson, TX
    #[doc(hidden)]
    TX148,
    /// Cherokee, TX
    #[doc(hidden)]
    TX149,
    /// Rusk, TX
    #[doc(hidden)]
    TX150,
    /// Panola, TX
    #[doc(hidden)]
    TX151,
    /// Nacogdoches, TX
    #[doc(hidden)]
    TX152,
    /// Shelby, TX
    #[doc(hidden)]
    TX153,
    /// McCulloch, TX
    #[doc(hidden)]
    TX154,
    /// San Saba, TX
    #[doc(hidden)]
    TX155,
    /// Lampasas, TX
    #[doc(hidden)]
    TX156,
    /// Coryell, TX
    #[doc(hidden)]
    TX157,
    /// Bell, TX
    #[doc(hidden)]
    TX158,
    /// McLennan, TX
    #[doc(hidden)]
    TX159,
    /// Falls, TX
    #[doc(hidden)]
    TX160,
    /// Limestone, TX
    #[doc(hidden)]
    TX161,
    /// Leon, TX
    #[doc(hidden)]
    TX162,
    /// Houston, TX
    #[doc(hidden)]
    TX163,
    /// Trinity, TX
    #[doc(hidden)]
    TX164,
    /// Angelina, TX
    #[doc(hidden)]
    TX165,
    /// San Augustine, TX
    #[doc(hidden)]
    TX166,
    /// Sabine, TX
    #[doc(hidden)]
    TX167,
    /// Menard, TX
    #[doc(hidden)]
    TX168,
    /// Kimble, TX
    #[doc(hidden)]
    TX169,
    /// Mason, TX
    #[doc(hidden)]
    TX170,
    /// Llano, TX
    #[doc(hidden)]
    TX171,
    /// Burnet, TX
    #[doc(hidden)]
    TX172,
    /// Williamson, TX
    #[doc(hidden)]
    TX173,
    /// Milam, TX
    #[doc(hidden)]
    TX174,
    /// Robertson, TX
    #[doc(hidden)]
    TX175,
    /// Madison, TX
    #[doc(hidden)]
    TX176,
    /// Walker, TX
    #[doc(hidden)]
    TX177,
    /// San Jacinto, TX
    #[doc(hidden)]
    TX178,
    /// Polk, TX
    #[doc(hidden)]
    TX179,
    /// Tyler, TX
    #[doc(hidden)]
    TX180,
    /// Val Verde, TX
    #[doc(hidden)]
    TX183,
    /// Edwards, TX
    #[doc(hidden)]
    TX184,
    /// Real, TX
    #[doc(hidden)]
    TX185,
    /// Kerr, TX
    #[doc(hidden)]
    TX186,
    /// Bandera, TX
    #[doc(hidden)]
    TX187,
    /// Gillespie, TX
    #[doc(hidden)]
    TX188,
    /// Kendall, TX
    #[doc(hidden)]
    TX189,
    /// Blanco, TX
    #[doc(hidden)]
    TX190,
    /// Hays, TX
    #[doc(hidden)]
    TX191,
    /// Travis, TX
    #[doc(hidden)]
    TX192,
    /// Bastrop, TX
    #[doc(hidden)]
    TX193,
    /// Lee, TX
    #[doc(hidden)]
    TX194,
    /// Burleson, TX
    #[doc(hidden)]
    TX195,
    /// Brazos, TX
    #[doc(hidden)]
    TX196,
    /// Washington, TX
    #[doc(hidden)]
    TX197,
    /// Grimes, TX
    #[doc(hidden)]
    TX198,
    /// Montgomery, TX
    #[doc(hidden)]
    TX199,
    /// Northern Liberty, TX
    #[doc(hidden)]
    TX200,
    /// Hardin, TX
    #[doc(hidden)]
    TX201,
    /// Kinney, TX
    #[doc(hidden)]
    TX202,
    /// Uvalde, TX
    #[doc(hidden)]
    TX203,
    /// Medina, TX
    #[doc(hidden)]
    TX204,
    /// Bexar, TX
    #[doc(hidden)]
    TX205,
    /// Comal, TX
    #[doc(hidden)]
    TX206,
    /// Guadalupe, TX
    #[doc(hidden)]
    TX207,
    /// Caldwell, TX
    #[doc(hidden)]
    TX208,
    /// Fayette, TX
    #[doc(hidden)]
    TX209,
    /// Colorado, TX
    #[doc(hidden)]
    TX210,
    /// Austin, TX
    #[doc(hidden)]
    TX211,
    /// Waller, TX
    #[doc(hidden)]
    TX212,
    /// Inland Harris, TX
    #[doc(hidden)]
    TX213,
    /// Chambers, TX
    #[doc(hidden)]
    TX214,
    /// Jefferson, TX
    #[doc(hidden)]
    TX215,
    /// Orange, TX
    #[doc(hidden)]
    TX216,
    /// Maverick, TX
    #[doc(hidden)]
    TX217,
    /// Zavala, TX
    #[doc(hidden)]
    TX218,
    /// Frio, TX
    #[doc(hidden)]
    TX219,
    /// Atascosa, TX
    #[doc(hidden)]
    TX220,
    /// Wilson, TX
    #[doc(hidden)]
    TX221,
    /// Karnes, TX
    #[doc(hidden)]
    TX222,
    /// Gonzales, TX
    #[doc(hidden)]
    TX223,
    /// De Witt, TX
    #[doc(hidden)]
    TX224,
    /// Lavaca, TX
    #[doc(hidden)]
    TX225,
    /// Wharton, TX
    #[doc(hidden)]
    TX226,
    /// Fort Bend, TX
    #[doc(hidden)]
    TX227,
    /// Dimmit, TX
    #[doc(hidden)]
    TX228,
    /// La Salle, TX
    #[doc(hidden)]
    TX229,
    /// McMullen, TX
    #[doc(hidden)]
    TX230,
    /// Live Oak, TX
    #[doc(hidden)]
    TX231,
    /// Bee, TX
    #[doc(hidden)]
    TX232,
    /// Goliad, TX
    #[doc(hidden)]
    TX233,
    /// Victoria, TX
    #[doc(hidden)]
    TX234,
    /// Inland Jackson, TX
    #[doc(hidden)]
    TX235,
    /// Inland Matagorda, TX
    #[doc(hidden)]
    TX236,
    /// Inland Brazoria, TX
    #[doc(hidden)]
    TX237,
    /// Inland Galveston, TX
    #[doc(hidden)]
    TX238,
    /// Webb, TX
    #[doc(hidden)]
    TX239,
    /// Duval, TX
    #[doc(hidden)]
    TX240,
    /// Jim Wells, TX
    #[doc(hidden)]
    TX241,
    /// Inland Kleberg, TX
    #[doc(hidden)]
    TX242,
    /// Inland Nueces, TX
    #[doc(hidden)]
    TX243,
    /// Inland San Patricio, TX
    #[doc(hidden)]
    TX244,
    /// Coastal Aransas, TX
    #[doc(hidden)]
    TX245,
    /// Inland Refugio, TX
    #[doc(hidden)]
    TX246,
    /// Inland Calhoun, TX
    #[doc(hidden)]
    TX247,
    /// Zapata, TX
    #[doc(hidden)]
    TX248,
    /// Jim Hogg, TX
    #[doc(hidden)]
    TX249,
    /// Brooks, TX
    #[doc(hidden)]
    TX250,
    /// Inland Kenedy, TX
    #[doc(hidden)]
    TX251,
    /// Starr, TX
    #[doc(hidden)]
    TX252,
    /// Southern Hidalgo, TX
    #[doc(hidden)]
    TX253,
    /// Inland Willacy, TX
    #[doc(hidden)]
    TX254,
    /// Inland Cameron, TX
    #[doc(hidden)]
    TX255,
    /// Northern Jasper, TX
    #[doc(hidden)]
    TX259,
    /// Northern Newton, TX
    #[doc(hidden)]
    TX260,
    /// Southern Jasper, TX
    #[doc(hidden)]
    TX261,
    /// Southern Newton, TX
    #[doc(hidden)]
    TX262,
    /// Guadalupe Mountains Above 7000 Feet, TX
    #[doc(hidden)]
    TX270,
    /// Guadalupe and Delaware Mountains, TX
    #[doc(hidden)]
    TX271,
    /// Van Horn and Highway 54 Corridor, TX
    #[doc(hidden)]
    TX272,
    /// Eastern Culberson County, TX
    #[doc(hidden)]
    TX273,
    /// Reeves County Plains, TX
    #[doc(hidden)]
    TX274,
    /// Chinati Mountains, TX
    #[doc(hidden)]
    TX275,
    /// Marfa Plateau, TX
    #[doc(hidden)]
    TX276,
    /// Davis Mountains, TX
    #[doc(hidden)]
    TX277,
    /// Davis Mountains Foothills, TX
    #[doc(hidden)]
    TX278,
    /// Central Brewster County, TX
    #[doc(hidden)]
    TX279,
    /// Chisos Basin, TX
    #[doc(hidden)]
    TX280,
    /// Presidio Valley, TX
    #[doc(hidden)]
    TX281,
    /// Lower Brewster County, TX
    #[doc(hidden)]
    TX282,
    /// Southern Liberty, TX
    #[doc(hidden)]
    TX300,
    /// Coastal Harris, TX
    #[doc(hidden)]
    TX313,
    /// Palo Duro Canyon, TX
    #[doc(hidden)]
    TX317,
    /// Coastal Jackson, TX
    #[doc(hidden)]
    TX335,
    /// Coastal Matagorda, TX
    #[doc(hidden)]
    TX336,
    /// Coastal Brazoria, TX
    #[doc(hidden)]
    TX337,
    /// Coastal Galveston, TX
    #[doc(hidden)]
    TX338,
    /// Coastal Kleberg, TX
    #[doc(hidden)]
    TX342,
    /// Coastal Nueces, TX
    #[doc(hidden)]
    TX343,
    /// Coastal San Patricio, TX
    #[doc(hidden)]
    TX344,
    /// Aransas Islands, TX
    #[doc(hidden)]
    TX345,
    /// Coastal Refugio, TX
    #[doc(hidden)]
    TX346,
    /// Coastal Calhoun, TX
    #[doc(hidden)]
    TX347,
    /// Coastal Kenedy, TX
    #[doc(hidden)]
    TX351,
    /// Northern Hidalgo, TX
    #[doc(hidden)]
    TX353,
    /// Coastal Willacy, TX
    #[doc(hidden)]
    TX354,
    /// Coastal Cameron, TX
    #[doc(hidden)]
    TX355,
    /// Western El Paso County, TX
    #[doc(hidden)]
    TX418,
    /// Eastern/Central El Paso County, TX
    #[doc(hidden)]
    TX419,
    /// Northern Hudspeth Highlands/Hueco Mountains, TX
    #[doc(hidden)]
    TX420,
    /// Salt Basin, TX
    #[doc(hidden)]
    TX421,
    /// Southern Hudspeth Highlands, TX
    #[doc(hidden)]
    TX422,
    /// Rio Grande Valley of Eastern El Paso/Western Hudspeth Counties, TX
    #[doc(hidden)]
    TX423,
    /// Rio Grande Valley of Eastern Hudspeth County, TX
    #[doc(hidden)]
    TX424,
    /// Matagorda Islands, TX
    #[doc(hidden)]
    TX436,
    /// Brazoria Islands, TX
    #[doc(hidden)]
    TX437,
    /// Galveston Island, TX
    #[doc(hidden)]
    TX438,
    /// Bolivar Peninsula, TX
    #[doc(hidden)]
    TX439,
    /// Kleberg Islands, TX
    #[doc(hidden)]
    TX442,
    /// Nueces Islands, TX
    #[doc(hidden)]
    TX443,
    /// Calhoun Islands, TX
    #[doc(hidden)]
    TX447,
    /// Kenedy Island, TX
    #[doc(hidden)]
    TX451,
    /// Willacy Island, TX
    #[doc(hidden)]
    TX454,
    /// Cameron Island, TX
    #[doc(hidden)]
    TX455,
    /// Southeast Utah, UT
    #[doc(hidden)]
    UT022,
    /// Eastern Uinta Mountains, UT
    #[doc(hidden)]
    UT023,
    /// Eastern Uinta Basin, UT
    #[doc(hidden)]
    UT024,
    /// Tavaputs Plateau, UT
    #[doc(hidden)]
    UT025,
    /// Arches/Grand Flat, UT
    #[doc(hidden)]
    UT027,
    /// La Sal and Abajo Mountains, UT
    #[doc(hidden)]
    UT028,
    /// Canyonlands/Natural Bridges, UT
    #[doc(hidden)]
    UT029,
    /// Great Salt Lake Desert and Mountains, UT
    #[doc(hidden)]
    UT101,
    /// Tooele and Rush Valleys, UT
    #[doc(hidden)]
    UT102,
    /// Eastern Box Elder County, UT
    #[doc(hidden)]
    UT103,
    /// Northern Wasatch Front, UT
    #[doc(hidden)]
    UT104,
    /// Salt Lake Valley, UT
    #[doc(hidden)]
    UT105,
    /// Utah Valley, UT
    #[doc(hidden)]
    UT106,
    /// Cache Valley/Utah Portion, UT
    #[doc(hidden)]
    UT107,
    /// Wasatch Back, UT
    #[doc(hidden)]
    UT108,
    /// Bear Lake and Bear River Valley, UT
    #[doc(hidden)]
    UT109,
    /// Wasatch Mountains I-80 North, UT
    #[doc(hidden)]
    UT110,
    /// Wasatch Mountains South of I-80, UT
    #[doc(hidden)]
    UT111,
    /// Western Uinta Mountains, UT
    #[doc(hidden)]
    UT112,
    /// Wasatch Plateau/Book Cliffs, UT
    #[doc(hidden)]
    UT113,
    /// Western Uinta Basin, UT
    #[doc(hidden)]
    UT114,
    /// Western Millard and Juab Counties, UT
    #[doc(hidden)]
    UT115,
    /// Eastern Juab/Millard Counties, UT
    #[doc(hidden)]
    UT116,
    /// Central Mountains, UT
    #[doc(hidden)]
    UT117,
    /// Sanpete Valley, UT
    #[doc(hidden)]
    UT118,
    /// Sevier Valley, UT
    #[doc(hidden)]
    UT119,
    /// Castle Country, UT
    #[doc(hidden)]
    UT120,
    /// San Rafael Swell, UT
    #[doc(hidden)]
    UT121,
    /// Southwest Utah, UT
    #[doc(hidden)]
    UT122,
    /// Lower Washington County, UT
    #[doc(hidden)]
    UT123,
    /// Zion National Park, UT
    #[doc(hidden)]
    UT124,
    /// Southern Mountains, UT
    #[doc(hidden)]
    UT125,
    /// Upper Sevier River Valleys, UT
    #[doc(hidden)]
    UT126,
    /// Bryce Canyon Country, UT
    #[doc(hidden)]
    UT127,
    /// South Central Utah, UT
    #[doc(hidden)]
    UT128,
    /// Capitol Reef National Park and Vicinity, UT
    #[doc(hidden)]
    UT129,
    /// Western Canyonlands, UT
    #[doc(hidden)]
    UT130,
    /// Glen Canyon Recreation Area/Lake Powell, UT
    #[doc(hidden)]
    UT131,
    /// Lee, VA
    #[doc(hidden)]
    VA001,
    /// Wise, VA
    #[doc(hidden)]
    VA002,
    /// Dickenson, VA
    #[doc(hidden)]
    VA003,
    /// Buchanan, VA
    #[doc(hidden)]
    VA004,
    /// Scott, VA
    #[doc(hidden)]
    VA005,
    /// Russell, VA
    #[doc(hidden)]
    VA006,
    /// Tazewell, VA
    #[doc(hidden)]
    VA007,
    /// Washington, VA
    #[doc(hidden)]
    VA008,
    /// Smyth, VA
    #[doc(hidden)]
    VA009,
    /// Bland, VA
    #[doc(hidden)]
    VA010,
    /// Giles, VA
    #[doc(hidden)]
    VA011,
    /// Wythe, VA
    #[doc(hidden)]
    VA012,
    /// Pulaski, VA
    #[doc(hidden)]
    VA013,
    /// Montgomery, VA
    #[doc(hidden)]
    VA014,
    /// Grayson, VA
    #[doc(hidden)]
    VA015,
    /// Carroll, VA
    #[doc(hidden)]
    VA016,
    /// Floyd, VA
    #[doc(hidden)]
    VA017,
    /// Craig, VA
    #[doc(hidden)]
    VA018,
    /// Alleghany, VA
    #[doc(hidden)]
    VA019,
    /// Bath, VA
    #[doc(hidden)]
    VA020,
    /// Roanoke, VA
    #[doc(hidden)]
    VA022,
    /// Botetourt, VA
    #[doc(hidden)]
    VA023,
    /// Rockbridge, VA
    #[doc(hidden)]
    VA024,
    /// Augusta, VA
    #[doc(hidden)]
    VA025,
    /// Rockingham, VA
    #[doc(hidden)]
    VA026,
    /// Shenandoah, VA
    #[doc(hidden)]
    VA027,
    /// Frederick, VA
    #[doc(hidden)]
    VA028,
    /// Page, VA
    #[doc(hidden)]
    VA029,
    /// Warren, VA
    #[doc(hidden)]
    VA030,
    /// Clarke, VA
    #[doc(hidden)]
    VA031,
    /// Patrick, VA
    #[doc(hidden)]
    VA032,
    /// Franklin, VA
    #[doc(hidden)]
    VA033,
    /// Bedford, VA
    #[doc(hidden)]
    VA034,
    /// Amherst, VA
    #[doc(hidden)]
    VA035,
    /// Nelson, VA
    #[doc(hidden)]
    VA036,
    /// Albemarle, VA
    #[doc(hidden)]
    VA037,
    /// Greene, VA
    #[doc(hidden)]
    VA038,
    /// Madison, VA
    #[doc(hidden)]
    VA039,
    /// Rappahannock, VA
    #[doc(hidden)]
    VA040,
    /// Henry, VA
    #[doc(hidden)]
    VA043,
    /// Pittsylvania, VA
    #[doc(hidden)]
    VA044,
    /// Campbell, VA
    #[doc(hidden)]
    VA045,
    /// Appomattox, VA
    #[doc(hidden)]
    VA046,
    /// Buckingham, VA
    #[doc(hidden)]
    VA047,
    /// Fluvanna, VA
    #[doc(hidden)]
    VA048,
    /// Orange, VA
    #[doc(hidden)]
    VA050,
    /// Culpeper, VA
    #[doc(hidden)]
    VA051,
    /// Prince William/Manassas/Manassas Park, VA
    #[doc(hidden)]
    VA052,
    /// Fairfax, VA
    #[doc(hidden)]
    VA053,
    /// Arlington/Falls Church/Alexandria, VA
    #[doc(hidden)]
    VA054,
    /// Stafford, VA
    #[doc(hidden)]
    VA055,
    /// Spotsylvania, VA
    #[doc(hidden)]
    VA056,
    /// King George, VA
    #[doc(hidden)]
    VA057,
    /// Halifax, VA
    #[doc(hidden)]
    VA058,
    /// Charlotte, VA
    #[doc(hidden)]
    VA059,
    /// Prince Edward, VA
    #[doc(hidden)]
    VA060,
    /// Cumberland, VA
    #[doc(hidden)]
    VA061,
    /// Goochland, VA
    #[doc(hidden)]
    VA062,
    /// Caroline, VA
    #[doc(hidden)]
    VA064,
    /// Mecklenburg, VA
    #[doc(hidden)]
    VA065,
    /// Lunenburg, VA
    #[doc(hidden)]
    VA066,
    /// Nottoway, VA
    #[doc(hidden)]
    VA067,
    /// Amelia, VA
    #[doc(hidden)]
    VA068,
    /// Powhatan, VA
    #[doc(hidden)]
    VA069,
    /// Westmoreland, VA
    #[doc(hidden)]
    VA075,
    /// Richmond, VA
    #[doc(hidden)]
    VA076,
    /// Northumberland, VA
    #[doc(hidden)]
    VA077,
    /// Lancaster, VA
    #[doc(hidden)]
    VA078,
    /// Brunswick, VA
    #[doc(hidden)]
    VA079,
    /// Dinwiddie, VA
    #[doc(hidden)]
    VA080,
    /// Prince George, VA
    #[doc(hidden)]
    VA081,
    /// Charles City, VA
    #[doc(hidden)]
    VA082,
    /// New Kent, VA
    #[doc(hidden)]
    VA083,
    /// Gloucester, VA
    #[doc(hidden)]
    VA084,
    /// Middlesex, VA
    #[doc(hidden)]
    VA085,
    /// Mathews, VA
    #[doc(hidden)]
    VA086,
    /// Greensville, VA
    #[doc(hidden)]
    VA087,
    /// Sussex, VA
    #[doc(hidden)]
    VA088,
    /// Surry, VA
    #[doc(hidden)]
    VA089,
    /// James City, VA
    #[doc(hidden)]
    VA090,
    /// Southampton, VA
    #[doc(hidden)]
    VA092,
    /// Isle of Wight, VA
    #[doc(hidden)]
    VA093,
    /// Norfolk/Portsmouth, VA
    #[doc(hidden)]
    VA095,
    /// Suffolk, VA
    #[doc(hidden)]
    VA096,
    /// Chesapeake, VA
    #[doc(hidden)]
    VA097,
    /// Virginia Beach, VA
    #[doc(hidden)]
    VA098,
    /// Accomack, VA
    #[doc(hidden)]
    VA099,
    /// Northampton, VA
    #[doc(hidden)]
    VA100,
    /// Northern Fauquier, VA
    #[doc(hidden)]
    VA501,
    /// Southern Fauquier, VA
    #[doc(hidden)]
    VA502,
    /// Western Highland, VA
    #[doc(hidden)]
    VA503,
    /// Eastern Highland, VA
    #[doc(hidden)]
    VA504,
    /// Western Loudoun, VA
    #[doc(hidden)]
    VA505,
    /// Eastern Loudoun, VA
    #[doc(hidden)]
    VA506,
    /// Northern Virginia Blue Ridge, VA
    #[doc(hidden)]
    VA507,
    /// Central Virginia Blue Ridge, VA
    #[doc(hidden)]
    VA508,
    /// Western Louisa, VA
    #[doc(hidden)]
    VA509,
    /// Eastern Louisa, VA
    #[doc(hidden)]
    VA510,
    /// Western Hanover, VA
    #[doc(hidden)]
    VA511,
    /// Eastern Hanover, VA
    #[doc(hidden)]
    VA512,
    /// Western Chesterfield, VA
    #[doc(hidden)]
    VA513,
    /// Eastern Chesterfield (Including Col. Heights), VA
    #[doc(hidden)]
    VA514,
    /// Western Henrico (Including the City of Richmond), VA
    #[doc(hidden)]
    VA515,
    /// Eastern Henrico, VA
    #[doc(hidden)]
    VA516,
    /// Western King William, VA
    #[doc(hidden)]
    VA517,
    /// Eastern King William, VA
    #[doc(hidden)]
    VA518,
    /// Western King and Queen, VA
    #[doc(hidden)]
    VA519,
    /// Eastern King and Queen, VA
    #[doc(hidden)]
    VA520,
    /// Western Essex, VA
    #[doc(hidden)]
    VA521,
    /// Eastern Essex, VA
    #[doc(hidden)]
    VA522,
    /// York, VA
    #[doc(hidden)]
    VA523,
    /// Newport News, VA
    #[doc(hidden)]
    VA524,
    /// Hampton/Poquoson, VA
    #[doc(hidden)]
    VA525,
    /// St.Thomas...St. John.. and Adjacent Islands, VI
    #[doc(hidden)]
    VI001,
    /// St Croix, VI
    #[doc(hidden)]
    VI002,
    /// Grand Isle, VT
    #[doc(hidden)]
    VT001,
    /// Western Franklin, VT
    #[doc(hidden)]
    VT002,
    /// Orleans, VT
    #[doc(hidden)]
    VT003,
    /// Essex, VT
    #[doc(hidden)]
    VT004,
    /// Western Chittenden, VT
    #[doc(hidden)]
    VT005,
    /// Lamoille, VT
    #[doc(hidden)]
    VT006,
    /// Caledonia, VT
    #[doc(hidden)]
    VT007,
    /// Washington, VT
    #[doc(hidden)]
    VT008,
    /// Western Addison, VT
    #[doc(hidden)]
    VT009,
    /// Orange, VT
    #[doc(hidden)]
    VT010,
    /// Western Rutland, VT
    #[doc(hidden)]
    VT011,
    /// Bennington, VT
    #[doc(hidden)]
    VT013,
    /// Western Windham, VT
    #[doc(hidden)]
    VT014,
    /// Eastern Windham, VT
    #[doc(hidden)]
    VT015,
    /// Eastern Franklin, VT
    #[doc(hidden)]
    VT016,
    /// Eastern Chittenden, VT
    #[doc(hidden)]
    VT017,
    /// Eastern Addison, VT
    #[doc(hidden)]
    VT018,
    /// Eastern Rutland, VT
    #[doc(hidden)]
    VT019,
    /// Western Windsor, VT
    #[doc(hidden)]
    VT020,
    /// Eastern Windsor, VT
    #[doc(hidden)]
    VT021,
    /// San Juan County, WA
    #[doc(hidden)]
    WA001,
    /// South Washington Cascades, WA
    #[doc(hidden)]
    WA019,
    /// Willapa Hills, WA
    #[doc(hidden)]
    WA020,
    /// South Washington Coast, WA
    #[doc(hidden)]
    WA021,
    /// Lower Columbia and I - 5 Corridor in Cowlitz County, WA
    #[doc(hidden)]
    WA022,
    /// Eastern Columbia River Gorge of Washington, WA
    #[doc(hidden)]
    WA024,
    /// Kittitas Valley, WA
    #[doc(hidden)]
    WA026,
    /// Yakima Valley, WA
    #[doc(hidden)]
    WA027,
    /// Lower Columbia Basin of Washington, WA
    #[doc(hidden)]
    WA028,
    /// Foothills of the Blue Mountains of Washington, WA
    #[doc(hidden)]
    WA029,
    /// Northwest Blue Mountains, WA
    #[doc(hidden)]
    WA030,
    /// Northeast Blue Mountains, WA
    #[doc(hidden)]
    WA031,
    /// Lower Garfield and Asotin Counties, WA
    #[doc(hidden)]
    WA032,
    /// Washington Palouse, WA
    #[doc(hidden)]
    WA033,
    /// Moses Lake Area, WA
    #[doc(hidden)]
    WA034,
    /// Upper Columbia Basin, WA
    #[doc(hidden)]
    WA035,
    /// Spokane Area, WA
    #[doc(hidden)]
    WA036,
    /// Northeast Mountains, WA
    #[doc(hidden)]
    WA037,
    /// Okanogan Highlands, WA
    #[doc(hidden)]
    WA038,
    /// Greater Vancouver Area, WA
    #[doc(hidden)]
    WA039,
    /// South Washington Cascade Foothills, WA
    #[doc(hidden)]
    WA040,
    /// Wenatchee Area, WA
    #[doc(hidden)]
    WA041,
    /// Okanogan Valley, WA
    #[doc(hidden)]
    WA043,
    /// Waterville Plateau, WA
    #[doc(hidden)]
    WA044,
    /// Western Columbia River Gorge, WA
    #[doc(hidden)]
    WA045,
    /// Central Columbia River Gorge, WA
    #[doc(hidden)]
    WA046,
    /// Central Chelan County, WA
    #[doc(hidden)]
    WA047,
    /// Western Chelan County, WA
    #[doc(hidden)]
    WA048,
    /// Western Okanogan County, WA
    #[doc(hidden)]
    WA049,
    /// Western Whatcom County, WA
    #[doc(hidden)]
    WA503,
    /// Southwest Interior, WA
    #[doc(hidden)]
    WA504,
    /// Western Skagit County, WA
    #[doc(hidden)]
    WA506,
    /// Everett and Vicinity, WA
    #[doc(hidden)]
    WA507,
    /// Tacoma Area, WA
    #[doc(hidden)]
    WA509,
    /// Admiralty Inlet Area, WA
    #[doc(hidden)]
    WA510,
    /// Hood Canal Area, WA
    #[doc(hidden)]
    WA511,
    /// Lower Chehalis Valley Area, WA
    #[doc(hidden)]
    WA512,
    /// Olympics, WA
    #[doc(hidden)]
    WA513,
    /// Eastern Strait of Juan de Fuca, WA
    #[doc(hidden)]
    WA514,
    /// Western Strait of Juan De Fuca, WA
    #[doc(hidden)]
    WA515,
    /// North Coast, WA
    #[doc(hidden)]
    WA516,
    /// Central Coast, WA
    #[doc(hidden)]
    WA517,
    /// East Slopes of the Washington Cascades, WA
    #[doc(hidden)]
    WA520,
    /// Simcoe Highlands, WA
    #[doc(hidden)]
    WA521,
    /// East Puget Sound Lowlands, WA
    #[doc(hidden)]
    WA555,
    /// Bellevue and Vicinity, WA
    #[doc(hidden)]
    WA556,
    /// Seattle and Vicinity, WA
    #[doc(hidden)]
    WA558,
    /// Bremerton and Vicinity, WA
    #[doc(hidden)]
    WA559,
    /// West Slopes North Cascades and Passes, WA
    #[doc(hidden)]
    WA567,
    /// West Slopes North Central Cascades and Passes, WA
    #[doc(hidden)]
    WA568,
    /// West Slopes South Central Cascades and Passes, WA
    #[doc(hidden)]
    WA569,
    /// Douglas, WI
    #[doc(hidden)]
    WI001,
    /// Bayfield, WI
    #[doc(hidden)]
    WI002,
    /// Ashland, WI
    #[doc(hidden)]
    WI003,
    /// Iron, WI
    #[doc(hidden)]
    WI004,
    /// Vilas, WI
    #[doc(hidden)]
    WI005,
    /// Burnett, WI
    #[doc(hidden)]
    WI006,
    /// Washburn, WI
    #[doc(hidden)]
    WI007,
    /// Sawyer, WI
    #[doc(hidden)]
    WI008,
    /// Price, WI
    #[doc(hidden)]
    WI009,
    /// Oneida, WI
    #[doc(hidden)]
    WI010,
    /// Forest, WI
    #[doc(hidden)]
    WI011,
    /// Florence, WI
    #[doc(hidden)]
    WI012,
    /// Northern Marinette County, WI
    #[doc(hidden)]
    WI013,
    /// Polk, WI
    #[doc(hidden)]
    WI014,
    /// Barron, WI
    #[doc(hidden)]
    WI015,
    /// Rusk, WI
    #[doc(hidden)]
    WI016,
    /// Taylor, WI
    #[doc(hidden)]
    WI017,
    /// Lincoln, WI
    #[doc(hidden)]
    WI018,
    /// Langlade, WI
    #[doc(hidden)]
    WI019,
    /// Menominee, WI
    #[doc(hidden)]
    WI020,
    /// Northern Oconto County, WI
    #[doc(hidden)]
    WI021,
    /// Door, WI
    #[doc(hidden)]
    WI022,
    /// St. Croix, WI
    #[doc(hidden)]
    WI023,
    /// Pierce, WI
    #[doc(hidden)]
    WI024,
    /// Dunn, WI
    #[doc(hidden)]
    WI025,
    /// Pepin, WI
    #[doc(hidden)]
    WI026,
    /// Chippewa, WI
    #[doc(hidden)]
    WI027,
    /// Eau Claire, WI
    #[doc(hidden)]
    WI028,
    /// Clark, WI
    #[doc(hidden)]
    WI029,
    /// Marathon, WI
    #[doc(hidden)]
    WI030,
    /// Shawano, WI
    #[doc(hidden)]
    WI031,
    /// Buffalo, WI
    #[doc(hidden)]
    WI032,
    /// Trempealeau, WI
    #[doc(hidden)]
    WI033,
    /// Jackson, WI
    #[doc(hidden)]
    WI034,
    /// Wood, WI
    #[doc(hidden)]
    WI035,
    /// Portage, WI
    #[doc(hidden)]
    WI036,
    /// Waupaca, WI
    #[doc(hidden)]
    WI037,
    /// Outagamie, WI
    #[doc(hidden)]
    WI038,
    /// Brown, WI
    #[doc(hidden)]
    WI039,
    /// Kewaunee, WI
    #[doc(hidden)]
    WI040,
    /// La Crosse, WI
    #[doc(hidden)]
    WI041,
    /// Monroe, WI
    #[doc(hidden)]
    WI042,
    /// Juneau, WI
    #[doc(hidden)]
    WI043,
    /// Adams, WI
    #[doc(hidden)]
    WI044,
    /// Waushara, WI
    #[doc(hidden)]
    WI045,
    /// Marquette, WI
    #[doc(hidden)]
    WI046,
    /// Green Lake, WI
    #[doc(hidden)]
    WI047,
    /// Winnebago, WI
    #[doc(hidden)]
    WI048,
    /// Calumet, WI
    #[doc(hidden)]
    WI049,
    /// Manitowoc, WI
    #[doc(hidden)]
    WI050,
    /// Fond Du Lac, WI
    #[doc(hidden)]
    WI051,
    /// Sheboygan, WI
    #[doc(hidden)]
    WI052,
    /// Vernon, WI
    #[doc(hidden)]
    WI053,
    /// Crawford, WI
    #[doc(hidden)]
    WI054,
    /// Richland, WI
    #[doc(hidden)]
    WI055,
    /// Sauk, WI
    #[doc(hidden)]
    WI056,
    /// Columbia, WI
    #[doc(hidden)]
    WI057,
    /// Dodge, WI
    #[doc(hidden)]
    WI058,
    /// Washington, WI
    #[doc(hidden)]
    WI059,
    /// Ozaukee, WI
    #[doc(hidden)]
    WI060,
    /// Grant, WI
    #[doc(hidden)]
    WI061,
    /// Iowa, WI
    #[doc(hidden)]
    WI062,
    /// Dane, WI
    #[doc(hidden)]
    WI063,
    /// Jefferson, WI
    #[doc(hidden)]
    WI064,
    /// Waukesha, WI
    #[doc(hidden)]
    WI065,
    /// Milwaukee, WI
    #[doc(hidden)]
    WI066,
    /// Lafayette, WI
    #[doc(hidden)]
    WI067,
    /// Green, WI
    #[doc(hidden)]
    WI068,
    /// Rock, WI
    #[doc(hidden)]
    WI069,
    /// Walworth, WI
    #[doc(hidden)]
    WI070,
    /// Racine, WI
    #[doc(hidden)]
    WI071,
    /// Kenosha, WI
    #[doc(hidden)]
    WI072,
    /// Southern Marinette County, WI
    #[doc(hidden)]
    WI073,
    /// Southern Oconto County, WI
    #[doc(hidden)]
    WI074,
    /// Hancock, WV
    #[doc(hidden)]
    WV001,
    /// Brooke, WV
    #[doc(hidden)]
    WV002,
    /// Ohio, WV
    #[doc(hidden)]
    WV003,
    /// Marshall, WV
    #[doc(hidden)]
    WV004,
    /// Wayne, WV
    #[doc(hidden)]
    WV005,
    /// Cabell, WV
    #[doc(hidden)]
    WV006,
    /// Mason, WV
    #[doc(hidden)]
    WV007,
    /// Jackson, WV
    #[doc(hidden)]
    WV008,
    /// Wood, WV
    #[doc(hidden)]
    WV009,
    /// Pleasants, WV
    #[doc(hidden)]
    WV010,
    /// Tyler, WV
    #[doc(hidden)]
    WV011,
    /// Wetzel, WV
    #[doc(hidden)]
    WV012,
    /// Lincoln, WV
    #[doc(hidden)]
    WV013,
    /// Putnam, WV
    #[doc(hidden)]
    WV014,
    /// Kanawha, WV
    #[doc(hidden)]
    WV015,
    /// Roane, WV
    #[doc(hidden)]
    WV016,
    /// Wirt, WV
    #[doc(hidden)]
    WV017,
    /// Calhoun, WV
    #[doc(hidden)]
    WV018,
    /// Ritchie, WV
    #[doc(hidden)]
    WV019,
    /// Doddridge, WV
    #[doc(hidden)]
    WV020,
    /// Marion, WV
    #[doc(hidden)]
    WV021,
    /// Mingo, WV
    #[doc(hidden)]
    WV024,
    /// Logan, WV
    #[doc(hidden)]
    WV025,
    /// Boone, WV
    #[doc(hidden)]
    WV026,
    /// Clay, WV
    #[doc(hidden)]
    WV027,
    /// Braxton, WV
    #[doc(hidden)]
    WV028,
    /// Gilmer, WV
    #[doc(hidden)]
    WV029,
    /// Lewis, WV
    #[doc(hidden)]
    WV030,
    /// Harrison, WV
    #[doc(hidden)]
    WV031,
    /// Taylor, WV
    #[doc(hidden)]
    WV032,
    /// McDowell, WV
    #[doc(hidden)]
    WV033,
    /// Wyoming, WV
    #[doc(hidden)]
    WV034,
    /// Upshur, WV
    #[doc(hidden)]
    WV039,
    /// Barbour, WV
    #[doc(hidden)]
    WV040,
    /// Mercer, WV
    #[doc(hidden)]
    WV042,
    /// Summers, WV
    #[doc(hidden)]
    WV043,
    /// Monroe, WV
    #[doc(hidden)]
    WV044,
    /// Hampshire, WV
    #[doc(hidden)]
    WV050,
    /// Morgan, WV
    #[doc(hidden)]
    WV051,
    /// Berkeley, WV
    #[doc(hidden)]
    WV052,
    /// Jefferson, WV
    #[doc(hidden)]
    WV053,
    /// Hardy, WV
    #[doc(hidden)]
    WV055,
    /// Western Grant, WV
    #[doc(hidden)]
    WV501,
    /// Eastern Grant, WV
    #[doc(hidden)]
    WV502,
    /// Western Mineral, WV
    #[doc(hidden)]
    WV503,
    /// Eastern Mineral, WV
    #[doc(hidden)]
    WV504,
    /// Western Pendleton, WV
    #[doc(hidden)]
    WV505,
    /// Eastern Pendleton, WV
    #[doc(hidden)]
    WV506,
    /// Eastern Greenbrier, WV
    #[doc(hidden)]
    WV507,
    /// Western Greenbrier, WV
    #[doc(hidden)]
    WV508,
    /// Monongalia, WV
    #[doc(hidden)]
    WV509,
    /// Ridges of Eastern Monongalia and Northwestern Preston, WV
    #[doc(hidden)]
    WV510,
    /// Preston, WV
    #[doc(hidden)]
    WV511,
    /// Eastern Preston, WV
    #[doc(hidden)]
    WV512,
    /// Western Tucker, WV
    #[doc(hidden)]
    WV513,
    /// Eastern Tucker, WV
    #[doc(hidden)]
    WV514,
    /// Northwest Raleigh, WV
    #[doc(hidden)]
    WV515,
    /// Southeast Raleigh, WV
    #[doc(hidden)]
    WV516,
    /// Northwest Fayette, WV
    #[doc(hidden)]
    WV517,
    /// Southeast Fayette, WV
    #[doc(hidden)]
    WV518,
    /// Northwest Nicholas, WV
    #[doc(hidden)]
    WV519,
    /// Southeast Nicholas, WV
    #[doc(hidden)]
    WV520,
    /// Northwest Webster, WV
    #[doc(hidden)]
    WV521,
    /// Southeast Webster, WV
    #[doc(hidden)]
    WV522,
    /// Northwest Pocahontas, WV
    #[doc(hidden)]
    WV523,
    /// Southeast Pocahontas, WV
    #[doc(hidden)]
    WV524,
    /// Northwest Randolph, WV
    #[doc(hidden)]
    WV525,
    /// Southeast Randolph, WV
    #[doc(hidden)]
    WV526,
    /// Yellowstone National Park, WY
    #[doc(hidden)]
    WY001,
    /// Absaroka Mountains, WY
    #[doc(hidden)]
    WY002,
    /// Cody Foothills, WY
    #[doc(hidden)]
    WY003,
    /// North Big Horn Basin, WY
    #[doc(hidden)]
    WY004,
    /// Southwest Big Horn Basin, WY
    #[doc(hidden)]
    WY005,
    /// Southeast Big Horn Basin, WY
    #[doc(hidden)]
    WY006,
    /// Owl Creek and Bridger Mountains, WY
    #[doc(hidden)]
    WY007,
    /// Bighorn Mountains West, WY
    #[doc(hidden)]
    WY008,
    /// Bighorn Mountains Southeast, WY
    #[doc(hidden)]
    WY009,
    /// Northeast Johnson County, WY
    #[doc(hidden)]
    WY010,
    /// Southeast Johnson County, WY
    #[doc(hidden)]
    WY011,
    /// Teton and Gros Ventre Mountains, WY
    #[doc(hidden)]
    WY012,
    /// Jackson Hole, WY
    #[doc(hidden)]
    WY013,
    /// Wind River Mountains West, WY
    #[doc(hidden)]
    WY014,
    /// Wind River Mountains East, WY
    #[doc(hidden)]
    WY015,
    /// Upper Wind River Basin, WY
    #[doc(hidden)]
    WY016,
    /// Wind River Basin, WY
    #[doc(hidden)]
    WY017,
    /// Lander Foothills, WY
    #[doc(hidden)]
    WY018,
    /// Green Mountains and Rattlesnake Range, WY
    #[doc(hidden)]
    WY019,
    /// Natrona County Lower Elevations, WY
    #[doc(hidden)]
    WY020,
    /// Southwest Wyoming, WY
    #[doc(hidden)]
    WY021,
    /// Casper Mountain, WY
    #[doc(hidden)]
    WY022,
    /// Star Valley, WY
    #[doc(hidden)]
    WY023,
    /// Salt River and Wyoming Ranges, WY
    #[doc(hidden)]
    WY024,
    /// Upper Green River Basin Foothills, WY
    #[doc(hidden)]
    WY025,
    /// Upper Green River Basin, WY
    #[doc(hidden)]
    WY026,
    /// South Lincoln County, WY
    #[doc(hidden)]
    WY027,
    /// Rock Springs and Green River, WY
    #[doc(hidden)]
    WY028,
    /// Flaming Gorge, WY
    #[doc(hidden)]
    WY029,
    /// East Sweetwater County, WY
    #[doc(hidden)]
    WY030,
    /// Northern Campbell, WY
    #[doc(hidden)]
    WY054,
    /// Southern Campbell, WY
    #[doc(hidden)]
    WY055,
    /// Western Crook, WY
    #[doc(hidden)]
    WY056,
    /// Wyoming Black Hills, WY
    #[doc(hidden)]
    WY057,
    /// Weston, WY
    #[doc(hidden)]
    WY058,
    /// Northeastern Crook, WY
    #[doc(hidden)]
    WY071,
    /// Converse County Lower Elevations, WY
    #[doc(hidden)]
    WY101,
    /// Niobrara County, WY
    #[doc(hidden)]
    WY102,
    /// North Laramie Range, WY
    #[doc(hidden)]
    WY103,
    /// Ferris/Seminoe/Shirley Mountains, WY
    #[doc(hidden)]
    WY104,
    /// Shirley Basin, WY
    #[doc(hidden)]
    WY105,
    /// Central Laramie Range and Southwest Platte County, WY
    #[doc(hidden)]
    WY106,
    /// East Platte County, WY
    #[doc(hidden)]
    WY107,
    /// Goshen County, WY
    #[doc(hidden)]
    WY108,
    /// Central Carbon County, WY
    #[doc(hidden)]
    WY109,
    /// North Snowy Range Foothills, WY
    #[doc(hidden)]
    WY110,
    /// Southwest Carbon County, WY
    #[doc(hidden)]
    WY111,
    /// Sierra Madre Range, WY
    #[doc(hidden)]
    WY112,
    /// Upper North Platte River Basin, WY
    #[doc(hidden)]
    WY113,
    /// Snowy Range, WY
    #[doc(hidden)]
    WY114,
    /// Laramie Valley, WY
    #[doc(hidden)]
    WY115,
    /// South Laramie Range, WY
    #[doc(hidden)]
    WY116,
    /// South Laramie Range Foothills, WY
    #[doc(hidden)]
    WY117,
    /// Central Laramie County, WY
    #[doc(hidden)]
    WY118,
    /// East Laramie County, WY
    #[doc(hidden)]
    WY119,
    /// Northeast Bighorn Mountains, WY
    #[doc(hidden)]
    WY198,
    /// Sheridan Foothills, WY
    #[doc(hidden)]
    WY199,
}
impl ::std::str::FromStr for ForecastZone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "AK017" => Ok(ForecastZone::AK017),
            "AK018" => Ok(ForecastZone::AK018),
            "AK019" => Ok(ForecastZone::AK019),
            "AK020" => Ok(ForecastZone::AK020),
            "AK021" => Ok(ForecastZone::AK021),
            "AK022" => Ok(ForecastZone::AK022),
            "AK023" => Ok(ForecastZone::AK023),
            "AK024" => Ok(ForecastZone::AK024),
            "AK025" => Ok(ForecastZone::AK025),
            "AK026" => Ok(ForecastZone::AK026),
            "AK027" => Ok(ForecastZone::AK027),
            "AK028" => Ok(ForecastZone::AK028),
            "AK029" => Ok(ForecastZone::AK029),
            "AK101" => Ok(ForecastZone::AK101),
            "AK111" => Ok(ForecastZone::AK111),
            "AK121" => Ok(ForecastZone::AK121),
            "AK125" => Ok(ForecastZone::AK125),
            "AK131" => Ok(ForecastZone::AK131),
            "AK135" => Ok(ForecastZone::AK135),
            "AK141" => Ok(ForecastZone::AK141),
            "AK145" => Ok(ForecastZone::AK145),
            "AK152" => Ok(ForecastZone::AK152),
            "AK155" => Ok(ForecastZone::AK155),
            "AK161" => Ok(ForecastZone::AK161),
            "AK171" => Ok(ForecastZone::AK171),
            "AK181" => Ok(ForecastZone::AK181),
            "AK185" => Ok(ForecastZone::AK185),
            "AK187" => Ok(ForecastZone::AK187),
            "AK191" => Ok(ForecastZone::AK191),
            "AK195" => Ok(ForecastZone::AK195),
            "AK201" => Ok(ForecastZone::AK201),
            "AK202" => Ok(ForecastZone::AK202),
            "AK203" => Ok(ForecastZone::AK203),
            "AK204" => Ok(ForecastZone::AK204),
            "AK205" => Ok(ForecastZone::AK205),
            "AK206" => Ok(ForecastZone::AK206),
            "AK207" => Ok(ForecastZone::AK207),
            "AK208" => Ok(ForecastZone::AK208),
            "AK209" => Ok(ForecastZone::AK209),
            "AK210" => Ok(ForecastZone::AK210),
            "AK211" => Ok(ForecastZone::AK211),
            "AK212" => Ok(ForecastZone::AK212),
            "AK213" => Ok(ForecastZone::AK213),
            "AK214" => Ok(ForecastZone::AK214),
            "AK215" => Ok(ForecastZone::AK215),
            "AK216" => Ok(ForecastZone::AK216),
            "AK217" => Ok(ForecastZone::AK217),
            "AK218" => Ok(ForecastZone::AK218),
            "AK219" => Ok(ForecastZone::AK219),
            "AK220" => Ok(ForecastZone::AK220),
            "AK221" => Ok(ForecastZone::AK221),
            "AK222" => Ok(ForecastZone::AK222),
            "AK223" => Ok(ForecastZone::AK223),
            "AK224" => Ok(ForecastZone::AK224),
            "AK225" => Ok(ForecastZone::AK225),
            "AK226" => Ok(ForecastZone::AK226),
            "AK227" => Ok(ForecastZone::AK227),
            "AL001" => Ok(ForecastZone::AL001),
            "AL002" => Ok(ForecastZone::AL002),
            "AL003" => Ok(ForecastZone::AL003),
            "AL004" => Ok(ForecastZone::AL004),
            "AL005" => Ok(ForecastZone::AL005),
            "AL006" => Ok(ForecastZone::AL006),
            "AL007" => Ok(ForecastZone::AL007),
            "AL008" => Ok(ForecastZone::AL008),
            "AL009" => Ok(ForecastZone::AL009),
            "AL010" => Ok(ForecastZone::AL010),
            "AL011" => Ok(ForecastZone::AL011),
            "AL012" => Ok(ForecastZone::AL012),
            "AL013" => Ok(ForecastZone::AL013),
            "AL014" => Ok(ForecastZone::AL014),
            "AL015" => Ok(ForecastZone::AL015),
            "AL016" => Ok(ForecastZone::AL016),
            "AL017" => Ok(ForecastZone::AL017),
            "AL018" => Ok(ForecastZone::AL018),
            "AL019" => Ok(ForecastZone::AL019),
            "AL020" => Ok(ForecastZone::AL020),
            "AL021" => Ok(ForecastZone::AL021),
            "AL022" => Ok(ForecastZone::AL022),
            "AL023" => Ok(ForecastZone::AL023),
            "AL024" => Ok(ForecastZone::AL024),
            "AL025" => Ok(ForecastZone::AL025),
            "AL026" => Ok(ForecastZone::AL026),
            "AL027" => Ok(ForecastZone::AL027),
            "AL028" => Ok(ForecastZone::AL028),
            "AL029" => Ok(ForecastZone::AL029),
            "AL030" => Ok(ForecastZone::AL030),
            "AL031" => Ok(ForecastZone::AL031),
            "AL032" => Ok(ForecastZone::AL032),
            "AL033" => Ok(ForecastZone::AL033),
            "AL034" => Ok(ForecastZone::AL034),
            "AL035" => Ok(ForecastZone::AL035),
            "AL036" => Ok(ForecastZone::AL036),
            "AL037" => Ok(ForecastZone::AL037),
            "AL038" => Ok(ForecastZone::AL038),
            "AL039" => Ok(ForecastZone::AL039),
            "AL040" => Ok(ForecastZone::AL040),
            "AL041" => Ok(ForecastZone::AL041),
            "AL042" => Ok(ForecastZone::AL042),
            "AL043" => Ok(ForecastZone::AL043),
            "AL044" => Ok(ForecastZone::AL044),
            "AL045" => Ok(ForecastZone::AL045),
            "AL046" => Ok(ForecastZone::AL046),
            "AL047" => Ok(ForecastZone::AL047),
            "AL048" => Ok(ForecastZone::AL048),
            "AL049" => Ok(ForecastZone::AL049),
            "AL050" => Ok(ForecastZone::AL050),
            "AL051" => Ok(ForecastZone::AL051),
            "AL052" => Ok(ForecastZone::AL052),
            "AL053" => Ok(ForecastZone::AL053),
            "AL054" => Ok(ForecastZone::AL054),
            "AL055" => Ok(ForecastZone::AL055),
            "AL056" => Ok(ForecastZone::AL056),
            "AL057" => Ok(ForecastZone::AL057),
            "AL058" => Ok(ForecastZone::AL058),
            "AL059" => Ok(ForecastZone::AL059),
            "AL060" => Ok(ForecastZone::AL060),
            "AL065" => Ok(ForecastZone::AL065),
            "AL066" => Ok(ForecastZone::AL066),
            "AL067" => Ok(ForecastZone::AL067),
            "AL068" => Ok(ForecastZone::AL068),
            "AL069" => Ok(ForecastZone::AL069),
            "AL261" => Ok(ForecastZone::AL261),
            "AL262" => Ok(ForecastZone::AL262),
            "AL263" => Ok(ForecastZone::AL263),
            "AL264" => Ok(ForecastZone::AL264),
            "AL265" => Ok(ForecastZone::AL265),
            "AL266" => Ok(ForecastZone::AL266),
            "AR001" => Ok(ForecastZone::AR001),
            "AR002" => Ok(ForecastZone::AR002),
            "AR004" => Ok(ForecastZone::AR004),
            "AR005" => Ok(ForecastZone::AR005),
            "AR006" => Ok(ForecastZone::AR006),
            "AR007" => Ok(ForecastZone::AR007),
            "AR008" => Ok(ForecastZone::AR008),
            "AR009" => Ok(ForecastZone::AR009),
            "AR010" => Ok(ForecastZone::AR010),
            "AR011" => Ok(ForecastZone::AR011),
            "AR014" => Ok(ForecastZone::AR014),
            "AR015" => Ok(ForecastZone::AR015),
            "AR016" => Ok(ForecastZone::AR016),
            "AR017" => Ok(ForecastZone::AR017),
            "AR018" => Ok(ForecastZone::AR018),
            "AR019" => Ok(ForecastZone::AR019),
            "AR020" => Ok(ForecastZone::AR020),
            "AR024" => Ok(ForecastZone::AR024),
            "AR025" => Ok(ForecastZone::AR025),
            "AR026" => Ok(ForecastZone::AR026),
            "AR027" => Ok(ForecastZone::AR027),
            "AR028" => Ok(ForecastZone::AR028),
            "AR029" => Ok(ForecastZone::AR029),
            "AR031" => Ok(ForecastZone::AR031),
            "AR032" => Ok(ForecastZone::AR032),
            "AR033" => Ok(ForecastZone::AR033),
            "AR034" => Ok(ForecastZone::AR034),
            "AR035" => Ok(ForecastZone::AR035),
            "AR036" => Ok(ForecastZone::AR036),
            "AR039" => Ok(ForecastZone::AR039),
            "AR042" => Ok(ForecastZone::AR042),
            "AR043" => Ok(ForecastZone::AR043),
            "AR044" => Ok(ForecastZone::AR044),
            "AR045" => Ok(ForecastZone::AR045),
            "AR046" => Ok(ForecastZone::AR046),
            "AR047" => Ok(ForecastZone::AR047),
            "AR048" => Ok(ForecastZone::AR048),
            "AR049" => Ok(ForecastZone::AR049),
            "AR050" => Ok(ForecastZone::AR050),
            "AR051" => Ok(ForecastZone::AR051),
            "AR052" => Ok(ForecastZone::AR052),
            "AR053" => Ok(ForecastZone::AR053),
            "AR054" => Ok(ForecastZone::AR054),
            "AR055" => Ok(ForecastZone::AR055),
            "AR056" => Ok(ForecastZone::AR056),
            "AR057" => Ok(ForecastZone::AR057),
            "AR058" => Ok(ForecastZone::AR058),
            "AR059" => Ok(ForecastZone::AR059),
            "AR060" => Ok(ForecastZone::AR060),
            "AR061" => Ok(ForecastZone::AR061),
            "AR062" => Ok(ForecastZone::AR062),
            "AR063" => Ok(ForecastZone::AR063),
            "AR064" => Ok(ForecastZone::AR064),
            "AR065" => Ok(ForecastZone::AR065),
            "AR066" => Ok(ForecastZone::AR066),
            "AR067" => Ok(ForecastZone::AR067),
            "AR068" => Ok(ForecastZone::AR068),
            "AR069" => Ok(ForecastZone::AR069),
            "AR070" => Ok(ForecastZone::AR070),
            "AR071" => Ok(ForecastZone::AR071),
            "AR072" => Ok(ForecastZone::AR072),
            "AR073" => Ok(ForecastZone::AR073),
            "AR074" => Ok(ForecastZone::AR074),
            "AR075" => Ok(ForecastZone::AR075),
            "AR103" => Ok(ForecastZone::AR103),
            "AR112" => Ok(ForecastZone::AR112),
            "AR113" => Ok(ForecastZone::AR113),
            "AR121" => Ok(ForecastZone::AR121),
            "AR122" => Ok(ForecastZone::AR122),
            "AR123" => Ok(ForecastZone::AR123),
            "AR130" => Ok(ForecastZone::AR130),
            "AR137" => Ok(ForecastZone::AR137),
            "AR138" => Ok(ForecastZone::AR138),
            "AR140" => Ok(ForecastZone::AR140),
            "AR141" => Ok(ForecastZone::AR141),
            "AR203" => Ok(ForecastZone::AR203),
            "AR212" => Ok(ForecastZone::AR212),
            "AR213" => Ok(ForecastZone::AR213),
            "AR221" => Ok(ForecastZone::AR221),
            "AR222" => Ok(ForecastZone::AR222),
            "AR223" => Ok(ForecastZone::AR223),
            "AR230" => Ok(ForecastZone::AR230),
            "AR237" => Ok(ForecastZone::AR237),
            "AR238" => Ok(ForecastZone::AR238),
            "AR240" => Ok(ForecastZone::AR240),
            "AR241" => Ok(ForecastZone::AR241),
            "AR313" => Ok(ForecastZone::AR313),
            "AR340" => Ok(ForecastZone::AR340),
            "AR341" => Ok(ForecastZone::AR341),
            "AS001" => Ok(ForecastZone::AS001),
            "AS002" => Ok(ForecastZone::AS002),
            "AS003" => Ok(ForecastZone::AS003),
            "AZ001" => Ok(ForecastZone::AZ001),
            "AZ002" => Ok(ForecastZone::AZ002),
            "AZ003" => Ok(ForecastZone::AZ003),
            "AZ004" => Ok(ForecastZone::AZ004),
            "AZ005" => Ok(ForecastZone::AZ005),
            "AZ006" => Ok(ForecastZone::AZ006),
            "AZ007" => Ok(ForecastZone::AZ007),
            "AZ008" => Ok(ForecastZone::AZ008),
            "AZ009" => Ok(ForecastZone::AZ009),
            "AZ010" => Ok(ForecastZone::AZ010),
            "AZ011" => Ok(ForecastZone::AZ011),
            "AZ012" => Ok(ForecastZone::AZ012),
            "AZ013" => Ok(ForecastZone::AZ013),
            "AZ014" => Ok(ForecastZone::AZ014),
            "AZ015" => Ok(ForecastZone::AZ015),
            "AZ016" => Ok(ForecastZone::AZ016),
            "AZ017" => Ok(ForecastZone::AZ017),
            "AZ018" => Ok(ForecastZone::AZ018),
            "AZ036" => Ok(ForecastZone::AZ036),
            "AZ037" => Ok(ForecastZone::AZ037),
            "AZ038" => Ok(ForecastZone::AZ038),
            "AZ039" => Ok(ForecastZone::AZ039),
            "AZ040" => Ok(ForecastZone::AZ040),
            "AZ501" => Ok(ForecastZone::AZ501),
            "AZ502" => Ok(ForecastZone::AZ502),
            "AZ503" => Ok(ForecastZone::AZ503),
            "AZ504" => Ok(ForecastZone::AZ504),
            "AZ505" => Ok(ForecastZone::AZ505),
            "AZ506" => Ok(ForecastZone::AZ506),
            "AZ507" => Ok(ForecastZone::AZ507),
            "AZ508" => Ok(ForecastZone::AZ508),
            "AZ509" => Ok(ForecastZone::AZ509),
            "AZ510" => Ok(ForecastZone::AZ510),
            "AZ511" => Ok(ForecastZone::AZ511),
            "AZ512" => Ok(ForecastZone::AZ512),
            "AZ513" => Ok(ForecastZone::AZ513),
            "AZ514" => Ok(ForecastZone::AZ514),
            "AZ515" => Ok(ForecastZone::AZ515),
            "AZ530" => Ok(ForecastZone::AZ530),
            "AZ531" => Ok(ForecastZone::AZ531),
            "AZ532" => Ok(ForecastZone::AZ532),
            "AZ533" => Ok(ForecastZone::AZ533),
            "AZ534" => Ok(ForecastZone::AZ534),
            "AZ535" => Ok(ForecastZone::AZ535),
            "AZ536" => Ok(ForecastZone::AZ536),
            "AZ537" => Ok(ForecastZone::AZ537),
            "AZ538" => Ok(ForecastZone::AZ538),
            "AZ539" => Ok(ForecastZone::AZ539),
            "AZ540" => Ok(ForecastZone::AZ540),
            "AZ541" => Ok(ForecastZone::AZ541),
            "AZ542" => Ok(ForecastZone::AZ542),
            "AZ543" => Ok(ForecastZone::AZ543),
            "AZ544" => Ok(ForecastZone::AZ544),
            "AZ545" => Ok(ForecastZone::AZ545),
            "AZ546" => Ok(ForecastZone::AZ546),
            "AZ547" => Ok(ForecastZone::AZ547),
            "AZ548" => Ok(ForecastZone::AZ548),
            "AZ549" => Ok(ForecastZone::AZ549),
            "AZ550" => Ok(ForecastZone::AZ550),
            "AZ551" => Ok(ForecastZone::AZ551),
            "AZ552" => Ok(ForecastZone::AZ552),
            "AZ553" => Ok(ForecastZone::AZ553),
            "AZ554" => Ok(ForecastZone::AZ554),
            "AZ555" => Ok(ForecastZone::AZ555),
            "AZ556" => Ok(ForecastZone::AZ556),
            "AZ557" => Ok(ForecastZone::AZ557),
            "AZ558" => Ok(ForecastZone::AZ558),
            "AZ559" => Ok(ForecastZone::AZ559),
            "AZ560" => Ok(ForecastZone::AZ560),
            "AZ561" => Ok(ForecastZone::AZ561),
            "AZ562" => Ok(ForecastZone::AZ562),
            "AZ563" => Ok(ForecastZone::AZ563),
            "CA006" => Ok(ForecastZone::CA006),
            "CA013" => Ok(ForecastZone::CA013),
            "CA014" => Ok(ForecastZone::CA014),
            "CA015" => Ok(ForecastZone::CA015),
            "CA016" => Ok(ForecastZone::CA016),
            "CA017" => Ok(ForecastZone::CA017),
            "CA018" => Ok(ForecastZone::CA018),
            "CA019" => Ok(ForecastZone::CA019),
            "CA038" => Ok(ForecastZone::CA038),
            "CA043" => Ok(ForecastZone::CA043),
            "CA048" => Ok(ForecastZone::CA048),
            "CA050" => Ok(ForecastZone::CA050),
            "CA053" => Ok(ForecastZone::CA053),
            "CA054" => Ok(ForecastZone::CA054),
            "CA055" => Ok(ForecastZone::CA055),
            "CA056" => Ok(ForecastZone::CA056),
            "CA057" => Ok(ForecastZone::CA057),
            "CA058" => Ok(ForecastZone::CA058),
            "CA059" => Ok(ForecastZone::CA059),
            "CA060" => Ok(ForecastZone::CA060),
            "CA061" => Ok(ForecastZone::CA061),
            "CA062" => Ok(ForecastZone::CA062),
            "CA063" => Ok(ForecastZone::CA063),
            "CA065" => Ok(ForecastZone::CA065),
            "CA066" => Ok(ForecastZone::CA066),
            "CA067" => Ok(ForecastZone::CA067),
            "CA068" => Ok(ForecastZone::CA068),
            "CA069" => Ok(ForecastZone::CA069),
            "CA070" => Ok(ForecastZone::CA070),
            "CA071" => Ok(ForecastZone::CA071),
            "CA072" => Ok(ForecastZone::CA072),
            "CA073" => Ok(ForecastZone::CA073),
            "CA080" => Ok(ForecastZone::CA080),
            "CA081" => Ok(ForecastZone::CA081),
            "CA082" => Ok(ForecastZone::CA082),
            "CA083" => Ok(ForecastZone::CA083),
            "CA084" => Ok(ForecastZone::CA084),
            "CA085" => Ok(ForecastZone::CA085),
            "CA087" => Ok(ForecastZone::CA087),
            "CA088" => Ok(ForecastZone::CA088),
            "CA101" => Ok(ForecastZone::CA101),
            "CA102" => Ok(ForecastZone::CA102),
            "CA103" => Ok(ForecastZone::CA103),
            "CA104" => Ok(ForecastZone::CA104),
            "CA105" => Ok(ForecastZone::CA105),
            "CA106" => Ok(ForecastZone::CA106),
            "CA107" => Ok(ForecastZone::CA107),
            "CA108" => Ok(ForecastZone::CA108),
            "CA109" => Ok(ForecastZone::CA109),
            "CA110" => Ok(ForecastZone::CA110),
            "CA111" => Ok(ForecastZone::CA111),
            "CA112" => Ok(ForecastZone::CA112),
            "CA113" => Ok(ForecastZone::CA113),
            "CA114" => Ok(ForecastZone::CA114),
            "CA115" => Ok(ForecastZone::CA115),
            "CA300" => Ok(ForecastZone::CA300),
            "CA301" => Ok(ForecastZone::CA301),
            "CA302" => Ok(ForecastZone::CA302),
            "CA303" => Ok(ForecastZone::CA303),
            "CA304" => Ok(ForecastZone::CA304),
            "CA305" => Ok(ForecastZone::CA305),
            "CA306" => Ok(ForecastZone::CA306),
            "CA307" => Ok(ForecastZone::CA307),
            "CA308" => Ok(ForecastZone::CA308),
            "CA309" => Ok(ForecastZone::CA309),
            "CA310" => Ok(ForecastZone::CA310),
            "CA311" => Ok(ForecastZone::CA311),
            "CA312" => Ok(ForecastZone::CA312),
            "CA313" => Ok(ForecastZone::CA313),
            "CA314" => Ok(ForecastZone::CA314),
            "CA315" => Ok(ForecastZone::CA315),
            "CA316" => Ok(ForecastZone::CA316),
            "CA317" => Ok(ForecastZone::CA317),
            "CA318" => Ok(ForecastZone::CA318),
            "CA319" => Ok(ForecastZone::CA319),
            "CA320" => Ok(ForecastZone::CA320),
            "CA321" => Ok(ForecastZone::CA321),
            "CA322" => Ok(ForecastZone::CA322),
            "CA323" => Ok(ForecastZone::CA323),
            "CA324" => Ok(ForecastZone::CA324),
            "CA325" => Ok(ForecastZone::CA325),
            "CA326" => Ok(ForecastZone::CA326),
            "CA327" => Ok(ForecastZone::CA327),
            "CA328" => Ok(ForecastZone::CA328),
            "CA329" => Ok(ForecastZone::CA329),
            "CA330" => Ok(ForecastZone::CA330),
            "CA331" => Ok(ForecastZone::CA331),
            "CA332" => Ok(ForecastZone::CA332),
            "CA333" => Ok(ForecastZone::CA333),
            "CA334" => Ok(ForecastZone::CA334),
            "CA335" => Ok(ForecastZone::CA335),
            "CA336" => Ok(ForecastZone::CA336),
            "CA337" => Ok(ForecastZone::CA337),
            "CA338" => Ok(ForecastZone::CA338),
            "CA339" => Ok(ForecastZone::CA339),
            "CA340" => Ok(ForecastZone::CA340),
            "CA341" => Ok(ForecastZone::CA341),
            "CA342" => Ok(ForecastZone::CA342),
            "CA343" => Ok(ForecastZone::CA343),
            "CA344" => Ok(ForecastZone::CA344),
            "CA345" => Ok(ForecastZone::CA345),
            "CA346" => Ok(ForecastZone::CA346),
            "CA347" => Ok(ForecastZone::CA347),
            "CA348" => Ok(ForecastZone::CA348),
            "CA349" => Ok(ForecastZone::CA349),
            "CA350" => Ok(ForecastZone::CA350),
            "CA351" => Ok(ForecastZone::CA351),
            "CA352" => Ok(ForecastZone::CA352),
            "CA353" => Ok(ForecastZone::CA353),
            "CA354" => Ok(ForecastZone::CA354),
            "CA355" => Ok(ForecastZone::CA355),
            "CA356" => Ok(ForecastZone::CA356),
            "CA357" => Ok(ForecastZone::CA357),
            "CA358" => Ok(ForecastZone::CA358),
            "CA359" => Ok(ForecastZone::CA359),
            "CA362" => Ok(ForecastZone::CA362),
            "CA363" => Ok(ForecastZone::CA363),
            "CA364" => Ok(ForecastZone::CA364),
            "CA365" => Ok(ForecastZone::CA365),
            "CA502" => Ok(ForecastZone::CA502),
            "CA503" => Ok(ForecastZone::CA503),
            "CA504" => Ok(ForecastZone::CA504),
            "CA505" => Ok(ForecastZone::CA505),
            "CA506" => Ok(ForecastZone::CA506),
            "CA508" => Ok(ForecastZone::CA508),
            "CA509" => Ok(ForecastZone::CA509),
            "CA510" => Ok(ForecastZone::CA510),
            "CA512" => Ok(ForecastZone::CA512),
            "CA513" => Ok(ForecastZone::CA513),
            "CA514" => Ok(ForecastZone::CA514),
            "CA515" => Ok(ForecastZone::CA515),
            "CA516" => Ok(ForecastZone::CA516),
            "CA517" => Ok(ForecastZone::CA517),
            "CA518" => Ok(ForecastZone::CA518),
            "CA519" => Ok(ForecastZone::CA519),
            "CA520" => Ok(ForecastZone::CA520),
            "CA521" => Ok(ForecastZone::CA521),
            "CA522" => Ok(ForecastZone::CA522),
            "CA523" => Ok(ForecastZone::CA523),
            "CA524" => Ok(ForecastZone::CA524),
            "CA525" => Ok(ForecastZone::CA525),
            "CA526" => Ok(ForecastZone::CA526),
            "CA527" => Ok(ForecastZone::CA527),
            "CA528" => Ok(ForecastZone::CA528),
            "CA529" => Ok(ForecastZone::CA529),
            "CA530" => Ok(ForecastZone::CA530),
            "CA547" => Ok(ForecastZone::CA547),
            "CA548" => Ok(ForecastZone::CA548),
            "CA549" => Ok(ForecastZone::CA549),
            "CA550" => Ok(ForecastZone::CA550),
            "CA552" => Ok(ForecastZone::CA552),
            "CA554" => Ok(ForecastZone::CA554),
            "CA560" => Ok(ForecastZone::CA560),
            "CA561" => Ok(ForecastZone::CA561),
            "CA562" => Ok(ForecastZone::CA562),
            "CA563" => Ok(ForecastZone::CA563),
            "CA564" => Ok(ForecastZone::CA564),
            "CA565" => Ok(ForecastZone::CA565),
            "CA566" => Ok(ForecastZone::CA566),
            "CA567" => Ok(ForecastZone::CA567),
            "CA568" => Ok(ForecastZone::CA568),
            "CA569" => Ok(ForecastZone::CA569),
            "CA570" => Ok(ForecastZone::CA570),
            "CO001" => Ok(ForecastZone::CO001),
            "CO002" => Ok(ForecastZone::CO002),
            "CO003" => Ok(ForecastZone::CO003),
            "CO004" => Ok(ForecastZone::CO004),
            "CO005" => Ok(ForecastZone::CO005),
            "CO006" => Ok(ForecastZone::CO006),
            "CO007" => Ok(ForecastZone::CO007),
            "CO008" => Ok(ForecastZone::CO008),
            "CO009" => Ok(ForecastZone::CO009),
            "CO010" => Ok(ForecastZone::CO010),
            "CO011" => Ok(ForecastZone::CO011),
            "CO012" => Ok(ForecastZone::CO012),
            "CO013" => Ok(ForecastZone::CO013),
            "CO014" => Ok(ForecastZone::CO014),
            "CO017" => Ok(ForecastZone::CO017),
            "CO018" => Ok(ForecastZone::CO018),
            "CO019" => Ok(ForecastZone::CO019),
            "CO020" => Ok(ForecastZone::CO020),
            "CO021" => Ok(ForecastZone::CO021),
            "CO022" => Ok(ForecastZone::CO022),
            "CO023" => Ok(ForecastZone::CO023),
            "CO030" => Ok(ForecastZone::CO030),
            "CO031" => Ok(ForecastZone::CO031),
            "CO032" => Ok(ForecastZone::CO032),
            "CO033" => Ok(ForecastZone::CO033),
            "CO034" => Ok(ForecastZone::CO034),
            "CO035" => Ok(ForecastZone::CO035),
            "CO036" => Ok(ForecastZone::CO036),
            "CO037" => Ok(ForecastZone::CO037),
            "CO038" => Ok(ForecastZone::CO038),
            "CO039" => Ok(ForecastZone::CO039),
            "CO040" => Ok(ForecastZone::CO040),
            "CO041" => Ok(ForecastZone::CO041),
            "CO042" => Ok(ForecastZone::CO042),
            "CO043" => Ok(ForecastZone::CO043),
            "CO044" => Ok(ForecastZone::CO044),
            "CO045" => Ok(ForecastZone::CO045),
            "CO046" => Ok(ForecastZone::CO046),
            "CO047" => Ok(ForecastZone::CO047),
            "CO048" => Ok(ForecastZone::CO048),
            "CO049" => Ok(ForecastZone::CO049),
            "CO050" => Ok(ForecastZone::CO050),
            "CO051" => Ok(ForecastZone::CO051),
            "CO058" => Ok(ForecastZone::CO058),
            "CO059" => Ok(ForecastZone::CO059),
            "CO060" => Ok(ForecastZone::CO060),
            "CO061" => Ok(ForecastZone::CO061),
            "CO062" => Ok(ForecastZone::CO062),
            "CO063" => Ok(ForecastZone::CO063),
            "CO064" => Ok(ForecastZone::CO064),
            "CO065" => Ok(ForecastZone::CO065),
            "CO066" => Ok(ForecastZone::CO066),
            "CO067" => Ok(ForecastZone::CO067),
            "CO068" => Ok(ForecastZone::CO068),
            "CO069" => Ok(ForecastZone::CO069),
            "CO070" => Ok(ForecastZone::CO070),
            "CO071" => Ok(ForecastZone::CO071),
            "CO072" => Ok(ForecastZone::CO072),
            "CO073" => Ok(ForecastZone::CO073),
            "CO074" => Ok(ForecastZone::CO074),
            "CO075" => Ok(ForecastZone::CO075),
            "CO076" => Ok(ForecastZone::CO076),
            "CO077" => Ok(ForecastZone::CO077),
            "CO078" => Ok(ForecastZone::CO078),
            "CO079" => Ok(ForecastZone::CO079),
            "CO080" => Ok(ForecastZone::CO080),
            "CO081" => Ok(ForecastZone::CO081),
            "CO082" => Ok(ForecastZone::CO082),
            "CO083" => Ok(ForecastZone::CO083),
            "CO084" => Ok(ForecastZone::CO084),
            "CO085" => Ok(ForecastZone::CO085),
            "CO086" => Ok(ForecastZone::CO086),
            "CO087" => Ok(ForecastZone::CO087),
            "CO088" => Ok(ForecastZone::CO088),
            "CO089" => Ok(ForecastZone::CO089),
            "CO090" => Ok(ForecastZone::CO090),
            "CO091" => Ok(ForecastZone::CO091),
            "CO092" => Ok(ForecastZone::CO092),
            "CO093" => Ok(ForecastZone::CO093),
            "CO094" => Ok(ForecastZone::CO094),
            "CO095" => Ok(ForecastZone::CO095),
            "CO096" => Ok(ForecastZone::CO096),
            "CO097" => Ok(ForecastZone::CO097),
            "CO098" => Ok(ForecastZone::CO098),
            "CO099" => Ok(ForecastZone::CO099),
            "CT001" => Ok(ForecastZone::CT001),
            "CT002" => Ok(ForecastZone::CT002),
            "CT003" => Ok(ForecastZone::CT003),
            "CT004" => Ok(ForecastZone::CT004),
            "CT005" => Ok(ForecastZone::CT005),
            "CT006" => Ok(ForecastZone::CT006),
            "CT007" => Ok(ForecastZone::CT007),
            "CT008" => Ok(ForecastZone::CT008),
            "CT009" => Ok(ForecastZone::CT009),
            "CT010" => Ok(ForecastZone::CT010),
            "CT011" => Ok(ForecastZone::CT011),
            "CT012" => Ok(ForecastZone::CT012),
            "CT013" => Ok(ForecastZone::CT013),
            "DC001" => Ok(ForecastZone::DC001),
            "DE001" => Ok(ForecastZone::DE001),
            "DE002" => Ok(ForecastZone::DE002),
            "DE003" => Ok(ForecastZone::DE003),
            "DE004" => Ok(ForecastZone::DE004),
            "FL007" => Ok(ForecastZone::FL007),
            "FL008" => Ok(ForecastZone::FL008),
            "FL009" => Ok(ForecastZone::FL009),
            "FL010" => Ok(ForecastZone::FL010),
            "FL011" => Ok(ForecastZone::FL011),
            "FL012" => Ok(ForecastZone::FL012),
            "FL013" => Ok(ForecastZone::FL013),
            "FL014" => Ok(ForecastZone::FL014),
            "FL015" => Ok(ForecastZone::FL015),
            "FL016" => Ok(ForecastZone::FL016),
            "FL017" => Ok(ForecastZone::FL017),
            "FL018" => Ok(ForecastZone::FL018),
            "FL019" => Ok(ForecastZone::FL019),
            "FL020" => Ok(ForecastZone::FL020),
            "FL021" => Ok(ForecastZone::FL021),
            "FL023" => Ok(ForecastZone::FL023),
            "FL024" => Ok(ForecastZone::FL024),
            "FL026" => Ok(ForecastZone::FL026),
            "FL027" => Ok(ForecastZone::FL027),
            "FL028" => Ok(ForecastZone::FL028),
            "FL029" => Ok(ForecastZone::FL029),
            "FL030" => Ok(ForecastZone::FL030),
            "FL031" => Ok(ForecastZone::FL031),
            "FL033" => Ok(ForecastZone::FL033),
            "FL034" => Ok(ForecastZone::FL034),
            "FL035" => Ok(ForecastZone::FL035),
            "FL038" => Ok(ForecastZone::FL038),
            "FL041" => Ok(ForecastZone::FL041),
            "FL043" => Ok(ForecastZone::FL043),
            "FL044" => Ok(ForecastZone::FL044),
            "FL045" => Ok(ForecastZone::FL045),
            "FL046" => Ok(ForecastZone::FL046),
            "FL050" => Ok(ForecastZone::FL050),
            "FL052" => Ok(ForecastZone::FL052),
            "FL053" => Ok(ForecastZone::FL053),
            "FL056" => Ok(ForecastZone::FL056),
            "FL057" => Ok(ForecastZone::FL057),
            "FL058" => Ok(ForecastZone::FL058),
            "FL061" => Ok(ForecastZone::FL061),
            "FL063" => Ok(ForecastZone::FL063),
            "FL066" => Ok(ForecastZone::FL066),
            "FL067" => Ok(ForecastZone::FL067),
            "FL068" => Ok(ForecastZone::FL068),
            "FL069" => Ok(ForecastZone::FL069),
            "FL070" => Ok(ForecastZone::FL070),
            "FL071" => Ok(ForecastZone::FL071),
            "FL072" => Ok(ForecastZone::FL072),
            "FL073" => Ok(ForecastZone::FL073),
            "FL074" => Ok(ForecastZone::FL074),
            "FL075" => Ok(ForecastZone::FL075),
            "FL076" => Ok(ForecastZone::FL076),
            "FL077" => Ok(ForecastZone::FL077),
            "FL078" => Ok(ForecastZone::FL078),
            "FL108" => Ok(ForecastZone::FL108),
            "FL112" => Ok(ForecastZone::FL112),
            "FL114" => Ok(ForecastZone::FL114),
            "FL115" => Ok(ForecastZone::FL115),
            "FL118" => Ok(ForecastZone::FL118),
            "FL122" => Ok(ForecastZone::FL122),
            "FL124" => Ok(ForecastZone::FL124),
            "FL125" => Ok(ForecastZone::FL125),
            "FL127" => Ok(ForecastZone::FL127),
            "FL128" => Ok(ForecastZone::FL128),
            "FL132" => Ok(ForecastZone::FL132),
            "FL133" => Ok(ForecastZone::FL133),
            "FL134" => Ok(ForecastZone::FL134),
            "FL136" => Ok(ForecastZone::FL136),
            "FL137" => Ok(ForecastZone::FL137),
            "FL138" => Ok(ForecastZone::FL138),
            "FL139" => Ok(ForecastZone::FL139),
            "FL140" => Ok(ForecastZone::FL140),
            "FL141" => Ok(ForecastZone::FL141),
            "FL142" => Ok(ForecastZone::FL142),
            "FL144" => Ok(ForecastZone::FL144),
            "FL148" => Ok(ForecastZone::FL148),
            "FL149" => Ok(ForecastZone::FL149),
            "FL151" => Ok(ForecastZone::FL151),
            "FL154" => Ok(ForecastZone::FL154),
            "FL155" => Ok(ForecastZone::FL155),
            "FL159" => Ok(ForecastZone::FL159),
            "FL160" => Ok(ForecastZone::FL160),
            "FL162" => Ok(ForecastZone::FL162),
            "FL164" => Ok(ForecastZone::FL164),
            "FL165" => Ok(ForecastZone::FL165),
            "FL168" => Ok(ForecastZone::FL168),
            "FL172" => Ok(ForecastZone::FL172),
            "FL173" => Ok(ForecastZone::FL173),
            "FL174" => Ok(ForecastZone::FL174),
            "FL201" => Ok(ForecastZone::FL201),
            "FL202" => Ok(ForecastZone::FL202),
            "FL203" => Ok(ForecastZone::FL203),
            "FL204" => Ok(ForecastZone::FL204),
            "FL205" => Ok(ForecastZone::FL205),
            "FL206" => Ok(ForecastZone::FL206),
            "FL222" => Ok(ForecastZone::FL222),
            "FL225" => Ok(ForecastZone::FL225),
            "FL232" => Ok(ForecastZone::FL232),
            "FL236" => Ok(ForecastZone::FL236),
            "FL237" => Ok(ForecastZone::FL237),
            "FL239" => Ok(ForecastZone::FL239),
            "FL240" => Ok(ForecastZone::FL240),
            "FL242" => Ok(ForecastZone::FL242),
            "FL247" => Ok(ForecastZone::FL247),
            "FL248" => Ok(ForecastZone::FL248),
            "FL249" => Ok(ForecastZone::FL249),
            "FL251" => Ok(ForecastZone::FL251),
            "FL254" => Ok(ForecastZone::FL254),
            "FL255" => Ok(ForecastZone::FL255),
            "FL259" => Ok(ForecastZone::FL259),
            "FL260" => Ok(ForecastZone::FL260),
            "FL262" => Ok(ForecastZone::FL262),
            "FL264" => Ok(ForecastZone::FL264),
            "FL265" => Ok(ForecastZone::FL265),
            "FL325" => Ok(ForecastZone::FL325),
            "FL340" => Ok(ForecastZone::FL340),
            "FL347" => Ok(ForecastZone::FL347),
            "FL425" => Ok(ForecastZone::FL425),
            "FL447" => Ok(ForecastZone::FL447),
            "FL547" => Ok(ForecastZone::FL547),
            "FL647" => Ok(ForecastZone::FL647),
            "FL747" => Ok(ForecastZone::FL747),
            "FM001" => Ok(ForecastZone::FM001),
            "FM011" => Ok(ForecastZone::FM011),
            "FM012" => Ok(ForecastZone::FM012),
            "FM013" => Ok(ForecastZone::FM013),
            "FM014" => Ok(ForecastZone::FM014),
            "FM015" => Ok(ForecastZone::FM015),
            "FM016" => Ok(ForecastZone::FM016),
            "FM017" => Ok(ForecastZone::FM017),
            "FM018" => Ok(ForecastZone::FM018),
            "FM021" => Ok(ForecastZone::FM021),
            "FM022" => Ok(ForecastZone::FM022),
            "FM023" => Ok(ForecastZone::FM023),
            "FM024" => Ok(ForecastZone::FM024),
            "FM025" => Ok(ForecastZone::FM025),
            "FM026" => Ok(ForecastZone::FM026),
            "FM031" => Ok(ForecastZone::FM031),
            "FM032" => Ok(ForecastZone::FM032),
            "FM033" => Ok(ForecastZone::FM033),
            "FM034" => Ok(ForecastZone::FM034),
            "FM035" => Ok(ForecastZone::FM035),
            "FM036" => Ok(ForecastZone::FM036),
            "FM037" => Ok(ForecastZone::FM037),
            "FM038" => Ok(ForecastZone::FM038),
            "GA001" => Ok(ForecastZone::GA001),
            "GA002" => Ok(ForecastZone::GA002),
            "GA003" => Ok(ForecastZone::GA003),
            "GA004" => Ok(ForecastZone::GA004),
            "GA005" => Ok(ForecastZone::GA005),
            "GA006" => Ok(ForecastZone::GA006),
            "GA007" => Ok(ForecastZone::GA007),
            "GA008" => Ok(ForecastZone::GA008),
            "GA009" => Ok(ForecastZone::GA009),
            "GA010" => Ok(ForecastZone::GA010),
            "GA011" => Ok(ForecastZone::GA011),
            "GA012" => Ok(ForecastZone::GA012),
            "GA013" => Ok(ForecastZone::GA013),
            "GA014" => Ok(ForecastZone::GA014),
            "GA015" => Ok(ForecastZone::GA015),
            "GA016" => Ok(ForecastZone::GA016),
            "GA017" => Ok(ForecastZone::GA017),
            "GA018" => Ok(ForecastZone::GA018),
            "GA019" => Ok(ForecastZone::GA019),
            "GA020" => Ok(ForecastZone::GA020),
            "GA021" => Ok(ForecastZone::GA021),
            "GA022" => Ok(ForecastZone::GA022),
            "GA023" => Ok(ForecastZone::GA023),
            "GA024" => Ok(ForecastZone::GA024),
            "GA025" => Ok(ForecastZone::GA025),
            "GA026" => Ok(ForecastZone::GA026),
            "GA027" => Ok(ForecastZone::GA027),
            "GA028" => Ok(ForecastZone::GA028),
            "GA029" => Ok(ForecastZone::GA029),
            "GA030" => Ok(ForecastZone::GA030),
            "GA031" => Ok(ForecastZone::GA031),
            "GA032" => Ok(ForecastZone::GA032),
            "GA033" => Ok(ForecastZone::GA033),
            "GA034" => Ok(ForecastZone::GA034),
            "GA035" => Ok(ForecastZone::GA035),
            "GA036" => Ok(ForecastZone::GA036),
            "GA037" => Ok(ForecastZone::GA037),
            "GA038" => Ok(ForecastZone::GA038),
            "GA039" => Ok(ForecastZone::GA039),
            "GA040" => Ok(ForecastZone::GA040),
            "GA041" => Ok(ForecastZone::GA041),
            "GA042" => Ok(ForecastZone::GA042),
            "GA043" => Ok(ForecastZone::GA043),
            "GA044" => Ok(ForecastZone::GA044),
            "GA045" => Ok(ForecastZone::GA045),
            "GA046" => Ok(ForecastZone::GA046),
            "GA047" => Ok(ForecastZone::GA047),
            "GA048" => Ok(ForecastZone::GA048),
            "GA049" => Ok(ForecastZone::GA049),
            "GA050" => Ok(ForecastZone::GA050),
            "GA051" => Ok(ForecastZone::GA051),
            "GA052" => Ok(ForecastZone::GA052),
            "GA053" => Ok(ForecastZone::GA053),
            "GA054" => Ok(ForecastZone::GA054),
            "GA055" => Ok(ForecastZone::GA055),
            "GA056" => Ok(ForecastZone::GA056),
            "GA057" => Ok(ForecastZone::GA057),
            "GA058" => Ok(ForecastZone::GA058),
            "GA059" => Ok(ForecastZone::GA059),
            "GA060" => Ok(ForecastZone::GA060),
            "GA061" => Ok(ForecastZone::GA061),
            "GA062" => Ok(ForecastZone::GA062),
            "GA063" => Ok(ForecastZone::GA063),
            "GA064" => Ok(ForecastZone::GA064),
            "GA065" => Ok(ForecastZone::GA065),
            "GA066" => Ok(ForecastZone::GA066),
            "GA067" => Ok(ForecastZone::GA067),
            "GA068" => Ok(ForecastZone::GA068),
            "GA069" => Ok(ForecastZone::GA069),
            "GA070" => Ok(ForecastZone::GA070),
            "GA071" => Ok(ForecastZone::GA071),
            "GA072" => Ok(ForecastZone::GA072),
            "GA073" => Ok(ForecastZone::GA073),
            "GA074" => Ok(ForecastZone::GA074),
            "GA075" => Ok(ForecastZone::GA075),
            "GA076" => Ok(ForecastZone::GA076),
            "GA077" => Ok(ForecastZone::GA077),
            "GA078" => Ok(ForecastZone::GA078),
            "GA079" => Ok(ForecastZone::GA079),
            "GA080" => Ok(ForecastZone::GA080),
            "GA081" => Ok(ForecastZone::GA081),
            "GA082" => Ok(ForecastZone::GA082),
            "GA083" => Ok(ForecastZone::GA083),
            "GA084" => Ok(ForecastZone::GA084),
            "GA085" => Ok(ForecastZone::GA085),
            "GA086" => Ok(ForecastZone::GA086),
            "GA087" => Ok(ForecastZone::GA087),
            "GA088" => Ok(ForecastZone::GA088),
            "GA089" => Ok(ForecastZone::GA089),
            "GA090" => Ok(ForecastZone::GA090),
            "GA091" => Ok(ForecastZone::GA091),
            "GA092" => Ok(ForecastZone::GA092),
            "GA093" => Ok(ForecastZone::GA093),
            "GA094" => Ok(ForecastZone::GA094),
            "GA095" => Ok(ForecastZone::GA095),
            "GA096" => Ok(ForecastZone::GA096),
            "GA097" => Ok(ForecastZone::GA097),
            "GA098" => Ok(ForecastZone::GA098),
            "GA099" => Ok(ForecastZone::GA099),
            "GA100" => Ok(ForecastZone::GA100),
            "GA101" => Ok(ForecastZone::GA101),
            "GA102" => Ok(ForecastZone::GA102),
            "GA103" => Ok(ForecastZone::GA103),
            "GA104" => Ok(ForecastZone::GA104),
            "GA105" => Ok(ForecastZone::GA105),
            "GA106" => Ok(ForecastZone::GA106),
            "GA107" => Ok(ForecastZone::GA107),
            "GA108" => Ok(ForecastZone::GA108),
            "GA109" => Ok(ForecastZone::GA109),
            "GA110" => Ok(ForecastZone::GA110),
            "GA111" => Ok(ForecastZone::GA111),
            "GA112" => Ok(ForecastZone::GA112),
            "GA113" => Ok(ForecastZone::GA113),
            "GA114" => Ok(ForecastZone::GA114),
            "GA115" => Ok(ForecastZone::GA115),
            "GA116" => Ok(ForecastZone::GA116),
            "GA117" => Ok(ForecastZone::GA117),
            "GA118" => Ok(ForecastZone::GA118),
            "GA119" => Ok(ForecastZone::GA119),
            "GA120" => Ok(ForecastZone::GA120),
            "GA121" => Ok(ForecastZone::GA121),
            "GA122" => Ok(ForecastZone::GA122),
            "GA123" => Ok(ForecastZone::GA123),
            "GA124" => Ok(ForecastZone::GA124),
            "GA125" => Ok(ForecastZone::GA125),
            "GA126" => Ok(ForecastZone::GA126),
            "GA127" => Ok(ForecastZone::GA127),
            "GA128" => Ok(ForecastZone::GA128),
            "GA129" => Ok(ForecastZone::GA129),
            "GA130" => Ok(ForecastZone::GA130),
            "GA131" => Ok(ForecastZone::GA131),
            "GA132" => Ok(ForecastZone::GA132),
            "GA133" => Ok(ForecastZone::GA133),
            "GA134" => Ok(ForecastZone::GA134),
            "GA135" => Ok(ForecastZone::GA135),
            "GA136" => Ok(ForecastZone::GA136),
            "GA137" => Ok(ForecastZone::GA137),
            "GA138" => Ok(ForecastZone::GA138),
            "GA139" => Ok(ForecastZone::GA139),
            "GA140" => Ok(ForecastZone::GA140),
            "GA141" => Ok(ForecastZone::GA141),
            "GA142" => Ok(ForecastZone::GA142),
            "GA143" => Ok(ForecastZone::GA143),
            "GA144" => Ok(ForecastZone::GA144),
            "GA145" => Ok(ForecastZone::GA145),
            "GA146" => Ok(ForecastZone::GA146),
            "GA147" => Ok(ForecastZone::GA147),
            "GA148" => Ok(ForecastZone::GA148),
            "GA149" => Ok(ForecastZone::GA149),
            "GA151" => Ok(ForecastZone::GA151),
            "GA152" => Ok(ForecastZone::GA152),
            "GA153" => Ok(ForecastZone::GA153),
            "GA154" => Ok(ForecastZone::GA154),
            "GA155" => Ok(ForecastZone::GA155),
            "GA156" => Ok(ForecastZone::GA156),
            "GA157" => Ok(ForecastZone::GA157),
            "GA158" => Ok(ForecastZone::GA158),
            "GA159" => Ok(ForecastZone::GA159),
            "GA160" => Ok(ForecastZone::GA160),
            "GA161" => Ok(ForecastZone::GA161),
            "GA162" => Ok(ForecastZone::GA162),
            "GA163" => Ok(ForecastZone::GA163),
            "GA165" => Ok(ForecastZone::GA165),
            "GA166" => Ok(ForecastZone::GA166),
            "GA250" => Ok(ForecastZone::GA250),
            "GA264" => Ok(ForecastZone::GA264),
            "GA350" => Ok(ForecastZone::GA350),
            "GA364" => Ok(ForecastZone::GA364),
            "GU001" => Ok(ForecastZone::GU001),
            "HI001" => Ok(ForecastZone::HI001),
            "HI003" => Ok(ForecastZone::HI003),
            "HI004" => Ok(ForecastZone::HI004),
            "HI006" => Ok(ForecastZone::HI006),
            "HI007" => Ok(ForecastZone::HI007),
            "HI009" => Ok(ForecastZone::HI009),
            "HI010" => Ok(ForecastZone::HI010),
            "HI011" => Ok(ForecastZone::HI011),
            "HI015" => Ok(ForecastZone::HI015),
            "HI016" => Ok(ForecastZone::HI016),
            "HI017" => Ok(ForecastZone::HI017),
            "HI018" => Ok(ForecastZone::HI018),
            "HI022" => Ok(ForecastZone::HI022),
            "HI023" => Ok(ForecastZone::HI023),
            "HI026" => Ok(ForecastZone::HI026),
            "HI027" => Ok(ForecastZone::HI027),
            "HI028" => Ok(ForecastZone::HI028),
            "HI029" => Ok(ForecastZone::HI029),
            "HI030" => Ok(ForecastZone::HI030),
            "HI031" => Ok(ForecastZone::HI031),
            "HI032" => Ok(ForecastZone::HI032),
            "HI033" => Ok(ForecastZone::HI033),
            "HI034" => Ok(ForecastZone::HI034),
            "HI035" => Ok(ForecastZone::HI035),
            "HI036" => Ok(ForecastZone::HI036),
            "HI037" => Ok(ForecastZone::HI037),
            "HI038" => Ok(ForecastZone::HI038),
            "HI039" => Ok(ForecastZone::HI039),
            "HI040" => Ok(ForecastZone::HI040),
            "HI041" => Ok(ForecastZone::HI041),
            "HI042" => Ok(ForecastZone::HI042),
            "HI043" => Ok(ForecastZone::HI043),
            "HI044" => Ok(ForecastZone::HI044),
            "HI045" => Ok(ForecastZone::HI045),
            "HI046" => Ok(ForecastZone::HI046),
            "HI047" => Ok(ForecastZone::HI047),
            "HI048" => Ok(ForecastZone::HI048),
            "HI049" => Ok(ForecastZone::HI049),
            "HI050" => Ok(ForecastZone::HI050),
            "HI051" => Ok(ForecastZone::HI051),
            "HI052" => Ok(ForecastZone::HI052),
            "HI053" => Ok(ForecastZone::HI053),
            "HI054" => Ok(ForecastZone::HI054),
            "IA001" => Ok(ForecastZone::IA001),
            "IA002" => Ok(ForecastZone::IA002),
            "IA003" => Ok(ForecastZone::IA003),
            "IA004" => Ok(ForecastZone::IA004),
            "IA005" => Ok(ForecastZone::IA005),
            "IA006" => Ok(ForecastZone::IA006),
            "IA007" => Ok(ForecastZone::IA007),
            "IA008" => Ok(ForecastZone::IA008),
            "IA009" => Ok(ForecastZone::IA009),
            "IA010" => Ok(ForecastZone::IA010),
            "IA011" => Ok(ForecastZone::IA011),
            "IA012" => Ok(ForecastZone::IA012),
            "IA013" => Ok(ForecastZone::IA013),
            "IA014" => Ok(ForecastZone::IA014),
            "IA015" => Ok(ForecastZone::IA015),
            "IA016" => Ok(ForecastZone::IA016),
            "IA017" => Ok(ForecastZone::IA017),
            "IA018" => Ok(ForecastZone::IA018),
            "IA019" => Ok(ForecastZone::IA019),
            "IA020" => Ok(ForecastZone::IA020),
            "IA021" => Ok(ForecastZone::IA021),
            "IA022" => Ok(ForecastZone::IA022),
            "IA023" => Ok(ForecastZone::IA023),
            "IA024" => Ok(ForecastZone::IA024),
            "IA025" => Ok(ForecastZone::IA025),
            "IA026" => Ok(ForecastZone::IA026),
            "IA027" => Ok(ForecastZone::IA027),
            "IA028" => Ok(ForecastZone::IA028),
            "IA029" => Ok(ForecastZone::IA029),
            "IA030" => Ok(ForecastZone::IA030),
            "IA031" => Ok(ForecastZone::IA031),
            "IA032" => Ok(ForecastZone::IA032),
            "IA033" => Ok(ForecastZone::IA033),
            "IA034" => Ok(ForecastZone::IA034),
            "IA035" => Ok(ForecastZone::IA035),
            "IA036" => Ok(ForecastZone::IA036),
            "IA037" => Ok(ForecastZone::IA037),
            "IA038" => Ok(ForecastZone::IA038),
            "IA039" => Ok(ForecastZone::IA039),
            "IA040" => Ok(ForecastZone::IA040),
            "IA041" => Ok(ForecastZone::IA041),
            "IA042" => Ok(ForecastZone::IA042),
            "IA043" => Ok(ForecastZone::IA043),
            "IA044" => Ok(ForecastZone::IA044),
            "IA045" => Ok(ForecastZone::IA045),
            "IA046" => Ok(ForecastZone::IA046),
            "IA047" => Ok(ForecastZone::IA047),
            "IA048" => Ok(ForecastZone::IA048),
            "IA049" => Ok(ForecastZone::IA049),
            "IA050" => Ok(ForecastZone::IA050),
            "IA051" => Ok(ForecastZone::IA051),
            "IA052" => Ok(ForecastZone::IA052),
            "IA053" => Ok(ForecastZone::IA053),
            "IA054" => Ok(ForecastZone::IA054),
            "IA055" => Ok(ForecastZone::IA055),
            "IA056" => Ok(ForecastZone::IA056),
            "IA057" => Ok(ForecastZone::IA057),
            "IA058" => Ok(ForecastZone::IA058),
            "IA059" => Ok(ForecastZone::IA059),
            "IA060" => Ok(ForecastZone::IA060),
            "IA061" => Ok(ForecastZone::IA061),
            "IA062" => Ok(ForecastZone::IA062),
            "IA063" => Ok(ForecastZone::IA063),
            "IA064" => Ok(ForecastZone::IA064),
            "IA065" => Ok(ForecastZone::IA065),
            "IA066" => Ok(ForecastZone::IA066),
            "IA067" => Ok(ForecastZone::IA067),
            "IA068" => Ok(ForecastZone::IA068),
            "IA069" => Ok(ForecastZone::IA069),
            "IA070" => Ok(ForecastZone::IA070),
            "IA071" => Ok(ForecastZone::IA071),
            "IA072" => Ok(ForecastZone::IA072),
            "IA073" => Ok(ForecastZone::IA073),
            "IA074" => Ok(ForecastZone::IA074),
            "IA075" => Ok(ForecastZone::IA075),
            "IA076" => Ok(ForecastZone::IA076),
            "IA077" => Ok(ForecastZone::IA077),
            "IA078" => Ok(ForecastZone::IA078),
            "IA079" => Ok(ForecastZone::IA079),
            "IA080" => Ok(ForecastZone::IA080),
            "IA081" => Ok(ForecastZone::IA081),
            "IA082" => Ok(ForecastZone::IA082),
            "IA083" => Ok(ForecastZone::IA083),
            "IA084" => Ok(ForecastZone::IA084),
            "IA085" => Ok(ForecastZone::IA085),
            "IA086" => Ok(ForecastZone::IA086),
            "IA087" => Ok(ForecastZone::IA087),
            "IA088" => Ok(ForecastZone::IA088),
            "IA089" => Ok(ForecastZone::IA089),
            "IA090" => Ok(ForecastZone::IA090),
            "IA091" => Ok(ForecastZone::IA091),
            "IA092" => Ok(ForecastZone::IA092),
            "IA093" => Ok(ForecastZone::IA093),
            "IA094" => Ok(ForecastZone::IA094),
            "IA095" => Ok(ForecastZone::IA095),
            "IA096" => Ok(ForecastZone::IA096),
            "IA097" => Ok(ForecastZone::IA097),
            "IA098" => Ok(ForecastZone::IA098),
            "IA099" => Ok(ForecastZone::IA099),
            "ID001" => Ok(ForecastZone::ID001),
            "ID002" => Ok(ForecastZone::ID002),
            "ID003" => Ok(ForecastZone::ID003),
            "ID004" => Ok(ForecastZone::ID004),
            "ID005" => Ok(ForecastZone::ID005),
            "ID006" => Ok(ForecastZone::ID006),
            "ID007" => Ok(ForecastZone::ID007),
            "ID008" => Ok(ForecastZone::ID008),
            "ID009" => Ok(ForecastZone::ID009),
            "ID010" => Ok(ForecastZone::ID010),
            "ID011" => Ok(ForecastZone::ID011),
            "ID012" => Ok(ForecastZone::ID012),
            "ID013" => Ok(ForecastZone::ID013),
            "ID014" => Ok(ForecastZone::ID014),
            "ID015" => Ok(ForecastZone::ID015),
            "ID016" => Ok(ForecastZone::ID016),
            "ID026" => Ok(ForecastZone::ID026),
            "ID027" => Ok(ForecastZone::ID027),
            "ID028" => Ok(ForecastZone::ID028),
            "ID029" => Ok(ForecastZone::ID029),
            "ID030" => Ok(ForecastZone::ID030),
            "ID033" => Ok(ForecastZone::ID033),
            "ID051" => Ok(ForecastZone::ID051),
            "ID052" => Ok(ForecastZone::ID052),
            "ID053" => Ok(ForecastZone::ID053),
            "ID054" => Ok(ForecastZone::ID054),
            "ID055" => Ok(ForecastZone::ID055),
            "ID056" => Ok(ForecastZone::ID056),
            "ID057" => Ok(ForecastZone::ID057),
            "ID058" => Ok(ForecastZone::ID058),
            "ID059" => Ok(ForecastZone::ID059),
            "ID060" => Ok(ForecastZone::ID060),
            "ID061" => Ok(ForecastZone::ID061),
            "ID062" => Ok(ForecastZone::ID062),
            "ID063" => Ok(ForecastZone::ID063),
            "ID064" => Ok(ForecastZone::ID064),
            "ID065" => Ok(ForecastZone::ID065),
            "ID066" => Ok(ForecastZone::ID066),
            "ID067" => Ok(ForecastZone::ID067),
            "ID068" => Ok(ForecastZone::ID068),
            "ID069" => Ok(ForecastZone::ID069),
            "ID070" => Ok(ForecastZone::ID070),
            "ID071" => Ok(ForecastZone::ID071),
            "ID072" => Ok(ForecastZone::ID072),
            "ID073" => Ok(ForecastZone::ID073),
            "ID074" => Ok(ForecastZone::ID074),
            "ID075" => Ok(ForecastZone::ID075),
            "IL001" => Ok(ForecastZone::IL001),
            "IL002" => Ok(ForecastZone::IL002),
            "IL003" => Ok(ForecastZone::IL003),
            "IL004" => Ok(ForecastZone::IL004),
            "IL005" => Ok(ForecastZone::IL005),
            "IL006" => Ok(ForecastZone::IL006),
            "IL007" => Ok(ForecastZone::IL007),
            "IL008" => Ok(ForecastZone::IL008),
            "IL009" => Ok(ForecastZone::IL009),
            "IL010" => Ok(ForecastZone::IL010),
            "IL011" => Ok(ForecastZone::IL011),
            "IL012" => Ok(ForecastZone::IL012),
            "IL013" => Ok(ForecastZone::IL013),
            "IL015" => Ok(ForecastZone::IL015),
            "IL016" => Ok(ForecastZone::IL016),
            "IL017" => Ok(ForecastZone::IL017),
            "IL018" => Ok(ForecastZone::IL018),
            "IL019" => Ok(ForecastZone::IL019),
            "IL020" => Ok(ForecastZone::IL020),
            "IL021" => Ok(ForecastZone::IL021),
            "IL023" => Ok(ForecastZone::IL023),
            "IL024" => Ok(ForecastZone::IL024),
            "IL025" => Ok(ForecastZone::IL025),
            "IL026" => Ok(ForecastZone::IL026),
            "IL027" => Ok(ForecastZone::IL027),
            "IL028" => Ok(ForecastZone::IL028),
            "IL029" => Ok(ForecastZone::IL029),
            "IL030" => Ok(ForecastZone::IL030),
            "IL031" => Ok(ForecastZone::IL031),
            "IL032" => Ok(ForecastZone::IL032),
            "IL033" => Ok(ForecastZone::IL033),
            "IL034" => Ok(ForecastZone::IL034),
            "IL035" => Ok(ForecastZone::IL035),
            "IL036" => Ok(ForecastZone::IL036),
            "IL037" => Ok(ForecastZone::IL037),
            "IL038" => Ok(ForecastZone::IL038),
            "IL039" => Ok(ForecastZone::IL039),
            "IL040" => Ok(ForecastZone::IL040),
            "IL041" => Ok(ForecastZone::IL041),
            "IL042" => Ok(ForecastZone::IL042),
            "IL043" => Ok(ForecastZone::IL043),
            "IL044" => Ok(ForecastZone::IL044),
            "IL045" => Ok(ForecastZone::IL045),
            "IL046" => Ok(ForecastZone::IL046),
            "IL047" => Ok(ForecastZone::IL047),
            "IL048" => Ok(ForecastZone::IL048),
            "IL049" => Ok(ForecastZone::IL049),
            "IL050" => Ok(ForecastZone::IL050),
            "IL051" => Ok(ForecastZone::IL051),
            "IL052" => Ok(ForecastZone::IL052),
            "IL053" => Ok(ForecastZone::IL053),
            "IL054" => Ok(ForecastZone::IL054),
            "IL055" => Ok(ForecastZone::IL055),
            "IL056" => Ok(ForecastZone::IL056),
            "IL057" => Ok(ForecastZone::IL057),
            "IL058" => Ok(ForecastZone::IL058),
            "IL059" => Ok(ForecastZone::IL059),
            "IL060" => Ok(ForecastZone::IL060),
            "IL061" => Ok(ForecastZone::IL061),
            "IL062" => Ok(ForecastZone::IL062),
            "IL063" => Ok(ForecastZone::IL063),
            "IL064" => Ok(ForecastZone::IL064),
            "IL065" => Ok(ForecastZone::IL065),
            "IL066" => Ok(ForecastZone::IL066),
            "IL067" => Ok(ForecastZone::IL067),
            "IL068" => Ok(ForecastZone::IL068),
            "IL069" => Ok(ForecastZone::IL069),
            "IL070" => Ok(ForecastZone::IL070),
            "IL071" => Ok(ForecastZone::IL071),
            "IL072" => Ok(ForecastZone::IL072),
            "IL073" => Ok(ForecastZone::IL073),
            "IL074" => Ok(ForecastZone::IL074),
            "IL075" => Ok(ForecastZone::IL075),
            "IL076" => Ok(ForecastZone::IL076),
            "IL077" => Ok(ForecastZone::IL077),
            "IL078" => Ok(ForecastZone::IL078),
            "IL079" => Ok(ForecastZone::IL079),
            "IL080" => Ok(ForecastZone::IL080),
            "IL081" => Ok(ForecastZone::IL081),
            "IL082" => Ok(ForecastZone::IL082),
            "IL083" => Ok(ForecastZone::IL083),
            "IL084" => Ok(ForecastZone::IL084),
            "IL085" => Ok(ForecastZone::IL085),
            "IL086" => Ok(ForecastZone::IL086),
            "IL087" => Ok(ForecastZone::IL087),
            "IL088" => Ok(ForecastZone::IL088),
            "IL089" => Ok(ForecastZone::IL089),
            "IL090" => Ok(ForecastZone::IL090),
            "IL091" => Ok(ForecastZone::IL091),
            "IL092" => Ok(ForecastZone::IL092),
            "IL093" => Ok(ForecastZone::IL093),
            "IL094" => Ok(ForecastZone::IL094),
            "IL095" => Ok(ForecastZone::IL095),
            "IL096" => Ok(ForecastZone::IL096),
            "IL097" => Ok(ForecastZone::IL097),
            "IL098" => Ok(ForecastZone::IL098),
            "IL099" => Ok(ForecastZone::IL099),
            "IL100" => Ok(ForecastZone::IL100),
            "IL101" => Ok(ForecastZone::IL101),
            "IL102" => Ok(ForecastZone::IL102),
            "IL103" => Ok(ForecastZone::IL103),
            "IL104" => Ok(ForecastZone::IL104),
            "IL105" => Ok(ForecastZone::IL105),
            "IL106" => Ok(ForecastZone::IL106),
            "IL107" => Ok(ForecastZone::IL107),
            "IL108" => Ok(ForecastZone::IL108),
            "IN001" => Ok(ForecastZone::IN001),
            "IN002" => Ok(ForecastZone::IN002),
            "IN003" => Ok(ForecastZone::IN003),
            "IN004" => Ok(ForecastZone::IN004),
            "IN005" => Ok(ForecastZone::IN005),
            "IN006" => Ok(ForecastZone::IN006),
            "IN007" => Ok(ForecastZone::IN007),
            "IN008" => Ok(ForecastZone::IN008),
            "IN009" => Ok(ForecastZone::IN009),
            "IN010" => Ok(ForecastZone::IN010),
            "IN011" => Ok(ForecastZone::IN011),
            "IN012" => Ok(ForecastZone::IN012),
            "IN013" => Ok(ForecastZone::IN013),
            "IN014" => Ok(ForecastZone::IN014),
            "IN015" => Ok(ForecastZone::IN015),
            "IN016" => Ok(ForecastZone::IN016),
            "IN017" => Ok(ForecastZone::IN017),
            "IN018" => Ok(ForecastZone::IN018),
            "IN019" => Ok(ForecastZone::IN019),
            "IN020" => Ok(ForecastZone::IN020),
            "IN021" => Ok(ForecastZone::IN021),
            "IN022" => Ok(ForecastZone::IN022),
            "IN023" => Ok(ForecastZone::IN023),
            "IN024" => Ok(ForecastZone::IN024),
            "IN025" => Ok(ForecastZone::IN025),
            "IN026" => Ok(ForecastZone::IN026),
            "IN027" => Ok(ForecastZone::IN027),
            "IN028" => Ok(ForecastZone::IN028),
            "IN029" => Ok(ForecastZone::IN029),
            "IN030" => Ok(ForecastZone::IN030),
            "IN031" => Ok(ForecastZone::IN031),
            "IN032" => Ok(ForecastZone::IN032),
            "IN033" => Ok(ForecastZone::IN033),
            "IN034" => Ok(ForecastZone::IN034),
            "IN035" => Ok(ForecastZone::IN035),
            "IN036" => Ok(ForecastZone::IN036),
            "IN037" => Ok(ForecastZone::IN037),
            "IN038" => Ok(ForecastZone::IN038),
            "IN039" => Ok(ForecastZone::IN039),
            "IN040" => Ok(ForecastZone::IN040),
            "IN041" => Ok(ForecastZone::IN041),
            "IN042" => Ok(ForecastZone::IN042),
            "IN043" => Ok(ForecastZone::IN043),
            "IN044" => Ok(ForecastZone::IN044),
            "IN045" => Ok(ForecastZone::IN045),
            "IN046" => Ok(ForecastZone::IN046),
            "IN047" => Ok(ForecastZone::IN047),
            "IN048" => Ok(ForecastZone::IN048),
            "IN049" => Ok(ForecastZone::IN049),
            "IN050" => Ok(ForecastZone::IN050),
            "IN051" => Ok(ForecastZone::IN051),
            "IN052" => Ok(ForecastZone::IN052),
            "IN053" => Ok(ForecastZone::IN053),
            "IN054" => Ok(ForecastZone::IN054),
            "IN055" => Ok(ForecastZone::IN055),
            "IN056" => Ok(ForecastZone::IN056),
            "IN057" => Ok(ForecastZone::IN057),
            "IN058" => Ok(ForecastZone::IN058),
            "IN059" => Ok(ForecastZone::IN059),
            "IN060" => Ok(ForecastZone::IN060),
            "IN061" => Ok(ForecastZone::IN061),
            "IN062" => Ok(ForecastZone::IN062),
            "IN063" => Ok(ForecastZone::IN063),
            "IN064" => Ok(ForecastZone::IN064),
            "IN065" => Ok(ForecastZone::IN065),
            "IN066" => Ok(ForecastZone::IN066),
            "IN067" => Ok(ForecastZone::IN067),
            "IN068" => Ok(ForecastZone::IN068),
            "IN069" => Ok(ForecastZone::IN069),
            "IN070" => Ok(ForecastZone::IN070),
            "IN071" => Ok(ForecastZone::IN071),
            "IN072" => Ok(ForecastZone::IN072),
            "IN073" => Ok(ForecastZone::IN073),
            "IN074" => Ok(ForecastZone::IN074),
            "IN075" => Ok(ForecastZone::IN075),
            "IN076" => Ok(ForecastZone::IN076),
            "IN077" => Ok(ForecastZone::IN077),
            "IN078" => Ok(ForecastZone::IN078),
            "IN079" => Ok(ForecastZone::IN079),
            "IN080" => Ok(ForecastZone::IN080),
            "IN081" => Ok(ForecastZone::IN081),
            "IN082" => Ok(ForecastZone::IN082),
            "IN083" => Ok(ForecastZone::IN083),
            "IN084" => Ok(ForecastZone::IN084),
            "IN085" => Ok(ForecastZone::IN085),
            "IN086" => Ok(ForecastZone::IN086),
            "IN087" => Ok(ForecastZone::IN087),
            "IN088" => Ok(ForecastZone::IN088),
            "IN089" => Ok(ForecastZone::IN089),
            "IN090" => Ok(ForecastZone::IN090),
            "IN091" => Ok(ForecastZone::IN091),
            "IN092" => Ok(ForecastZone::IN092),
            "KS001" => Ok(ForecastZone::KS001),
            "KS002" => Ok(ForecastZone::KS002),
            "KS003" => Ok(ForecastZone::KS003),
            "KS004" => Ok(ForecastZone::KS004),
            "KS005" => Ok(ForecastZone::KS005),
            "KS006" => Ok(ForecastZone::KS006),
            "KS007" => Ok(ForecastZone::KS007),
            "KS008" => Ok(ForecastZone::KS008),
            "KS009" => Ok(ForecastZone::KS009),
            "KS010" => Ok(ForecastZone::KS010),
            "KS011" => Ok(ForecastZone::KS011),
            "KS012" => Ok(ForecastZone::KS012),
            "KS013" => Ok(ForecastZone::KS013),
            "KS014" => Ok(ForecastZone::KS014),
            "KS015" => Ok(ForecastZone::KS015),
            "KS016" => Ok(ForecastZone::KS016),
            "KS017" => Ok(ForecastZone::KS017),
            "KS018" => Ok(ForecastZone::KS018),
            "KS019" => Ok(ForecastZone::KS019),
            "KS020" => Ok(ForecastZone::KS020),
            "KS021" => Ok(ForecastZone::KS021),
            "KS022" => Ok(ForecastZone::KS022),
            "KS023" => Ok(ForecastZone::KS023),
            "KS024" => Ok(ForecastZone::KS024),
            "KS025" => Ok(ForecastZone::KS025),
            "KS026" => Ok(ForecastZone::KS026),
            "KS027" => Ok(ForecastZone::KS027),
            "KS028" => Ok(ForecastZone::KS028),
            "KS029" => Ok(ForecastZone::KS029),
            "KS030" => Ok(ForecastZone::KS030),
            "KS031" => Ok(ForecastZone::KS031),
            "KS032" => Ok(ForecastZone::KS032),
            "KS033" => Ok(ForecastZone::KS033),
            "KS034" => Ok(ForecastZone::KS034),
            "KS035" => Ok(ForecastZone::KS035),
            "KS036" => Ok(ForecastZone::KS036),
            "KS037" => Ok(ForecastZone::KS037),
            "KS038" => Ok(ForecastZone::KS038),
            "KS039" => Ok(ForecastZone::KS039),
            "KS040" => Ok(ForecastZone::KS040),
            "KS041" => Ok(ForecastZone::KS041),
            "KS042" => Ok(ForecastZone::KS042),
            "KS043" => Ok(ForecastZone::KS043),
            "KS044" => Ok(ForecastZone::KS044),
            "KS045" => Ok(ForecastZone::KS045),
            "KS046" => Ok(ForecastZone::KS046),
            "KS047" => Ok(ForecastZone::KS047),
            "KS048" => Ok(ForecastZone::KS048),
            "KS049" => Ok(ForecastZone::KS049),
            "KS050" => Ok(ForecastZone::KS050),
            "KS051" => Ok(ForecastZone::KS051),
            "KS052" => Ok(ForecastZone::KS052),
            "KS053" => Ok(ForecastZone::KS053),
            "KS054" => Ok(ForecastZone::KS054),
            "KS055" => Ok(ForecastZone::KS055),
            "KS056" => Ok(ForecastZone::KS056),
            "KS057" => Ok(ForecastZone::KS057),
            "KS058" => Ok(ForecastZone::KS058),
            "KS059" => Ok(ForecastZone::KS059),
            "KS060" => Ok(ForecastZone::KS060),
            "KS061" => Ok(ForecastZone::KS061),
            "KS062" => Ok(ForecastZone::KS062),
            "KS063" => Ok(ForecastZone::KS063),
            "KS064" => Ok(ForecastZone::KS064),
            "KS065" => Ok(ForecastZone::KS065),
            "KS066" => Ok(ForecastZone::KS066),
            "KS067" => Ok(ForecastZone::KS067),
            "KS068" => Ok(ForecastZone::KS068),
            "KS069" => Ok(ForecastZone::KS069),
            "KS070" => Ok(ForecastZone::KS070),
            "KS071" => Ok(ForecastZone::KS071),
            "KS072" => Ok(ForecastZone::KS072),
            "KS073" => Ok(ForecastZone::KS073),
            "KS074" => Ok(ForecastZone::KS074),
            "KS075" => Ok(ForecastZone::KS075),
            "KS076" => Ok(ForecastZone::KS076),
            "KS077" => Ok(ForecastZone::KS077),
            "KS078" => Ok(ForecastZone::KS078),
            "KS079" => Ok(ForecastZone::KS079),
            "KS080" => Ok(ForecastZone::KS080),
            "KS081" => Ok(ForecastZone::KS081),
            "KS082" => Ok(ForecastZone::KS082),
            "KS083" => Ok(ForecastZone::KS083),
            "KS084" => Ok(ForecastZone::KS084),
            "KS085" => Ok(ForecastZone::KS085),
            "KS086" => Ok(ForecastZone::KS086),
            "KS087" => Ok(ForecastZone::KS087),
            "KS088" => Ok(ForecastZone::KS088),
            "KS089" => Ok(ForecastZone::KS089),
            "KS090" => Ok(ForecastZone::KS090),
            "KS091" => Ok(ForecastZone::KS091),
            "KS092" => Ok(ForecastZone::KS092),
            "KS093" => Ok(ForecastZone::KS093),
            "KS094" => Ok(ForecastZone::KS094),
            "KS095" => Ok(ForecastZone::KS095),
            "KS096" => Ok(ForecastZone::KS096),
            "KS097" => Ok(ForecastZone::KS097),
            "KS098" => Ok(ForecastZone::KS098),
            "KS099" => Ok(ForecastZone::KS099),
            "KS100" => Ok(ForecastZone::KS100),
            "KS101" => Ok(ForecastZone::KS101),
            "KS102" => Ok(ForecastZone::KS102),
            "KS103" => Ok(ForecastZone::KS103),
            "KS104" => Ok(ForecastZone::KS104),
            "KS105" => Ok(ForecastZone::KS105),
            "KY001" => Ok(ForecastZone::KY001),
            "KY002" => Ok(ForecastZone::KY002),
            "KY003" => Ok(ForecastZone::KY003),
            "KY004" => Ok(ForecastZone::KY004),
            "KY005" => Ok(ForecastZone::KY005),
            "KY006" => Ok(ForecastZone::KY006),
            "KY007" => Ok(ForecastZone::KY007),
            "KY008" => Ok(ForecastZone::KY008),
            "KY009" => Ok(ForecastZone::KY009),
            "KY010" => Ok(ForecastZone::KY010),
            "KY011" => Ok(ForecastZone::KY011),
            "KY012" => Ok(ForecastZone::KY012),
            "KY013" => Ok(ForecastZone::KY013),
            "KY014" => Ok(ForecastZone::KY014),
            "KY015" => Ok(ForecastZone::KY015),
            "KY016" => Ok(ForecastZone::KY016),
            "KY017" => Ok(ForecastZone::KY017),
            "KY018" => Ok(ForecastZone::KY018),
            "KY019" => Ok(ForecastZone::KY019),
            "KY020" => Ok(ForecastZone::KY020),
            "KY021" => Ok(ForecastZone::KY021),
            "KY022" => Ok(ForecastZone::KY022),
            "KY023" => Ok(ForecastZone::KY023),
            "KY024" => Ok(ForecastZone::KY024),
            "KY025" => Ok(ForecastZone::KY025),
            "KY026" => Ok(ForecastZone::KY026),
            "KY027" => Ok(ForecastZone::KY027),
            "KY028" => Ok(ForecastZone::KY028),
            "KY029" => Ok(ForecastZone::KY029),
            "KY030" => Ok(ForecastZone::KY030),
            "KY031" => Ok(ForecastZone::KY031),
            "KY032" => Ok(ForecastZone::KY032),
            "KY033" => Ok(ForecastZone::KY033),
            "KY034" => Ok(ForecastZone::KY034),
            "KY035" => Ok(ForecastZone::KY035),
            "KY036" => Ok(ForecastZone::KY036),
            "KY037" => Ok(ForecastZone::KY037),
            "KY038" => Ok(ForecastZone::KY038),
            "KY039" => Ok(ForecastZone::KY039),
            "KY040" => Ok(ForecastZone::KY040),
            "KY041" => Ok(ForecastZone::KY041),
            "KY042" => Ok(ForecastZone::KY042),
            "KY043" => Ok(ForecastZone::KY043),
            "KY044" => Ok(ForecastZone::KY044),
            "KY045" => Ok(ForecastZone::KY045),
            "KY046" => Ok(ForecastZone::KY046),
            "KY047" => Ok(ForecastZone::KY047),
            "KY048" => Ok(ForecastZone::KY048),
            "KY049" => Ok(ForecastZone::KY049),
            "KY050" => Ok(ForecastZone::KY050),
            "KY051" => Ok(ForecastZone::KY051),
            "KY052" => Ok(ForecastZone::KY052),
            "KY053" => Ok(ForecastZone::KY053),
            "KY054" => Ok(ForecastZone::KY054),
            "KY055" => Ok(ForecastZone::KY055),
            "KY056" => Ok(ForecastZone::KY056),
            "KY057" => Ok(ForecastZone::KY057),
            "KY058" => Ok(ForecastZone::KY058),
            "KY059" => Ok(ForecastZone::KY059),
            "KY060" => Ok(ForecastZone::KY060),
            "KY061" => Ok(ForecastZone::KY061),
            "KY062" => Ok(ForecastZone::KY062),
            "KY063" => Ok(ForecastZone::KY063),
            "KY064" => Ok(ForecastZone::KY064),
            "KY065" => Ok(ForecastZone::KY065),
            "KY066" => Ok(ForecastZone::KY066),
            "KY067" => Ok(ForecastZone::KY067),
            "KY068" => Ok(ForecastZone::KY068),
            "KY069" => Ok(ForecastZone::KY069),
            "KY070" => Ok(ForecastZone::KY070),
            "KY071" => Ok(ForecastZone::KY071),
            "KY072" => Ok(ForecastZone::KY072),
            "KY073" => Ok(ForecastZone::KY073),
            "KY074" => Ok(ForecastZone::KY074),
            "KY075" => Ok(ForecastZone::KY075),
            "KY076" => Ok(ForecastZone::KY076),
            "KY077" => Ok(ForecastZone::KY077),
            "KY078" => Ok(ForecastZone::KY078),
            "KY079" => Ok(ForecastZone::KY079),
            "KY080" => Ok(ForecastZone::KY080),
            "KY081" => Ok(ForecastZone::KY081),
            "KY082" => Ok(ForecastZone::KY082),
            "KY083" => Ok(ForecastZone::KY083),
            "KY084" => Ok(ForecastZone::KY084),
            "KY085" => Ok(ForecastZone::KY085),
            "KY086" => Ok(ForecastZone::KY086),
            "KY087" => Ok(ForecastZone::KY087),
            "KY088" => Ok(ForecastZone::KY088),
            "KY089" => Ok(ForecastZone::KY089),
            "KY090" => Ok(ForecastZone::KY090),
            "KY091" => Ok(ForecastZone::KY091),
            "KY092" => Ok(ForecastZone::KY092),
            "KY093" => Ok(ForecastZone::KY093),
            "KY094" => Ok(ForecastZone::KY094),
            "KY095" => Ok(ForecastZone::KY095),
            "KY096" => Ok(ForecastZone::KY096),
            "KY097" => Ok(ForecastZone::KY097),
            "KY098" => Ok(ForecastZone::KY098),
            "KY099" => Ok(ForecastZone::KY099),
            "KY100" => Ok(ForecastZone::KY100),
            "KY101" => Ok(ForecastZone::KY101),
            "KY102" => Ok(ForecastZone::KY102),
            "KY103" => Ok(ForecastZone::KY103),
            "KY104" => Ok(ForecastZone::KY104),
            "KY105" => Ok(ForecastZone::KY105),
            "KY106" => Ok(ForecastZone::KY106),
            "KY107" => Ok(ForecastZone::KY107),
            "KY108" => Ok(ForecastZone::KY108),
            "KY109" => Ok(ForecastZone::KY109),
            "KY110" => Ok(ForecastZone::KY110),
            "KY111" => Ok(ForecastZone::KY111),
            "KY112" => Ok(ForecastZone::KY112),
            "KY113" => Ok(ForecastZone::KY113),
            "KY114" => Ok(ForecastZone::KY114),
            "KY115" => Ok(ForecastZone::KY115),
            "KY116" => Ok(ForecastZone::KY116),
            "KY117" => Ok(ForecastZone::KY117),
            "KY118" => Ok(ForecastZone::KY118),
            "KY119" => Ok(ForecastZone::KY119),
            "KY120" => Ok(ForecastZone::KY120),
            "LA001" => Ok(ForecastZone::LA001),
            "LA002" => Ok(ForecastZone::LA002),
            "LA003" => Ok(ForecastZone::LA003),
            "LA004" => Ok(ForecastZone::LA004),
            "LA005" => Ok(ForecastZone::LA005),
            "LA006" => Ok(ForecastZone::LA006),
            "LA007" => Ok(ForecastZone::LA007),
            "LA008" => Ok(ForecastZone::LA008),
            "LA009" => Ok(ForecastZone::LA009),
            "LA010" => Ok(ForecastZone::LA010),
            "LA011" => Ok(ForecastZone::LA011),
            "LA012" => Ok(ForecastZone::LA012),
            "LA013" => Ok(ForecastZone::LA013),
            "LA014" => Ok(ForecastZone::LA014),
            "LA015" => Ok(ForecastZone::LA015),
            "LA016" => Ok(ForecastZone::LA016),
            "LA017" => Ok(ForecastZone::LA017),
            "LA018" => Ok(ForecastZone::LA018),
            "LA019" => Ok(ForecastZone::LA019),
            "LA020" => Ok(ForecastZone::LA020),
            "LA021" => Ok(ForecastZone::LA021),
            "LA022" => Ok(ForecastZone::LA022),
            "LA023" => Ok(ForecastZone::LA023),
            "LA024" => Ok(ForecastZone::LA024),
            "LA025" => Ok(ForecastZone::LA025),
            "LA026" => Ok(ForecastZone::LA026),
            "LA027" => Ok(ForecastZone::LA027),
            "LA028" => Ok(ForecastZone::LA028),
            "LA029" => Ok(ForecastZone::LA029),
            "LA030" => Ok(ForecastZone::LA030),
            "LA031" => Ok(ForecastZone::LA031),
            "LA032" => Ok(ForecastZone::LA032),
            "LA033" => Ok(ForecastZone::LA033),
            "LA034" => Ok(ForecastZone::LA034),
            "LA035" => Ok(ForecastZone::LA035),
            "LA036" => Ok(ForecastZone::LA036),
            "LA037" => Ok(ForecastZone::LA037),
            "LA039" => Ok(ForecastZone::LA039),
            "LA041" => Ok(ForecastZone::LA041),
            "LA042" => Ok(ForecastZone::LA042),
            "LA043" => Ok(ForecastZone::LA043),
            "LA044" => Ok(ForecastZone::LA044),
            "LA045" => Ok(ForecastZone::LA045),
            "LA046" => Ok(ForecastZone::LA046),
            "LA047" => Ok(ForecastZone::LA047),
            "LA048" => Ok(ForecastZone::LA048),
            "LA052" => Ok(ForecastZone::LA052),
            "LA053" => Ok(ForecastZone::LA053),
            "LA054" => Ok(ForecastZone::LA054),
            "LA055" => Ok(ForecastZone::LA055),
            "LA056" => Ok(ForecastZone::LA056),
            "LA057" => Ok(ForecastZone::LA057),
            "LA058" => Ok(ForecastZone::LA058),
            "LA059" => Ok(ForecastZone::LA059),
            "LA060" => Ok(ForecastZone::LA060),
            "LA064" => Ok(ForecastZone::LA064),
            "LA065" => Ok(ForecastZone::LA065),
            "LA066" => Ok(ForecastZone::LA066),
            "LA067" => Ok(ForecastZone::LA067),
            "LA068" => Ok(ForecastZone::LA068),
            "LA069" => Ok(ForecastZone::LA069),
            "LA070" => Ok(ForecastZone::LA070),
            "LA071" => Ok(ForecastZone::LA071),
            "LA073" => Ok(ForecastZone::LA073),
            "LA074" => Ok(ForecastZone::LA074),
            "LA076" => Ok(ForecastZone::LA076),
            "LA077" => Ok(ForecastZone::LA077),
            "LA078" => Ok(ForecastZone::LA078),
            "LA079" => Ok(ForecastZone::LA079),
            "LA080" => Ok(ForecastZone::LA080),
            "LA081" => Ok(ForecastZone::LA081),
            "LA082" => Ok(ForecastZone::LA082),
            "LA083" => Ok(ForecastZone::LA083),
            "LA084" => Ok(ForecastZone::LA084),
            "LA085" => Ok(ForecastZone::LA085),
            "LA086" => Ok(ForecastZone::LA086),
            "LA087" => Ok(ForecastZone::LA087),
            "LA088" => Ok(ForecastZone::LA088),
            "LA089" => Ok(ForecastZone::LA089),
            "LA090" => Ok(ForecastZone::LA090),
            "MA001" => Ok(ForecastZone::MA001),
            "MA002" => Ok(ForecastZone::MA002),
            "MA003" => Ok(ForecastZone::MA003),
            "MA004" => Ok(ForecastZone::MA004),
            "MA005" => Ok(ForecastZone::MA005),
            "MA006" => Ok(ForecastZone::MA006),
            "MA007" => Ok(ForecastZone::MA007),
            "MA008" => Ok(ForecastZone::MA008),
            "MA009" => Ok(ForecastZone::MA009),
            "MA010" => Ok(ForecastZone::MA010),
            "MA011" => Ok(ForecastZone::MA011),
            "MA012" => Ok(ForecastZone::MA012),
            "MA013" => Ok(ForecastZone::MA013),
            "MA014" => Ok(ForecastZone::MA014),
            "MA015" => Ok(ForecastZone::MA015),
            "MA016" => Ok(ForecastZone::MA016),
            "MA017" => Ok(ForecastZone::MA017),
            "MA018" => Ok(ForecastZone::MA018),
            "MA019" => Ok(ForecastZone::MA019),
            "MA020" => Ok(ForecastZone::MA020),
            "MA021" => Ok(ForecastZone::MA021),
            "MA022" => Ok(ForecastZone::MA022),
            "MA023" => Ok(ForecastZone::MA023),
            "MA024" => Ok(ForecastZone::MA024),
            "MA025" => Ok(ForecastZone::MA025),
            "MA026" => Ok(ForecastZone::MA026),
            "MD001" => Ok(ForecastZone::MD001),
            "MD003" => Ok(ForecastZone::MD003),
            "MD004" => Ok(ForecastZone::MD004),
            "MD005" => Ok(ForecastZone::MD005),
            "MD006" => Ok(ForecastZone::MD006),
            "MD008" => Ok(ForecastZone::MD008),
            "MD011" => Ok(ForecastZone::MD011),
            "MD012" => Ok(ForecastZone::MD012),
            "MD013" => Ok(ForecastZone::MD013),
            "MD014" => Ok(ForecastZone::MD014),
            "MD015" => Ok(ForecastZone::MD015),
            "MD016" => Ok(ForecastZone::MD016),
            "MD017" => Ok(ForecastZone::MD017),
            "MD018" => Ok(ForecastZone::MD018),
            "MD019" => Ok(ForecastZone::MD019),
            "MD020" => Ok(ForecastZone::MD020),
            "MD021" => Ok(ForecastZone::MD021),
            "MD022" => Ok(ForecastZone::MD022),
            "MD023" => Ok(ForecastZone::MD023),
            "MD024" => Ok(ForecastZone::MD024),
            "MD025" => Ok(ForecastZone::MD025),
            "MD501" => Ok(ForecastZone::MD501),
            "MD502" => Ok(ForecastZone::MD502),
            "MD503" => Ok(ForecastZone::MD503),
            "MD504" => Ok(ForecastZone::MD504),
            "MD505" => Ok(ForecastZone::MD505),
            "MD506" => Ok(ForecastZone::MD506),
            "MD507" => Ok(ForecastZone::MD507),
            "MD508" => Ok(ForecastZone::MD508),
            "ME001" => Ok(ForecastZone::ME001),
            "ME002" => Ok(ForecastZone::ME002),
            "ME003" => Ok(ForecastZone::ME003),
            "ME004" => Ok(ForecastZone::ME004),
            "ME005" => Ok(ForecastZone::ME005),
            "ME006" => Ok(ForecastZone::ME006),
            "ME007" => Ok(ForecastZone::ME007),
            "ME008" => Ok(ForecastZone::ME008),
            "ME009" => Ok(ForecastZone::ME009),
            "ME010" => Ok(ForecastZone::ME010),
            "ME011" => Ok(ForecastZone::ME011),
            "ME012" => Ok(ForecastZone::ME012),
            "ME013" => Ok(ForecastZone::ME013),
            "ME014" => Ok(ForecastZone::ME014),
            "ME015" => Ok(ForecastZone::ME015),
            "ME016" => Ok(ForecastZone::ME016),
            "ME017" => Ok(ForecastZone::ME017),
            "ME018" => Ok(ForecastZone::ME018),
            "ME019" => Ok(ForecastZone::ME019),
            "ME020" => Ok(ForecastZone::ME020),
            "ME021" => Ok(ForecastZone::ME021),
            "ME022" => Ok(ForecastZone::ME022),
            "ME023" => Ok(ForecastZone::ME023),
            "ME024" => Ok(ForecastZone::ME024),
            "ME025" => Ok(ForecastZone::ME025),
            "ME026" => Ok(ForecastZone::ME026),
            "ME027" => Ok(ForecastZone::ME027),
            "ME028" => Ok(ForecastZone::ME028),
            "ME029" => Ok(ForecastZone::ME029),
            "ME030" => Ok(ForecastZone::ME030),
            "ME031" => Ok(ForecastZone::ME031),
            "ME032" => Ok(ForecastZone::ME032),
            "ME033" => Ok(ForecastZone::ME033),
            "MH001" => Ok(ForecastZone::MH001),
            "MH002" => Ok(ForecastZone::MH002),
            "MH003" => Ok(ForecastZone::MH003),
            "MH004" => Ok(ForecastZone::MH004),
            "MH005" => Ok(ForecastZone::MH005),
            "MH006" => Ok(ForecastZone::MH006),
            "MH007" => Ok(ForecastZone::MH007),
            "MH008" => Ok(ForecastZone::MH008),
            "MH009" => Ok(ForecastZone::MH009),
            "MH010" => Ok(ForecastZone::MH010),
            "MI001" => Ok(ForecastZone::MI001),
            "MI002" => Ok(ForecastZone::MI002),
            "MI003" => Ok(ForecastZone::MI003),
            "MI004" => Ok(ForecastZone::MI004),
            "MI005" => Ok(ForecastZone::MI005),
            "MI006" => Ok(ForecastZone::MI006),
            "MI007" => Ok(ForecastZone::MI007),
            "MI009" => Ok(ForecastZone::MI009),
            "MI010" => Ok(ForecastZone::MI010),
            "MI011" => Ok(ForecastZone::MI011),
            "MI012" => Ok(ForecastZone::MI012),
            "MI013" => Ok(ForecastZone::MI013),
            "MI014" => Ok(ForecastZone::MI014),
            "MI016" => Ok(ForecastZone::MI016),
            "MI017" => Ok(ForecastZone::MI017),
            "MI018" => Ok(ForecastZone::MI018),
            "MI020" => Ok(ForecastZone::MI020),
            "MI021" => Ok(ForecastZone::MI021),
            "MI022" => Ok(ForecastZone::MI022),
            "MI023" => Ok(ForecastZone::MI023),
            "MI024" => Ok(ForecastZone::MI024),
            "MI025" => Ok(ForecastZone::MI025),
            "MI026" => Ok(ForecastZone::MI026),
            "MI027" => Ok(ForecastZone::MI027),
            "MI028" => Ok(ForecastZone::MI028),
            "MI029" => Ok(ForecastZone::MI029),
            "MI030" => Ok(ForecastZone::MI030),
            "MI031" => Ok(ForecastZone::MI031),
            "MI032" => Ok(ForecastZone::MI032),
            "MI033" => Ok(ForecastZone::MI033),
            "MI034" => Ok(ForecastZone::MI034),
            "MI035" => Ok(ForecastZone::MI035),
            "MI036" => Ok(ForecastZone::MI036),
            "MI037" => Ok(ForecastZone::MI037),
            "MI038" => Ok(ForecastZone::MI038),
            "MI039" => Ok(ForecastZone::MI039),
            "MI040" => Ok(ForecastZone::MI040),
            "MI041" => Ok(ForecastZone::MI041),
            "MI042" => Ok(ForecastZone::MI042),
            "MI043" => Ok(ForecastZone::MI043),
            "MI044" => Ok(ForecastZone::MI044),
            "MI045" => Ok(ForecastZone::MI045),
            "MI046" => Ok(ForecastZone::MI046),
            "MI047" => Ok(ForecastZone::MI047),
            "MI048" => Ok(ForecastZone::MI048),
            "MI049" => Ok(ForecastZone::MI049),
            "MI050" => Ok(ForecastZone::MI050),
            "MI051" => Ok(ForecastZone::MI051),
            "MI052" => Ok(ForecastZone::MI052),
            "MI053" => Ok(ForecastZone::MI053),
            "MI054" => Ok(ForecastZone::MI054),
            "MI055" => Ok(ForecastZone::MI055),
            "MI056" => Ok(ForecastZone::MI056),
            "MI057" => Ok(ForecastZone::MI057),
            "MI058" => Ok(ForecastZone::MI058),
            "MI059" => Ok(ForecastZone::MI059),
            "MI060" => Ok(ForecastZone::MI060),
            "MI061" => Ok(ForecastZone::MI061),
            "MI062" => Ok(ForecastZone::MI062),
            "MI063" => Ok(ForecastZone::MI063),
            "MI064" => Ok(ForecastZone::MI064),
            "MI065" => Ok(ForecastZone::MI065),
            "MI066" => Ok(ForecastZone::MI066),
            "MI067" => Ok(ForecastZone::MI067),
            "MI068" => Ok(ForecastZone::MI068),
            "MI069" => Ok(ForecastZone::MI069),
            "MI070" => Ok(ForecastZone::MI070),
            "MI071" => Ok(ForecastZone::MI071),
            "MI072" => Ok(ForecastZone::MI072),
            "MI073" => Ok(ForecastZone::MI073),
            "MI074" => Ok(ForecastZone::MI074),
            "MI075" => Ok(ForecastZone::MI075),
            "MI076" => Ok(ForecastZone::MI076),
            "MI077" => Ok(ForecastZone::MI077),
            "MI078" => Ok(ForecastZone::MI078),
            "MI079" => Ok(ForecastZone::MI079),
            "MI080" => Ok(ForecastZone::MI080),
            "MI081" => Ok(ForecastZone::MI081),
            "MI082" => Ok(ForecastZone::MI082),
            "MI083" => Ok(ForecastZone::MI083),
            "MI084" => Ok(ForecastZone::MI084),
            "MI085" => Ok(ForecastZone::MI085),
            "MI086" => Ok(ForecastZone::MI086),
            "MI087" => Ok(ForecastZone::MI087),
            "MI088" => Ok(ForecastZone::MI088),
            "MI095" => Ok(ForecastZone::MI095),
            "MI096" => Ok(ForecastZone::MI096),
            "MI097" => Ok(ForecastZone::MI097),
            "MI098" => Ok(ForecastZone::MI098),
            "MI099" => Ok(ForecastZone::MI099),
            "MN001" => Ok(ForecastZone::MN001),
            "MN002" => Ok(ForecastZone::MN002),
            "MN003" => Ok(ForecastZone::MN003),
            "MN004" => Ok(ForecastZone::MN004),
            "MN005" => Ok(ForecastZone::MN005),
            "MN006" => Ok(ForecastZone::MN006),
            "MN007" => Ok(ForecastZone::MN007),
            "MN008" => Ok(ForecastZone::MN008),
            "MN009" => Ok(ForecastZone::MN009),
            "MN010" => Ok(ForecastZone::MN010),
            "MN011" => Ok(ForecastZone::MN011),
            "MN012" => Ok(ForecastZone::MN012),
            "MN013" => Ok(ForecastZone::MN013),
            "MN014" => Ok(ForecastZone::MN014),
            "MN015" => Ok(ForecastZone::MN015),
            "MN016" => Ok(ForecastZone::MN016),
            "MN017" => Ok(ForecastZone::MN017),
            "MN018" => Ok(ForecastZone::MN018),
            "MN019" => Ok(ForecastZone::MN019),
            "MN020" => Ok(ForecastZone::MN020),
            "MN021" => Ok(ForecastZone::MN021),
            "MN022" => Ok(ForecastZone::MN022),
            "MN023" => Ok(ForecastZone::MN023),
            "MN024" => Ok(ForecastZone::MN024),
            "MN025" => Ok(ForecastZone::MN025),
            "MN026" => Ok(ForecastZone::MN026),
            "MN027" => Ok(ForecastZone::MN027),
            "MN028" => Ok(ForecastZone::MN028),
            "MN029" => Ok(ForecastZone::MN029),
            "MN030" => Ok(ForecastZone::MN030),
            "MN031" => Ok(ForecastZone::MN031),
            "MN032" => Ok(ForecastZone::MN032),
            "MN033" => Ok(ForecastZone::MN033),
            "MN034" => Ok(ForecastZone::MN034),
            "MN035" => Ok(ForecastZone::MN035),
            "MN036" => Ok(ForecastZone::MN036),
            "MN037" => Ok(ForecastZone::MN037),
            "MN038" => Ok(ForecastZone::MN038),
            "MN039" => Ok(ForecastZone::MN039),
            "MN040" => Ok(ForecastZone::MN040),
            "MN041" => Ok(ForecastZone::MN041),
            "MN042" => Ok(ForecastZone::MN042),
            "MN043" => Ok(ForecastZone::MN043),
            "MN044" => Ok(ForecastZone::MN044),
            "MN045" => Ok(ForecastZone::MN045),
            "MN046" => Ok(ForecastZone::MN046),
            "MN047" => Ok(ForecastZone::MN047),
            "MN048" => Ok(ForecastZone::MN048),
            "MN049" => Ok(ForecastZone::MN049),
            "MN050" => Ok(ForecastZone::MN050),
            "MN051" => Ok(ForecastZone::MN051),
            "MN052" => Ok(ForecastZone::MN052),
            "MN053" => Ok(ForecastZone::MN053),
            "MN054" => Ok(ForecastZone::MN054),
            "MN055" => Ok(ForecastZone::MN055),
            "MN056" => Ok(ForecastZone::MN056),
            "MN057" => Ok(ForecastZone::MN057),
            "MN058" => Ok(ForecastZone::MN058),
            "MN059" => Ok(ForecastZone::MN059),
            "MN060" => Ok(ForecastZone::MN060),
            "MN061" => Ok(ForecastZone::MN061),
            "MN062" => Ok(ForecastZone::MN062),
            "MN063" => Ok(ForecastZone::MN063),
            "MN064" => Ok(ForecastZone::MN064),
            "MN065" => Ok(ForecastZone::MN065),
            "MN066" => Ok(ForecastZone::MN066),
            "MN067" => Ok(ForecastZone::MN067),
            "MN068" => Ok(ForecastZone::MN068),
            "MN069" => Ok(ForecastZone::MN069),
            "MN070" => Ok(ForecastZone::MN070),
            "MN071" => Ok(ForecastZone::MN071),
            "MN072" => Ok(ForecastZone::MN072),
            "MN073" => Ok(ForecastZone::MN073),
            "MN074" => Ok(ForecastZone::MN074),
            "MN075" => Ok(ForecastZone::MN075),
            "MN076" => Ok(ForecastZone::MN076),
            "MN077" => Ok(ForecastZone::MN077),
            "MN078" => Ok(ForecastZone::MN078),
            "MN079" => Ok(ForecastZone::MN079),
            "MN080" => Ok(ForecastZone::MN080),
            "MN081" => Ok(ForecastZone::MN081),
            "MN082" => Ok(ForecastZone::MN082),
            "MN083" => Ok(ForecastZone::MN083),
            "MN084" => Ok(ForecastZone::MN084),
            "MN085" => Ok(ForecastZone::MN085),
            "MN086" => Ok(ForecastZone::MN086),
            "MN087" => Ok(ForecastZone::MN087),
            "MN088" => Ok(ForecastZone::MN088),
            "MN089" => Ok(ForecastZone::MN089),
            "MN090" => Ok(ForecastZone::MN090),
            "MN091" => Ok(ForecastZone::MN091),
            "MN092" => Ok(ForecastZone::MN092),
            "MN093" => Ok(ForecastZone::MN093),
            "MN094" => Ok(ForecastZone::MN094),
            "MN095" => Ok(ForecastZone::MN095),
            "MN096" => Ok(ForecastZone::MN096),
            "MN097" => Ok(ForecastZone::MN097),
            "MN098" => Ok(ForecastZone::MN098),
            "MO001" => Ok(ForecastZone::MO001),
            "MO002" => Ok(ForecastZone::MO002),
            "MO003" => Ok(ForecastZone::MO003),
            "MO004" => Ok(ForecastZone::MO004),
            "MO005" => Ok(ForecastZone::MO005),
            "MO006" => Ok(ForecastZone::MO006),
            "MO007" => Ok(ForecastZone::MO007),
            "MO008" => Ok(ForecastZone::MO008),
            "MO009" => Ok(ForecastZone::MO009),
            "MO010" => Ok(ForecastZone::MO010),
            "MO011" => Ok(ForecastZone::MO011),
            "MO012" => Ok(ForecastZone::MO012),
            "MO013" => Ok(ForecastZone::MO013),
            "MO014" => Ok(ForecastZone::MO014),
            "MO015" => Ok(ForecastZone::MO015),
            "MO016" => Ok(ForecastZone::MO016),
            "MO017" => Ok(ForecastZone::MO017),
            "MO018" => Ok(ForecastZone::MO018),
            "MO019" => Ok(ForecastZone::MO019),
            "MO020" => Ok(ForecastZone::MO020),
            "MO021" => Ok(ForecastZone::MO021),
            "MO022" => Ok(ForecastZone::MO022),
            "MO023" => Ok(ForecastZone::MO023),
            "MO024" => Ok(ForecastZone::MO024),
            "MO025" => Ok(ForecastZone::MO025),
            "MO026" => Ok(ForecastZone::MO026),
            "MO027" => Ok(ForecastZone::MO027),
            "MO028" => Ok(ForecastZone::MO028),
            "MO029" => Ok(ForecastZone::MO029),
            "MO030" => Ok(ForecastZone::MO030),
            "MO031" => Ok(ForecastZone::MO031),
            "MO032" => Ok(ForecastZone::MO032),
            "MO033" => Ok(ForecastZone::MO033),
            "MO034" => Ok(ForecastZone::MO034),
            "MO035" => Ok(ForecastZone::MO035),
            "MO036" => Ok(ForecastZone::MO036),
            "MO037" => Ok(ForecastZone::MO037),
            "MO038" => Ok(ForecastZone::MO038),
            "MO039" => Ok(ForecastZone::MO039),
            "MO040" => Ok(ForecastZone::MO040),
            "MO041" => Ok(ForecastZone::MO041),
            "MO042" => Ok(ForecastZone::MO042),
            "MO043" => Ok(ForecastZone::MO043),
            "MO044" => Ok(ForecastZone::MO044),
            "MO045" => Ok(ForecastZone::MO045),
            "MO046" => Ok(ForecastZone::MO046),
            "MO047" => Ok(ForecastZone::MO047),
            "MO048" => Ok(ForecastZone::MO048),
            "MO049" => Ok(ForecastZone::MO049),
            "MO050" => Ok(ForecastZone::MO050),
            "MO051" => Ok(ForecastZone::MO051),
            "MO052" => Ok(ForecastZone::MO052),
            "MO053" => Ok(ForecastZone::MO053),
            "MO054" => Ok(ForecastZone::MO054),
            "MO055" => Ok(ForecastZone::MO055),
            "MO056" => Ok(ForecastZone::MO056),
            "MO057" => Ok(ForecastZone::MO057),
            "MO058" => Ok(ForecastZone::MO058),
            "MO059" => Ok(ForecastZone::MO059),
            "MO060" => Ok(ForecastZone::MO060),
            "MO061" => Ok(ForecastZone::MO061),
            "MO062" => Ok(ForecastZone::MO062),
            "MO063" => Ok(ForecastZone::MO063),
            "MO064" => Ok(ForecastZone::MO064),
            "MO065" => Ok(ForecastZone::MO065),
            "MO066" => Ok(ForecastZone::MO066),
            "MO067" => Ok(ForecastZone::MO067),
            "MO068" => Ok(ForecastZone::MO068),
            "MO069" => Ok(ForecastZone::MO069),
            "MO070" => Ok(ForecastZone::MO070),
            "MO071" => Ok(ForecastZone::MO071),
            "MO072" => Ok(ForecastZone::MO072),
            "MO073" => Ok(ForecastZone::MO073),
            "MO074" => Ok(ForecastZone::MO074),
            "MO075" => Ok(ForecastZone::MO075),
            "MO076" => Ok(ForecastZone::MO076),
            "MO077" => Ok(ForecastZone::MO077),
            "MO078" => Ok(ForecastZone::MO078),
            "MO079" => Ok(ForecastZone::MO079),
            "MO080" => Ok(ForecastZone::MO080),
            "MO081" => Ok(ForecastZone::MO081),
            "MO082" => Ok(ForecastZone::MO082),
            "MO083" => Ok(ForecastZone::MO083),
            "MO084" => Ok(ForecastZone::MO084),
            "MO085" => Ok(ForecastZone::MO085),
            "MO086" => Ok(ForecastZone::MO086),
            "MO087" => Ok(ForecastZone::MO087),
            "MO088" => Ok(ForecastZone::MO088),
            "MO089" => Ok(ForecastZone::MO089),
            "MO090" => Ok(ForecastZone::MO090),
            "MO091" => Ok(ForecastZone::MO091),
            "MO092" => Ok(ForecastZone::MO092),
            "MO093" => Ok(ForecastZone::MO093),
            "MO094" => Ok(ForecastZone::MO094),
            "MO095" => Ok(ForecastZone::MO095),
            "MO096" => Ok(ForecastZone::MO096),
            "MO097" => Ok(ForecastZone::MO097),
            "MO098" => Ok(ForecastZone::MO098),
            "MO099" => Ok(ForecastZone::MO099),
            "MO100" => Ok(ForecastZone::MO100),
            "MO101" => Ok(ForecastZone::MO101),
            "MO102" => Ok(ForecastZone::MO102),
            "MO103" => Ok(ForecastZone::MO103),
            "MO104" => Ok(ForecastZone::MO104),
            "MO105" => Ok(ForecastZone::MO105),
            "MO106" => Ok(ForecastZone::MO106),
            "MO107" => Ok(ForecastZone::MO107),
            "MO108" => Ok(ForecastZone::MO108),
            "MO109" => Ok(ForecastZone::MO109),
            "MO110" => Ok(ForecastZone::MO110),
            "MO111" => Ok(ForecastZone::MO111),
            "MO112" => Ok(ForecastZone::MO112),
            "MO113" => Ok(ForecastZone::MO113),
            "MO114" => Ok(ForecastZone::MO114),
            "MO115" => Ok(ForecastZone::MO115),
            "MP001" => Ok(ForecastZone::MP001),
            "MP002" => Ok(ForecastZone::MP002),
            "MP003" => Ok(ForecastZone::MP003),
            "MP004" => Ok(ForecastZone::MP004),
            "MP005" => Ok(ForecastZone::MP005),
            "MP006" => Ok(ForecastZone::MP006),
            "MP007" => Ok(ForecastZone::MP007),
            "MS001" => Ok(ForecastZone::MS001),
            "MS002" => Ok(ForecastZone::MS002),
            "MS003" => Ok(ForecastZone::MS003),
            "MS004" => Ok(ForecastZone::MS004),
            "MS005" => Ok(ForecastZone::MS005),
            "MS006" => Ok(ForecastZone::MS006),
            "MS007" => Ok(ForecastZone::MS007),
            "MS008" => Ok(ForecastZone::MS008),
            "MS009" => Ok(ForecastZone::MS009),
            "MS010" => Ok(ForecastZone::MS010),
            "MS011" => Ok(ForecastZone::MS011),
            "MS012" => Ok(ForecastZone::MS012),
            "MS013" => Ok(ForecastZone::MS013),
            "MS014" => Ok(ForecastZone::MS014),
            "MS015" => Ok(ForecastZone::MS015),
            "MS016" => Ok(ForecastZone::MS016),
            "MS017" => Ok(ForecastZone::MS017),
            "MS018" => Ok(ForecastZone::MS018),
            "MS019" => Ok(ForecastZone::MS019),
            "MS020" => Ok(ForecastZone::MS020),
            "MS021" => Ok(ForecastZone::MS021),
            "MS022" => Ok(ForecastZone::MS022),
            "MS023" => Ok(ForecastZone::MS023),
            "MS024" => Ok(ForecastZone::MS024),
            "MS025" => Ok(ForecastZone::MS025),
            "MS026" => Ok(ForecastZone::MS026),
            "MS027" => Ok(ForecastZone::MS027),
            "MS028" => Ok(ForecastZone::MS028),
            "MS029" => Ok(ForecastZone::MS029),
            "MS030" => Ok(ForecastZone::MS030),
            "MS031" => Ok(ForecastZone::MS031),
            "MS032" => Ok(ForecastZone::MS032),
            "MS033" => Ok(ForecastZone::MS033),
            "MS034" => Ok(ForecastZone::MS034),
            "MS035" => Ok(ForecastZone::MS035),
            "MS036" => Ok(ForecastZone::MS036),
            "MS037" => Ok(ForecastZone::MS037),
            "MS038" => Ok(ForecastZone::MS038),
            "MS039" => Ok(ForecastZone::MS039),
            "MS040" => Ok(ForecastZone::MS040),
            "MS041" => Ok(ForecastZone::MS041),
            "MS042" => Ok(ForecastZone::MS042),
            "MS043" => Ok(ForecastZone::MS043),
            "MS044" => Ok(ForecastZone::MS044),
            "MS045" => Ok(ForecastZone::MS045),
            "MS046" => Ok(ForecastZone::MS046),
            "MS047" => Ok(ForecastZone::MS047),
            "MS048" => Ok(ForecastZone::MS048),
            "MS049" => Ok(ForecastZone::MS049),
            "MS050" => Ok(ForecastZone::MS050),
            "MS051" => Ok(ForecastZone::MS051),
            "MS052" => Ok(ForecastZone::MS052),
            "MS053" => Ok(ForecastZone::MS053),
            "MS054" => Ok(ForecastZone::MS054),
            "MS055" => Ok(ForecastZone::MS055),
            "MS056" => Ok(ForecastZone::MS056),
            "MS057" => Ok(ForecastZone::MS057),
            "MS058" => Ok(ForecastZone::MS058),
            "MS059" => Ok(ForecastZone::MS059),
            "MS060" => Ok(ForecastZone::MS060),
            "MS061" => Ok(ForecastZone::MS061),
            "MS062" => Ok(ForecastZone::MS062),
            "MS063" => Ok(ForecastZone::MS063),
            "MS064" => Ok(ForecastZone::MS064),
            "MS065" => Ok(ForecastZone::MS065),
            "MS066" => Ok(ForecastZone::MS066),
            "MS067" => Ok(ForecastZone::MS067),
            "MS068" => Ok(ForecastZone::MS068),
            "MS069" => Ok(ForecastZone::MS069),
            "MS070" => Ok(ForecastZone::MS070),
            "MS071" => Ok(ForecastZone::MS071),
            "MS072" => Ok(ForecastZone::MS072),
            "MS073" => Ok(ForecastZone::MS073),
            "MS074" => Ok(ForecastZone::MS074),
            "MS075" => Ok(ForecastZone::MS075),
            "MS076" => Ok(ForecastZone::MS076),
            "MS077" => Ok(ForecastZone::MS077),
            "MS078" => Ok(ForecastZone::MS078),
            "MS079" => Ok(ForecastZone::MS079),
            "MS080" => Ok(ForecastZone::MS080),
            "MS081" => Ok(ForecastZone::MS081),
            "MS082" => Ok(ForecastZone::MS082),
            "MT001" => Ok(ForecastZone::MT001),
            "MT002" => Ok(ForecastZone::MT002),
            "MT003" => Ok(ForecastZone::MT003),
            "MT004" => Ok(ForecastZone::MT004),
            "MT005" => Ok(ForecastZone::MT005),
            "MT006" => Ok(ForecastZone::MT006),
            "MT007" => Ok(ForecastZone::MT007),
            "MT008" => Ok(ForecastZone::MT008),
            "MT009" => Ok(ForecastZone::MT009),
            "MT010" => Ok(ForecastZone::MT010),
            "MT011" => Ok(ForecastZone::MT011),
            "MT012" => Ok(ForecastZone::MT012),
            "MT013" => Ok(ForecastZone::MT013),
            "MT014" => Ok(ForecastZone::MT014),
            "MT015" => Ok(ForecastZone::MT015),
            "MT016" => Ok(ForecastZone::MT016),
            "MT017" => Ok(ForecastZone::MT017),
            "MT018" => Ok(ForecastZone::MT018),
            "MT019" => Ok(ForecastZone::MT019),
            "MT020" => Ok(ForecastZone::MT020),
            "MT021" => Ok(ForecastZone::MT021),
            "MT022" => Ok(ForecastZone::MT022),
            "MT023" => Ok(ForecastZone::MT023),
            "MT024" => Ok(ForecastZone::MT024),
            "MT025" => Ok(ForecastZone::MT025),
            "MT026" => Ok(ForecastZone::MT026),
            "MT027" => Ok(ForecastZone::MT027),
            "MT029" => Ok(ForecastZone::MT029),
            "MT030" => Ok(ForecastZone::MT030),
            "MT031" => Ok(ForecastZone::MT031),
            "MT032" => Ok(ForecastZone::MT032),
            "MT033" => Ok(ForecastZone::MT033),
            "MT034" => Ok(ForecastZone::MT034),
            "MT036" => Ok(ForecastZone::MT036),
            "MT037" => Ok(ForecastZone::MT037),
            "MT040" => Ok(ForecastZone::MT040),
            "MT042" => Ok(ForecastZone::MT042),
            "MT043" => Ok(ForecastZone::MT043),
            "MT044" => Ok(ForecastZone::MT044),
            "MT045" => Ok(ForecastZone::MT045),
            "MT046" => Ok(ForecastZone::MT046),
            "MT047" => Ok(ForecastZone::MT047),
            "MT048" => Ok(ForecastZone::MT048),
            "MT049" => Ok(ForecastZone::MT049),
            "MT050" => Ok(ForecastZone::MT050),
            "MT051" => Ok(ForecastZone::MT051),
            "MT052" => Ok(ForecastZone::MT052),
            "MT053" => Ok(ForecastZone::MT053),
            "MT054" => Ok(ForecastZone::MT054),
            "MT055" => Ok(ForecastZone::MT055),
            "MT056" => Ok(ForecastZone::MT056),
            "MT057" => Ok(ForecastZone::MT057),
            "MT058" => Ok(ForecastZone::MT058),
            "MT059" => Ok(ForecastZone::MT059),
            "MT060" => Ok(ForecastZone::MT060),
            "MT061" => Ok(ForecastZone::MT061),
            "MT062" => Ok(ForecastZone::MT062),
            "MT063" => Ok(ForecastZone::MT063),
            "MT064" => Ok(ForecastZone::MT064),
            "MT065" => Ok(ForecastZone::MT065),
            "MT066" => Ok(ForecastZone::MT066),
            "MT067" => Ok(ForecastZone::MT067),
            "MT068" => Ok(ForecastZone::MT068),
            "MT138" => Ok(ForecastZone::MT138),
            "MT139" => Ok(ForecastZone::MT139),
            "MT141" => Ok(ForecastZone::MT141),
            "MT169" => Ok(ForecastZone::MT169),
            "MT170" => Ok(ForecastZone::MT170),
            "MT171" => Ok(ForecastZone::MT171),
            "MT172" => Ok(ForecastZone::MT172),
            "MT173" => Ok(ForecastZone::MT173),
            "MT228" => Ok(ForecastZone::MT228),
            "MT235" => Ok(ForecastZone::MT235),
            "NC001" => Ok(ForecastZone::NC001),
            "NC002" => Ok(ForecastZone::NC002),
            "NC003" => Ok(ForecastZone::NC003),
            "NC004" => Ok(ForecastZone::NC004),
            "NC005" => Ok(ForecastZone::NC005),
            "NC006" => Ok(ForecastZone::NC006),
            "NC007" => Ok(ForecastZone::NC007),
            "NC008" => Ok(ForecastZone::NC008),
            "NC009" => Ok(ForecastZone::NC009),
            "NC010" => Ok(ForecastZone::NC010),
            "NC011" => Ok(ForecastZone::NC011),
            "NC012" => Ok(ForecastZone::NC012),
            "NC013" => Ok(ForecastZone::NC013),
            "NC014" => Ok(ForecastZone::NC014),
            "NC015" => Ok(ForecastZone::NC015),
            "NC016" => Ok(ForecastZone::NC016),
            "NC017" => Ok(ForecastZone::NC017),
            "NC018" => Ok(ForecastZone::NC018),
            "NC019" => Ok(ForecastZone::NC019),
            "NC020" => Ok(ForecastZone::NC020),
            "NC021" => Ok(ForecastZone::NC021),
            "NC022" => Ok(ForecastZone::NC022),
            "NC023" => Ok(ForecastZone::NC023),
            "NC024" => Ok(ForecastZone::NC024),
            "NC025" => Ok(ForecastZone::NC025),
            "NC026" => Ok(ForecastZone::NC026),
            "NC027" => Ok(ForecastZone::NC027),
            "NC028" => Ok(ForecastZone::NC028),
            "NC029" => Ok(ForecastZone::NC029),
            "NC030" => Ok(ForecastZone::NC030),
            "NC031" => Ok(ForecastZone::NC031),
            "NC032" => Ok(ForecastZone::NC032),
            "NC033" => Ok(ForecastZone::NC033),
            "NC035" => Ok(ForecastZone::NC035),
            "NC036" => Ok(ForecastZone::NC036),
            "NC037" => Ok(ForecastZone::NC037),
            "NC038" => Ok(ForecastZone::NC038),
            "NC039" => Ok(ForecastZone::NC039),
            "NC040" => Ok(ForecastZone::NC040),
            "NC041" => Ok(ForecastZone::NC041),
            "NC042" => Ok(ForecastZone::NC042),
            "NC043" => Ok(ForecastZone::NC043),
            "NC044" => Ok(ForecastZone::NC044),
            "NC045" => Ok(ForecastZone::NC045),
            "NC046" => Ok(ForecastZone::NC046),
            "NC047" => Ok(ForecastZone::NC047),
            "NC048" => Ok(ForecastZone::NC048),
            "NC049" => Ok(ForecastZone::NC049),
            "NC050" => Ok(ForecastZone::NC050),
            "NC051" => Ok(ForecastZone::NC051),
            "NC052" => Ok(ForecastZone::NC052),
            "NC053" => Ok(ForecastZone::NC053),
            "NC056" => Ok(ForecastZone::NC056),
            "NC057" => Ok(ForecastZone::NC057),
            "NC058" => Ok(ForecastZone::NC058),
            "NC059" => Ok(ForecastZone::NC059),
            "NC060" => Ok(ForecastZone::NC060),
            "NC061" => Ok(ForecastZone::NC061),
            "NC062" => Ok(ForecastZone::NC062),
            "NC063" => Ok(ForecastZone::NC063),
            "NC064" => Ok(ForecastZone::NC064),
            "NC065" => Ok(ForecastZone::NC065),
            "NC068" => Ok(ForecastZone::NC068),
            "NC069" => Ok(ForecastZone::NC069),
            "NC070" => Ok(ForecastZone::NC070),
            "NC071" => Ok(ForecastZone::NC071),
            "NC072" => Ok(ForecastZone::NC072),
            "NC073" => Ok(ForecastZone::NC073),
            "NC074" => Ok(ForecastZone::NC074),
            "NC075" => Ok(ForecastZone::NC075),
            "NC076" => Ok(ForecastZone::NC076),
            "NC077" => Ok(ForecastZone::NC077),
            "NC078" => Ok(ForecastZone::NC078),
            "NC079" => Ok(ForecastZone::NC079),
            "NC080" => Ok(ForecastZone::NC080),
            "NC081" => Ok(ForecastZone::NC081),
            "NC082" => Ok(ForecastZone::NC082),
            "NC083" => Ok(ForecastZone::NC083),
            "NC084" => Ok(ForecastZone::NC084),
            "NC085" => Ok(ForecastZone::NC085),
            "NC086" => Ok(ForecastZone::NC086),
            "NC087" => Ok(ForecastZone::NC087),
            "NC088" => Ok(ForecastZone::NC088),
            "NC089" => Ok(ForecastZone::NC089),
            "NC090" => Ok(ForecastZone::NC090),
            "NC091" => Ok(ForecastZone::NC091),
            "NC092" => Ok(ForecastZone::NC092),
            "NC094" => Ok(ForecastZone::NC094),
            "NC096" => Ok(ForecastZone::NC096),
            "NC099" => Ok(ForecastZone::NC099),
            "NC102" => Ok(ForecastZone::NC102),
            "NC105" => Ok(ForecastZone::NC105),
            "NC106" => Ok(ForecastZone::NC106),
            "NC107" => Ok(ForecastZone::NC107),
            "NC108" => Ok(ForecastZone::NC108),
            "NC109" => Ok(ForecastZone::NC109),
            "NC110" => Ok(ForecastZone::NC110),
            "NC193" => Ok(ForecastZone::NC193),
            "NC194" => Ok(ForecastZone::NC194),
            "NC195" => Ok(ForecastZone::NC195),
            "NC196" => Ok(ForecastZone::NC196),
            "NC198" => Ok(ForecastZone::NC198),
            "NC199" => Ok(ForecastZone::NC199),
            "NC203" => Ok(ForecastZone::NC203),
            "NC204" => Ok(ForecastZone::NC204),
            "NC205" => Ok(ForecastZone::NC205),
            "NC501" => Ok(ForecastZone::NC501),
            "NC502" => Ok(ForecastZone::NC502),
            "NC503" => Ok(ForecastZone::NC503),
            "NC504" => Ok(ForecastZone::NC504),
            "NC505" => Ok(ForecastZone::NC505),
            "NC506" => Ok(ForecastZone::NC506),
            "NC507" => Ok(ForecastZone::NC507),
            "NC508" => Ok(ForecastZone::NC508),
            "NC509" => Ok(ForecastZone::NC509),
            "NC510" => Ok(ForecastZone::NC510),
            "ND001" => Ok(ForecastZone::ND001),
            "ND002" => Ok(ForecastZone::ND002),
            "ND003" => Ok(ForecastZone::ND003),
            "ND004" => Ok(ForecastZone::ND004),
            "ND005" => Ok(ForecastZone::ND005),
            "ND006" => Ok(ForecastZone::ND006),
            "ND007" => Ok(ForecastZone::ND007),
            "ND008" => Ok(ForecastZone::ND008),
            "ND009" => Ok(ForecastZone::ND009),
            "ND010" => Ok(ForecastZone::ND010),
            "ND011" => Ok(ForecastZone::ND011),
            "ND012" => Ok(ForecastZone::ND012),
            "ND013" => Ok(ForecastZone::ND013),
            "ND014" => Ok(ForecastZone::ND014),
            "ND015" => Ok(ForecastZone::ND015),
            "ND016" => Ok(ForecastZone::ND016),
            "ND017" => Ok(ForecastZone::ND017),
            "ND018" => Ok(ForecastZone::ND018),
            "ND019" => Ok(ForecastZone::ND019),
            "ND020" => Ok(ForecastZone::ND020),
            "ND021" => Ok(ForecastZone::ND021),
            "ND022" => Ok(ForecastZone::ND022),
            "ND023" => Ok(ForecastZone::ND023),
            "ND024" => Ok(ForecastZone::ND024),
            "ND025" => Ok(ForecastZone::ND025),
            "ND026" => Ok(ForecastZone::ND026),
            "ND027" => Ok(ForecastZone::ND027),
            "ND028" => Ok(ForecastZone::ND028),
            "ND029" => Ok(ForecastZone::ND029),
            "ND030" => Ok(ForecastZone::ND030),
            "ND031" => Ok(ForecastZone::ND031),
            "ND032" => Ok(ForecastZone::ND032),
            "ND033" => Ok(ForecastZone::ND033),
            "ND034" => Ok(ForecastZone::ND034),
            "ND035" => Ok(ForecastZone::ND035),
            "ND036" => Ok(ForecastZone::ND036),
            "ND037" => Ok(ForecastZone::ND037),
            "ND038" => Ok(ForecastZone::ND038),
            "ND039" => Ok(ForecastZone::ND039),
            "ND040" => Ok(ForecastZone::ND040),
            "ND041" => Ok(ForecastZone::ND041),
            "ND042" => Ok(ForecastZone::ND042),
            "ND043" => Ok(ForecastZone::ND043),
            "ND044" => Ok(ForecastZone::ND044),
            "ND045" => Ok(ForecastZone::ND045),
            "ND046" => Ok(ForecastZone::ND046),
            "ND047" => Ok(ForecastZone::ND047),
            "ND048" => Ok(ForecastZone::ND048),
            "ND049" => Ok(ForecastZone::ND049),
            "ND050" => Ok(ForecastZone::ND050),
            "ND051" => Ok(ForecastZone::ND051),
            "ND052" => Ok(ForecastZone::ND052),
            "ND053" => Ok(ForecastZone::ND053),
            "ND054" => Ok(ForecastZone::ND054),
            "NE002" => Ok(ForecastZone::NE002),
            "NE003" => Ok(ForecastZone::NE003),
            "NE004" => Ok(ForecastZone::NE004),
            "NE005" => Ok(ForecastZone::NE005),
            "NE006" => Ok(ForecastZone::NE006),
            "NE007" => Ok(ForecastZone::NE007),
            "NE008" => Ok(ForecastZone::NE008),
            "NE009" => Ok(ForecastZone::NE009),
            "NE010" => Ok(ForecastZone::NE010),
            "NE011" => Ok(ForecastZone::NE011),
            "NE012" => Ok(ForecastZone::NE012),
            "NE013" => Ok(ForecastZone::NE013),
            "NE014" => Ok(ForecastZone::NE014),
            "NE015" => Ok(ForecastZone::NE015),
            "NE016" => Ok(ForecastZone::NE016),
            "NE017" => Ok(ForecastZone::NE017),
            "NE018" => Ok(ForecastZone::NE018),
            "NE019" => Ok(ForecastZone::NE019),
            "NE020" => Ok(ForecastZone::NE020),
            "NE021" => Ok(ForecastZone::NE021),
            "NE022" => Ok(ForecastZone::NE022),
            "NE023" => Ok(ForecastZone::NE023),
            "NE024" => Ok(ForecastZone::NE024),
            "NE025" => Ok(ForecastZone::NE025),
            "NE026" => Ok(ForecastZone::NE026),
            "NE027" => Ok(ForecastZone::NE027),
            "NE028" => Ok(ForecastZone::NE028),
            "NE029" => Ok(ForecastZone::NE029),
            "NE030" => Ok(ForecastZone::NE030),
            "NE031" => Ok(ForecastZone::NE031),
            "NE032" => Ok(ForecastZone::NE032),
            "NE033" => Ok(ForecastZone::NE033),
            "NE034" => Ok(ForecastZone::NE034),
            "NE035" => Ok(ForecastZone::NE035),
            "NE036" => Ok(ForecastZone::NE036),
            "NE037" => Ok(ForecastZone::NE037),
            "NE038" => Ok(ForecastZone::NE038),
            "NE039" => Ok(ForecastZone::NE039),
            "NE040" => Ok(ForecastZone::NE040),
            "NE041" => Ok(ForecastZone::NE041),
            "NE042" => Ok(ForecastZone::NE042),
            "NE043" => Ok(ForecastZone::NE043),
            "NE044" => Ok(ForecastZone::NE044),
            "NE045" => Ok(ForecastZone::NE045),
            "NE046" => Ok(ForecastZone::NE046),
            "NE047" => Ok(ForecastZone::NE047),
            "NE048" => Ok(ForecastZone::NE048),
            "NE049" => Ok(ForecastZone::NE049),
            "NE050" => Ok(ForecastZone::NE050),
            "NE051" => Ok(ForecastZone::NE051),
            "NE052" => Ok(ForecastZone::NE052),
            "NE053" => Ok(ForecastZone::NE053),
            "NE054" => Ok(ForecastZone::NE054),
            "NE055" => Ok(ForecastZone::NE055),
            "NE056" => Ok(ForecastZone::NE056),
            "NE057" => Ok(ForecastZone::NE057),
            "NE058" => Ok(ForecastZone::NE058),
            "NE059" => Ok(ForecastZone::NE059),
            "NE060" => Ok(ForecastZone::NE060),
            "NE061" => Ok(ForecastZone::NE061),
            "NE062" => Ok(ForecastZone::NE062),
            "NE063" => Ok(ForecastZone::NE063),
            "NE064" => Ok(ForecastZone::NE064),
            "NE065" => Ok(ForecastZone::NE065),
            "NE066" => Ok(ForecastZone::NE066),
            "NE067" => Ok(ForecastZone::NE067),
            "NE068" => Ok(ForecastZone::NE068),
            "NE069" => Ok(ForecastZone::NE069),
            "NE070" => Ok(ForecastZone::NE070),
            "NE071" => Ok(ForecastZone::NE071),
            "NE072" => Ok(ForecastZone::NE072),
            "NE073" => Ok(ForecastZone::NE073),
            "NE074" => Ok(ForecastZone::NE074),
            "NE075" => Ok(ForecastZone::NE075),
            "NE076" => Ok(ForecastZone::NE076),
            "NE077" => Ok(ForecastZone::NE077),
            "NE078" => Ok(ForecastZone::NE078),
            "NE079" => Ok(ForecastZone::NE079),
            "NE080" => Ok(ForecastZone::NE080),
            "NE081" => Ok(ForecastZone::NE081),
            "NE082" => Ok(ForecastZone::NE082),
            "NE083" => Ok(ForecastZone::NE083),
            "NE084" => Ok(ForecastZone::NE084),
            "NE085" => Ok(ForecastZone::NE085),
            "NE086" => Ok(ForecastZone::NE086),
            "NE087" => Ok(ForecastZone::NE087),
            "NE088" => Ok(ForecastZone::NE088),
            "NE089" => Ok(ForecastZone::NE089),
            "NE090" => Ok(ForecastZone::NE090),
            "NE091" => Ok(ForecastZone::NE091),
            "NE092" => Ok(ForecastZone::NE092),
            "NE093" => Ok(ForecastZone::NE093),
            "NE094" => Ok(ForecastZone::NE094),
            "NE095" => Ok(ForecastZone::NE095),
            "NE096" => Ok(ForecastZone::NE096),
            "NH001" => Ok(ForecastZone::NH001),
            "NH002" => Ok(ForecastZone::NH002),
            "NH003" => Ok(ForecastZone::NH003),
            "NH004" => Ok(ForecastZone::NH004),
            "NH005" => Ok(ForecastZone::NH005),
            "NH006" => Ok(ForecastZone::NH006),
            "NH007" => Ok(ForecastZone::NH007),
            "NH008" => Ok(ForecastZone::NH008),
            "NH009" => Ok(ForecastZone::NH009),
            "NH010" => Ok(ForecastZone::NH010),
            "NH011" => Ok(ForecastZone::NH011),
            "NH012" => Ok(ForecastZone::NH012),
            "NH013" => Ok(ForecastZone::NH013),
            "NH014" => Ok(ForecastZone::NH014),
            "NH015" => Ok(ForecastZone::NH015),
            "NJ001" => Ok(ForecastZone::NJ001),
            "NJ002" => Ok(ForecastZone::NJ002),
            "NJ004" => Ok(ForecastZone::NJ004),
            "NJ006" => Ok(ForecastZone::NJ006),
            "NJ007" => Ok(ForecastZone::NJ007),
            "NJ008" => Ok(ForecastZone::NJ008),
            "NJ009" => Ok(ForecastZone::NJ009),
            "NJ010" => Ok(ForecastZone::NJ010),
            "NJ012" => Ok(ForecastZone::NJ012),
            "NJ013" => Ok(ForecastZone::NJ013),
            "NJ014" => Ok(ForecastZone::NJ014),
            "NJ015" => Ok(ForecastZone::NJ015),
            "NJ016" => Ok(ForecastZone::NJ016),
            "NJ017" => Ok(ForecastZone::NJ017),
            "NJ018" => Ok(ForecastZone::NJ018),
            "NJ019" => Ok(ForecastZone::NJ019),
            "NJ020" => Ok(ForecastZone::NJ020),
            "NJ021" => Ok(ForecastZone::NJ021),
            "NJ022" => Ok(ForecastZone::NJ022),
            "NJ023" => Ok(ForecastZone::NJ023),
            "NJ024" => Ok(ForecastZone::NJ024),
            "NJ025" => Ok(ForecastZone::NJ025),
            "NJ026" => Ok(ForecastZone::NJ026),
            "NJ027" => Ok(ForecastZone::NJ027),
            "NJ103" => Ok(ForecastZone::NJ103),
            "NJ104" => Ok(ForecastZone::NJ104),
            "NJ105" => Ok(ForecastZone::NJ105),
            "NJ106" => Ok(ForecastZone::NJ106),
            "NJ107" => Ok(ForecastZone::NJ107),
            "NJ108" => Ok(ForecastZone::NJ108),
            "NM027" => Ok(ForecastZone::NM027),
            "NM028" => Ok(ForecastZone::NM028),
            "NM029" => Ok(ForecastZone::NM029),
            "NM033" => Ok(ForecastZone::NM033),
            "NM034" => Ok(ForecastZone::NM034),
            "NM201" => Ok(ForecastZone::NM201),
            "NM202" => Ok(ForecastZone::NM202),
            "NM203" => Ok(ForecastZone::NM203),
            "NM204" => Ok(ForecastZone::NM204),
            "NM205" => Ok(ForecastZone::NM205),
            "NM206" => Ok(ForecastZone::NM206),
            "NM207" => Ok(ForecastZone::NM207),
            "NM208" => Ok(ForecastZone::NM208),
            "NM209" => Ok(ForecastZone::NM209),
            "NM210" => Ok(ForecastZone::NM210),
            "NM211" => Ok(ForecastZone::NM211),
            "NM212" => Ok(ForecastZone::NM212),
            "NM213" => Ok(ForecastZone::NM213),
            "NM214" => Ok(ForecastZone::NM214),
            "NM215" => Ok(ForecastZone::NM215),
            "NM216" => Ok(ForecastZone::NM216),
            "NM217" => Ok(ForecastZone::NM217),
            "NM218" => Ok(ForecastZone::NM218),
            "NM219" => Ok(ForecastZone::NM219),
            "NM220" => Ok(ForecastZone::NM220),
            "NM221" => Ok(ForecastZone::NM221),
            "NM222" => Ok(ForecastZone::NM222),
            "NM223" => Ok(ForecastZone::NM223),
            "NM224" => Ok(ForecastZone::NM224),
            "NM225" => Ok(ForecastZone::NM225),
            "NM226" => Ok(ForecastZone::NM226),
            "NM227" => Ok(ForecastZone::NM227),
            "NM228" => Ok(ForecastZone::NM228),
            "NM229" => Ok(ForecastZone::NM229),
            "NM230" => Ok(ForecastZone::NM230),
            "NM231" => Ok(ForecastZone::NM231),
            "NM232" => Ok(ForecastZone::NM232),
            "NM233" => Ok(ForecastZone::NM233),
            "NM234" => Ok(ForecastZone::NM234),
            "NM235" => Ok(ForecastZone::NM235),
            "NM236" => Ok(ForecastZone::NM236),
            "NM237" => Ok(ForecastZone::NM237),
            "NM238" => Ok(ForecastZone::NM238),
            "NM239" => Ok(ForecastZone::NM239),
            "NM240" => Ok(ForecastZone::NM240),
            "NM241" => Ok(ForecastZone::NM241),
            "NM401" => Ok(ForecastZone::NM401),
            "NM402" => Ok(ForecastZone::NM402),
            "NM403" => Ok(ForecastZone::NM403),
            "NM404" => Ok(ForecastZone::NM404),
            "NM405" => Ok(ForecastZone::NM405),
            "NM406" => Ok(ForecastZone::NM406),
            "NM407" => Ok(ForecastZone::NM407),
            "NM408" => Ok(ForecastZone::NM408),
            "NM409" => Ok(ForecastZone::NM409),
            "NM410" => Ok(ForecastZone::NM410),
            "NM411" => Ok(ForecastZone::NM411),
            "NM412" => Ok(ForecastZone::NM412),
            "NM413" => Ok(ForecastZone::NM413),
            "NM414" => Ok(ForecastZone::NM414),
            "NM415" => Ok(ForecastZone::NM415),
            "NM416" => Ok(ForecastZone::NM416),
            "NM417" => Ok(ForecastZone::NM417),
            "NV001" => Ok(ForecastZone::NV001),
            "NV002" => Ok(ForecastZone::NV002),
            "NV003" => Ok(ForecastZone::NV003),
            "NV004" => Ok(ForecastZone::NV004),
            "NV005" => Ok(ForecastZone::NV005),
            "NV014" => Ok(ForecastZone::NV014),
            "NV015" => Ok(ForecastZone::NV015),
            "NV016" => Ok(ForecastZone::NV016),
            "NV017" => Ok(ForecastZone::NV017),
            "NV018" => Ok(ForecastZone::NV018),
            "NV019" => Ok(ForecastZone::NV019),
            "NV020" => Ok(ForecastZone::NV020),
            "NV021" => Ok(ForecastZone::NV021),
            "NV022" => Ok(ForecastZone::NV022),
            "NV030" => Ok(ForecastZone::NV030),
            "NV031" => Ok(ForecastZone::NV031),
            "NV033" => Ok(ForecastZone::NV033),
            "NV034" => Ok(ForecastZone::NV034),
            "NV035" => Ok(ForecastZone::NV035),
            "NV036" => Ok(ForecastZone::NV036),
            "NV037" => Ok(ForecastZone::NV037),
            "NV038" => Ok(ForecastZone::NV038),
            "NV039" => Ok(ForecastZone::NV039),
            "NV040" => Ok(ForecastZone::NV040),
            "NV041" => Ok(ForecastZone::NV041),
            "NY001" => Ok(ForecastZone::NY001),
            "NY002" => Ok(ForecastZone::NY002),
            "NY003" => Ok(ForecastZone::NY003),
            "NY004" => Ok(ForecastZone::NY004),
            "NY005" => Ok(ForecastZone::NY005),
            "NY006" => Ok(ForecastZone::NY006),
            "NY007" => Ok(ForecastZone::NY007),
            "NY008" => Ok(ForecastZone::NY008),
            "NY009" => Ok(ForecastZone::NY009),
            "NY010" => Ok(ForecastZone::NY010),
            "NY011" => Ok(ForecastZone::NY011),
            "NY012" => Ok(ForecastZone::NY012),
            "NY013" => Ok(ForecastZone::NY013),
            "NY014" => Ok(ForecastZone::NY014),
            "NY015" => Ok(ForecastZone::NY015),
            "NY016" => Ok(ForecastZone::NY016),
            "NY017" => Ok(ForecastZone::NY017),
            "NY018" => Ok(ForecastZone::NY018),
            "NY019" => Ok(ForecastZone::NY019),
            "NY020" => Ok(ForecastZone::NY020),
            "NY021" => Ok(ForecastZone::NY021),
            "NY022" => Ok(ForecastZone::NY022),
            "NY023" => Ok(ForecastZone::NY023),
            "NY024" => Ok(ForecastZone::NY024),
            "NY025" => Ok(ForecastZone::NY025),
            "NY026" => Ok(ForecastZone::NY026),
            "NY027" => Ok(ForecastZone::NY027),
            "NY028" => Ok(ForecastZone::NY028),
            "NY029" => Ok(ForecastZone::NY029),
            "NY030" => Ok(ForecastZone::NY030),
            "NY031" => Ok(ForecastZone::NY031),
            "NY032" => Ok(ForecastZone::NY032),
            "NY033" => Ok(ForecastZone::NY033),
            "NY034" => Ok(ForecastZone::NY034),
            "NY035" => Ok(ForecastZone::NY035),
            "NY036" => Ok(ForecastZone::NY036),
            "NY037" => Ok(ForecastZone::NY037),
            "NY038" => Ok(ForecastZone::NY038),
            "NY039" => Ok(ForecastZone::NY039),
            "NY040" => Ok(ForecastZone::NY040),
            "NY041" => Ok(ForecastZone::NY041),
            "NY042" => Ok(ForecastZone::NY042),
            "NY043" => Ok(ForecastZone::NY043),
            "NY044" => Ok(ForecastZone::NY044),
            "NY045" => Ok(ForecastZone::NY045),
            "NY046" => Ok(ForecastZone::NY046),
            "NY047" => Ok(ForecastZone::NY047),
            "NY048" => Ok(ForecastZone::NY048),
            "NY049" => Ok(ForecastZone::NY049),
            "NY050" => Ok(ForecastZone::NY050),
            "NY051" => Ok(ForecastZone::NY051),
            "NY052" => Ok(ForecastZone::NY052),
            "NY053" => Ok(ForecastZone::NY053),
            "NY054" => Ok(ForecastZone::NY054),
            "NY055" => Ok(ForecastZone::NY055),
            "NY056" => Ok(ForecastZone::NY056),
            "NY057" => Ok(ForecastZone::NY057),
            "NY058" => Ok(ForecastZone::NY058),
            "NY059" => Ok(ForecastZone::NY059),
            "NY060" => Ok(ForecastZone::NY060),
            "NY061" => Ok(ForecastZone::NY061),
            "NY062" => Ok(ForecastZone::NY062),
            "NY063" => Ok(ForecastZone::NY063),
            "NY064" => Ok(ForecastZone::NY064),
            "NY065" => Ok(ForecastZone::NY065),
            "NY066" => Ok(ForecastZone::NY066),
            "NY067" => Ok(ForecastZone::NY067),
            "NY068" => Ok(ForecastZone::NY068),
            "NY069" => Ok(ForecastZone::NY069),
            "NY070" => Ok(ForecastZone::NY070),
            "NY071" => Ok(ForecastZone::NY071),
            "NY072" => Ok(ForecastZone::NY072),
            "NY073" => Ok(ForecastZone::NY073),
            "NY074" => Ok(ForecastZone::NY074),
            "NY075" => Ok(ForecastZone::NY075),
            "NY078" => Ok(ForecastZone::NY078),
            "NY079" => Ok(ForecastZone::NY079),
            "NY080" => Ok(ForecastZone::NY080),
            "NY081" => Ok(ForecastZone::NY081),
            "NY082" => Ok(ForecastZone::NY082),
            "NY083" => Ok(ForecastZone::NY083),
            "NY084" => Ok(ForecastZone::NY084),
            "NY085" => Ok(ForecastZone::NY085),
            "NY087" => Ok(ForecastZone::NY087),
            "NY176" => Ok(ForecastZone::NY176),
            "NY177" => Ok(ForecastZone::NY177),
            "NY178" => Ok(ForecastZone::NY178),
            "NY179" => Ok(ForecastZone::NY179),
            "OH001" => Ok(ForecastZone::OH001),
            "OH002" => Ok(ForecastZone::OH002),
            "OH003" => Ok(ForecastZone::OH003),
            "OH004" => Ok(ForecastZone::OH004),
            "OH005" => Ok(ForecastZone::OH005),
            "OH006" => Ok(ForecastZone::OH006),
            "OH007" => Ok(ForecastZone::OH007),
            "OH008" => Ok(ForecastZone::OH008),
            "OH009" => Ok(ForecastZone::OH009),
            "OH010" => Ok(ForecastZone::OH010),
            "OH011" => Ok(ForecastZone::OH011),
            "OH012" => Ok(ForecastZone::OH012),
            "OH013" => Ok(ForecastZone::OH013),
            "OH014" => Ok(ForecastZone::OH014),
            "OH015" => Ok(ForecastZone::OH015),
            "OH016" => Ok(ForecastZone::OH016),
            "OH017" => Ok(ForecastZone::OH017),
            "OH018" => Ok(ForecastZone::OH018),
            "OH019" => Ok(ForecastZone::OH019),
            "OH020" => Ok(ForecastZone::OH020),
            "OH021" => Ok(ForecastZone::OH021),
            "OH022" => Ok(ForecastZone::OH022),
            "OH023" => Ok(ForecastZone::OH023),
            "OH024" => Ok(ForecastZone::OH024),
            "OH025" => Ok(ForecastZone::OH025),
            "OH026" => Ok(ForecastZone::OH026),
            "OH027" => Ok(ForecastZone::OH027),
            "OH028" => Ok(ForecastZone::OH028),
            "OH029" => Ok(ForecastZone::OH029),
            "OH030" => Ok(ForecastZone::OH030),
            "OH031" => Ok(ForecastZone::OH031),
            "OH032" => Ok(ForecastZone::OH032),
            "OH033" => Ok(ForecastZone::OH033),
            "OH034" => Ok(ForecastZone::OH034),
            "OH035" => Ok(ForecastZone::OH035),
            "OH036" => Ok(ForecastZone::OH036),
            "OH037" => Ok(ForecastZone::OH037),
            "OH038" => Ok(ForecastZone::OH038),
            "OH039" => Ok(ForecastZone::OH039),
            "OH040" => Ok(ForecastZone::OH040),
            "OH041" => Ok(ForecastZone::OH041),
            "OH042" => Ok(ForecastZone::OH042),
            "OH043" => Ok(ForecastZone::OH043),
            "OH044" => Ok(ForecastZone::OH044),
            "OH045" => Ok(ForecastZone::OH045),
            "OH046" => Ok(ForecastZone::OH046),
            "OH047" => Ok(ForecastZone::OH047),
            "OH048" => Ok(ForecastZone::OH048),
            "OH049" => Ok(ForecastZone::OH049),
            "OH050" => Ok(ForecastZone::OH050),
            "OH051" => Ok(ForecastZone::OH051),
            "OH052" => Ok(ForecastZone::OH052),
            "OH053" => Ok(ForecastZone::OH053),
            "OH054" => Ok(ForecastZone::OH054),
            "OH055" => Ok(ForecastZone::OH055),
            "OH056" => Ok(ForecastZone::OH056),
            "OH057" => Ok(ForecastZone::OH057),
            "OH058" => Ok(ForecastZone::OH058),
            "OH059" => Ok(ForecastZone::OH059),
            "OH060" => Ok(ForecastZone::OH060),
            "OH061" => Ok(ForecastZone::OH061),
            "OH062" => Ok(ForecastZone::OH062),
            "OH063" => Ok(ForecastZone::OH063),
            "OH064" => Ok(ForecastZone::OH064),
            "OH065" => Ok(ForecastZone::OH065),
            "OH066" => Ok(ForecastZone::OH066),
            "OH067" => Ok(ForecastZone::OH067),
            "OH068" => Ok(ForecastZone::OH068),
            "OH069" => Ok(ForecastZone::OH069),
            "OH070" => Ok(ForecastZone::OH070),
            "OH071" => Ok(ForecastZone::OH071),
            "OH072" => Ok(ForecastZone::OH072),
            "OH073" => Ok(ForecastZone::OH073),
            "OH074" => Ok(ForecastZone::OH074),
            "OH075" => Ok(ForecastZone::OH075),
            "OH076" => Ok(ForecastZone::OH076),
            "OH077" => Ok(ForecastZone::OH077),
            "OH078" => Ok(ForecastZone::OH078),
            "OH079" => Ok(ForecastZone::OH079),
            "OH080" => Ok(ForecastZone::OH080),
            "OH081" => Ok(ForecastZone::OH081),
            "OH082" => Ok(ForecastZone::OH082),
            "OH083" => Ok(ForecastZone::OH083),
            "OH084" => Ok(ForecastZone::OH084),
            "OH085" => Ok(ForecastZone::OH085),
            "OH086" => Ok(ForecastZone::OH086),
            "OH087" => Ok(ForecastZone::OH087),
            "OH088" => Ok(ForecastZone::OH088),
            "OH089" => Ok(ForecastZone::OH089),
            "OK001" => Ok(ForecastZone::OK001),
            "OK002" => Ok(ForecastZone::OK002),
            "OK003" => Ok(ForecastZone::OK003),
            "OK004" => Ok(ForecastZone::OK004),
            "OK005" => Ok(ForecastZone::OK005),
            "OK006" => Ok(ForecastZone::OK006),
            "OK007" => Ok(ForecastZone::OK007),
            "OK008" => Ok(ForecastZone::OK008),
            "OK009" => Ok(ForecastZone::OK009),
            "OK010" => Ok(ForecastZone::OK010),
            "OK011" => Ok(ForecastZone::OK011),
            "OK012" => Ok(ForecastZone::OK012),
            "OK013" => Ok(ForecastZone::OK013),
            "OK014" => Ok(ForecastZone::OK014),
            "OK015" => Ok(ForecastZone::OK015),
            "OK016" => Ok(ForecastZone::OK016),
            "OK017" => Ok(ForecastZone::OK017),
            "OK018" => Ok(ForecastZone::OK018),
            "OK019" => Ok(ForecastZone::OK019),
            "OK020" => Ok(ForecastZone::OK020),
            "OK021" => Ok(ForecastZone::OK021),
            "OK022" => Ok(ForecastZone::OK022),
            "OK023" => Ok(ForecastZone::OK023),
            "OK024" => Ok(ForecastZone::OK024),
            "OK025" => Ok(ForecastZone::OK025),
            "OK026" => Ok(ForecastZone::OK026),
            "OK027" => Ok(ForecastZone::OK027),
            "OK028" => Ok(ForecastZone::OK028),
            "OK029" => Ok(ForecastZone::OK029),
            "OK030" => Ok(ForecastZone::OK030),
            "OK031" => Ok(ForecastZone::OK031),
            "OK032" => Ok(ForecastZone::OK032),
            "OK033" => Ok(ForecastZone::OK033),
            "OK034" => Ok(ForecastZone::OK034),
            "OK035" => Ok(ForecastZone::OK035),
            "OK036" => Ok(ForecastZone::OK036),
            "OK037" => Ok(ForecastZone::OK037),
            "OK038" => Ok(ForecastZone::OK038),
            "OK039" => Ok(ForecastZone::OK039),
            "OK040" => Ok(ForecastZone::OK040),
            "OK041" => Ok(ForecastZone::OK041),
            "OK042" => Ok(ForecastZone::OK042),
            "OK043" => Ok(ForecastZone::OK043),
            "OK044" => Ok(ForecastZone::OK044),
            "OK045" => Ok(ForecastZone::OK045),
            "OK046" => Ok(ForecastZone::OK046),
            "OK047" => Ok(ForecastZone::OK047),
            "OK048" => Ok(ForecastZone::OK048),
            "OK049" => Ok(ForecastZone::OK049),
            "OK050" => Ok(ForecastZone::OK050),
            "OK051" => Ok(ForecastZone::OK051),
            "OK052" => Ok(ForecastZone::OK052),
            "OK053" => Ok(ForecastZone::OK053),
            "OK054" => Ok(ForecastZone::OK054),
            "OK055" => Ok(ForecastZone::OK055),
            "OK056" => Ok(ForecastZone::OK056),
            "OK057" => Ok(ForecastZone::OK057),
            "OK058" => Ok(ForecastZone::OK058),
            "OK059" => Ok(ForecastZone::OK059),
            "OK060" => Ok(ForecastZone::OK060),
            "OK061" => Ok(ForecastZone::OK061),
            "OK062" => Ok(ForecastZone::OK062),
            "OK063" => Ok(ForecastZone::OK063),
            "OK064" => Ok(ForecastZone::OK064),
            "OK065" => Ok(ForecastZone::OK065),
            "OK066" => Ok(ForecastZone::OK066),
            "OK067" => Ok(ForecastZone::OK067),
            "OK068" => Ok(ForecastZone::OK068),
            "OK069" => Ok(ForecastZone::OK069),
            "OK070" => Ok(ForecastZone::OK070),
            "OK071" => Ok(ForecastZone::OK071),
            "OK072" => Ok(ForecastZone::OK072),
            "OK073" => Ok(ForecastZone::OK073),
            "OK074" => Ok(ForecastZone::OK074),
            "OK075" => Ok(ForecastZone::OK075),
            "OK076" => Ok(ForecastZone::OK076),
            "OK077" => Ok(ForecastZone::OK077),
            "OR001" => Ok(ForecastZone::OR001),
            "OR002" => Ok(ForecastZone::OR002),
            "OR003" => Ok(ForecastZone::OR003),
            "OR004" => Ok(ForecastZone::OR004),
            "OR005" => Ok(ForecastZone::OR005),
            "OR006" => Ok(ForecastZone::OR006),
            "OR007" => Ok(ForecastZone::OR007),
            "OR008" => Ok(ForecastZone::OR008),
            "OR010" => Ok(ForecastZone::OR010),
            "OR011" => Ok(ForecastZone::OR011),
            "OR012" => Ok(ForecastZone::OR012),
            "OR013" => Ok(ForecastZone::OR013),
            "OR014" => Ok(ForecastZone::OR014),
            "OR015" => Ok(ForecastZone::OR015),
            "OR016" => Ok(ForecastZone::OR016),
            "OR021" => Ok(ForecastZone::OR021),
            "OR022" => Ok(ForecastZone::OR022),
            "OR023" => Ok(ForecastZone::OR023),
            "OR024" => Ok(ForecastZone::OR024),
            "OR025" => Ok(ForecastZone::OR025),
            "OR026" => Ok(ForecastZone::OR026),
            "OR027" => Ok(ForecastZone::OR027),
            "OR028" => Ok(ForecastZone::OR028),
            "OR029" => Ok(ForecastZone::OR029),
            "OR030" => Ok(ForecastZone::OR030),
            "OR031" => Ok(ForecastZone::OR031),
            "OR041" => Ok(ForecastZone::OR041),
            "OR044" => Ok(ForecastZone::OR044),
            "OR049" => Ok(ForecastZone::OR049),
            "OR050" => Ok(ForecastZone::OR050),
            "OR061" => Ok(ForecastZone::OR061),
            "OR062" => Ok(ForecastZone::OR062),
            "OR063" => Ok(ForecastZone::OR063),
            "OR064" => Ok(ForecastZone::OR064),
            "OR502" => Ok(ForecastZone::OR502),
            "OR503" => Ok(ForecastZone::OR503),
            "OR505" => Ok(ForecastZone::OR505),
            "OR506" => Ok(ForecastZone::OR506),
            "OR507" => Ok(ForecastZone::OR507),
            "OR508" => Ok(ForecastZone::OR508),
            "OR509" => Ok(ForecastZone::OR509),
            "OR510" => Ok(ForecastZone::OR510),
            "OR511" => Ok(ForecastZone::OR511),
            "PA001" => Ok(ForecastZone::PA001),
            "PA002" => Ok(ForecastZone::PA002),
            "PA003" => Ok(ForecastZone::PA003),
            "PA004" => Ok(ForecastZone::PA004),
            "PA005" => Ok(ForecastZone::PA005),
            "PA006" => Ok(ForecastZone::PA006),
            "PA007" => Ok(ForecastZone::PA007),
            "PA008" => Ok(ForecastZone::PA008),
            "PA009" => Ok(ForecastZone::PA009),
            "PA010" => Ok(ForecastZone::PA010),
            "PA011" => Ok(ForecastZone::PA011),
            "PA012" => Ok(ForecastZone::PA012),
            "PA013" => Ok(ForecastZone::PA013),
            "PA014" => Ok(ForecastZone::PA014),
            "PA015" => Ok(ForecastZone::PA015),
            "PA016" => Ok(ForecastZone::PA016),
            "PA017" => Ok(ForecastZone::PA017),
            "PA018" => Ok(ForecastZone::PA018),
            "PA019" => Ok(ForecastZone::PA019),
            "PA020" => Ok(ForecastZone::PA020),
            "PA021" => Ok(ForecastZone::PA021),
            "PA022" => Ok(ForecastZone::PA022),
            "PA023" => Ok(ForecastZone::PA023),
            "PA024" => Ok(ForecastZone::PA024),
            "PA025" => Ok(ForecastZone::PA025),
            "PA026" => Ok(ForecastZone::PA026),
            "PA027" => Ok(ForecastZone::PA027),
            "PA028" => Ok(ForecastZone::PA028),
            "PA029" => Ok(ForecastZone::PA029),
            "PA031" => Ok(ForecastZone::PA031),
            "PA033" => Ok(ForecastZone::PA033),
            "PA034" => Ok(ForecastZone::PA034),
            "PA035" => Ok(ForecastZone::PA035),
            "PA036" => Ok(ForecastZone::PA036),
            "PA037" => Ok(ForecastZone::PA037),
            "PA038" => Ok(ForecastZone::PA038),
            "PA039" => Ok(ForecastZone::PA039),
            "PA040" => Ok(ForecastZone::PA040),
            "PA041" => Ok(ForecastZone::PA041),
            "PA042" => Ok(ForecastZone::PA042),
            "PA043" => Ok(ForecastZone::PA043),
            "PA044" => Ok(ForecastZone::PA044),
            "PA045" => Ok(ForecastZone::PA045),
            "PA046" => Ok(ForecastZone::PA046),
            "PA047" => Ok(ForecastZone::PA047),
            "PA048" => Ok(ForecastZone::PA048),
            "PA049" => Ok(ForecastZone::PA049),
            "PA050" => Ok(ForecastZone::PA050),
            "PA051" => Ok(ForecastZone::PA051),
            "PA052" => Ok(ForecastZone::PA052),
            "PA053" => Ok(ForecastZone::PA053),
            "PA054" => Ok(ForecastZone::PA054),
            "PA055" => Ok(ForecastZone::PA055),
            "PA056" => Ok(ForecastZone::PA056),
            "PA057" => Ok(ForecastZone::PA057),
            "PA058" => Ok(ForecastZone::PA058),
            "PA059" => Ok(ForecastZone::PA059),
            "PA060" => Ok(ForecastZone::PA060),
            "PA061" => Ok(ForecastZone::PA061),
            "PA062" => Ok(ForecastZone::PA062),
            "PA063" => Ok(ForecastZone::PA063),
            "PA064" => Ok(ForecastZone::PA064),
            "PA065" => Ok(ForecastZone::PA065),
            "PA066" => Ok(ForecastZone::PA066),
            "PA070" => Ok(ForecastZone::PA070),
            "PA071" => Ok(ForecastZone::PA071),
            "PA072" => Ok(ForecastZone::PA072),
            "PA073" => Ok(ForecastZone::PA073),
            "PA074" => Ok(ForecastZone::PA074),
            "PA075" => Ok(ForecastZone::PA075),
            "PA076" => Ok(ForecastZone::PA076),
            "PA101" => Ok(ForecastZone::PA101),
            "PA102" => Ok(ForecastZone::PA102),
            "PA103" => Ok(ForecastZone::PA103),
            "PA104" => Ok(ForecastZone::PA104),
            "PA105" => Ok(ForecastZone::PA105),
            "PA106" => Ok(ForecastZone::PA106),
            "PR001" => Ok(ForecastZone::PR001),
            "PR002" => Ok(ForecastZone::PR002),
            "PR003" => Ok(ForecastZone::PR003),
            "PR004" => Ok(ForecastZone::PR004),
            "PR005" => Ok(ForecastZone::PR005),
            "PR006" => Ok(ForecastZone::PR006),
            "PR007" => Ok(ForecastZone::PR007),
            "PR008" => Ok(ForecastZone::PR008),
            "PR009" => Ok(ForecastZone::PR009),
            "PR010" => Ok(ForecastZone::PR010),
            "PR011" => Ok(ForecastZone::PR011),
            "PR012" => Ok(ForecastZone::PR012),
            "PR013" => Ok(ForecastZone::PR013),
            "PW001" => Ok(ForecastZone::PW001),
            "PW002" => Ok(ForecastZone::PW002),
            "PW003" => Ok(ForecastZone::PW003),
            "PW004" => Ok(ForecastZone::PW004),
            "PW005" => Ok(ForecastZone::PW005),
            "PW006" => Ok(ForecastZone::PW006),
            "PW007" => Ok(ForecastZone::PW007),
            "PW008" => Ok(ForecastZone::PW008),
            "RI001" => Ok(ForecastZone::RI001),
            "RI002" => Ok(ForecastZone::RI002),
            "RI003" => Ok(ForecastZone::RI003),
            "RI004" => Ok(ForecastZone::RI004),
            "RI005" => Ok(ForecastZone::RI005),
            "RI006" => Ok(ForecastZone::RI006),
            "RI007" => Ok(ForecastZone::RI007),
            "RI008" => Ok(ForecastZone::RI008),
            "SC008" => Ok(ForecastZone::SC008),
            "SC009" => Ok(ForecastZone::SC009),
            "SC010" => Ok(ForecastZone::SC010),
            "SC011" => Ok(ForecastZone::SC011),
            "SC012" => Ok(ForecastZone::SC012),
            "SC013" => Ok(ForecastZone::SC013),
            "SC014" => Ok(ForecastZone::SC014),
            "SC016" => Ok(ForecastZone::SC016),
            "SC017" => Ok(ForecastZone::SC017),
            "SC018" => Ok(ForecastZone::SC018),
            "SC019" => Ok(ForecastZone::SC019),
            "SC020" => Ok(ForecastZone::SC020),
            "SC021" => Ok(ForecastZone::SC021),
            "SC022" => Ok(ForecastZone::SC022),
            "SC023" => Ok(ForecastZone::SC023),
            "SC024" => Ok(ForecastZone::SC024),
            "SC025" => Ok(ForecastZone::SC025),
            "SC026" => Ok(ForecastZone::SC026),
            "SC027" => Ok(ForecastZone::SC027),
            "SC028" => Ok(ForecastZone::SC028),
            "SC029" => Ok(ForecastZone::SC029),
            "SC030" => Ok(ForecastZone::SC030),
            "SC031" => Ok(ForecastZone::SC031),
            "SC032" => Ok(ForecastZone::SC032),
            "SC033" => Ok(ForecastZone::SC033),
            "SC035" => Ok(ForecastZone::SC035),
            "SC037" => Ok(ForecastZone::SC037),
            "SC038" => Ok(ForecastZone::SC038),
            "SC039" => Ok(ForecastZone::SC039),
            "SC040" => Ok(ForecastZone::SC040),
            "SC041" => Ok(ForecastZone::SC041),
            "SC042" => Ok(ForecastZone::SC042),
            "SC043" => Ok(ForecastZone::SC043),
            "SC044" => Ok(ForecastZone::SC044),
            "SC045" => Ok(ForecastZone::SC045),
            "SC047" => Ok(ForecastZone::SC047),
            "SC048" => Ok(ForecastZone::SC048),
            "SC049" => Ok(ForecastZone::SC049),
            "SC050" => Ok(ForecastZone::SC050),
            "SC051" => Ok(ForecastZone::SC051),
            "SC052" => Ok(ForecastZone::SC052),
            "SC054" => Ok(ForecastZone::SC054),
            "SC055" => Ok(ForecastZone::SC055),
            "SC056" => Ok(ForecastZone::SC056),
            "SC058" => Ok(ForecastZone::SC058),
            "SC059" => Ok(ForecastZone::SC059),
            "SC101" => Ok(ForecastZone::SC101),
            "SC102" => Ok(ForecastZone::SC102),
            "SC103" => Ok(ForecastZone::SC103),
            "SC104" => Ok(ForecastZone::SC104),
            "SC105" => Ok(ForecastZone::SC105),
            "SC106" => Ok(ForecastZone::SC106),
            "SC107" => Ok(ForecastZone::SC107),
            "SC108" => Ok(ForecastZone::SC108),
            "SC109" => Ok(ForecastZone::SC109),
            "SC115" => Ok(ForecastZone::SC115),
            "SC116" => Ok(ForecastZone::SC116),
            "SC135" => Ok(ForecastZone::SC135),
            "SC136" => Ok(ForecastZone::SC136),
            "SC137" => Ok(ForecastZone::SC137),
            "SD001" => Ok(ForecastZone::SD001),
            "SD002" => Ok(ForecastZone::SD002),
            "SD003" => Ok(ForecastZone::SD003),
            "SD004" => Ok(ForecastZone::SD004),
            "SD005" => Ok(ForecastZone::SD005),
            "SD006" => Ok(ForecastZone::SD006),
            "SD007" => Ok(ForecastZone::SD007),
            "SD008" => Ok(ForecastZone::SD008),
            "SD009" => Ok(ForecastZone::SD009),
            "SD010" => Ok(ForecastZone::SD010),
            "SD011" => Ok(ForecastZone::SD011),
            "SD012" => Ok(ForecastZone::SD012),
            "SD013" => Ok(ForecastZone::SD013),
            "SD014" => Ok(ForecastZone::SD014),
            "SD015" => Ok(ForecastZone::SD015),
            "SD016" => Ok(ForecastZone::SD016),
            "SD017" => Ok(ForecastZone::SD017),
            "SD018" => Ok(ForecastZone::SD018),
            "SD019" => Ok(ForecastZone::SD019),
            "SD020" => Ok(ForecastZone::SD020),
            "SD021" => Ok(ForecastZone::SD021),
            "SD022" => Ok(ForecastZone::SD022),
            "SD023" => Ok(ForecastZone::SD023),
            "SD024" => Ok(ForecastZone::SD024),
            "SD025" => Ok(ForecastZone::SD025),
            "SD026" => Ok(ForecastZone::SD026),
            "SD027" => Ok(ForecastZone::SD027),
            "SD028" => Ok(ForecastZone::SD028),
            "SD029" => Ok(ForecastZone::SD029),
            "SD030" => Ok(ForecastZone::SD030),
            "SD031" => Ok(ForecastZone::SD031),
            "SD032" => Ok(ForecastZone::SD032),
            "SD033" => Ok(ForecastZone::SD033),
            "SD034" => Ok(ForecastZone::SD034),
            "SD035" => Ok(ForecastZone::SD035),
            "SD036" => Ok(ForecastZone::SD036),
            "SD037" => Ok(ForecastZone::SD037),
            "SD038" => Ok(ForecastZone::SD038),
            "SD039" => Ok(ForecastZone::SD039),
            "SD040" => Ok(ForecastZone::SD040),
            "SD041" => Ok(ForecastZone::SD041),
            "SD042" => Ok(ForecastZone::SD042),
            "SD043" => Ok(ForecastZone::SD043),
            "SD044" => Ok(ForecastZone::SD044),
            "SD045" => Ok(ForecastZone::SD045),
            "SD046" => Ok(ForecastZone::SD046),
            "SD047" => Ok(ForecastZone::SD047),
            "SD048" => Ok(ForecastZone::SD048),
            "SD049" => Ok(ForecastZone::SD049),
            "SD050" => Ok(ForecastZone::SD050),
            "SD051" => Ok(ForecastZone::SD051),
            "SD052" => Ok(ForecastZone::SD052),
            "SD053" => Ok(ForecastZone::SD053),
            "SD054" => Ok(ForecastZone::SD054),
            "SD055" => Ok(ForecastZone::SD055),
            "SD056" => Ok(ForecastZone::SD056),
            "SD057" => Ok(ForecastZone::SD057),
            "SD058" => Ok(ForecastZone::SD058),
            "SD059" => Ok(ForecastZone::SD059),
            "SD060" => Ok(ForecastZone::SD060),
            "SD061" => Ok(ForecastZone::SD061),
            "SD062" => Ok(ForecastZone::SD062),
            "SD063" => Ok(ForecastZone::SD063),
            "SD064" => Ok(ForecastZone::SD064),
            "SD065" => Ok(ForecastZone::SD065),
            "SD066" => Ok(ForecastZone::SD066),
            "SD067" => Ok(ForecastZone::SD067),
            "SD068" => Ok(ForecastZone::SD068),
            "SD069" => Ok(ForecastZone::SD069),
            "SD070" => Ok(ForecastZone::SD070),
            "SD071" => Ok(ForecastZone::SD071),
            "SD072" => Ok(ForecastZone::SD072),
            "SD073" => Ok(ForecastZone::SD073),
            "SD074" => Ok(ForecastZone::SD074),
            "TN001" => Ok(ForecastZone::TN001),
            "TN002" => Ok(ForecastZone::TN002),
            "TN003" => Ok(ForecastZone::TN003),
            "TN004" => Ok(ForecastZone::TN004),
            "TN005" => Ok(ForecastZone::TN005),
            "TN006" => Ok(ForecastZone::TN006),
            "TN007" => Ok(ForecastZone::TN007),
            "TN008" => Ok(ForecastZone::TN008),
            "TN009" => Ok(ForecastZone::TN009),
            "TN010" => Ok(ForecastZone::TN010),
            "TN011" => Ok(ForecastZone::TN011),
            "TN012" => Ok(ForecastZone::TN012),
            "TN013" => Ok(ForecastZone::TN013),
            "TN014" => Ok(ForecastZone::TN014),
            "TN015" => Ok(ForecastZone::TN015),
            "TN016" => Ok(ForecastZone::TN016),
            "TN017" => Ok(ForecastZone::TN017),
            "TN018" => Ok(ForecastZone::TN018),
            "TN019" => Ok(ForecastZone::TN019),
            "TN020" => Ok(ForecastZone::TN020),
            "TN021" => Ok(ForecastZone::TN021),
            "TN022" => Ok(ForecastZone::TN022),
            "TN023" => Ok(ForecastZone::TN023),
            "TN024" => Ok(ForecastZone::TN024),
            "TN025" => Ok(ForecastZone::TN025),
            "TN026" => Ok(ForecastZone::TN026),
            "TN027" => Ok(ForecastZone::TN027),
            "TN028" => Ok(ForecastZone::TN028),
            "TN029" => Ok(ForecastZone::TN029),
            "TN030" => Ok(ForecastZone::TN030),
            "TN031" => Ok(ForecastZone::TN031),
            "TN032" => Ok(ForecastZone::TN032),
            "TN033" => Ok(ForecastZone::TN033),
            "TN034" => Ok(ForecastZone::TN034),
            "TN035" => Ok(ForecastZone::TN035),
            "TN036" => Ok(ForecastZone::TN036),
            "TN037" => Ok(ForecastZone::TN037),
            "TN038" => Ok(ForecastZone::TN038),
            "TN039" => Ok(ForecastZone::TN039),
            "TN040" => Ok(ForecastZone::TN040),
            "TN041" => Ok(ForecastZone::TN041),
            "TN042" => Ok(ForecastZone::TN042),
            "TN043" => Ok(ForecastZone::TN043),
            "TN044" => Ok(ForecastZone::TN044),
            "TN045" => Ok(ForecastZone::TN045),
            "TN046" => Ok(ForecastZone::TN046),
            "TN047" => Ok(ForecastZone::TN047),
            "TN048" => Ok(ForecastZone::TN048),
            "TN049" => Ok(ForecastZone::TN049),
            "TN050" => Ok(ForecastZone::TN050),
            "TN051" => Ok(ForecastZone::TN051),
            "TN052" => Ok(ForecastZone::TN052),
            "TN053" => Ok(ForecastZone::TN053),
            "TN054" => Ok(ForecastZone::TN054),
            "TN055" => Ok(ForecastZone::TN055),
            "TN056" => Ok(ForecastZone::TN056),
            "TN057" => Ok(ForecastZone::TN057),
            "TN058" => Ok(ForecastZone::TN058),
            "TN059" => Ok(ForecastZone::TN059),
            "TN060" => Ok(ForecastZone::TN060),
            "TN061" => Ok(ForecastZone::TN061),
            "TN062" => Ok(ForecastZone::TN062),
            "TN063" => Ok(ForecastZone::TN063),
            "TN064" => Ok(ForecastZone::TN064),
            "TN065" => Ok(ForecastZone::TN065),
            "TN066" => Ok(ForecastZone::TN066),
            "TN067" => Ok(ForecastZone::TN067),
            "TN068" => Ok(ForecastZone::TN068),
            "TN069" => Ok(ForecastZone::TN069),
            "TN070" => Ok(ForecastZone::TN070),
            "TN071" => Ok(ForecastZone::TN071),
            "TN072" => Ok(ForecastZone::TN072),
            "TN073" => Ok(ForecastZone::TN073),
            "TN074" => Ok(ForecastZone::TN074),
            "TN075" => Ok(ForecastZone::TN075),
            "TN076" => Ok(ForecastZone::TN076),
            "TN077" => Ok(ForecastZone::TN077),
            "TN078" => Ok(ForecastZone::TN078),
            "TN079" => Ok(ForecastZone::TN079),
            "TN080" => Ok(ForecastZone::TN080),
            "TN081" => Ok(ForecastZone::TN081),
            "TN082" => Ok(ForecastZone::TN082),
            "TN083" => Ok(ForecastZone::TN083),
            "TN084" => Ok(ForecastZone::TN084),
            "TN085" => Ok(ForecastZone::TN085),
            "TN086" => Ok(ForecastZone::TN086),
            "TN087" => Ok(ForecastZone::TN087),
            "TN088" => Ok(ForecastZone::TN088),
            "TN089" => Ok(ForecastZone::TN089),
            "TN090" => Ok(ForecastZone::TN090),
            "TN091" => Ok(ForecastZone::TN091),
            "TN092" => Ok(ForecastZone::TN092),
            "TN093" => Ok(ForecastZone::TN093),
            "TN094" => Ok(ForecastZone::TN094),
            "TN095" => Ok(ForecastZone::TN095),
            "TN096" => Ok(ForecastZone::TN096),
            "TN097" => Ok(ForecastZone::TN097),
            "TN098" => Ok(ForecastZone::TN098),
            "TN099" => Ok(ForecastZone::TN099),
            "TN100" => Ok(ForecastZone::TN100),
            "TN101" => Ok(ForecastZone::TN101),
            "TN102" => Ok(ForecastZone::TN102),
            "TX001" => Ok(ForecastZone::TX001),
            "TX002" => Ok(ForecastZone::TX002),
            "TX003" => Ok(ForecastZone::TX003),
            "TX004" => Ok(ForecastZone::TX004),
            "TX005" => Ok(ForecastZone::TX005),
            "TX006" => Ok(ForecastZone::TX006),
            "TX007" => Ok(ForecastZone::TX007),
            "TX008" => Ok(ForecastZone::TX008),
            "TX009" => Ok(ForecastZone::TX009),
            "TX010" => Ok(ForecastZone::TX010),
            "TX011" => Ok(ForecastZone::TX011),
            "TX012" => Ok(ForecastZone::TX012),
            "TX013" => Ok(ForecastZone::TX013),
            "TX014" => Ok(ForecastZone::TX014),
            "TX015" => Ok(ForecastZone::TX015),
            "TX016" => Ok(ForecastZone::TX016),
            "TX017" => Ok(ForecastZone::TX017),
            "TX018" => Ok(ForecastZone::TX018),
            "TX019" => Ok(ForecastZone::TX019),
            "TX020" => Ok(ForecastZone::TX020),
            "TX021" => Ok(ForecastZone::TX021),
            "TX022" => Ok(ForecastZone::TX022),
            "TX023" => Ok(ForecastZone::TX023),
            "TX024" => Ok(ForecastZone::TX024),
            "TX025" => Ok(ForecastZone::TX025),
            "TX026" => Ok(ForecastZone::TX026),
            "TX027" => Ok(ForecastZone::TX027),
            "TX028" => Ok(ForecastZone::TX028),
            "TX029" => Ok(ForecastZone::TX029),
            "TX030" => Ok(ForecastZone::TX030),
            "TX031" => Ok(ForecastZone::TX031),
            "TX032" => Ok(ForecastZone::TX032),
            "TX033" => Ok(ForecastZone::TX033),
            "TX034" => Ok(ForecastZone::TX034),
            "TX035" => Ok(ForecastZone::TX035),
            "TX036" => Ok(ForecastZone::TX036),
            "TX037" => Ok(ForecastZone::TX037),
            "TX038" => Ok(ForecastZone::TX038),
            "TX039" => Ok(ForecastZone::TX039),
            "TX040" => Ok(ForecastZone::TX040),
            "TX041" => Ok(ForecastZone::TX041),
            "TX042" => Ok(ForecastZone::TX042),
            "TX043" => Ok(ForecastZone::TX043),
            "TX044" => Ok(ForecastZone::TX044),
            "TX045" => Ok(ForecastZone::TX045),
            "TX046" => Ok(ForecastZone::TX046),
            "TX047" => Ok(ForecastZone::TX047),
            "TX048" => Ok(ForecastZone::TX048),
            "TX049" => Ok(ForecastZone::TX049),
            "TX050" => Ok(ForecastZone::TX050),
            "TX051" => Ok(ForecastZone::TX051),
            "TX052" => Ok(ForecastZone::TX052),
            "TX053" => Ok(ForecastZone::TX053),
            "TX054" => Ok(ForecastZone::TX054),
            "TX059" => Ok(ForecastZone::TX059),
            "TX060" => Ok(ForecastZone::TX060),
            "TX061" => Ok(ForecastZone::TX061),
            "TX062" => Ok(ForecastZone::TX062),
            "TX063" => Ok(ForecastZone::TX063),
            "TX064" => Ok(ForecastZone::TX064),
            "TX065" => Ok(ForecastZone::TX065),
            "TX066" => Ok(ForecastZone::TX066),
            "TX067" => Ok(ForecastZone::TX067),
            "TX068" => Ok(ForecastZone::TX068),
            "TX069" => Ok(ForecastZone::TX069),
            "TX070" => Ok(ForecastZone::TX070),
            "TX071" => Ok(ForecastZone::TX071),
            "TX072" => Ok(ForecastZone::TX072),
            "TX073" => Ok(ForecastZone::TX073),
            "TX075" => Ok(ForecastZone::TX075),
            "TX076" => Ok(ForecastZone::TX076),
            "TX077" => Ok(ForecastZone::TX077),
            "TX078" => Ok(ForecastZone::TX078),
            "TX082" => Ok(ForecastZone::TX082),
            "TX083" => Ok(ForecastZone::TX083),
            "TX084" => Ok(ForecastZone::TX084),
            "TX085" => Ok(ForecastZone::TX085),
            "TX086" => Ok(ForecastZone::TX086),
            "TX087" => Ok(ForecastZone::TX087),
            "TX088" => Ok(ForecastZone::TX088),
            "TX089" => Ok(ForecastZone::TX089),
            "TX090" => Ok(ForecastZone::TX090),
            "TX091" => Ok(ForecastZone::TX091),
            "TX092" => Ok(ForecastZone::TX092),
            "TX093" => Ok(ForecastZone::TX093),
            "TX094" => Ok(ForecastZone::TX094),
            "TX095" => Ok(ForecastZone::TX095),
            "TX096" => Ok(ForecastZone::TX096),
            "TX097" => Ok(ForecastZone::TX097),
            "TX098" => Ok(ForecastZone::TX098),
            "TX099" => Ok(ForecastZone::TX099),
            "TX100" => Ok(ForecastZone::TX100),
            "TX101" => Ok(ForecastZone::TX101),
            "TX102" => Ok(ForecastZone::TX102),
            "TX103" => Ok(ForecastZone::TX103),
            "TX104" => Ok(ForecastZone::TX104),
            "TX105" => Ok(ForecastZone::TX105),
            "TX106" => Ok(ForecastZone::TX106),
            "TX107" => Ok(ForecastZone::TX107),
            "TX108" => Ok(ForecastZone::TX108),
            "TX109" => Ok(ForecastZone::TX109),
            "TX110" => Ok(ForecastZone::TX110),
            "TX111" => Ok(ForecastZone::TX111),
            "TX112" => Ok(ForecastZone::TX112),
            "TX113" => Ok(ForecastZone::TX113),
            "TX114" => Ok(ForecastZone::TX114),
            "TX115" => Ok(ForecastZone::TX115),
            "TX116" => Ok(ForecastZone::TX116),
            "TX117" => Ok(ForecastZone::TX117),
            "TX118" => Ok(ForecastZone::TX118),
            "TX119" => Ok(ForecastZone::TX119),
            "TX120" => Ok(ForecastZone::TX120),
            "TX121" => Ok(ForecastZone::TX121),
            "TX122" => Ok(ForecastZone::TX122),
            "TX123" => Ok(ForecastZone::TX123),
            "TX124" => Ok(ForecastZone::TX124),
            "TX125" => Ok(ForecastZone::TX125),
            "TX126" => Ok(ForecastZone::TX126),
            "TX127" => Ok(ForecastZone::TX127),
            "TX128" => Ok(ForecastZone::TX128),
            "TX129" => Ok(ForecastZone::TX129),
            "TX130" => Ok(ForecastZone::TX130),
            "TX131" => Ok(ForecastZone::TX131),
            "TX132" => Ok(ForecastZone::TX132),
            "TX133" => Ok(ForecastZone::TX133),
            "TX134" => Ok(ForecastZone::TX134),
            "TX135" => Ok(ForecastZone::TX135),
            "TX136" => Ok(ForecastZone::TX136),
            "TX137" => Ok(ForecastZone::TX137),
            "TX138" => Ok(ForecastZone::TX138),
            "TX139" => Ok(ForecastZone::TX139),
            "TX140" => Ok(ForecastZone::TX140),
            "TX141" => Ok(ForecastZone::TX141),
            "TX142" => Ok(ForecastZone::TX142),
            "TX143" => Ok(ForecastZone::TX143),
            "TX144" => Ok(ForecastZone::TX144),
            "TX145" => Ok(ForecastZone::TX145),
            "TX146" => Ok(ForecastZone::TX146),
            "TX147" => Ok(ForecastZone::TX147),
            "TX148" => Ok(ForecastZone::TX148),
            "TX149" => Ok(ForecastZone::TX149),
            "TX150" => Ok(ForecastZone::TX150),
            "TX151" => Ok(ForecastZone::TX151),
            "TX152" => Ok(ForecastZone::TX152),
            "TX153" => Ok(ForecastZone::TX153),
            "TX154" => Ok(ForecastZone::TX154),
            "TX155" => Ok(ForecastZone::TX155),
            "TX156" => Ok(ForecastZone::TX156),
            "TX157" => Ok(ForecastZone::TX157),
            "TX158" => Ok(ForecastZone::TX158),
            "TX159" => Ok(ForecastZone::TX159),
            "TX160" => Ok(ForecastZone::TX160),
            "TX161" => Ok(ForecastZone::TX161),
            "TX162" => Ok(ForecastZone::TX162),
            "TX163" => Ok(ForecastZone::TX163),
            "TX164" => Ok(ForecastZone::TX164),
            "TX165" => Ok(ForecastZone::TX165),
            "TX166" => Ok(ForecastZone::TX166),
            "TX167" => Ok(ForecastZone::TX167),
            "TX168" => Ok(ForecastZone::TX168),
            "TX169" => Ok(ForecastZone::TX169),
            "TX170" => Ok(ForecastZone::TX170),
            "TX171" => Ok(ForecastZone::TX171),
            "TX172" => Ok(ForecastZone::TX172),
            "TX173" => Ok(ForecastZone::TX173),
            "TX174" => Ok(ForecastZone::TX174),
            "TX175" => Ok(ForecastZone::TX175),
            "TX176" => Ok(ForecastZone::TX176),
            "TX177" => Ok(ForecastZone::TX177),
            "TX178" => Ok(ForecastZone::TX178),
            "TX179" => Ok(ForecastZone::TX179),
            "TX180" => Ok(ForecastZone::TX180),
            "TX183" => Ok(ForecastZone::TX183),
            "TX184" => Ok(ForecastZone::TX184),
            "TX185" => Ok(ForecastZone::TX185),
            "TX186" => Ok(ForecastZone::TX186),
            "TX187" => Ok(ForecastZone::TX187),
            "TX188" => Ok(ForecastZone::TX188),
            "TX189" => Ok(ForecastZone::TX189),
            "TX190" => Ok(ForecastZone::TX190),
            "TX191" => Ok(ForecastZone::TX191),
            "TX192" => Ok(ForecastZone::TX192),
            "TX193" => Ok(ForecastZone::TX193),
            "TX194" => Ok(ForecastZone::TX194),
            "TX195" => Ok(ForecastZone::TX195),
            "TX196" => Ok(ForecastZone::TX196),
            "TX197" => Ok(ForecastZone::TX197),
            "TX198" => Ok(ForecastZone::TX198),
            "TX199" => Ok(ForecastZone::TX199),
            "TX200" => Ok(ForecastZone::TX200),
            "TX201" => Ok(ForecastZone::TX201),
            "TX202" => Ok(ForecastZone::TX202),
            "TX203" => Ok(ForecastZone::TX203),
            "TX204" => Ok(ForecastZone::TX204),
            "TX205" => Ok(ForecastZone::TX205),
            "TX206" => Ok(ForecastZone::TX206),
            "TX207" => Ok(ForecastZone::TX207),
            "TX208" => Ok(ForecastZone::TX208),
            "TX209" => Ok(ForecastZone::TX209),
            "TX210" => Ok(ForecastZone::TX210),
            "TX211" => Ok(ForecastZone::TX211),
            "TX212" => Ok(ForecastZone::TX212),
            "TX213" => Ok(ForecastZone::TX213),
            "TX214" => Ok(ForecastZone::TX214),
            "TX215" => Ok(ForecastZone::TX215),
            "TX216" => Ok(ForecastZone::TX216),
            "TX217" => Ok(ForecastZone::TX217),
            "TX218" => Ok(ForecastZone::TX218),
            "TX219" => Ok(ForecastZone::TX219),
            "TX220" => Ok(ForecastZone::TX220),
            "TX221" => Ok(ForecastZone::TX221),
            "TX222" => Ok(ForecastZone::TX222),
            "TX223" => Ok(ForecastZone::TX223),
            "TX224" => Ok(ForecastZone::TX224),
            "TX225" => Ok(ForecastZone::TX225),
            "TX226" => Ok(ForecastZone::TX226),
            "TX227" => Ok(ForecastZone::TX227),
            "TX228" => Ok(ForecastZone::TX228),
            "TX229" => Ok(ForecastZone::TX229),
            "TX230" => Ok(ForecastZone::TX230),
            "TX231" => Ok(ForecastZone::TX231),
            "TX232" => Ok(ForecastZone::TX232),
            "TX233" => Ok(ForecastZone::TX233),
            "TX234" => Ok(ForecastZone::TX234),
            "TX235" => Ok(ForecastZone::TX235),
            "TX236" => Ok(ForecastZone::TX236),
            "TX237" => Ok(ForecastZone::TX237),
            "TX238" => Ok(ForecastZone::TX238),
            "TX239" => Ok(ForecastZone::TX239),
            "TX240" => Ok(ForecastZone::TX240),
            "TX241" => Ok(ForecastZone::TX241),
            "TX242" => Ok(ForecastZone::TX242),
            "TX243" => Ok(ForecastZone::TX243),
            "TX244" => Ok(ForecastZone::TX244),
            "TX245" => Ok(ForecastZone::TX245),
            "TX246" => Ok(ForecastZone::TX246),
            "TX247" => Ok(ForecastZone::TX247),
            "TX248" => Ok(ForecastZone::TX248),
            "TX249" => Ok(ForecastZone::TX249),
            "TX250" => Ok(ForecastZone::TX250),
            "TX251" => Ok(ForecastZone::TX251),
            "TX252" => Ok(ForecastZone::TX252),
            "TX253" => Ok(ForecastZone::TX253),
            "TX254" => Ok(ForecastZone::TX254),
            "TX255" => Ok(ForecastZone::TX255),
            "TX259" => Ok(ForecastZone::TX259),
            "TX260" => Ok(ForecastZone::TX260),
            "TX261" => Ok(ForecastZone::TX261),
            "TX262" => Ok(ForecastZone::TX262),
            "TX270" => Ok(ForecastZone::TX270),
            "TX271" => Ok(ForecastZone::TX271),
            "TX272" => Ok(ForecastZone::TX272),
            "TX273" => Ok(ForecastZone::TX273),
            "TX274" => Ok(ForecastZone::TX274),
            "TX275" => Ok(ForecastZone::TX275),
            "TX276" => Ok(ForecastZone::TX276),
            "TX277" => Ok(ForecastZone::TX277),
            "TX278" => Ok(ForecastZone::TX278),
            "TX279" => Ok(ForecastZone::TX279),
            "TX280" => Ok(ForecastZone::TX280),
            "TX281" => Ok(ForecastZone::TX281),
            "TX282" => Ok(ForecastZone::TX282),
            "TX300" => Ok(ForecastZone::TX300),
            "TX313" => Ok(ForecastZone::TX313),
            "TX317" => Ok(ForecastZone::TX317),
            "TX335" => Ok(ForecastZone::TX335),
            "TX336" => Ok(ForecastZone::TX336),
            "TX337" => Ok(ForecastZone::TX337),
            "TX338" => Ok(ForecastZone::TX338),
            "TX342" => Ok(ForecastZone::TX342),
            "TX343" => Ok(ForecastZone::TX343),
            "TX344" => Ok(ForecastZone::TX344),
            "TX345" => Ok(ForecastZone::TX345),
            "TX346" => Ok(ForecastZone::TX346),
            "TX347" => Ok(ForecastZone::TX347),
            "TX351" => Ok(ForecastZone::TX351),
            "TX353" => Ok(ForecastZone::TX353),
            "TX354" => Ok(ForecastZone::TX354),
            "TX355" => Ok(ForecastZone::TX355),
            "TX418" => Ok(ForecastZone::TX418),
            "TX419" => Ok(ForecastZone::TX419),
            "TX420" => Ok(ForecastZone::TX420),
            "TX421" => Ok(ForecastZone::TX421),
            "TX422" => Ok(ForecastZone::TX422),
            "TX423" => Ok(ForecastZone::TX423),
            "TX424" => Ok(ForecastZone::TX424),
            "TX436" => Ok(ForecastZone::TX436),
            "TX437" => Ok(ForecastZone::TX437),
            "TX438" => Ok(ForecastZone::TX438),
            "TX439" => Ok(ForecastZone::TX439),
            "TX442" => Ok(ForecastZone::TX442),
            "TX443" => Ok(ForecastZone::TX443),
            "TX447" => Ok(ForecastZone::TX447),
            "TX451" => Ok(ForecastZone::TX451),
            "TX454" => Ok(ForecastZone::TX454),
            "TX455" => Ok(ForecastZone::TX455),
            "UT022" => Ok(ForecastZone::UT022),
            "UT023" => Ok(ForecastZone::UT023),
            "UT024" => Ok(ForecastZone::UT024),
            "UT025" => Ok(ForecastZone::UT025),
            "UT027" => Ok(ForecastZone::UT027),
            "UT028" => Ok(ForecastZone::UT028),
            "UT029" => Ok(ForecastZone::UT029),
            "UT101" => Ok(ForecastZone::UT101),
            "UT102" => Ok(ForecastZone::UT102),
            "UT103" => Ok(ForecastZone::UT103),
            "UT104" => Ok(ForecastZone::UT104),
            "UT105" => Ok(ForecastZone::UT105),
            "UT106" => Ok(ForecastZone::UT106),
            "UT107" => Ok(ForecastZone::UT107),
            "UT108" => Ok(ForecastZone::UT108),
            "UT109" => Ok(ForecastZone::UT109),
            "UT110" => Ok(ForecastZone::UT110),
            "UT111" => Ok(ForecastZone::UT111),
            "UT112" => Ok(ForecastZone::UT112),
            "UT113" => Ok(ForecastZone::UT113),
            "UT114" => Ok(ForecastZone::UT114),
            "UT115" => Ok(ForecastZone::UT115),
            "UT116" => Ok(ForecastZone::UT116),
            "UT117" => Ok(ForecastZone::UT117),
            "UT118" => Ok(ForecastZone::UT118),
            "UT119" => Ok(ForecastZone::UT119),
            "UT120" => Ok(ForecastZone::UT120),
            "UT121" => Ok(ForecastZone::UT121),
            "UT122" => Ok(ForecastZone::UT122),
            "UT123" => Ok(ForecastZone::UT123),
            "UT124" => Ok(ForecastZone::UT124),
            "UT125" => Ok(ForecastZone::UT125),
            "UT126" => Ok(ForecastZone::UT126),
            "UT127" => Ok(ForecastZone::UT127),
            "UT128" => Ok(ForecastZone::UT128),
            "UT129" => Ok(ForecastZone::UT129),
            "UT130" => Ok(ForecastZone::UT130),
            "UT131" => Ok(ForecastZone::UT131),
            "VA001" => Ok(ForecastZone::VA001),
            "VA002" => Ok(ForecastZone::VA002),
            "VA003" => Ok(ForecastZone::VA003),
            "VA004" => Ok(ForecastZone::VA004),
            "VA005" => Ok(ForecastZone::VA005),
            "VA006" => Ok(ForecastZone::VA006),
            "VA007" => Ok(ForecastZone::VA007),
            "VA008" => Ok(ForecastZone::VA008),
            "VA009" => Ok(ForecastZone::VA009),
            "VA010" => Ok(ForecastZone::VA010),
            "VA011" => Ok(ForecastZone::VA011),
            "VA012" => Ok(ForecastZone::VA012),
            "VA013" => Ok(ForecastZone::VA013),
            "VA014" => Ok(ForecastZone::VA014),
            "VA015" => Ok(ForecastZone::VA015),
            "VA016" => Ok(ForecastZone::VA016),
            "VA017" => Ok(ForecastZone::VA017),
            "VA018" => Ok(ForecastZone::VA018),
            "VA019" => Ok(ForecastZone::VA019),
            "VA020" => Ok(ForecastZone::VA020),
            "VA022" => Ok(ForecastZone::VA022),
            "VA023" => Ok(ForecastZone::VA023),
            "VA024" => Ok(ForecastZone::VA024),
            "VA025" => Ok(ForecastZone::VA025),
            "VA026" => Ok(ForecastZone::VA026),
            "VA027" => Ok(ForecastZone::VA027),
            "VA028" => Ok(ForecastZone::VA028),
            "VA029" => Ok(ForecastZone::VA029),
            "VA030" => Ok(ForecastZone::VA030),
            "VA031" => Ok(ForecastZone::VA031),
            "VA032" => Ok(ForecastZone::VA032),
            "VA033" => Ok(ForecastZone::VA033),
            "VA034" => Ok(ForecastZone::VA034),
            "VA035" => Ok(ForecastZone::VA035),
            "VA036" => Ok(ForecastZone::VA036),
            "VA037" => Ok(ForecastZone::VA037),
            "VA038" => Ok(ForecastZone::VA038),
            "VA039" => Ok(ForecastZone::VA039),
            "VA040" => Ok(ForecastZone::VA040),
            "VA043" => Ok(ForecastZone::VA043),
            "VA044" => Ok(ForecastZone::VA044),
            "VA045" => Ok(ForecastZone::VA045),
            "VA046" => Ok(ForecastZone::VA046),
            "VA047" => Ok(ForecastZone::VA047),
            "VA048" => Ok(ForecastZone::VA048),
            "VA050" => Ok(ForecastZone::VA050),
            "VA051" => Ok(ForecastZone::VA051),
            "VA052" => Ok(ForecastZone::VA052),
            "VA053" => Ok(ForecastZone::VA053),
            "VA054" => Ok(ForecastZone::VA054),
            "VA055" => Ok(ForecastZone::VA055),
            "VA056" => Ok(ForecastZone::VA056),
            "VA057" => Ok(ForecastZone::VA057),
            "VA058" => Ok(ForecastZone::VA058),
            "VA059" => Ok(ForecastZone::VA059),
            "VA060" => Ok(ForecastZone::VA060),
            "VA061" => Ok(ForecastZone::VA061),
            "VA062" => Ok(ForecastZone::VA062),
            "VA064" => Ok(ForecastZone::VA064),
            "VA065" => Ok(ForecastZone::VA065),
            "VA066" => Ok(ForecastZone::VA066),
            "VA067" => Ok(ForecastZone::VA067),
            "VA068" => Ok(ForecastZone::VA068),
            "VA069" => Ok(ForecastZone::VA069),
            "VA075" => Ok(ForecastZone::VA075),
            "VA076" => Ok(ForecastZone::VA076),
            "VA077" => Ok(ForecastZone::VA077),
            "VA078" => Ok(ForecastZone::VA078),
            "VA079" => Ok(ForecastZone::VA079),
            "VA080" => Ok(ForecastZone::VA080),
            "VA081" => Ok(ForecastZone::VA081),
            "VA082" => Ok(ForecastZone::VA082),
            "VA083" => Ok(ForecastZone::VA083),
            "VA084" => Ok(ForecastZone::VA084),
            "VA085" => Ok(ForecastZone::VA085),
            "VA086" => Ok(ForecastZone::VA086),
            "VA087" => Ok(ForecastZone::VA087),
            "VA088" => Ok(ForecastZone::VA088),
            "VA089" => Ok(ForecastZone::VA089),
            "VA090" => Ok(ForecastZone::VA090),
            "VA092" => Ok(ForecastZone::VA092),
            "VA093" => Ok(ForecastZone::VA093),
            "VA095" => Ok(ForecastZone::VA095),
            "VA096" => Ok(ForecastZone::VA096),
            "VA097" => Ok(ForecastZone::VA097),
            "VA098" => Ok(ForecastZone::VA098),
            "VA099" => Ok(ForecastZone::VA099),
            "VA100" => Ok(ForecastZone::VA100),
            "VA501" => Ok(ForecastZone::VA501),
            "VA502" => Ok(ForecastZone::VA502),
            "VA503" => Ok(ForecastZone::VA503),
            "VA504" => Ok(ForecastZone::VA504),
            "VA505" => Ok(ForecastZone::VA505),
            "VA506" => Ok(ForecastZone::VA506),
            "VA507" => Ok(ForecastZone::VA507),
            "VA508" => Ok(ForecastZone::VA508),
            "VA509" => Ok(ForecastZone::VA509),
            "VA510" => Ok(ForecastZone::VA510),
            "VA511" => Ok(ForecastZone::VA511),
            "VA512" => Ok(ForecastZone::VA512),
            "VA513" => Ok(ForecastZone::VA513),
            "VA514" => Ok(ForecastZone::VA514),
            "VA515" => Ok(ForecastZone::VA515),
            "VA516" => Ok(ForecastZone::VA516),
            "VA517" => Ok(ForecastZone::VA517),
            "VA518" => Ok(ForecastZone::VA518),
            "VA519" => Ok(ForecastZone::VA519),
            "VA520" => Ok(ForecastZone::VA520),
            "VA521" => Ok(ForecastZone::VA521),
            "VA522" => Ok(ForecastZone::VA522),
            "VA523" => Ok(ForecastZone::VA523),
            "VA524" => Ok(ForecastZone::VA524),
            "VA525" => Ok(ForecastZone::VA525),
            "VI001" => Ok(ForecastZone::VI001),
            "VI002" => Ok(ForecastZone::VI002),
            "VT001" => Ok(ForecastZone::VT001),
            "VT002" => Ok(ForecastZone::VT002),
            "VT003" => Ok(ForecastZone::VT003),
            "VT004" => Ok(ForecastZone::VT004),
            "VT005" => Ok(ForecastZone::VT005),
            "VT006" => Ok(ForecastZone::VT006),
            "VT007" => Ok(ForecastZone::VT007),
            "VT008" => Ok(ForecastZone::VT008),
            "VT009" => Ok(ForecastZone::VT009),
            "VT010" => Ok(ForecastZone::VT010),
            "VT011" => Ok(ForecastZone::VT011),
            "VT013" => Ok(ForecastZone::VT013),
            "VT014" => Ok(ForecastZone::VT014),
            "VT015" => Ok(ForecastZone::VT015),
            "VT016" => Ok(ForecastZone::VT016),
            "VT017" => Ok(ForecastZone::VT017),
            "VT018" => Ok(ForecastZone::VT018),
            "VT019" => Ok(ForecastZone::VT019),
            "VT020" => Ok(ForecastZone::VT020),
            "VT021" => Ok(ForecastZone::VT021),
            "WA001" => Ok(ForecastZone::WA001),
            "WA019" => Ok(ForecastZone::WA019),
            "WA020" => Ok(ForecastZone::WA020),
            "WA021" => Ok(ForecastZone::WA021),
            "WA022" => Ok(ForecastZone::WA022),
            "WA024" => Ok(ForecastZone::WA024),
            "WA026" => Ok(ForecastZone::WA026),
            "WA027" => Ok(ForecastZone::WA027),
            "WA028" => Ok(ForecastZone::WA028),
            "WA029" => Ok(ForecastZone::WA029),
            "WA030" => Ok(ForecastZone::WA030),
            "WA031" => Ok(ForecastZone::WA031),
            "WA032" => Ok(ForecastZone::WA032),
            "WA033" => Ok(ForecastZone::WA033),
            "WA034" => Ok(ForecastZone::WA034),
            "WA035" => Ok(ForecastZone::WA035),
            "WA036" => Ok(ForecastZone::WA036),
            "WA037" => Ok(ForecastZone::WA037),
            "WA038" => Ok(ForecastZone::WA038),
            "WA039" => Ok(ForecastZone::WA039),
            "WA040" => Ok(ForecastZone::WA040),
            "WA041" => Ok(ForecastZone::WA041),
            "WA043" => Ok(ForecastZone::WA043),
            "WA044" => Ok(ForecastZone::WA044),
            "WA045" => Ok(ForecastZone::WA045),
            "WA046" => Ok(ForecastZone::WA046),
            "WA047" => Ok(ForecastZone::WA047),
            "WA048" => Ok(ForecastZone::WA048),
            "WA049" => Ok(ForecastZone::WA049),
            "WA503" => Ok(ForecastZone::WA503),
            "WA504" => Ok(ForecastZone::WA504),
            "WA506" => Ok(ForecastZone::WA506),
            "WA507" => Ok(ForecastZone::WA507),
            "WA509" => Ok(ForecastZone::WA509),
            "WA510" => Ok(ForecastZone::WA510),
            "WA511" => Ok(ForecastZone::WA511),
            "WA512" => Ok(ForecastZone::WA512),
            "WA513" => Ok(ForecastZone::WA513),
            "WA514" => Ok(ForecastZone::WA514),
            "WA515" => Ok(ForecastZone::WA515),
            "WA516" => Ok(ForecastZone::WA516),
            "WA517" => Ok(ForecastZone::WA517),
            "WA520" => Ok(ForecastZone::WA520),
            "WA521" => Ok(ForecastZone::WA521),
            "WA555" => Ok(ForecastZone::WA555),
            "WA556" => Ok(ForecastZone::WA556),
            "WA558" => Ok(ForecastZone::WA558),
            "WA559" => Ok(ForecastZone::WA559),
            "WA567" => Ok(ForecastZone::WA567),
            "WA568" => Ok(ForecastZone::WA568),
            "WA569" => Ok(ForecastZone::WA569),
            "WI001" => Ok(ForecastZone::WI001),
            "WI002" => Ok(ForecastZone::WI002),
            "WI003" => Ok(ForecastZone::WI003),
            "WI004" => Ok(ForecastZone::WI004),
            "WI005" => Ok(ForecastZone::WI005),
            "WI006" => Ok(ForecastZone::WI006),
            "WI007" => Ok(ForecastZone::WI007),
            "WI008" => Ok(ForecastZone::WI008),
            "WI009" => Ok(ForecastZone::WI009),
            "WI010" => Ok(ForecastZone::WI010),
            "WI011" => Ok(ForecastZone::WI011),
            "WI012" => Ok(ForecastZone::WI012),
            "WI013" => Ok(ForecastZone::WI013),
            "WI014" => Ok(ForecastZone::WI014),
            "WI015" => Ok(ForecastZone::WI015),
            "WI016" => Ok(ForecastZone::WI016),
            "WI017" => Ok(ForecastZone::WI017),
            "WI018" => Ok(ForecastZone::WI018),
            "WI019" => Ok(ForecastZone::WI019),
            "WI020" => Ok(ForecastZone::WI020),
            "WI021" => Ok(ForecastZone::WI021),
            "WI022" => Ok(ForecastZone::WI022),
            "WI023" => Ok(ForecastZone::WI023),
            "WI024" => Ok(ForecastZone::WI024),
            "WI025" => Ok(ForecastZone::WI025),
            "WI026" => Ok(ForecastZone::WI026),
            "WI027" => Ok(ForecastZone::WI027),
            "WI028" => Ok(ForecastZone::WI028),
            "WI029" => Ok(ForecastZone::WI029),
            "WI030" => Ok(ForecastZone::WI030),
            "WI031" => Ok(ForecastZone::WI031),
            "WI032" => Ok(ForecastZone::WI032),
            "WI033" => Ok(ForecastZone::WI033),
            "WI034" => Ok(ForecastZone::WI034),
            "WI035" => Ok(ForecastZone::WI035),
            "WI036" => Ok(ForecastZone::WI036),
            "WI037" => Ok(ForecastZone::WI037),
            "WI038" => Ok(ForecastZone::WI038),
            "WI039" => Ok(ForecastZone::WI039),
            "WI040" => Ok(ForecastZone::WI040),
            "WI041" => Ok(ForecastZone::WI041),
            "WI042" => Ok(ForecastZone::WI042),
            "WI043" => Ok(ForecastZone::WI043),
            "WI044" => Ok(ForecastZone::WI044),
            "WI045" => Ok(ForecastZone::WI045),
            "WI046" => Ok(ForecastZone::WI046),
            "WI047" => Ok(ForecastZone::WI047),
            "WI048" => Ok(ForecastZone::WI048),
            "WI049" => Ok(ForecastZone::WI049),
            "WI050" => Ok(ForecastZone::WI050),
            "WI051" => Ok(ForecastZone::WI051),
            "WI052" => Ok(ForecastZone::WI052),
            "WI053" => Ok(ForecastZone::WI053),
            "WI054" => Ok(ForecastZone::WI054),
            "WI055" => Ok(ForecastZone::WI055),
            "WI056" => Ok(ForecastZone::WI056),
            "WI057" => Ok(ForecastZone::WI057),
            "WI058" => Ok(ForecastZone::WI058),
            "WI059" => Ok(ForecastZone::WI059),
            "WI060" => Ok(ForecastZone::WI060),
            "WI061" => Ok(ForecastZone::WI061),
            "WI062" => Ok(ForecastZone::WI062),
            "WI063" => Ok(ForecastZone::WI063),
            "WI064" => Ok(ForecastZone::WI064),
            "WI065" => Ok(ForecastZone::WI065),
            "WI066" => Ok(ForecastZone::WI066),
            "WI067" => Ok(ForecastZone::WI067),
            "WI068" => Ok(ForecastZone::WI068),
            "WI069" => Ok(ForecastZone::WI069),
            "WI070" => Ok(ForecastZone::WI070),
            "WI071" => Ok(ForecastZone::WI071),
            "WI072" => Ok(ForecastZone::WI072),
            "WI073" => Ok(ForecastZone::WI073),
            "WI074" => Ok(ForecastZone::WI074),
            "WV001" => Ok(ForecastZone::WV001),
            "WV002" => Ok(ForecastZone::WV002),
            "WV003" => Ok(ForecastZone::WV003),
            "WV004" => Ok(ForecastZone::WV004),
            "WV005" => Ok(ForecastZone::WV005),
            "WV006" => Ok(ForecastZone::WV006),
            "WV007" => Ok(ForecastZone::WV007),
            "WV008" => Ok(ForecastZone::WV008),
            "WV009" => Ok(ForecastZone::WV009),
            "WV010" => Ok(ForecastZone::WV010),
            "WV011" => Ok(ForecastZone::WV011),
            "WV012" => Ok(ForecastZone::WV012),
            "WV013" => Ok(ForecastZone::WV013),
            "WV014" => Ok(ForecastZone::WV014),
            "WV015" => Ok(ForecastZone::WV015),
            "WV016" => Ok(ForecastZone::WV016),
            "WV017" => Ok(ForecastZone::WV017),
            "WV018" => Ok(ForecastZone::WV018),
            "WV019" => Ok(ForecastZone::WV019),
            "WV020" => Ok(ForecastZone::WV020),
            "WV021" => Ok(ForecastZone::WV021),
            "WV024" => Ok(ForecastZone::WV024),
            "WV025" => Ok(ForecastZone::WV025),
            "WV026" => Ok(ForecastZone::WV026),
            "WV027" => Ok(ForecastZone::WV027),
            "WV028" => Ok(ForecastZone::WV028),
            "WV029" => Ok(ForecastZone::WV029),
            "WV030" => Ok(ForecastZone::WV030),
            "WV031" => Ok(ForecastZone::WV031),
            "WV032" => Ok(ForecastZone::WV032),
            "WV033" => Ok(ForecastZone::WV033),
            "WV034" => Ok(ForecastZone::WV034),
            "WV039" => Ok(ForecastZone::WV039),
            "WV040" => Ok(ForecastZone::WV040),
            "WV042" => Ok(ForecastZone::WV042),
            "WV043" => Ok(ForecastZone::WV043),
            "WV044" => Ok(ForecastZone::WV044),
            "WV050" => Ok(ForecastZone::WV050),
            "WV051" => Ok(ForecastZone::WV051),
            "WV052" => Ok(ForecastZone::WV052),
            "WV053" => Ok(ForecastZone::WV053),
            "WV055" => Ok(ForecastZone::WV055),
            "WV501" => Ok(ForecastZone::WV501),
            "WV502" => Ok(ForecastZone::WV502),
            "WV503" => Ok(ForecastZone::WV503),
            "WV504" => Ok(ForecastZone::WV504),
            "WV505" => Ok(ForecastZone::WV505),
            "WV506" => Ok(ForecastZone::WV506),
            "WV507" => Ok(ForecastZone::WV507),
            "WV508" => Ok(ForecastZone::WV508),
            "WV509" => Ok(ForecastZone::WV509),
            "WV510" => Ok(ForecastZone::WV510),
            "WV511" => Ok(ForecastZone::WV511),
            "WV512" => Ok(ForecastZone::WV512),
            "WV513" => Ok(ForecastZone::WV513),
            "WV514" => Ok(ForecastZone::WV514),
            "WV515" => Ok(ForecastZone::WV515),
            "WV516" => Ok(ForecastZone::WV516),
            "WV517" => Ok(ForecastZone::WV517),
            "WV518" => Ok(ForecastZone::WV518),
            "WV519" => Ok(ForecastZone::WV519),
            "WV520" => Ok(ForecastZone::WV520),
            "WV521" => Ok(ForecastZone::WV521),
            "WV522" => Ok(ForecastZone::WV522),
            "WV523" => Ok(ForecastZone::WV523),
            "WV524" => Ok(ForecastZone::WV524),
            "WV525" => Ok(ForecastZone::WV525),
            "WV526" => Ok(ForecastZone::WV526),
            "WY001" => Ok(ForecastZone::WY001),
            "WY002" => Ok(ForecastZone::WY002),
            "WY003" => Ok(ForecastZone::WY003),
            "WY004" => Ok(ForecastZone::WY004),
            "WY005" => Ok(ForecastZone::WY005),
            "WY006" => Ok(ForecastZone::WY006),
            "WY007" => Ok(ForecastZone::WY007),
            "WY008" => Ok(ForecastZone::WY008),
            "WY009" => Ok(ForecastZone::WY009),
            "WY010" => Ok(ForecastZone::WY010),
            "WY011" => Ok(ForecastZone::WY011),
            "WY012" => Ok(ForecastZone::WY012),
            "WY013" => Ok(ForecastZone::WY013),
            "WY014" => Ok(ForecastZone::WY014),
            "WY015" => Ok(ForecastZone::WY015),
            "WY016" => Ok(ForecastZone::WY016),
            "WY017" => Ok(ForecastZone::WY017),
            "WY018" => Ok(ForecastZone::WY018),
            "WY019" => Ok(ForecastZone::WY019),
            "WY020" => Ok(ForecastZone::WY020),
            "WY021" => Ok(ForecastZone::WY021),
            "WY022" => Ok(ForecastZone::WY022),
            "WY023" => Ok(ForecastZone::WY023),
            "WY024" => Ok(ForecastZone::WY024),
            "WY025" => Ok(ForecastZone::WY025),
            "WY026" => Ok(ForecastZone::WY026),
            "WY027" => Ok(ForecastZone::WY027),
            "WY028" => Ok(ForecastZone::WY028),
            "WY029" => Ok(ForecastZone::WY029),
            "WY030" => Ok(ForecastZone::WY030),
            "WY054" => Ok(ForecastZone::WY054),
            "WY055" => Ok(ForecastZone::WY055),
            "WY056" => Ok(ForecastZone::WY056),
            "WY057" => Ok(ForecastZone::WY057),
            "WY058" => Ok(ForecastZone::WY058),
            "WY071" => Ok(ForecastZone::WY071),
            "WY101" => Ok(ForecastZone::WY101),
            "WY102" => Ok(ForecastZone::WY102),
            "WY103" => Ok(ForecastZone::WY103),
            "WY104" => Ok(ForecastZone::WY104),
            "WY105" => Ok(ForecastZone::WY105),
            "WY106" => Ok(ForecastZone::WY106),
            "WY107" => Ok(ForecastZone::WY107),
            "WY108" => Ok(ForecastZone::WY108),
            "WY109" => Ok(ForecastZone::WY109),
            "WY110" => Ok(ForecastZone::WY110),
            "WY111" => Ok(ForecastZone::WY111),
            "WY112" => Ok(ForecastZone::WY112),
            "WY113" => Ok(ForecastZone::WY113),
            "WY114" => Ok(ForecastZone::WY114),
            "WY115" => Ok(ForecastZone::WY115),
            "WY116" => Ok(ForecastZone::WY116),
            "WY117" => Ok(ForecastZone::WY117),
            "WY118" => Ok(ForecastZone::WY118),
            "WY119" => Ok(ForecastZone::WY119),
            "WY198" => Ok(ForecastZone::WY198),
            "WY199" => Ok(ForecastZone::WY199),
            _ => Err(()),
        }
    }
}
impl ForecastZone {
    pub fn details(&self) -> crate::ZoneDetails {
        match self {
ForecastZone::AK017 => crate::ZoneDetails {state: "AK", zone: "017", zone_numeric: 017, name: "Cape Fairweather to Cape Suckling Coastal Area", wfo: "AJK" },
ForecastZone::AK018 => crate::ZoneDetails {state: "AK", zone: "018", zone_numeric: 018, name: "Taiya Inlet and Klondike Highway", wfo: "AJK" },
ForecastZone::AK019 => crate::ZoneDetails {state: "AK", zone: "019", zone_numeric: 019, name: "Haines Borough and Lynn Canal", wfo: "AJK" },
ForecastZone::AK020 => crate::ZoneDetails {state: "AK", zone: "020", zone_numeric: 020, name: "Glacier Bay", wfo: "AJK" },
ForecastZone::AK021 => crate::ZoneDetails {state: "AK", zone: "021", zone_numeric: 021, name: "Eastern Chichagof Island", wfo: "AJK" },
ForecastZone::AK022 => crate::ZoneDetails {state: "AK", zone: "022", zone_numeric: 022, name: "Salisbury Sound to Cape Fairweather Coastal Area", wfo: "AJK" },
ForecastZone::AK023 => crate::ZoneDetails {state: "AK", zone: "023", zone_numeric: 023, name: "Cape Decision to Salisbury Sound Coastal Area", wfo: "AJK" },
ForecastZone::AK024 => crate::ZoneDetails {state: "AK", zone: "024", zone_numeric: 024, name: "Eastern Baranof Island and Southern Admiralty Island", wfo: "AJK" },
ForecastZone::AK025 => crate::ZoneDetails {state: "AK", zone: "025", zone_numeric: 025, name: "Juneau Borough and Northern Admiralty Island", wfo: "AJK" },
ForecastZone::AK026 => crate::ZoneDetails {state: "AK", zone: "026", zone_numeric: 026, name: "Inner Channels from Kupreanof Island to Etolin Island", wfo: "AJK" },
ForecastZone::AK027 => crate::ZoneDetails {state: "AK", zone: "027", zone_numeric: 027, name: "Dixon Entrance to Cape Decision Coastal Area", wfo: "AJK" },
ForecastZone::AK028 => crate::ZoneDetails {state: "AK", zone: "028", zone_numeric: 028, name: "Southern Inner Channels", wfo: "AJK" },
ForecastZone::AK029 => crate::ZoneDetails {state: "AK", zone: "029", zone_numeric: 029, name: "Misty Fjords", wfo: "AJK" },
ForecastZone::AK101 => crate::ZoneDetails {state: "AK", zone: "101", zone_numeric: 101, name: "Anchorage", wfo: "AFC" },
ForecastZone::AK111 => crate::ZoneDetails {state: "AK", zone: "111", zone_numeric: 111, name: "Matanuska Valley", wfo: "AFC" },
ForecastZone::AK121 => crate::ZoneDetails {state: "AK", zone: "121", zone_numeric: 121, name: "Western Kenai Peninsula", wfo: "AFC" },
ForecastZone::AK125 => crate::ZoneDetails {state: "AK", zone: "125", zone_numeric: 125, name: "Western Prince William Sound", wfo: "AFC" },
ForecastZone::AK131 => crate::ZoneDetails {state: "AK", zone: "131", zone_numeric: 131, name: "Northeast Prince William Sound", wfo: "AFC" },
ForecastZone::AK135 => crate::ZoneDetails {state: "AK", zone: "135", zone_numeric: 135, name: "Southeast Prince William Sound", wfo: "AFC" },
ForecastZone::AK141 => crate::ZoneDetails {state: "AK", zone: "141", zone_numeric: 141, name: "Copper River Basin", wfo: "AFC" },
ForecastZone::AK145 => crate::ZoneDetails {state: "AK", zone: "145", zone_numeric: 145, name: "Susitna Valley", wfo: "AFC" },
ForecastZone::AK152 => crate::ZoneDetails {state: "AK", zone: "152", zone_numeric: 152, name: "Lower Kuskokwim Valley", wfo: "AFC" },
ForecastZone::AK155 => crate::ZoneDetails {state: "AK", zone: "155", zone_numeric: 155, name: "Kuskokwim Delta", wfo: "AFC" },
ForecastZone::AK161 => crate::ZoneDetails {state: "AK", zone: "161", zone_numeric: 161, name: "Bristol Bay", wfo: "AFC" },
ForecastZone::AK171 => crate::ZoneDetails {state: "AK", zone: "171", zone_numeric: 171, name: "Kodiak Island", wfo: "AFC" },
ForecastZone::AK181 => crate::ZoneDetails {state: "AK", zone: "181", zone_numeric: 181, name: "Alaska Peninsula", wfo: "AFC" },
ForecastZone::AK185 => crate::ZoneDetails {state: "AK", zone: "185", zone_numeric: 185, name: "Eastern Aleutians", wfo: "AFC" },
ForecastZone::AK187 => crate::ZoneDetails {state: "AK", zone: "187", zone_numeric: 187, name: "Central Aleutians", wfo: "AFC" },
ForecastZone::AK191 => crate::ZoneDetails {state: "AK", zone: "191", zone_numeric: 191, name: "Western Aleutians", wfo: "AFC" },
ForecastZone::AK195 => crate::ZoneDetails {state: "AK", zone: "195", zone_numeric: 195, name: "Pribilof Islands", wfo: "AFC" },
ForecastZone::AK201 => crate::ZoneDetails {state: "AK", zone: "201", zone_numeric: 201, name: "Western Arctic Coast", wfo: "AFG" },
ForecastZone::AK202 => crate::ZoneDetails {state: "AK", zone: "202", zone_numeric: 202, name: "Northern Arctic Coast", wfo: "AFG" },
ForecastZone::AK203 => crate::ZoneDetails {state: "AK", zone: "203", zone_numeric: 203, name: "Central Beaufort Sea Coast", wfo: "AFG" },
ForecastZone::AK204 => crate::ZoneDetails {state: "AK", zone: "204", zone_numeric: 204, name: "Eastern Beaufort Sea Coast", wfo: "AFG" },
ForecastZone::AK205 => crate::ZoneDetails {state: "AK", zone: "205", zone_numeric: 205, name: "Northwestern Brooks Range", wfo: "AFG" },
ForecastZone::AK206 => crate::ZoneDetails {state: "AK", zone: "206", zone_numeric: 206, name: "Northeastern Brooks Range", wfo: "AFG" },
ForecastZone::AK207 => crate::ZoneDetails {state: "AK", zone: "207", zone_numeric: 207, name: "Chukchi Sea Coast", wfo: "AFG" },
ForecastZone::AK208 => crate::ZoneDetails {state: "AK", zone: "208", zone_numeric: 208, name: "Lower Kobuk and Noatak Valleys", wfo: "AFG" },
ForecastZone::AK209 => crate::ZoneDetails {state: "AK", zone: "209", zone_numeric: 209, name: "Baldwin Peninsula and Selawik Valley", wfo: "AFG" },
ForecastZone::AK210 => crate::ZoneDetails {state: "AK", zone: "210", zone_numeric: 210, name: "Northern and Interior Seward Peninsula", wfo: "AFG" },
ForecastZone::AK211 => crate::ZoneDetails {state: "AK", zone: "211", zone_numeric: 211, name: "Southern Seward Peninsula Coast", wfo: "AFG" },
ForecastZone::AK212 => crate::ZoneDetails {state: "AK", zone: "212", zone_numeric: 212, name: "Eastern Norton Sound and Nulato Hills", wfo: "AFG" },
ForecastZone::AK213 => crate::ZoneDetails {state: "AK", zone: "213", zone_numeric: 213, name: "St Lawrence Island and Bering Strait Coast", wfo: "AFG" },
ForecastZone::AK214 => crate::ZoneDetails {state: "AK", zone: "214", zone_numeric: 214, name: "Yukon Delta", wfo: "AFG" },
ForecastZone::AK215 => crate::ZoneDetails {state: "AK", zone: "215", zone_numeric: 215, name: "Lower Yukon Valley", wfo: "AFG" },
ForecastZone::AK216 => crate::ZoneDetails {state: "AK", zone: "216", zone_numeric: 216, name: "Lower Koyukuk and Middle Yukon Valleys", wfo: "AFG" },
ForecastZone::AK217 => crate::ZoneDetails {state: "AK", zone: "217", zone_numeric: 217, name: "Upper Kobuk and Noatak Valleys", wfo: "AFG" },
ForecastZone::AK218 => crate::ZoneDetails {state: "AK", zone: "218", zone_numeric: 218, name: "Southeastern Brooks Range", wfo: "AFG" },
ForecastZone::AK219 => crate::ZoneDetails {state: "AK", zone: "219", zone_numeric: 219, name: "Upper Koyukuk Valley", wfo: "AFG" },
ForecastZone::AK220 => crate::ZoneDetails {state: "AK", zone: "220", zone_numeric: 220, name: "Yukon Flats and Surrounding Uplands", wfo: "AFG" },
ForecastZone::AK221 => crate::ZoneDetails {state: "AK", zone: "221", zone_numeric: 221, name: "Central Interior", wfo: "AFG" },
ForecastZone::AK222 => crate::ZoneDetails {state: "AK", zone: "222", zone_numeric: 222, name: "Middle Tanana Valley", wfo: "AFG" },
ForecastZone::AK223 => crate::ZoneDetails {state: "AK", zone: "223", zone_numeric: 223, name: "Deltana and Tanana Flats", wfo: "AFG" },
ForecastZone::AK224 => crate::ZoneDetails {state: "AK", zone: "224", zone_numeric: 224, name: "Upper Tanana Valley and the Fortymile Country", wfo: "AFG" },
ForecastZone::AK225 => crate::ZoneDetails {state: "AK", zone: "225", zone_numeric: 225, name: "Denali", wfo: "AFG" },
ForecastZone::AK226 => crate::ZoneDetails {state: "AK", zone: "226", zone_numeric: 226, name: "Eastern Alaska Range", wfo: "AFG" },
ForecastZone::AK227 => crate::ZoneDetails {state: "AK", zone: "227", zone_numeric: 227, name: "Upper Kuskokwim Valley", wfo: "AFG" },
ForecastZone::AL001 => crate::ZoneDetails {state: "AL", zone: "001", zone_numeric: 001, name: "Lauderdale", wfo: "HUN" },
ForecastZone::AL002 => crate::ZoneDetails {state: "AL", zone: "002", zone_numeric: 002, name: "Colbert", wfo: "HUN" },
ForecastZone::AL003 => crate::ZoneDetails {state: "AL", zone: "003", zone_numeric: 003, name: "Franklin", wfo: "HUN" },
ForecastZone::AL004 => crate::ZoneDetails {state: "AL", zone: "004", zone_numeric: 004, name: "Lawrence", wfo: "HUN" },
ForecastZone::AL005 => crate::ZoneDetails {state: "AL", zone: "005", zone_numeric: 005, name: "Limestone", wfo: "HUN" },
ForecastZone::AL006 => crate::ZoneDetails {state: "AL", zone: "006", zone_numeric: 006, name: "Madison", wfo: "HUN" },
ForecastZone::AL007 => crate::ZoneDetails {state: "AL", zone: "007", zone_numeric: 007, name: "Morgan", wfo: "HUN" },
ForecastZone::AL008 => crate::ZoneDetails {state: "AL", zone: "008", zone_numeric: 008, name: "Marshall", wfo: "HUN" },
ForecastZone::AL009 => crate::ZoneDetails {state: "AL", zone: "009", zone_numeric: 009, name: "Jackson", wfo: "HUN" },
ForecastZone::AL010 => crate::ZoneDetails {state: "AL", zone: "010", zone_numeric: 010, name: "DeKalb", wfo: "HUN" },
ForecastZone::AL011 => crate::ZoneDetails {state: "AL", zone: "011", zone_numeric: 011, name: "Marion", wfo: "BMX" },
ForecastZone::AL012 => crate::ZoneDetails {state: "AL", zone: "012", zone_numeric: 012, name: "Lamar", wfo: "BMX" },
ForecastZone::AL013 => crate::ZoneDetails {state: "AL", zone: "013", zone_numeric: 013, name: "Fayette", wfo: "BMX" },
ForecastZone::AL014 => crate::ZoneDetails {state: "AL", zone: "014", zone_numeric: 014, name: "Winston", wfo: "BMX" },
ForecastZone::AL015 => crate::ZoneDetails {state: "AL", zone: "015", zone_numeric: 015, name: "Walker", wfo: "BMX" },
ForecastZone::AL016 => crate::ZoneDetails {state: "AL", zone: "016", zone_numeric: 016, name: "Cullman", wfo: "HUN" },
ForecastZone::AL017 => crate::ZoneDetails {state: "AL", zone: "017", zone_numeric: 017, name: "Blount", wfo: "BMX" },
ForecastZone::AL018 => crate::ZoneDetails {state: "AL", zone: "018", zone_numeric: 018, name: "Etowah", wfo: "BMX" },
ForecastZone::AL019 => crate::ZoneDetails {state: "AL", zone: "019", zone_numeric: 019, name: "Calhoun", wfo: "BMX" },
ForecastZone::AL020 => crate::ZoneDetails {state: "AL", zone: "020", zone_numeric: 020, name: "Cherokee", wfo: "BMX" },
ForecastZone::AL021 => crate::ZoneDetails {state: "AL", zone: "021", zone_numeric: 021, name: "Cleburne", wfo: "BMX" },
ForecastZone::AL022 => crate::ZoneDetails {state: "AL", zone: "022", zone_numeric: 022, name: "Pickens", wfo: "BMX" },
ForecastZone::AL023 => crate::ZoneDetails {state: "AL", zone: "023", zone_numeric: 023, name: "Tuscaloosa", wfo: "BMX" },
ForecastZone::AL024 => crate::ZoneDetails {state: "AL", zone: "024", zone_numeric: 024, name: "Jefferson", wfo: "BMX" },
ForecastZone::AL025 => crate::ZoneDetails {state: "AL", zone: "025", zone_numeric: 025, name: "Shelby", wfo: "BMX" },
ForecastZone::AL026 => crate::ZoneDetails {state: "AL", zone: "026", zone_numeric: 026, name: "St. Clair", wfo: "BMX" },
ForecastZone::AL027 => crate::ZoneDetails {state: "AL", zone: "027", zone_numeric: 027, name: "Talladega", wfo: "BMX" },
ForecastZone::AL028 => crate::ZoneDetails {state: "AL", zone: "028", zone_numeric: 028, name: "Clay", wfo: "BMX" },
ForecastZone::AL029 => crate::ZoneDetails {state: "AL", zone: "029", zone_numeric: 029, name: "Randolph", wfo: "BMX" },
ForecastZone::AL030 => crate::ZoneDetails {state: "AL", zone: "030", zone_numeric: 030, name: "Sumter", wfo: "BMX" },
ForecastZone::AL031 => crate::ZoneDetails {state: "AL", zone: "031", zone_numeric: 031, name: "Greene", wfo: "BMX" },
ForecastZone::AL032 => crate::ZoneDetails {state: "AL", zone: "032", zone_numeric: 032, name: "Hale", wfo: "BMX" },
ForecastZone::AL033 => crate::ZoneDetails {state: "AL", zone: "033", zone_numeric: 033, name: "Perry", wfo: "BMX" },
ForecastZone::AL034 => crate::ZoneDetails {state: "AL", zone: "034", zone_numeric: 034, name: "Bibb", wfo: "BMX" },
ForecastZone::AL035 => crate::ZoneDetails {state: "AL", zone: "035", zone_numeric: 035, name: "Chilton", wfo: "BMX" },
ForecastZone::AL036 => crate::ZoneDetails {state: "AL", zone: "036", zone_numeric: 036, name: "Coosa", wfo: "BMX" },
ForecastZone::AL037 => crate::ZoneDetails {state: "AL", zone: "037", zone_numeric: 037, name: "Tallapoosa", wfo: "BMX" },
ForecastZone::AL038 => crate::ZoneDetails {state: "AL", zone: "038", zone_numeric: 038, name: "Chambers", wfo: "BMX" },
ForecastZone::AL039 => crate::ZoneDetails {state: "AL", zone: "039", zone_numeric: 039, name: "Marengo", wfo: "BMX" },
ForecastZone::AL040 => crate::ZoneDetails {state: "AL", zone: "040", zone_numeric: 040, name: "Dallas", wfo: "BMX" },
ForecastZone::AL041 => crate::ZoneDetails {state: "AL", zone: "041", zone_numeric: 041, name: "Autauga", wfo: "BMX" },
ForecastZone::AL042 => crate::ZoneDetails {state: "AL", zone: "042", zone_numeric: 042, name: "Lowndes", wfo: "BMX" },
ForecastZone::AL043 => crate::ZoneDetails {state: "AL", zone: "043", zone_numeric: 043, name: "Elmore", wfo: "BMX" },
ForecastZone::AL044 => crate::ZoneDetails {state: "AL", zone: "044", zone_numeric: 044, name: "Montgomery", wfo: "BMX" },
ForecastZone::AL045 => crate::ZoneDetails {state: "AL", zone: "045", zone_numeric: 045, name: "Macon", wfo: "BMX" },
ForecastZone::AL046 => crate::ZoneDetails {state: "AL", zone: "046", zone_numeric: 046, name: "Bullock", wfo: "BMX" },
ForecastZone::AL047 => crate::ZoneDetails {state: "AL", zone: "047", zone_numeric: 047, name: "Lee", wfo: "BMX" },
ForecastZone::AL048 => crate::ZoneDetails {state: "AL", zone: "048", zone_numeric: 048, name: "Russell", wfo: "BMX" },
ForecastZone::AL049 => crate::ZoneDetails {state: "AL", zone: "049", zone_numeric: 049, name: "Pike", wfo: "BMX" },
ForecastZone::AL050 => crate::ZoneDetails {state: "AL", zone: "050", zone_numeric: 050, name: "Barbour", wfo: "BMX" },
ForecastZone::AL051 => crate::ZoneDetails {state: "AL", zone: "051", zone_numeric: 051, name: "Choctaw", wfo: "MOB" },
ForecastZone::AL052 => crate::ZoneDetails {state: "AL", zone: "052", zone_numeric: 052, name: "Washington", wfo: "MOB" },
ForecastZone::AL053 => crate::ZoneDetails {state: "AL", zone: "053", zone_numeric: 053, name: "Clarke", wfo: "MOB" },
ForecastZone::AL054 => crate::ZoneDetails {state: "AL", zone: "054", zone_numeric: 054, name: "Wilcox", wfo: "MOB" },
ForecastZone::AL055 => crate::ZoneDetails {state: "AL", zone: "055", zone_numeric: 055, name: "Monroe", wfo: "MOB" },
ForecastZone::AL056 => crate::ZoneDetails {state: "AL", zone: "056", zone_numeric: 056, name: "Conecuh", wfo: "MOB" },
ForecastZone::AL057 => crate::ZoneDetails {state: "AL", zone: "057", zone_numeric: 057, name: "Butler", wfo: "MOB" },
ForecastZone::AL058 => crate::ZoneDetails {state: "AL", zone: "058", zone_numeric: 058, name: "Crenshaw", wfo: "MOB" },
ForecastZone::AL059 => crate::ZoneDetails {state: "AL", zone: "059", zone_numeric: 059, name: "Escambia", wfo: "MOB" },
ForecastZone::AL060 => crate::ZoneDetails {state: "AL", zone: "060", zone_numeric: 060, name: "Covington", wfo: "MOB" },
ForecastZone::AL065 => crate::ZoneDetails {state: "AL", zone: "065", zone_numeric: 065, name: "Coffee", wfo: "TAE" },
ForecastZone::AL066 => crate::ZoneDetails {state: "AL", zone: "066", zone_numeric: 066, name: "Dale", wfo: "TAE" },
ForecastZone::AL067 => crate::ZoneDetails {state: "AL", zone: "067", zone_numeric: 067, name: "Henry", wfo: "TAE" },
ForecastZone::AL068 => crate::ZoneDetails {state: "AL", zone: "068", zone_numeric: 068, name: "Geneva", wfo: "TAE" },
ForecastZone::AL069 => crate::ZoneDetails {state: "AL", zone: "069", zone_numeric: 069, name: "Houston", wfo: "TAE" },
ForecastZone::AL261 => crate::ZoneDetails {state: "AL", zone: "261", zone_numeric: 261, name: "Mobile Inland", wfo: "MOB" },
ForecastZone::AL262 => crate::ZoneDetails {state: "AL", zone: "262", zone_numeric: 262, name: "Baldwin Inland", wfo: "MOB" },
ForecastZone::AL263 => crate::ZoneDetails {state: "AL", zone: "263", zone_numeric: 263, name: "Mobile Central", wfo: "MOB" },
ForecastZone::AL264 => crate::ZoneDetails {state: "AL", zone: "264", zone_numeric: 264, name: "Baldwin Central", wfo: "MOB" },
ForecastZone::AL265 => crate::ZoneDetails {state: "AL", zone: "265", zone_numeric: 265, name: "Mobile Coastal", wfo: "MOB" },
ForecastZone::AL266 => crate::ZoneDetails {state: "AL", zone: "266", zone_numeric: 266, name: "Baldwin Coastal", wfo: "MOB" },
ForecastZone::AR001 => crate::ZoneDetails {state: "AR", zone: "001", zone_numeric: 001, name: "Benton", wfo: "TSA" },
ForecastZone::AR002 => crate::ZoneDetails {state: "AR", zone: "002", zone_numeric: 002, name: "Carroll", wfo: "TSA" },
ForecastZone::AR004 => crate::ZoneDetails {state: "AR", zone: "004", zone_numeric: 004, name: "Marion", wfo: "LZK" },
ForecastZone::AR005 => crate::ZoneDetails {state: "AR", zone: "005", zone_numeric: 005, name: "Baxter", wfo: "LZK" },
ForecastZone::AR006 => crate::ZoneDetails {state: "AR", zone: "006", zone_numeric: 006, name: "Fulton", wfo: "LZK" },
ForecastZone::AR007 => crate::ZoneDetails {state: "AR", zone: "007", zone_numeric: 007, name: "Sharp", wfo: "LZK" },
ForecastZone::AR008 => crate::ZoneDetails {state: "AR", zone: "008", zone_numeric: 008, name: "Randolph", wfo: "LZK" },
ForecastZone::AR009 => crate::ZoneDetails {state: "AR", zone: "009", zone_numeric: 009, name: "Clay", wfo: "MEG" },
ForecastZone::AR010 => crate::ZoneDetails {state: "AR", zone: "010", zone_numeric: 010, name: "Washington", wfo: "TSA" },
ForecastZone::AR011 => crate::ZoneDetails {state: "AR", zone: "011", zone_numeric: 011, name: "Madison", wfo: "TSA" },
ForecastZone::AR014 => crate::ZoneDetails {state: "AR", zone: "014", zone_numeric: 014, name: "Stone", wfo: "LZK" },
ForecastZone::AR015 => crate::ZoneDetails {state: "AR", zone: "015", zone_numeric: 015, name: "Izard", wfo: "LZK" },
ForecastZone::AR016 => crate::ZoneDetails {state: "AR", zone: "016", zone_numeric: 016, name: "Independence", wfo: "LZK" },
ForecastZone::AR017 => crate::ZoneDetails {state: "AR", zone: "017", zone_numeric: 017, name: "Lawrence", wfo: "LZK" },
ForecastZone::AR018 => crate::ZoneDetails {state: "AR", zone: "018", zone_numeric: 018, name: "Greene", wfo: "MEG" },
ForecastZone::AR019 => crate::ZoneDetails {state: "AR", zone: "019", zone_numeric: 019, name: "Crawford", wfo: "TSA" },
ForecastZone::AR020 => crate::ZoneDetails {state: "AR", zone: "020", zone_numeric: 020, name: "Franklin", wfo: "TSA" },
ForecastZone::AR024 => crate::ZoneDetails {state: "AR", zone: "024", zone_numeric: 024, name: "Cleburne", wfo: "LZK" },
ForecastZone::AR025 => crate::ZoneDetails {state: "AR", zone: "025", zone_numeric: 025, name: "Jackson", wfo: "LZK" },
ForecastZone::AR026 => crate::ZoneDetails {state: "AR", zone: "026", zone_numeric: 026, name: "Craighead", wfo: "MEG" },
ForecastZone::AR027 => crate::ZoneDetails {state: "AR", zone: "027", zone_numeric: 027, name: "Poinsett", wfo: "MEG" },
ForecastZone::AR028 => crate::ZoneDetails {state: "AR", zone: "028", zone_numeric: 028, name: "Mississippi", wfo: "MEG" },
ForecastZone::AR029 => crate::ZoneDetails {state: "AR", zone: "029", zone_numeric: 029, name: "Sebastian", wfo: "TSA" },
ForecastZone::AR031 => crate::ZoneDetails {state: "AR", zone: "031", zone_numeric: 031, name: "Conway", wfo: "LZK" },
ForecastZone::AR032 => crate::ZoneDetails {state: "AR", zone: "032", zone_numeric: 032, name: "Faulkner", wfo: "LZK" },
ForecastZone::AR033 => crate::ZoneDetails {state: "AR", zone: "033", zone_numeric: 033, name: "White", wfo: "LZK" },
ForecastZone::AR034 => crate::ZoneDetails {state: "AR", zone: "034", zone_numeric: 034, name: "Woodruff", wfo: "LZK" },
ForecastZone::AR035 => crate::ZoneDetails {state: "AR", zone: "035", zone_numeric: 035, name: "Cross", wfo: "MEG" },
ForecastZone::AR036 => crate::ZoneDetails {state: "AR", zone: "036", zone_numeric: 036, name: "Crittenden", wfo: "MEG" },
ForecastZone::AR039 => crate::ZoneDetails {state: "AR", zone: "039", zone_numeric: 039, name: "Perry", wfo: "LZK" },
ForecastZone::AR042 => crate::ZoneDetails {state: "AR", zone: "042", zone_numeric: 042, name: "Garland", wfo: "LZK" },
ForecastZone::AR043 => crate::ZoneDetails {state: "AR", zone: "043", zone_numeric: 043, name: "Saline", wfo: "LZK" },
ForecastZone::AR044 => crate::ZoneDetails {state: "AR", zone: "044", zone_numeric: 044, name: "Pulaski", wfo: "LZK" },
ForecastZone::AR045 => crate::ZoneDetails {state: "AR", zone: "045", zone_numeric: 045, name: "Lonoke", wfo: "LZK" },
ForecastZone::AR046 => crate::ZoneDetails {state: "AR", zone: "046", zone_numeric: 046, name: "Prairie", wfo: "LZK" },
ForecastZone::AR047 => crate::ZoneDetails {state: "AR", zone: "047", zone_numeric: 047, name: "Monroe", wfo: "LZK" },
ForecastZone::AR048 => crate::ZoneDetails {state: "AR", zone: "048", zone_numeric: 048, name: "St. Francis", wfo: "MEG" },
ForecastZone::AR049 => crate::ZoneDetails {state: "AR", zone: "049", zone_numeric: 049, name: "Lee", wfo: "MEG" },
ForecastZone::AR050 => crate::ZoneDetails {state: "AR", zone: "050", zone_numeric: 050, name: "Sevier", wfo: "SHV" },
ForecastZone::AR051 => crate::ZoneDetails {state: "AR", zone: "051", zone_numeric: 051, name: "Howard", wfo: "SHV" },
ForecastZone::AR052 => crate::ZoneDetails {state: "AR", zone: "052", zone_numeric: 052, name: "Pike", wfo: "LZK" },
ForecastZone::AR053 => crate::ZoneDetails {state: "AR", zone: "053", zone_numeric: 053, name: "Clark", wfo: "LZK" },
ForecastZone::AR054 => crate::ZoneDetails {state: "AR", zone: "054", zone_numeric: 054, name: "Hot Spring", wfo: "LZK" },
ForecastZone::AR055 => crate::ZoneDetails {state: "AR", zone: "055", zone_numeric: 055, name: "Grant", wfo: "LZK" },
ForecastZone::AR056 => crate::ZoneDetails {state: "AR", zone: "056", zone_numeric: 056, name: "Jefferson", wfo: "LZK" },
ForecastZone::AR057 => crate::ZoneDetails {state: "AR", zone: "057", zone_numeric: 057, name: "Arkansas", wfo: "LZK" },
ForecastZone::AR058 => crate::ZoneDetails {state: "AR", zone: "058", zone_numeric: 058, name: "Phillips", wfo: "MEG" },
ForecastZone::AR059 => crate::ZoneDetails {state: "AR", zone: "059", zone_numeric: 059, name: "Little River", wfo: "SHV" },
ForecastZone::AR060 => crate::ZoneDetails {state: "AR", zone: "060", zone_numeric: 060, name: "Hempstead", wfo: "SHV" },
ForecastZone::AR061 => crate::ZoneDetails {state: "AR", zone: "061", zone_numeric: 061, name: "Nevada", wfo: "SHV" },
ForecastZone::AR062 => crate::ZoneDetails {state: "AR", zone: "062", zone_numeric: 062, name: "Dallas", wfo: "LZK" },
ForecastZone::AR063 => crate::ZoneDetails {state: "AR", zone: "063", zone_numeric: 063, name: "Cleveland", wfo: "LZK" },
ForecastZone::AR064 => crate::ZoneDetails {state: "AR", zone: "064", zone_numeric: 064, name: "Lincoln", wfo: "LZK" },
ForecastZone::AR065 => crate::ZoneDetails {state: "AR", zone: "065", zone_numeric: 065, name: "Desha", wfo: "LZK" },
ForecastZone::AR066 => crate::ZoneDetails {state: "AR", zone: "066", zone_numeric: 066, name: "Ouachita", wfo: "LZK" },
ForecastZone::AR067 => crate::ZoneDetails {state: "AR", zone: "067", zone_numeric: 067, name: "Calhoun", wfo: "LZK" },
ForecastZone::AR068 => crate::ZoneDetails {state: "AR", zone: "068", zone_numeric: 068, name: "Bradley", wfo: "LZK" },
ForecastZone::AR069 => crate::ZoneDetails {state: "AR", zone: "069", zone_numeric: 069, name: "Drew", wfo: "LZK" },
ForecastZone::AR070 => crate::ZoneDetails {state: "AR", zone: "070", zone_numeric: 070, name: "Miller", wfo: "SHV" },
ForecastZone::AR071 => crate::ZoneDetails {state: "AR", zone: "071", zone_numeric: 071, name: "Lafayette", wfo: "SHV" },
ForecastZone::AR072 => crate::ZoneDetails {state: "AR", zone: "072", zone_numeric: 072, name: "Columbia", wfo: "SHV" },
ForecastZone::AR073 => crate::ZoneDetails {state: "AR", zone: "073", zone_numeric: 073, name: "Union", wfo: "SHV" },
ForecastZone::AR074 => crate::ZoneDetails {state: "AR", zone: "074", zone_numeric: 074, name: "Ashley", wfo: "JAN" },
ForecastZone::AR075 => crate::ZoneDetails {state: "AR", zone: "075", zone_numeric: 075, name: "Chicot", wfo: "JAN" },
ForecastZone::AR103 => crate::ZoneDetails {state: "AR", zone: "103", zone_numeric: 103, name: "Boone County Except Southwest", wfo: "LZK" },
ForecastZone::AR112 => crate::ZoneDetails {state: "AR", zone: "112", zone_numeric: 112, name: "Newton County Higher Elevations", wfo: "LZK" },
ForecastZone::AR113 => crate::ZoneDetails {state: "AR", zone: "113", zone_numeric: 113, name: "Searcy County Lower Elevations", wfo: "LZK" },
ForecastZone::AR121 => crate::ZoneDetails {state: "AR", zone: "121", zone_numeric: 121, name: "Southern Johnson County", wfo: "LZK" },
ForecastZone::AR122 => crate::ZoneDetails {state: "AR", zone: "122", zone_numeric: 122, name: "Southern Pope County", wfo: "LZK" },
ForecastZone::AR123 => crate::ZoneDetails {state: "AR", zone: "123", zone_numeric: 123, name: "Southeast Van Buren County", wfo: "LZK" },
ForecastZone::AR130 => crate::ZoneDetails {state: "AR", zone: "130", zone_numeric: 130, name: "Western and Northern Logan County", wfo: "LZK" },
ForecastZone::AR137 => crate::ZoneDetails {state: "AR", zone: "137", zone_numeric: 137, name: "Northern Scott County", wfo: "LZK" },
ForecastZone::AR138 => crate::ZoneDetails {state: "AR", zone: "138", zone_numeric: 138, name: "Northwest Yell County", wfo: "LZK" },
ForecastZone::AR140 => crate::ZoneDetails {state: "AR", zone: "140", zone_numeric: 140, name: "Polk County Lower Elevations", wfo: "LZK" },
ForecastZone::AR141 => crate::ZoneDetails {state: "AR", zone: "141", zone_numeric: 141, name: "Central and Eastern Montgomery County", wfo: "LZK" },
ForecastZone::AR203 => crate::ZoneDetails {state: "AR", zone: "203", zone_numeric: 203, name: "Boone County Higher Elevations", wfo: "LZK" },
ForecastZone::AR212 => crate::ZoneDetails {state: "AR", zone: "212", zone_numeric: 212, name: "Newton County Lower Elevations", wfo: "LZK" },
ForecastZone::AR213 => crate::ZoneDetails {state: "AR", zone: "213", zone_numeric: 213, name: "Northwest Searcy County Higher Elevations", wfo: "LZK" },
ForecastZone::AR221 => crate::ZoneDetails {state: "AR", zone: "221", zone_numeric: 221, name: "Johnson County Higher Elevations", wfo: "LZK" },
ForecastZone::AR222 => crate::ZoneDetails {state: "AR", zone: "222", zone_numeric: 222, name: "Pope County Higher Elevations", wfo: "LZK" },
ForecastZone::AR223 => crate::ZoneDetails {state: "AR", zone: "223", zone_numeric: 223, name: "Van Buren County Higher Elevations", wfo: "LZK" },
ForecastZone::AR230 => crate::ZoneDetails {state: "AR", zone: "230", zone_numeric: 230, name: "Southern and Eastern Logan County", wfo: "LZK" },
ForecastZone::AR237 => crate::ZoneDetails {state: "AR", zone: "237", zone_numeric: 237, name: "Central and Southern Scott County", wfo: "LZK" },
ForecastZone::AR238 => crate::ZoneDetails {state: "AR", zone: "238", zone_numeric: 238, name: "Yell Excluding Northwest", wfo: "LZK" },
ForecastZone::AR240 => crate::ZoneDetails {state: "AR", zone: "240", zone_numeric: 240, name: "Northern Polk County Higher Elevations", wfo: "LZK" },
ForecastZone::AR241 => crate::ZoneDetails {state: "AR", zone: "241", zone_numeric: 241, name: "Northern Montgomery County Higher Elevations", wfo: "LZK" },
ForecastZone::AR313 => crate::ZoneDetails {state: "AR", zone: "313", zone_numeric: 313, name: "Eastern, Central, and Southern Searcy County Higher Elevations", wfo: "LZK" },
ForecastZone::AR340 => crate::ZoneDetails {state: "AR", zone: "340", zone_numeric: 340, name: "Southeast Polk County Higher Elevations", wfo: "LZK" },
ForecastZone::AR341 => crate::ZoneDetails {state: "AR", zone: "341", zone_numeric: 341, name: "Southwest Montgomery County Higher Elevations", wfo: "LZK" },
ForecastZone::AS001 => crate::ZoneDetails {state: "AS", zone: "001", zone_numeric: 001, name: "Tutuila and Aunuu", wfo: "PPG" },
ForecastZone::AS002 => crate::ZoneDetails {state: "AS", zone: "002", zone_numeric: 002, name: "Manua", wfo: "PPG" },
ForecastZone::AS003 => crate::ZoneDetails {state: "AS", zone: "003", zone_numeric: 003, name: "Swains Island", wfo: "PPG" },
ForecastZone::AZ001 => crate::ZoneDetails {state: "AZ", zone: "001", zone_numeric: 001, name: "Northwest Plateau", wfo: "VEF" },
ForecastZone::AZ002 => crate::ZoneDetails {state: "AZ", zone: "002", zone_numeric: 002, name: "Lake Havasu and Fort Mohave", wfo: "VEF" },
ForecastZone::AZ003 => crate::ZoneDetails {state: "AZ", zone: "003", zone_numeric: 003, name: "Northwest Deserts", wfo: "VEF" },
ForecastZone::AZ004 => crate::ZoneDetails {state: "AZ", zone: "004", zone_numeric: 004, name: "Kaibab Plateau", wfo: "FGZ" },
ForecastZone::AZ005 => crate::ZoneDetails {state: "AZ", zone: "005", zone_numeric: 005, name: "Marble and Glen Canyons", wfo: "FGZ" },
ForecastZone::AZ006 => crate::ZoneDetails {state: "AZ", zone: "006", zone_numeric: 006, name: "Grand Canyon Country", wfo: "FGZ" },
ForecastZone::AZ007 => crate::ZoneDetails {state: "AZ", zone: "007", zone_numeric: 007, name: "Coconino Plateau", wfo: "FGZ" },
ForecastZone::AZ008 => crate::ZoneDetails {state: "AZ", zone: "008", zone_numeric: 008, name: "Yavapai County  Mountains", wfo: "FGZ" },
ForecastZone::AZ009 => crate::ZoneDetails {state: "AZ", zone: "009", zone_numeric: 009, name: "Northeast Plateaus and Mesas Hwy 264 Northward", wfo: "FGZ" },
ForecastZone::AZ010 => crate::ZoneDetails {state: "AZ", zone: "010", zone_numeric: 010, name: "Chinle Valley", wfo: "FGZ" },
ForecastZone::AZ011 => crate::ZoneDetails {state: "AZ", zone: "011", zone_numeric: 011, name: "Chuska Mountains and Defiance Plateau", wfo: "FGZ" },
ForecastZone::AZ012 => crate::ZoneDetails {state: "AZ", zone: "012", zone_numeric: 012, name: "Little Colorado River Valley in Coconino County", wfo: "FGZ" },
ForecastZone::AZ013 => crate::ZoneDetails {state: "AZ", zone: "013", zone_numeric: 013, name: "Little Colorado River Valley in Navajo County", wfo: "FGZ" },
ForecastZone::AZ014 => crate::ZoneDetails {state: "AZ", zone: "014", zone_numeric: 014, name: "Little Colorado River Valley in Apache County", wfo: "FGZ" },
ForecastZone::AZ015 => crate::ZoneDetails {state: "AZ", zone: "015", zone_numeric: 015, name: "Western Mogollon Rim", wfo: "FGZ" },
ForecastZone::AZ016 => crate::ZoneDetails {state: "AZ", zone: "016", zone_numeric: 016, name: "Eastern Mogollon Rim", wfo: "FGZ" },
ForecastZone::AZ017 => crate::ZoneDetails {state: "AZ", zone: "017", zone_numeric: 017, name: "White Mountains", wfo: "FGZ" },
ForecastZone::AZ018 => crate::ZoneDetails {state: "AZ", zone: "018", zone_numeric: 018, name: "Northern Gila County", wfo: "FGZ" },
ForecastZone::AZ036 => crate::ZoneDetails {state: "AZ", zone: "036", zone_numeric: 036, name: "Lake Mead National Recreation Area", wfo: "VEF" },
ForecastZone::AZ037 => crate::ZoneDetails {state: "AZ", zone: "037", zone_numeric: 037, name: "Yavapai County Valleys and Basins", wfo: "FGZ" },
ForecastZone::AZ038 => crate::ZoneDetails {state: "AZ", zone: "038", zone_numeric: 038, name: "Oak Creek and Sycamore Canyons", wfo: "FGZ" },
ForecastZone::AZ039 => crate::ZoneDetails {state: "AZ", zone: "039", zone_numeric: 039, name: "Black Mesa Area", wfo: "FGZ" },
ForecastZone::AZ040 => crate::ZoneDetails {state: "AZ", zone: "040", zone_numeric: 040, name: "Northeast Plateaus and Mesas South of Hwy 264", wfo: "FGZ" },
ForecastZone::AZ501 => crate::ZoneDetails {state: "AZ", zone: "501", zone_numeric: 501, name: "Western Pima County Including Ajo/Organ Pipe Cactus National Monument", wfo: "TWC" },
ForecastZone::AZ502 => crate::ZoneDetails {state: "AZ", zone: "502", zone_numeric: 502, name: "Tohono O'odham Nation including Sells", wfo: "TWC" },
ForecastZone::AZ503 => crate::ZoneDetails {state: "AZ", zone: "503", zone_numeric: 503, name: "Upper Santa Cruz River and Altar Valleys including Nogales", wfo: "TWC" },
ForecastZone::AZ504 => crate::ZoneDetails {state: "AZ", zone: "504", zone_numeric: 504, name: "Tucson Metro Area including Tucson/Green Valley/Marana/Vail", wfo: "TWC" },
ForecastZone::AZ505 => crate::ZoneDetails {state: "AZ", zone: "505", zone_numeric: 505, name: "South Central Pinal County including Eloy/Picacho Peak State Park", wfo: "TWC" },
ForecastZone::AZ506 => crate::ZoneDetails {state: "AZ", zone: "506", zone_numeric: 506, name: "Southeast Pinal County including Kearny/Mammoth/Oracle", wfo: "TWC" },
ForecastZone::AZ507 => crate::ZoneDetails {state: "AZ", zone: "507", zone_numeric: 507, name: "Upper San Pedro River Valley including Sierra Vista/Benson", wfo: "TWC" },
ForecastZone::AZ508 => crate::ZoneDetails {state: "AZ", zone: "508", zone_numeric: 508, name: "Eastern Cochise County Below 5000 Feet including Douglas/Willcox", wfo: "TWC" },
ForecastZone::AZ509 => crate::ZoneDetails {state: "AZ", zone: "509", zone_numeric: 509, name: "Upper Gila River and Aravaipa Valleys including Clifton/Safford", wfo: "TWC" },
ForecastZone::AZ510 => crate::ZoneDetails {state: "AZ", zone: "510", zone_numeric: 510, name: "White Mountains of Graham and Greenlee Counties including Hannagan Meadow", wfo: "TWC" },
ForecastZone::AZ511 => crate::ZoneDetails {state: "AZ", zone: "511", zone_numeric: 511, name: "Galiuro and Pinaleno Mountains including Mount Graham", wfo: "TWC" },
ForecastZone::AZ512 => crate::ZoneDetails {state: "AZ", zone: "512", zone_numeric: 512, name: "Chiricahua Mountains including Chiricahua National Monument", wfo: "TWC" },
ForecastZone::AZ513 => crate::ZoneDetails {state: "AZ", zone: "513", zone_numeric: 513, name: "Dragoon/Mule/Huachuca and Santa Rita Mountains including Bisbee/Canelo Hills/Madera Canyon", wfo: "TWC" },
ForecastZone::AZ514 => crate::ZoneDetails {state: "AZ", zone: "514", zone_numeric: 514, name: "Santa Catalina and Rincon Mountains including Mount Lemmon/Summerhaven", wfo: "TWC" },
ForecastZone::AZ515 => crate::ZoneDetails {state: "AZ", zone: "515", zone_numeric: 515, name: "Baboquivari Mountains including Kitt Peak", wfo: "TWC" },
ForecastZone::AZ530 => crate::ZoneDetails {state: "AZ", zone: "530", zone_numeric: 530, name: "Parker Valley", wfo: "PSR" },
ForecastZone::AZ531 => crate::ZoneDetails {state: "AZ", zone: "531", zone_numeric: 531, name: "Kofa", wfo: "PSR" },
ForecastZone::AZ532 => crate::ZoneDetails {state: "AZ", zone: "532", zone_numeric: 532, name: "Yuma", wfo: "PSR" },
ForecastZone::AZ533 => crate::ZoneDetails {state: "AZ", zone: "533", zone_numeric: 533, name: "Central La Paz", wfo: "PSR" },
ForecastZone::AZ534 => crate::ZoneDetails {state: "AZ", zone: "534", zone_numeric: 534, name: "Aguila Valley", wfo: "PSR" },
ForecastZone::AZ535 => crate::ZoneDetails {state: "AZ", zone: "535", zone_numeric: 535, name: "Southeast Yuma County", wfo: "PSR" },
ForecastZone::AZ536 => crate::ZoneDetails {state: "AZ", zone: "536", zone_numeric: 536, name: "Gila River Valley", wfo: "PSR" },
ForecastZone::AZ537 => crate::ZoneDetails {state: "AZ", zone: "537", zone_numeric: 537, name: "Northwest Valley", wfo: "PSR" },
ForecastZone::AZ538 => crate::ZoneDetails {state: "AZ", zone: "538", zone_numeric: 538, name: "Tonopah Desert", wfo: "PSR" },
ForecastZone::AZ539 => crate::ZoneDetails {state: "AZ", zone: "539", zone_numeric: 539, name: "Gila Bend", wfo: "PSR" },
ForecastZone::AZ540 => crate::ZoneDetails {state: "AZ", zone: "540", zone_numeric: 540, name: "Buckeye/Avondale", wfo: "PSR" },
ForecastZone::AZ541 => crate::ZoneDetails {state: "AZ", zone: "541", zone_numeric: 541, name: "Cave Creek/New River", wfo: "PSR" },
ForecastZone::AZ542 => crate::ZoneDetails {state: "AZ", zone: "542", zone_numeric: 542, name: "Deer Valley", wfo: "PSR" },
ForecastZone::AZ543 => crate::ZoneDetails {state: "AZ", zone: "543", zone_numeric: 543, name: "Central Phoenix", wfo: "PSR" },
ForecastZone::AZ544 => crate::ZoneDetails {state: "AZ", zone: "544", zone_numeric: 544, name: "North Phoenix/Glendale", wfo: "PSR" },
ForecastZone::AZ545 => crate::ZoneDetails {state: "AZ", zone: "545", zone_numeric: 545, name: "New River Mesa", wfo: "PSR" },
ForecastZone::AZ546 => crate::ZoneDetails {state: "AZ", zone: "546", zone_numeric: 546, name: "Scottsdale/Paradise Valley", wfo: "PSR" },
ForecastZone::AZ547 => crate::ZoneDetails {state: "AZ", zone: "547", zone_numeric: 547, name: "Rio Verde/Salt River", wfo: "PSR" },
ForecastZone::AZ548 => crate::ZoneDetails {state: "AZ", zone: "548", zone_numeric: 548, name: "East Valley", wfo: "PSR" },
ForecastZone::AZ549 => crate::ZoneDetails {state: "AZ", zone: "549", zone_numeric: 549, name: "Fountain Hills/East Mesa", wfo: "PSR" },
ForecastZone::AZ550 => crate::ZoneDetails {state: "AZ", zone: "550", zone_numeric: 550, name: "South Mountain/Ahwatukee", wfo: "PSR" },
ForecastZone::AZ551 => crate::ZoneDetails {state: "AZ", zone: "551", zone_numeric: 551, name: "Southeast Valley/Queen Creek", wfo: "PSR" },
ForecastZone::AZ552 => crate::ZoneDetails {state: "AZ", zone: "552", zone_numeric: 552, name: "Superior", wfo: "PSR" },
ForecastZone::AZ553 => crate::ZoneDetails {state: "AZ", zone: "553", zone_numeric: 553, name: "Northwest Pinal County", wfo: "PSR" },
ForecastZone::AZ554 => crate::ZoneDetails {state: "AZ", zone: "554", zone_numeric: 554, name: "West Pinal County", wfo: "PSR" },
ForecastZone::AZ555 => crate::ZoneDetails {state: "AZ", zone: "555", zone_numeric: 555, name: "Apache Junction/Gold Canyon", wfo: "PSR" },
ForecastZone::AZ556 => crate::ZoneDetails {state: "AZ", zone: "556", zone_numeric: 556, name: "Tonto Basin", wfo: "PSR" },
ForecastZone::AZ557 => crate::ZoneDetails {state: "AZ", zone: "557", zone_numeric: 557, name: "Mazatzal Mountains", wfo: "PSR" },
ForecastZone::AZ558 => crate::ZoneDetails {state: "AZ", zone: "558", zone_numeric: 558, name: "Pinal/Superstition Mountains", wfo: "PSR" },
ForecastZone::AZ559 => crate::ZoneDetails {state: "AZ", zone: "559", zone_numeric: 559, name: "Sonoran Desert Natl Monument", wfo: "PSR" },
ForecastZone::AZ560 => crate::ZoneDetails {state: "AZ", zone: "560", zone_numeric: 560, name: "San Carlos", wfo: "PSR" },
ForecastZone::AZ561 => crate::ZoneDetails {state: "AZ", zone: "561", zone_numeric: 561, name: "Dripping Springs", wfo: "PSR" },
ForecastZone::AZ562 => crate::ZoneDetails {state: "AZ", zone: "562", zone_numeric: 562, name: "Globe/Miami", wfo: "PSR" },
ForecastZone::AZ563 => crate::ZoneDetails {state: "AZ", zone: "563", zone_numeric: 563, name: "Southeast Gila County", wfo: "PSR" },
ForecastZone::CA006 => crate::ZoneDetails {state: "CA", zone: "006", zone_numeric: 006, name: "San Francisco", wfo: "MTR" },
ForecastZone::CA013 => crate::ZoneDetails {state: "CA", zone: "013", zone_numeric: 013, name: "Shasta Lake Area / Northern Shasta County", wfo: "STO" },
ForecastZone::CA014 => crate::ZoneDetails {state: "CA", zone: "014", zone_numeric: 014, name: "Burney Basin / Eastern Shasta County", wfo: "STO" },
ForecastZone::CA015 => crate::ZoneDetails {state: "CA", zone: "015", zone_numeric: 015, name: "Northern Sacramento Valley", wfo: "STO" },
ForecastZone::CA016 => crate::ZoneDetails {state: "CA", zone: "016", zone_numeric: 016, name: "Central Sacramento Valley", wfo: "STO" },
ForecastZone::CA017 => crate::ZoneDetails {state: "CA", zone: "017", zone_numeric: 017, name: "Southern Sacramento Valley", wfo: "STO" },
ForecastZone::CA018 => crate::ZoneDetails {state: "CA", zone: "018", zone_numeric: 018, name: "Carquinez Strait and Delta", wfo: "STO" },
ForecastZone::CA019 => crate::ZoneDetails {state: "CA", zone: "019", zone_numeric: 019, name: "Northern San Joaquin Valley", wfo: "STO" },
ForecastZone::CA038 => crate::ZoneDetails {state: "CA", zone: "038", zone_numeric: 038, name: "Cuyama Valley", wfo: "LOX" },
ForecastZone::CA043 => crate::ZoneDetails {state: "CA", zone: "043", zone_numeric: 043, name: "San Diego County Coastal Areas", wfo: "SGX" },
ForecastZone::CA048 => crate::ZoneDetails {state: "CA", zone: "048", zone_numeric: 048, name: "San Bernardino and Riverside County Valleys-The Inland Empire", wfo: "SGX" },
ForecastZone::CA050 => crate::ZoneDetails {state: "CA", zone: "050", zone_numeric: 050, name: "San Diego County Inland Valleys", wfo: "SGX" },
ForecastZone::CA053 => crate::ZoneDetails {state: "CA", zone: "053", zone_numeric: 053, name: "Ventura County Mountains", wfo: "LOX" },
ForecastZone::CA054 => crate::ZoneDetails {state: "CA", zone: "054", zone_numeric: 054, name: "Los Angeles County Mountains", wfo: "LOX" },
ForecastZone::CA055 => crate::ZoneDetails {state: "CA", zone: "055", zone_numeric: 055, name: "San Bernardino County Mountains", wfo: "SGX" },
ForecastZone::CA056 => crate::ZoneDetails {state: "CA", zone: "056", zone_numeric: 056, name: "Riverside County Mountains", wfo: "SGX" },
ForecastZone::CA057 => crate::ZoneDetails {state: "CA", zone: "057", zone_numeric: 057, name: "Santa Ana Mountains and Foothills", wfo: "SGX" },
ForecastZone::CA058 => crate::ZoneDetails {state: "CA", zone: "058", zone_numeric: 058, name: "San Diego County Mountains", wfo: "SGX" },
ForecastZone::CA059 => crate::ZoneDetails {state: "CA", zone: "059", zone_numeric: 059, name: "Antelope Valley", wfo: "LOX" },
ForecastZone::CA060 => crate::ZoneDetails {state: "CA", zone: "060", zone_numeric: 060, name: "Apple and Lucerne Valleys", wfo: "SGX" },
ForecastZone::CA061 => crate::ZoneDetails {state: "CA", zone: "061", zone_numeric: 061, name: "Coachella Valley", wfo: "SGX" },
ForecastZone::CA062 => crate::ZoneDetails {state: "CA", zone: "062", zone_numeric: 062, name: "San Diego County Deserts", wfo: "SGX" },
ForecastZone::CA063 => crate::ZoneDetails {state: "CA", zone: "063", zone_numeric: 063, name: "Mountains Southwestern Shasta County to Western Colusa County", wfo: "STO" },
ForecastZone::CA065 => crate::ZoneDetails {state: "CA", zone: "065", zone_numeric: 065, name: "San Gorgonio Pass Near Banning", wfo: "SGX" },
ForecastZone::CA066 => crate::ZoneDetails {state: "CA", zone: "066", zone_numeric: 066, name: "Northeast Foothills/Sacramento Valley", wfo: "STO" },
ForecastZone::CA067 => crate::ZoneDetails {state: "CA", zone: "067", zone_numeric: 067, name: "Motherlode", wfo: "STO" },
ForecastZone::CA068 => crate::ZoneDetails {state: "CA", zone: "068", zone_numeric: 068, name: "Western Plumas County/Lassen Park", wfo: "STO" },
ForecastZone::CA069 => crate::ZoneDetails {state: "CA", zone: "069", zone_numeric: 069, name: "West Slope Northern Sierra Nevada", wfo: "STO" },
ForecastZone::CA070 => crate::ZoneDetails {state: "CA", zone: "070", zone_numeric: 070, name: "Surprise Valley California", wfo: "REV" },
ForecastZone::CA071 => crate::ZoneDetails {state: "CA", zone: "071", zone_numeric: 071, name: "Lassen-Eastern Plumas-Eastern Sierra Counties", wfo: "REV" },
ForecastZone::CA072 => crate::ZoneDetails {state: "CA", zone: "072", zone_numeric: 072, name: "Greater Lake Tahoe Area", wfo: "REV" },
ForecastZone::CA073 => crate::ZoneDetails {state: "CA", zone: "073", zone_numeric: 073, name: "Mono", wfo: "REV" },
ForecastZone::CA080 => crate::ZoneDetails {state: "CA", zone: "080", zone_numeric: 080, name: "Western Siskiyou County", wfo: "MFR" },
ForecastZone::CA081 => crate::ZoneDetails {state: "CA", zone: "081", zone_numeric: 081, name: "Central Siskiyou County", wfo: "MFR" },
ForecastZone::CA082 => crate::ZoneDetails {state: "CA", zone: "082", zone_numeric: 082, name: "South Central Siskiyou County", wfo: "MFR" },
ForecastZone::CA083 => crate::ZoneDetails {state: "CA", zone: "083", zone_numeric: 083, name: "North Central and Southeast Siskiyou County", wfo: "MFR" },
ForecastZone::CA084 => crate::ZoneDetails {state: "CA", zone: "084", zone_numeric: 084, name: "Northeast Siskiyou and Northwest Modoc Counties", wfo: "MFR" },
ForecastZone::CA085 => crate::ZoneDetails {state: "CA", zone: "085", zone_numeric: 085, name: "Modoc County", wfo: "MFR" },
ForecastZone::CA087 => crate::ZoneDetails {state: "CA", zone: "087", zone_numeric: 087, name: "Catalina and Santa Barbara Islands", wfo: "LOX" },
ForecastZone::CA088 => crate::ZoneDetails {state: "CA", zone: "088", zone_numeric: 088, name: "Santa Clarita Valley", wfo: "LOX" },
ForecastZone::CA101 => crate::ZoneDetails {state: "CA", zone: "101", zone_numeric: 101, name: "Coastal Del Norte", wfo: "EKA" },
ForecastZone::CA102 => crate::ZoneDetails {state: "CA", zone: "102", zone_numeric: 102, name: "Del Norte Interior", wfo: "EKA" },
ForecastZone::CA103 => crate::ZoneDetails {state: "CA", zone: "103", zone_numeric: 103, name: "Northern Humboldt Coast", wfo: "EKA" },
ForecastZone::CA104 => crate::ZoneDetails {state: "CA", zone: "104", zone_numeric: 104, name: "Southwestern Humboldt", wfo: "EKA" },
ForecastZone::CA105 => crate::ZoneDetails {state: "CA", zone: "105", zone_numeric: 105, name: "Northern Humboldt Interior", wfo: "EKA" },
ForecastZone::CA106 => crate::ZoneDetails {state: "CA", zone: "106", zone_numeric: 106, name: "Southern Humboldt Interior", wfo: "EKA" },
ForecastZone::CA107 => crate::ZoneDetails {state: "CA", zone: "107", zone_numeric: 107, name: "Northern Trinity", wfo: "EKA" },
ForecastZone::CA108 => crate::ZoneDetails {state: "CA", zone: "108", zone_numeric: 108, name: "Southern Trinity", wfo: "EKA" },
ForecastZone::CA109 => crate::ZoneDetails {state: "CA", zone: "109", zone_numeric: 109, name: "Mendocino Coast", wfo: "EKA" },
ForecastZone::CA110 => crate::ZoneDetails {state: "CA", zone: "110", zone_numeric: 110, name: "Northwestern Mendocino Interior", wfo: "EKA" },
ForecastZone::CA111 => crate::ZoneDetails {state: "CA", zone: "111", zone_numeric: 111, name: "Northeastern Mendocino Interior", wfo: "EKA" },
ForecastZone::CA112 => crate::ZoneDetails {state: "CA", zone: "112", zone_numeric: 112, name: "Southwestern Mendocino Interior", wfo: "EKA" },
ForecastZone::CA113 => crate::ZoneDetails {state: "CA", zone: "113", zone_numeric: 113, name: "Southeastern Mendocino Interior", wfo: "EKA" },
ForecastZone::CA114 => crate::ZoneDetails {state: "CA", zone: "114", zone_numeric: 114, name: "Northern Lake County", wfo: "EKA" },
ForecastZone::CA115 => crate::ZoneDetails {state: "CA", zone: "115", zone_numeric: 115, name: "Southern Lake County", wfo: "EKA" },
ForecastZone::CA300 => crate::ZoneDetails {state: "CA", zone: "300", zone_numeric: 300, name: "West Side Mountains north of 198", wfo: "HNX" },
ForecastZone::CA301 => crate::ZoneDetails {state: "CA", zone: "301", zone_numeric: 301, name: "Los Banos - Dos Palos", wfo: "HNX" },
ForecastZone::CA302 => crate::ZoneDetails {state: "CA", zone: "302", zone_numeric: 302, name: "Merced - Madera - Mendota", wfo: "HNX" },
ForecastZone::CA303 => crate::ZoneDetails {state: "CA", zone: "303", zone_numeric: 303, name: "Planada - Le Grand - Snelling", wfo: "HNX" },
ForecastZone::CA304 => crate::ZoneDetails {state: "CA", zone: "304", zone_numeric: 304, name: "Coalinga - Avenal", wfo: "HNX" },
ForecastZone::CA305 => crate::ZoneDetails {state: "CA", zone: "305", zone_numeric: 305, name: "West Side of Fresno and Kings Counties", wfo: "HNX" },
ForecastZone::CA306 => crate::ZoneDetails {state: "CA", zone: "306", zone_numeric: 306, name: "Caruthers - San Joaquin - Selma", wfo: "HNX" },
ForecastZone::CA307 => crate::ZoneDetails {state: "CA", zone: "307", zone_numeric: 307, name: "Fresno-Clovis", wfo: "HNX" },
ForecastZone::CA308 => crate::ZoneDetails {state: "CA", zone: "308", zone_numeric: 308, name: "West Side Mountains South of 198", wfo: "HNX" },
ForecastZone::CA309 => crate::ZoneDetails {state: "CA", zone: "309", zone_numeric: 309, name: "Buttonwillow - Lost Hills - I5", wfo: "HNX" },
ForecastZone::CA310 => crate::ZoneDetails {state: "CA", zone: "310", zone_numeric: 310, name: "Delano-Wasco-Shafter", wfo: "HNX" },
ForecastZone::CA311 => crate::ZoneDetails {state: "CA", zone: "311", zone_numeric: 311, name: "Hanford - Corcoran - Lemoore", wfo: "HNX" },
ForecastZone::CA312 => crate::ZoneDetails {state: "CA", zone: "312", zone_numeric: 312, name: "Visalia - Porterville - Reedley", wfo: "HNX" },
ForecastZone::CA313 => crate::ZoneDetails {state: "CA", zone: "313", zone_numeric: 313, name: "Buena Vista", wfo: "HNX" },
ForecastZone::CA314 => crate::ZoneDetails {state: "CA", zone: "314", zone_numeric: 314, name: "Bakersfield", wfo: "HNX" },
ForecastZone::CA315 => crate::ZoneDetails {state: "CA", zone: "315", zone_numeric: 315, name: "Southeast San Joaquin Valley", wfo: "HNX" },
ForecastZone::CA316 => crate::ZoneDetails {state: "CA", zone: "316", zone_numeric: 316, name: "South End San Joaquin Valley", wfo: "HNX" },
ForecastZone::CA317 => crate::ZoneDetails {state: "CA", zone: "317", zone_numeric: 317, name: "Mariposa Madera Foothills", wfo: "HNX" },
ForecastZone::CA318 => crate::ZoneDetails {state: "CA", zone: "318", zone_numeric: 318, name: "Mariposa-Madera Lower Sierra", wfo: "HNX" },
ForecastZone::CA319 => crate::ZoneDetails {state: "CA", zone: "319", zone_numeric: 319, name: "Fresno-Tulare Foothills", wfo: "HNX" },
ForecastZone::CA320 => crate::ZoneDetails {state: "CA", zone: "320", zone_numeric: 320, name: "Fresno-Tulare Lower Sierra", wfo: "HNX" },
ForecastZone::CA321 => crate::ZoneDetails {state: "CA", zone: "321", zone_numeric: 321, name: "South End Sierra Foothills", wfo: "HNX" },
ForecastZone::CA322 => crate::ZoneDetails {state: "CA", zone: "322", zone_numeric: 322, name: "South End of the Lower Sierra", wfo: "HNX" },
ForecastZone::CA323 => crate::ZoneDetails {state: "CA", zone: "323", zone_numeric: 323, name: "Yosemite NP outside of the valley", wfo: "HNX" },
ForecastZone::CA324 => crate::ZoneDetails {state: "CA", zone: "324", zone_numeric: 324, name: "Yosemite Valley", wfo: "HNX" },
ForecastZone::CA325 => crate::ZoneDetails {state: "CA", zone: "325", zone_numeric: 325, name: "San Joaquin River Canyon", wfo: "HNX" },
ForecastZone::CA326 => crate::ZoneDetails {state: "CA", zone: "326", zone_numeric: 326, name: "Upper San Joaquin River", wfo: "HNX" },
ForecastZone::CA327 => crate::ZoneDetails {state: "CA", zone: "327", zone_numeric: 327, name: "Kaiser to Rodgers Ridge", wfo: "HNX" },
ForecastZone::CA328 => crate::ZoneDetails {state: "CA", zone: "328", zone_numeric: 328, name: "Kings Canyon NP", wfo: "HNX" },
ForecastZone::CA329 => crate::ZoneDetails {state: "CA", zone: "329", zone_numeric: 329, name: "Grant Grove Area", wfo: "HNX" },
ForecastZone::CA330 => crate::ZoneDetails {state: "CA", zone: "330", zone_numeric: 330, name: "Sequoia NP", wfo: "HNX" },
ForecastZone::CA331 => crate::ZoneDetails {state: "CA", zone: "331", zone_numeric: 331, name: "South End of the Upper Sierra", wfo: "HNX" },
ForecastZone::CA332 => crate::ZoneDetails {state: "CA", zone: "332", zone_numeric: 332, name: "Kern River Valley", wfo: "HNX" },
ForecastZone::CA333 => crate::ZoneDetails {state: "CA", zone: "333", zone_numeric: 333, name: "Piute Walker Basin", wfo: "HNX" },
ForecastZone::CA334 => crate::ZoneDetails {state: "CA", zone: "334", zone_numeric: 334, name: "Tehachapi", wfo: "HNX" },
ForecastZone::CA335 => crate::ZoneDetails {state: "CA", zone: "335", zone_numeric: 335, name: "Grapevine", wfo: "HNX" },
ForecastZone::CA336 => crate::ZoneDetails {state: "CA", zone: "336", zone_numeric: 336, name: "Frazier Mountain Communities", wfo: "HNX" },
ForecastZone::CA337 => crate::ZoneDetails {state: "CA", zone: "337", zone_numeric: 337, name: "Indian Wells Valley", wfo: "HNX" },
ForecastZone::CA338 => crate::ZoneDetails {state: "CA", zone: "338", zone_numeric: 338, name: "Mojave Desert Slopes", wfo: "HNX" },
ForecastZone::CA339 => crate::ZoneDetails {state: "CA", zone: "339", zone_numeric: 339, name: "Mojave Desert", wfo: "HNX" },
ForecastZone::CA340 => crate::ZoneDetails {state: "CA", zone: "340", zone_numeric: 340, name: "San Luis Obispo County Beaches", wfo: "LOX" },
ForecastZone::CA341 => crate::ZoneDetails {state: "CA", zone: "341", zone_numeric: 341, name: "San Luis Obispo County Inland Central Coast", wfo: "LOX" },
ForecastZone::CA342 => crate::ZoneDetails {state: "CA", zone: "342", zone_numeric: 342, name: "Santa Lucia Mountains", wfo: "LOX" },
ForecastZone::CA343 => crate::ZoneDetails {state: "CA", zone: "343", zone_numeric: 343, name: "Southern Salinas Valley", wfo: "LOX" },
ForecastZone::CA344 => crate::ZoneDetails {state: "CA", zone: "344", zone_numeric: 344, name: "San Luis Obispo County Interior Valleys", wfo: "LOX" },
ForecastZone::CA345 => crate::ZoneDetails {state: "CA", zone: "345", zone_numeric: 345, name: "San Luis Obispo County Mountains", wfo: "LOX" },
ForecastZone::CA346 => crate::ZoneDetails {state: "CA", zone: "346", zone_numeric: 346, name: "Santa Barbara County Central Coast Beaches", wfo: "LOX" },
ForecastZone::CA347 => crate::ZoneDetails {state: "CA", zone: "347", zone_numeric: 347, name: "Santa Barbara County Inland Central Coast", wfo: "LOX" },
ForecastZone::CA348 => crate::ZoneDetails {state: "CA", zone: "348", zone_numeric: 348, name: "Santa Ynez Valley", wfo: "LOX" },
ForecastZone::CA349 => crate::ZoneDetails {state: "CA", zone: "349", zone_numeric: 349, name: "Santa Barbara County Southwestern Coast", wfo: "LOX" },
ForecastZone::CA350 => crate::ZoneDetails {state: "CA", zone: "350", zone_numeric: 350, name: "Santa Barbara County Southeastern Coast", wfo: "LOX" },
ForecastZone::CA351 => crate::ZoneDetails {state: "CA", zone: "351", zone_numeric: 351, name: "Santa Ynez Mountains Western Range", wfo: "LOX" },
ForecastZone::CA352 => crate::ZoneDetails {state: "CA", zone: "352", zone_numeric: 352, name: "Santa Ynez Mountains Eastern Range", wfo: "LOX" },
ForecastZone::CA353 => crate::ZoneDetails {state: "CA", zone: "353", zone_numeric: 353, name: "Santa Barbara County Interior Mountains", wfo: "LOX" },
ForecastZone::CA354 => crate::ZoneDetails {state: "CA", zone: "354", zone_numeric: 354, name: "Ventura County Beaches", wfo: "LOX" },
ForecastZone::CA355 => crate::ZoneDetails {state: "CA", zone: "355", zone_numeric: 355, name: "Ventura County Inland Coast", wfo: "LOX" },
ForecastZone::CA356 => crate::ZoneDetails {state: "CA", zone: "356", zone_numeric: 356, name: "Lake Casitas", wfo: "LOX" },
ForecastZone::CA357 => crate::ZoneDetails {state: "CA", zone: "357", zone_numeric: 357, name: "Ojai Valley", wfo: "LOX" },
ForecastZone::CA358 => crate::ZoneDetails {state: "CA", zone: "358", zone_numeric: 358, name: "Central Ventura County Valleys", wfo: "LOX" },
ForecastZone::CA359 => crate::ZoneDetails {state: "CA", zone: "359", zone_numeric: 359, name: "Southeastern Ventura County Valleys", wfo: "LOX" },
ForecastZone::CA362 => crate::ZoneDetails {state: "CA", zone: "362", zone_numeric: 362, name: "Malibu Coast", wfo: "LOX" },
ForecastZone::CA363 => crate::ZoneDetails {state: "CA", zone: "363", zone_numeric: 363, name: "Santa Monica Mountains", wfo: "LOX" },
ForecastZone::CA364 => crate::ZoneDetails {state: "CA", zone: "364", zone_numeric: 364, name: "Los Angeles County Beaches", wfo: "LOX" },
ForecastZone::CA365 => crate::ZoneDetails {state: "CA", zone: "365", zone_numeric: 365, name: "Los Angeles County Inland Coast including Downtown Los Angeles", wfo: "LOX" },
ForecastZone::CA502 => crate::ZoneDetails {state: "CA", zone: "502", zone_numeric: 502, name: "Marin Coastal Range", wfo: "MTR" },
ForecastZone::CA503 => crate::ZoneDetails {state: "CA", zone: "503", zone_numeric: 503, name: "Sonoma Coastal Range", wfo: "MTR" },
ForecastZone::CA504 => crate::ZoneDetails {state: "CA", zone: "504", zone_numeric: 504, name: "North Bay Interior Mountains", wfo: "MTR" },
ForecastZone::CA505 => crate::ZoneDetails {state: "CA", zone: "505", zone_numeric: 505, name: "Coastal North Bay Including Point Reyes National Seashore", wfo: "MTR" },
ForecastZone::CA506 => crate::ZoneDetails {state: "CA", zone: "506", zone_numeric: 506, name: "North Bay Interior Valleys", wfo: "MTR" },
ForecastZone::CA508 => crate::ZoneDetails {state: "CA", zone: "508", zone_numeric: 508, name: "San Francisco Bay Shoreline", wfo: "MTR" },
ForecastZone::CA509 => crate::ZoneDetails {state: "CA", zone: "509", zone_numeric: 509, name: "San Fransisco Peninsula Coast", wfo: "MTR" },
ForecastZone::CA510 => crate::ZoneDetails {state: "CA", zone: "510", zone_numeric: 510, name: "East Bay Interior Valleys", wfo: "MTR" },
ForecastZone::CA512 => crate::ZoneDetails {state: "CA", zone: "512", zone_numeric: 512, name: "Santa Cruz Mountains", wfo: "MTR" },
ForecastZone::CA513 => crate::ZoneDetails {state: "CA", zone: "513", zone_numeric: 513, name: "Santa Clara Valley Including San Jose", wfo: "MTR" },
ForecastZone::CA514 => crate::ZoneDetails {state: "CA", zone: "514", zone_numeric: 514, name: "Eastern Santa Clara Hills", wfo: "MTR" },
ForecastZone::CA515 => crate::ZoneDetails {state: "CA", zone: "515", zone_numeric: 515, name: "East Bay Hills", wfo: "MTR" },
ForecastZone::CA516 => crate::ZoneDetails {state: "CA", zone: "516", zone_numeric: 516, name: "Southern Salinas Valley/Arroyo Seco and Lake San Antonio", wfo: "MTR" },
ForecastZone::CA517 => crate::ZoneDetails {state: "CA", zone: "517", zone_numeric: 517, name: "Santa Lucia Mountains and Los Padres National Forest", wfo: "MTR" },
ForecastZone::CA518 => crate::ZoneDetails {state: "CA", zone: "518", zone_numeric: 518, name: "Mountains Of San Benito County And Interior Monterey County Including Pinnacles National Monument", wfo: "MTR" },
ForecastZone::CA519 => crate::ZoneDetails {state: "CA", zone: "519", zone_numeric: 519, name: "Eastern Sierra Slopes of Inyo County", wfo: "VEF" },
ForecastZone::CA520 => crate::ZoneDetails {state: "CA", zone: "520", zone_numeric: 520, name: "Owens Valley", wfo: "VEF" },
ForecastZone::CA521 => crate::ZoneDetails {state: "CA", zone: "521", zone_numeric: 521, name: "White Mountains of Inyo County", wfo: "VEF" },
ForecastZone::CA522 => crate::ZoneDetails {state: "CA", zone: "522", zone_numeric: 522, name: "Death Valley National Park", wfo: "VEF" },
ForecastZone::CA523 => crate::ZoneDetails {state: "CA", zone: "523", zone_numeric: 523, name: "Western Mojave Desert", wfo: "VEF" },
ForecastZone::CA524 => crate::ZoneDetails {state: "CA", zone: "524", zone_numeric: 524, name: "Eastern Mojave Desert, Including the Mojave National Preserve", wfo: "VEF" },
ForecastZone::CA525 => crate::ZoneDetails {state: "CA", zone: "525", zone_numeric: 525, name: "Morongo Basin", wfo: "VEF" },
ForecastZone::CA526 => crate::ZoneDetails {state: "CA", zone: "526", zone_numeric: 526, name: "Cadiz Basin", wfo: "VEF" },
ForecastZone::CA527 => crate::ZoneDetails {state: "CA", zone: "527", zone_numeric: 527, name: "San Bernardino County-Upper Colorado River Valley", wfo: "VEF" },
ForecastZone::CA528 => crate::ZoneDetails {state: "CA", zone: "528", zone_numeric: 528, name: "Northern Salinas Valley/Hollister Valley and Carmel Valley", wfo: "MTR" },
ForecastZone::CA529 => crate::ZoneDetails {state: "CA", zone: "529", zone_numeric: 529, name: "Northern Monterey Bay", wfo: "MTR" },
ForecastZone::CA530 => crate::ZoneDetails {state: "CA", zone: "530", zone_numeric: 530, name: "Southern Monterey Bay and Big Sur Coast", wfo: "MTR" },
ForecastZone::CA547 => crate::ZoneDetails {state: "CA", zone: "547", zone_numeric: 547, name: "Los Angeles County San Fernando Valley", wfo: "LOX" },
ForecastZone::CA548 => crate::ZoneDetails {state: "CA", zone: "548", zone_numeric: 548, name: "Los Angeles County San Gabriel Valley", wfo: "LOX" },
ForecastZone::CA549 => crate::ZoneDetails {state: "CA", zone: "549", zone_numeric: 549, name: "San Miguel and Santa Rosa Islands", wfo: "LOX" },
ForecastZone::CA550 => crate::ZoneDetails {state: "CA", zone: "550", zone_numeric: 550, name: "Santa Cruz and Anacapa Islands", wfo: "LOX" },
ForecastZone::CA552 => crate::ZoneDetails {state: "CA", zone: "552", zone_numeric: 552, name: "Orange County Coastal", wfo: "SGX" },
ForecastZone::CA554 => crate::ZoneDetails {state: "CA", zone: "554", zone_numeric: 554, name: "Orange County Inland", wfo: "SGX" },
ForecastZone::CA560 => crate::ZoneDetails {state: "CA", zone: "560", zone_numeric: 560, name: "Joshua Tree NP West", wfo: "PSR" },
ForecastZone::CA561 => crate::ZoneDetails {state: "CA", zone: "561", zone_numeric: 561, name: "Joshua Tree NP East", wfo: "PSR" },
ForecastZone::CA562 => crate::ZoneDetails {state: "CA", zone: "562", zone_numeric: 562, name: "Imperial County Southwest", wfo: "PSR" },
ForecastZone::CA563 => crate::ZoneDetails {state: "CA", zone: "563", zone_numeric: 563, name: "Salton Sea", wfo: "PSR" },
ForecastZone::CA564 => crate::ZoneDetails {state: "CA", zone: "564", zone_numeric: 564, name: "Chuckwalla Mountains", wfo: "PSR" },
ForecastZone::CA565 => crate::ZoneDetails {state: "CA", zone: "565", zone_numeric: 565, name: "Imperial County Southeast", wfo: "PSR" },
ForecastZone::CA566 => crate::ZoneDetails {state: "CA", zone: "566", zone_numeric: 566, name: "Imperial County West", wfo: "PSR" },
ForecastZone::CA567 => crate::ZoneDetails {state: "CA", zone: "567", zone_numeric: 567, name: "Imperial Valley", wfo: "PSR" },
ForecastZone::CA568 => crate::ZoneDetails {state: "CA", zone: "568", zone_numeric: 568, name: "Chiriaco Summit", wfo: "PSR" },
ForecastZone::CA569 => crate::ZoneDetails {state: "CA", zone: "569", zone_numeric: 569, name: "Palo Verde Valley", wfo: "PSR" },
ForecastZone::CA570 => crate::ZoneDetails {state: "CA", zone: "570", zone_numeric: 570, name: "Chuckwalla Valley", wfo: "PSR" },
ForecastZone::CO001 => crate::ZoneDetails {state: "CO", zone: "001", zone_numeric: 001, name: "Lower Yampa River Basin", wfo: "GJT" },
ForecastZone::CO002 => crate::ZoneDetails {state: "CO", zone: "002", zone_numeric: 002, name: "Central Yampa River Basin", wfo: "GJT" },
ForecastZone::CO003 => crate::ZoneDetails {state: "CO", zone: "003", zone_numeric: 003, name: "Roan and Tavaputs Plateaus", wfo: "GJT" },
ForecastZone::CO004 => crate::ZoneDetails {state: "CO", zone: "004", zone_numeric: 004, name: "Elkhead and Park Mountains", wfo: "GJT" },
ForecastZone::CO005 => crate::ZoneDetails {state: "CO", zone: "005", zone_numeric: 005, name: "Upper Yampa River Basin", wfo: "GJT" },
ForecastZone::CO006 => crate::ZoneDetails {state: "CO", zone: "006", zone_numeric: 006, name: "Grand Valley", wfo: "GJT" },
ForecastZone::CO007 => crate::ZoneDetails {state: "CO", zone: "007", zone_numeric: 007, name: "Debeque to Silt Corridor", wfo: "GJT" },
ForecastZone::CO008 => crate::ZoneDetails {state: "CO", zone: "008", zone_numeric: 008, name: "Central Colorado River Basin", wfo: "GJT" },
ForecastZone::CO009 => crate::ZoneDetails {state: "CO", zone: "009", zone_numeric: 009, name: "Grand and Battlement Mesas", wfo: "GJT" },
ForecastZone::CO010 => crate::ZoneDetails {state: "CO", zone: "010", zone_numeric: 010, name: "Gore and Elk Mountains/Central Mountain Valleys", wfo: "GJT" },
ForecastZone::CO011 => crate::ZoneDetails {state: "CO", zone: "011", zone_numeric: 011, name: "Central Gunnison and Uncompahgre River Basin", wfo: "GJT" },
ForecastZone::CO012 => crate::ZoneDetails {state: "CO", zone: "012", zone_numeric: 012, name: "West Elk and Sawatch Mountains", wfo: "GJT" },
ForecastZone::CO013 => crate::ZoneDetails {state: "CO", zone: "013", zone_numeric: 013, name: "Flat Tops", wfo: "GJT" },
ForecastZone::CO014 => crate::ZoneDetails {state: "CO", zone: "014", zone_numeric: 014, name: "Upper Gunnison River Valley", wfo: "GJT" },
ForecastZone::CO017 => crate::ZoneDetails {state: "CO", zone: "017", zone_numeric: 017, name: "Uncompahgre Plateau/Dallas Divide", wfo: "GJT" },
ForecastZone::CO018 => crate::ZoneDetails {state: "CO", zone: "018", zone_numeric: 018, name: "Northwestern San Juan Mountains", wfo: "GJT" },
ForecastZone::CO019 => crate::ZoneDetails {state: "CO", zone: "019", zone_numeric: 019, name: "Southwest San Juan Mountains", wfo: "GJT" },
ForecastZone::CO020 => crate::ZoneDetails {state: "CO", zone: "020", zone_numeric: 020, name: "Paradox Valley/Lower Dolores River", wfo: "GJT" },
ForecastZone::CO021 => crate::ZoneDetails {state: "CO", zone: "021", zone_numeric: 021, name: "Four Corners/Upper Dolores River", wfo: "GJT" },
ForecastZone::CO022 => crate::ZoneDetails {state: "CO", zone: "022", zone_numeric: 022, name: "Animas River Basin", wfo: "GJT" },
ForecastZone::CO023 => crate::ZoneDetails {state: "CO", zone: "023", zone_numeric: 023, name: "San Juan River Basin", wfo: "GJT" },
ForecastZone::CO030 => crate::ZoneDetails {state: "CO", zone: "030", zone_numeric: 030, name: "Jackson County Below 9000 Feet", wfo: "BOU" },
ForecastZone::CO031 => crate::ZoneDetails {state: "CO", zone: "031", zone_numeric: 031, name: "West Jackson and West Grand Counties Above 9000 Feet", wfo: "BOU" },
ForecastZone::CO032 => crate::ZoneDetails {state: "CO", zone: "032", zone_numeric: 032, name: "Grand and Summit Counties Below 9000 Feet", wfo: "BOU" },
ForecastZone::CO033 => crate::ZoneDetails {state: "CO", zone: "033", zone_numeric: 033, name: "South and East Jackson/Larimer/North and Northeast Grand/Northwest Boulder Counties Above 9000 Feet", wfo: "BOU" },
ForecastZone::CO034 => crate::ZoneDetails {state: "CO", zone: "034", zone_numeric: 034, name: "South and Southeast Grand/West Central and Southwest Boulder/Gilpin/Clear Creek/Summit/North and West Park Counties Above 9000 Feet", wfo: "BOU" },
ForecastZone::CO035 => crate::ZoneDetails {state: "CO", zone: "035", zone_numeric: 035, name: "Larimer and Boulder Counties Between 6000 and 9000 Feet", wfo: "BOU" },
ForecastZone::CO036 => crate::ZoneDetails {state: "CO", zone: "036", zone_numeric: 036, name: "Jefferson and West Douglas Counties Above  6000 Feet/Gilpin/Clear Creek/Northeast Park Counties Below 9000 Feet", wfo: "BOU" },
ForecastZone::CO037 => crate::ZoneDetails {state: "CO", zone: "037", zone_numeric: 037, name: "Central and Southeast Park County", wfo: "BOU" },
ForecastZone::CO038 => crate::ZoneDetails {state: "CO", zone: "038", zone_numeric: 038, name: "Larimer County Below 6000 Feet/Northwest Weld County", wfo: "BOU" },
ForecastZone::CO039 => crate::ZoneDetails {state: "CO", zone: "039", zone_numeric: 039, name: "Boulder And Jefferson Counties Below 6000 Feet/West Broomfield County", wfo: "BOU" },
ForecastZone::CO040 => crate::ZoneDetails {state: "CO", zone: "040", zone_numeric: 040, name: "North Douglas County Below 6000 Feet/Denver/West Adams and Arapahoe Counties/East Broomfield County", wfo: "BOU" },
ForecastZone::CO041 => crate::ZoneDetails {state: "CO", zone: "041", zone_numeric: 041, name: "Elbert/Central and East Douglas Counties Above 6000 Feet", wfo: "BOU" },
ForecastZone::CO042 => crate::ZoneDetails {state: "CO", zone: "042", zone_numeric: 042, name: "Northeast Weld County", wfo: "BOU" },
ForecastZone::CO043 => crate::ZoneDetails {state: "CO", zone: "043", zone_numeric: 043, name: "Central and South Weld County", wfo: "BOU" },
ForecastZone::CO044 => crate::ZoneDetails {state: "CO", zone: "044", zone_numeric: 044, name: "Morgan County", wfo: "BOU" },
ForecastZone::CO045 => crate::ZoneDetails {state: "CO", zone: "045", zone_numeric: 045, name: "Central and East Adams and Arapahoe Counties", wfo: "BOU" },
ForecastZone::CO046 => crate::ZoneDetails {state: "CO", zone: "046", zone_numeric: 046, name: "North and Northeast Elbert County Below 6000 Feet/North Lincoln County", wfo: "BOU" },
ForecastZone::CO047 => crate::ZoneDetails {state: "CO", zone: "047", zone_numeric: 047, name: "Southeast Elbert County Below 6000 Feet/South Lincoln County", wfo: "BOU" },
ForecastZone::CO048 => crate::ZoneDetails {state: "CO", zone: "048", zone_numeric: 048, name: "Logan County", wfo: "BOU" },
ForecastZone::CO049 => crate::ZoneDetails {state: "CO", zone: "049", zone_numeric: 049, name: "Washington County", wfo: "BOU" },
ForecastZone::CO050 => crate::ZoneDetails {state: "CO", zone: "050", zone_numeric: 050, name: "Sedgwick County", wfo: "BOU" },
ForecastZone::CO051 => crate::ZoneDetails {state: "CO", zone: "051", zone_numeric: 051, name: "Phillips County", wfo: "BOU" },
ForecastZone::CO058 => crate::ZoneDetails {state: "CO", zone: "058", zone_numeric: 058, name: "Western Mosquito Range/East Lake County Above 11000 Ft", wfo: "PUB" },
ForecastZone::CO059 => crate::ZoneDetails {state: "CO", zone: "059", zone_numeric: 059, name: "Leadville Vicinity/Lake County Below 11000 Ft", wfo: "PUB" },
ForecastZone::CO060 => crate::ZoneDetails {state: "CO", zone: "060", zone_numeric: 060, name: "Eastern Sawatch Mountains above 11000 Ft", wfo: "PUB" },
ForecastZone::CO061 => crate::ZoneDetails {state: "CO", zone: "061", zone_numeric: 061, name: "Western Chaffee County Between 9000 and 11000 Ft", wfo: "PUB" },
ForecastZone::CO062 => crate::ZoneDetails {state: "CO", zone: "062", zone_numeric: 062, name: "Central Chaffee County Below 9000 Ft", wfo: "PUB" },
ForecastZone::CO063 => crate::ZoneDetails {state: "CO", zone: "063", zone_numeric: 063, name: "Western Mosquito Range/East Chaffee County above 9000Ft", wfo: "PUB" },
ForecastZone::CO064 => crate::ZoneDetails {state: "CO", zone: "064", zone_numeric: 064, name: "Saguache County West of Continental Divide Below 10000 Ft", wfo: "PUB" },
ForecastZone::CO065 => crate::ZoneDetails {state: "CO", zone: "065", zone_numeric: 065, name: "Saguache County East of Continental Divide below 10000 Ft", wfo: "PUB" },
ForecastZone::CO066 => crate::ZoneDetails {state: "CO", zone: "066", zone_numeric: 066, name: "La Garita Mountains Above 10000 Ft", wfo: "PUB" },
ForecastZone::CO067 => crate::ZoneDetails {state: "CO", zone: "067", zone_numeric: 067, name: "Upper Rio Grande Valley/Eastern San Juan Mountains Below 10000 Ft", wfo: "PUB" },
ForecastZone::CO068 => crate::ZoneDetails {state: "CO", zone: "068", zone_numeric: 068, name: "Eastern San Juan Mountains Above 10000 Ft", wfo: "PUB" },
ForecastZone::CO069 => crate::ZoneDetails {state: "CO", zone: "069", zone_numeric: 069, name: "Del Norte Vicinity/Northern San Luis Valley Below 8500 Ft", wfo: "PUB" },
ForecastZone::CO070 => crate::ZoneDetails {state: "CO", zone: "070", zone_numeric: 070, name: "Alamosa  Vicinity/Central San Luis Valley Below 8500 Ft", wfo: "PUB" },
ForecastZone::CO071 => crate::ZoneDetails {state: "CO", zone: "071", zone_numeric: 071, name: "Southern San Luis Valley", wfo: "PUB" },
ForecastZone::CO072 => crate::ZoneDetails {state: "CO", zone: "072", zone_numeric: 072, name: "Northern Sangre de Cristo Mountains Between 8500 And 11000 Ft", wfo: "PUB" },
ForecastZone::CO073 => crate::ZoneDetails {state: "CO", zone: "073", zone_numeric: 073, name: "Northern Sangre de Cristo Mountains above 11000 Ft", wfo: "PUB" },
ForecastZone::CO074 => crate::ZoneDetails {state: "CO", zone: "074", zone_numeric: 074, name: "Southern Sangre De Cristo Mountains Between 7500 and 11000 Ft", wfo: "PUB" },
ForecastZone::CO075 => crate::ZoneDetails {state: "CO", zone: "075", zone_numeric: 075, name: "Southern Sangre De Cristo Mountains Above 11000 Ft", wfo: "PUB" },
ForecastZone::CO076 => crate::ZoneDetails {state: "CO", zone: "076", zone_numeric: 076, name: "Northwestern Fremont County  Above 8500Ft", wfo: "PUB" },
ForecastZone::CO077 => crate::ZoneDetails {state: "CO", zone: "077", zone_numeric: 077, name: "Western/Central Fremont County Below 8500 Ft", wfo: "PUB" },
ForecastZone::CO078 => crate::ZoneDetails {state: "CO", zone: "078", zone_numeric: 078, name: "Wet Mountain Valley Below 8500 Ft", wfo: "PUB" },
ForecastZone::CO079 => crate::ZoneDetails {state: "CO", zone: "079", zone_numeric: 079, name: "Wet Mountains between 6300 and 10000Ft", wfo: "PUB" },
ForecastZone::CO080 => crate::ZoneDetails {state: "CO", zone: "080", zone_numeric: 080, name: "Wet Mountains above 10000 Ft", wfo: "PUB" },
ForecastZone::CO081 => crate::ZoneDetails {state: "CO", zone: "081", zone_numeric: 081, name: "Teller County/Rampart Range above 7500fT/Pike's Peak Between 7500 And 11000 Ft", wfo: "PUB" },
ForecastZone::CO082 => crate::ZoneDetails {state: "CO", zone: "082", zone_numeric: 082, name: "Pikes Peak above 11000 Ft", wfo: "PUB" },
ForecastZone::CO083 => crate::ZoneDetails {state: "CO", zone: "083", zone_numeric: 083, name: "Canon City Vicinity/Eastern Fremont County", wfo: "PUB" },
ForecastZone::CO084 => crate::ZoneDetails {state: "CO", zone: "084", zone_numeric: 084, name: "Northern El Paso County/Monument Ridge/Rampart Range Below 7500 Ft", wfo: "PUB" },
ForecastZone::CO085 => crate::ZoneDetails {state: "CO", zone: "085", zone_numeric: 085, name: "Colorado Springs Vicinity/Southern El Paso County/Rampart Range Below 7400 Ft", wfo: "PUB" },
ForecastZone::CO086 => crate::ZoneDetails {state: "CO", zone: "086", zone_numeric: 086, name: "Pueblo Vicinity/Pueblo County Below 6300 Feet", wfo: "PUB" },
ForecastZone::CO087 => crate::ZoneDetails {state: "CO", zone: "087", zone_numeric: 087, name: "Walsenburg Vicinity/Upper Huerfano River Basin Below 7500 Ft", wfo: "PUB" },
ForecastZone::CO088 => crate::ZoneDetails {state: "CO", zone: "088", zone_numeric: 088, name: "Trinidad Vicinity/Western Las Animas County Below 7500 Ft", wfo: "PUB" },
ForecastZone::CO089 => crate::ZoneDetails {state: "CO", zone: "089", zone_numeric: 089, name: "Crowley County", wfo: "PUB" },
ForecastZone::CO090 => crate::ZoneDetails {state: "CO", zone: "090", zone_numeric: 090, name: "Yuma County", wfo: "GLD" },
ForecastZone::CO091 => crate::ZoneDetails {state: "CO", zone: "091", zone_numeric: 091, name: "Kit Carson County", wfo: "GLD" },
ForecastZone::CO092 => crate::ZoneDetails {state: "CO", zone: "092", zone_numeric: 092, name: "Cheyenne County", wfo: "GLD" },
ForecastZone::CO093 => crate::ZoneDetails {state: "CO", zone: "093", zone_numeric: 093, name: "La Junta Vicinity/Otero County", wfo: "PUB" },
ForecastZone::CO094 => crate::ZoneDetails {state: "CO", zone: "094", zone_numeric: 094, name: "Eastern Las Animas County", wfo: "PUB" },
ForecastZone::CO095 => crate::ZoneDetails {state: "CO", zone: "095", zone_numeric: 095, name: "Western Kiowa County", wfo: "PUB" },
ForecastZone::CO096 => crate::ZoneDetails {state: "CO", zone: "096", zone_numeric: 096, name: "Eastern Kiowa County", wfo: "PUB" },
ForecastZone::CO097 => crate::ZoneDetails {state: "CO", zone: "097", zone_numeric: 097, name: "Las Animas Vicinity/Bent County", wfo: "PUB" },
ForecastZone::CO098 => crate::ZoneDetails {state: "CO", zone: "098", zone_numeric: 098, name: "Lamar Vicinity/Prowers County", wfo: "PUB" },
ForecastZone::CO099 => crate::ZoneDetails {state: "CO", zone: "099", zone_numeric: 099, name: "Springfield Vicinity/Baca County", wfo: "PUB" },
ForecastZone::CT001 => crate::ZoneDetails {state: "CT", zone: "001", zone_numeric: 001, name: "Northern Litchfield", wfo: "ALY" },
ForecastZone::CT002 => crate::ZoneDetails {state: "CT", zone: "002", zone_numeric: 002, name: "Hartford", wfo: "BOX" },
ForecastZone::CT003 => crate::ZoneDetails {state: "CT", zone: "003", zone_numeric: 003, name: "Tolland", wfo: "BOX" },
ForecastZone::CT004 => crate::ZoneDetails {state: "CT", zone: "004", zone_numeric: 004, name: "Windham", wfo: "BOX" },
ForecastZone::CT005 => crate::ZoneDetails {state: "CT", zone: "005", zone_numeric: 005, name: "Northern Fairfield", wfo: "OKX" },
ForecastZone::CT006 => crate::ZoneDetails {state: "CT", zone: "006", zone_numeric: 006, name: "Northern New Haven", wfo: "OKX" },
ForecastZone::CT007 => crate::ZoneDetails {state: "CT", zone: "007", zone_numeric: 007, name: "Northern Middlesex", wfo: "OKX" },
ForecastZone::CT008 => crate::ZoneDetails {state: "CT", zone: "008", zone_numeric: 008, name: "Northern New London", wfo: "OKX" },
ForecastZone::CT009 => crate::ZoneDetails {state: "CT", zone: "009", zone_numeric: 009, name: "Southern Fairfield", wfo: "OKX" },
ForecastZone::CT010 => crate::ZoneDetails {state: "CT", zone: "010", zone_numeric: 010, name: "Southern New Haven", wfo: "OKX" },
ForecastZone::CT011 => crate::ZoneDetails {state: "CT", zone: "011", zone_numeric: 011, name: "Southern Middlesex", wfo: "OKX" },
ForecastZone::CT012 => crate::ZoneDetails {state: "CT", zone: "012", zone_numeric: 012, name: "Southern New London", wfo: "OKX" },
ForecastZone::CT013 => crate::ZoneDetails {state: "CT", zone: "013", zone_numeric: 013, name: "Southern Litchfield", wfo: "ALY" },
ForecastZone::DC001 => crate::ZoneDetails {state: "DC", zone: "001", zone_numeric: 001, name: "District of Columbia", wfo: "LWX" },
ForecastZone::DE001 => crate::ZoneDetails {state: "DE", zone: "001", zone_numeric: 001, name: "New Castle", wfo: "PHI" },
ForecastZone::DE002 => crate::ZoneDetails {state: "DE", zone: "002", zone_numeric: 002, name: "Kent", wfo: "PHI" },
ForecastZone::DE003 => crate::ZoneDetails {state: "DE", zone: "003", zone_numeric: 003, name: "Inland Sussex", wfo: "PHI" },
ForecastZone::DE004 => crate::ZoneDetails {state: "DE", zone: "004", zone_numeric: 004, name: "Delaware Beaches", wfo: "PHI" },
ForecastZone::FL007 => crate::ZoneDetails {state: "FL", zone: "007", zone_numeric: 007, name: "North Walton", wfo: "TAE" },
ForecastZone::FL008 => crate::ZoneDetails {state: "FL", zone: "008", zone_numeric: 008, name: "Central Walton", wfo: "TAE" },
ForecastZone::FL009 => crate::ZoneDetails {state: "FL", zone: "009", zone_numeric: 009, name: "Holmes", wfo: "TAE" },
ForecastZone::FL010 => crate::ZoneDetails {state: "FL", zone: "010", zone_numeric: 010, name: "Washington", wfo: "TAE" },
ForecastZone::FL011 => crate::ZoneDetails {state: "FL", zone: "011", zone_numeric: 011, name: "Jackson", wfo: "TAE" },
ForecastZone::FL012 => crate::ZoneDetails {state: "FL", zone: "012", zone_numeric: 012, name: "Inland Bay", wfo: "TAE" },
ForecastZone::FL013 => crate::ZoneDetails {state: "FL", zone: "013", zone_numeric: 013, name: "Calhoun", wfo: "TAE" },
ForecastZone::FL014 => crate::ZoneDetails {state: "FL", zone: "014", zone_numeric: 014, name: "Inland Gulf", wfo: "TAE" },
ForecastZone::FL015 => crate::ZoneDetails {state: "FL", zone: "015", zone_numeric: 015, name: "Inland Franklin", wfo: "TAE" },
ForecastZone::FL016 => crate::ZoneDetails {state: "FL", zone: "016", zone_numeric: 016, name: "Gadsden", wfo: "TAE" },
ForecastZone::FL017 => crate::ZoneDetails {state: "FL", zone: "017", zone_numeric: 017, name: "Leon", wfo: "TAE" },
ForecastZone::FL018 => crate::ZoneDetails {state: "FL", zone: "018", zone_numeric: 018, name: "Inland Jefferson", wfo: "TAE" },
ForecastZone::FL019 => crate::ZoneDetails {state: "FL", zone: "019", zone_numeric: 019, name: "Madison", wfo: "TAE" },
ForecastZone::FL020 => crate::ZoneDetails {state: "FL", zone: "020", zone_numeric: 020, name: "Hamilton", wfo: "JAX" },
ForecastZone::FL021 => crate::ZoneDetails {state: "FL", zone: "021", zone_numeric: 021, name: "Suwannee", wfo: "JAX" },
ForecastZone::FL023 => crate::ZoneDetails {state: "FL", zone: "023", zone_numeric: 023, name: "Baker", wfo: "JAX" },
ForecastZone::FL024 => crate::ZoneDetails {state: "FL", zone: "024", zone_numeric: 024, name: "Inland Nassau", wfo: "JAX" },
ForecastZone::FL026 => crate::ZoneDetails {state: "FL", zone: "026", zone_numeric: 026, name: "Liberty", wfo: "TAE" },
ForecastZone::FL027 => crate::ZoneDetails {state: "FL", zone: "027", zone_numeric: 027, name: "Inland Wakulla", wfo: "TAE" },
ForecastZone::FL028 => crate::ZoneDetails {state: "FL", zone: "028", zone_numeric: 028, name: "Inland Taylor", wfo: "TAE" },
ForecastZone::FL029 => crate::ZoneDetails {state: "FL", zone: "029", zone_numeric: 029, name: "Lafayette", wfo: "TAE" },
ForecastZone::FL030 => crate::ZoneDetails {state: "FL", zone: "030", zone_numeric: 030, name: "Union", wfo: "JAX" },
ForecastZone::FL031 => crate::ZoneDetails {state: "FL", zone: "031", zone_numeric: 031, name: "Bradford", wfo: "JAX" },
ForecastZone::FL033 => crate::ZoneDetails {state: "FL", zone: "033", zone_numeric: 033, name: "Inland St. Johns", wfo: "JAX" },
ForecastZone::FL034 => crate::ZoneDetails {state: "FL", zone: "034", zone_numeric: 034, name: "Inland Dixie", wfo: "TAE" },
ForecastZone::FL035 => crate::ZoneDetails {state: "FL", zone: "035", zone_numeric: 035, name: "Gilchrist", wfo: "JAX" },
ForecastZone::FL038 => crate::ZoneDetails {state: "FL", zone: "038", zone_numeric: 038, name: "Inland Flagler", wfo: "JAX" },
ForecastZone::FL041 => crate::ZoneDetails {state: "FL", zone: "041", zone_numeric: 041, name: "Inland Volusia", wfo: "MLB" },
ForecastZone::FL043 => crate::ZoneDetails {state: "FL", zone: "043", zone_numeric: 043, name: "Sumter", wfo: "TBW" },
ForecastZone::FL044 => crate::ZoneDetails {state: "FL", zone: "044", zone_numeric: 044, name: "Northern Lake County", wfo: "MLB" },
ForecastZone::FL045 => crate::ZoneDetails {state: "FL", zone: "045", zone_numeric: 045, name: "Orange", wfo: "MLB" },
ForecastZone::FL046 => crate::ZoneDetails {state: "FL", zone: "046", zone_numeric: 046, name: "Seminole", wfo: "MLB" },
ForecastZone::FL050 => crate::ZoneDetails {state: "FL", zone: "050", zone_numeric: 050, name: "Pinellas", wfo: "TBW" },
ForecastZone::FL052 => crate::ZoneDetails {state: "FL", zone: "052", zone_numeric: 052, name: "Polk", wfo: "TBW" },
ForecastZone::FL053 => crate::ZoneDetails {state: "FL", zone: "053", zone_numeric: 053, name: "Osceola", wfo: "MLB" },
ForecastZone::FL056 => crate::ZoneDetails {state: "FL", zone: "056", zone_numeric: 056, name: "Hardee", wfo: "TBW" },
ForecastZone::FL057 => crate::ZoneDetails {state: "FL", zone: "057", zone_numeric: 057, name: "Highlands", wfo: "TBW" },
ForecastZone::FL058 => crate::ZoneDetails {state: "FL", zone: "058", zone_numeric: 058, name: "Okeechobee", wfo: "MLB" },
ForecastZone::FL061 => crate::ZoneDetails {state: "FL", zone: "061", zone_numeric: 061, name: "DeSoto", wfo: "TBW" },
ForecastZone::FL063 => crate::ZoneDetails {state: "FL", zone: "063", zone_numeric: 063, name: "Glades", wfo: "MFL" },
ForecastZone::FL066 => crate::ZoneDetails {state: "FL", zone: "066", zone_numeric: 066, name: "Hendry", wfo: "MFL" },
ForecastZone::FL067 => crate::ZoneDetails {state: "FL", zone: "067", zone_numeric: 067, name: "Inland Palm Beach County", wfo: "MFL" },
ForecastZone::FL068 => crate::ZoneDetails {state: "FL", zone: "068", zone_numeric: 068, name: "Metro Palm Beach County", wfo: "MFL" },
ForecastZone::FL069 => crate::ZoneDetails {state: "FL", zone: "069", zone_numeric: 069, name: "Coastal Collier County", wfo: "MFL" },
ForecastZone::FL070 => crate::ZoneDetails {state: "FL", zone: "070", zone_numeric: 070, name: "Inland Collier County", wfo: "MFL" },
ForecastZone::FL071 => crate::ZoneDetails {state: "FL", zone: "071", zone_numeric: 071, name: "Inland Broward County", wfo: "MFL" },
ForecastZone::FL072 => crate::ZoneDetails {state: "FL", zone: "072", zone_numeric: 072, name: "Metro Broward County", wfo: "MFL" },
ForecastZone::FL073 => crate::ZoneDetails {state: "FL", zone: "073", zone_numeric: 073, name: "Inland Miami-Dade County", wfo: "MFL" },
ForecastZone::FL074 => crate::ZoneDetails {state: "FL", zone: "074", zone_numeric: 074, name: "Metropolitan Miami Dade", wfo: "MFL" },
ForecastZone::FL075 => crate::ZoneDetails {state: "FL", zone: "075", zone_numeric: 075, name: "Mainland Monroe", wfo: "MFL" },
ForecastZone::FL076 => crate::ZoneDetails {state: "FL", zone: "076", zone_numeric: 076, name: "Monroe Upper Keys", wfo: "KEY" },
ForecastZone::FL077 => crate::ZoneDetails {state: "FL", zone: "077", zone_numeric: 077, name: "Monroe Middle Keys", wfo: "KEY" },
ForecastZone::FL078 => crate::ZoneDetails {state: "FL", zone: "078", zone_numeric: 078, name: "Monroe Lower Keys", wfo: "KEY" },
ForecastZone::FL108 => crate::ZoneDetails {state: "FL", zone: "108", zone_numeric: 108, name: "South Walton", wfo: "TAE" },
ForecastZone::FL112 => crate::ZoneDetails {state: "FL", zone: "112", zone_numeric: 112, name: "Coastal Bay", wfo: "TAE" },
ForecastZone::FL114 => crate::ZoneDetails {state: "FL", zone: "114", zone_numeric: 114, name: "Coastal Gulf", wfo: "TAE" },
ForecastZone::FL115 => crate::ZoneDetails {state: "FL", zone: "115", zone_numeric: 115, name: "Coastal Franklin", wfo: "TAE" },
ForecastZone::FL118 => crate::ZoneDetails {state: "FL", zone: "118", zone_numeric: 118, name: "Coastal Jefferson", wfo: "TAE" },
ForecastZone::FL122 => crate::ZoneDetails {state: "FL", zone: "122", zone_numeric: 122, name: "Northern Columbia", wfo: "JAX" },
ForecastZone::FL124 => crate::ZoneDetails {state: "FL", zone: "124", zone_numeric: 124, name: "Coastal Nassau", wfo: "JAX" },
ForecastZone::FL125 => crate::ZoneDetails {state: "FL", zone: "125", zone_numeric: 125, name: "Coastal Duval", wfo: "JAX" },
ForecastZone::FL127 => crate::ZoneDetails {state: "FL", zone: "127", zone_numeric: 127, name: "Coastal Wakulla", wfo: "TAE" },
ForecastZone::FL128 => crate::ZoneDetails {state: "FL", zone: "128", zone_numeric: 128, name: "Coastal Taylor", wfo: "TAE" },
ForecastZone::FL132 => crate::ZoneDetails {state: "FL", zone: "132", zone_numeric: 132, name: "Eastern Clay", wfo: "JAX" },
ForecastZone::FL133 => crate::ZoneDetails {state: "FL", zone: "133", zone_numeric: 133, name: "Coastal St. Johns", wfo: "JAX" },
ForecastZone::FL134 => crate::ZoneDetails {state: "FL", zone: "134", zone_numeric: 134, name: "Coastal Dixie", wfo: "TAE" },
ForecastZone::FL136 => crate::ZoneDetails {state: "FL", zone: "136", zone_numeric: 136, name: "Eastern Alachua", wfo: "JAX" },
ForecastZone::FL137 => crate::ZoneDetails {state: "FL", zone: "137", zone_numeric: 137, name: "Eastern Putnam", wfo: "JAX" },
ForecastZone::FL138 => crate::ZoneDetails {state: "FL", zone: "138", zone_numeric: 138, name: "Coastal Flagler", wfo: "JAX" },
ForecastZone::FL139 => crate::ZoneDetails {state: "FL", zone: "139", zone_numeric: 139, name: "Coastal Levy", wfo: "TBW" },
ForecastZone::FL140 => crate::ZoneDetails {state: "FL", zone: "140", zone_numeric: 140, name: "Eastern Marion", wfo: "JAX" },
ForecastZone::FL141 => crate::ZoneDetails {state: "FL", zone: "141", zone_numeric: 141, name: "Coastal Volusia", wfo: "MLB" },
ForecastZone::FL142 => crate::ZoneDetails {state: "FL", zone: "142", zone_numeric: 142, name: "Coastal Citrus", wfo: "TBW" },
ForecastZone::FL144 => crate::ZoneDetails {state: "FL", zone: "144", zone_numeric: 144, name: "Southern Lake County", wfo: "MLB" },
ForecastZone::FL148 => crate::ZoneDetails {state: "FL", zone: "148", zone_numeric: 148, name: "Coastal Hernando", wfo: "TBW" },
ForecastZone::FL149 => crate::ZoneDetails {state: "FL", zone: "149", zone_numeric: 149, name: "Coastal Pasco", wfo: "TBW" },
ForecastZone::FL151 => crate::ZoneDetails {state: "FL", zone: "151", zone_numeric: 151, name: "Coastal Hillsborough", wfo: "TBW" },
ForecastZone::FL154 => crate::ZoneDetails {state: "FL", zone: "154", zone_numeric: 154, name: "Coastal Indian River", wfo: "MLB" },
ForecastZone::FL155 => crate::ZoneDetails {state: "FL", zone: "155", zone_numeric: 155, name: "Coastal Manatee", wfo: "TBW" },
ForecastZone::FL159 => crate::ZoneDetails {state: "FL", zone: "159", zone_numeric: 159, name: "Coastal St. Lucie", wfo: "MLB" },
ForecastZone::FL160 => crate::ZoneDetails {state: "FL", zone: "160", zone_numeric: 160, name: "Coastal Sarasota", wfo: "TBW" },
ForecastZone::FL162 => crate::ZoneDetails {state: "FL", zone: "162", zone_numeric: 162, name: "Coastal Charlotte", wfo: "TBW" },
ForecastZone::FL164 => crate::ZoneDetails {state: "FL", zone: "164", zone_numeric: 164, name: "Coastal Martin", wfo: "MLB" },
ForecastZone::FL165 => crate::ZoneDetails {state: "FL", zone: "165", zone_numeric: 165, name: "Coastal Lee", wfo: "TBW" },
ForecastZone::FL168 => crate::ZoneDetails {state: "FL", zone: "168", zone_numeric: 168, name: "Coastal Palm Beach County", wfo: "MFL" },
ForecastZone::FL172 => crate::ZoneDetails {state: "FL", zone: "172", zone_numeric: 172, name: "Coastal Broward County", wfo: "MFL" },
ForecastZone::FL173 => crate::ZoneDetails {state: "FL", zone: "173", zone_numeric: 173, name: "Coastal Miami Dade County", wfo: "MFL" },
ForecastZone::FL174 => crate::ZoneDetails {state: "FL", zone: "174", zone_numeric: 174, name: "Far South Miami-Dade County", wfo: "MFL" },
ForecastZone::FL201 => crate::ZoneDetails {state: "FL", zone: "201", zone_numeric: 201, name: "Escambia Inland", wfo: "MOB" },
ForecastZone::FL202 => crate::ZoneDetails {state: "FL", zone: "202", zone_numeric: 202, name: "Escambia Coastal", wfo: "MOB" },
ForecastZone::FL203 => crate::ZoneDetails {state: "FL", zone: "203", zone_numeric: 203, name: "Santa Rosa Inland", wfo: "MOB" },
ForecastZone::FL204 => crate::ZoneDetails {state: "FL", zone: "204", zone_numeric: 204, name: "Santa Rosa Coastal", wfo: "MOB" },
ForecastZone::FL205 => crate::ZoneDetails {state: "FL", zone: "205", zone_numeric: 205, name: "Okaloosa Inland", wfo: "MOB" },
ForecastZone::FL206 => crate::ZoneDetails {state: "FL", zone: "206", zone_numeric: 206, name: "Okaloosa Coastal", wfo: "MOB" },
ForecastZone::FL222 => crate::ZoneDetails {state: "FL", zone: "222", zone_numeric: 222, name: "Southern Columbia", wfo: "JAX" },
ForecastZone::FL225 => crate::ZoneDetails {state: "FL", zone: "225", zone_numeric: 225, name: "Trout River", wfo: "JAX" },
ForecastZone::FL232 => crate::ZoneDetails {state: "FL", zone: "232", zone_numeric: 232, name: "Western Clay", wfo: "JAX" },
ForecastZone::FL236 => crate::ZoneDetails {state: "FL", zone: "236", zone_numeric: 236, name: "Western Alachua", wfo: "JAX" },
ForecastZone::FL237 => crate::ZoneDetails {state: "FL", zone: "237", zone_numeric: 237, name: "Western Putnam", wfo: "JAX" },
ForecastZone::FL239 => crate::ZoneDetails {state: "FL", zone: "239", zone_numeric: 239, name: "Inland Levy", wfo: "TBW" },
ForecastZone::FL240 => crate::ZoneDetails {state: "FL", zone: "240", zone_numeric: 240, name: "Central Marion", wfo: "JAX" },
ForecastZone::FL242 => crate::ZoneDetails {state: "FL", zone: "242", zone_numeric: 242, name: "Inland Citrus", wfo: "TBW" },
ForecastZone::FL247 => crate::ZoneDetails {state: "FL", zone: "247", zone_numeric: 247, name: "Inland Northern Brevard", wfo: "MLB" },
ForecastZone::FL248 => crate::ZoneDetails {state: "FL", zone: "248", zone_numeric: 248, name: "Inland Hernando", wfo: "TBW" },
ForecastZone::FL249 => crate::ZoneDetails {state: "FL", zone: "249", zone_numeric: 249, name: "Inland Pasco", wfo: "TBW" },
ForecastZone::FL251 => crate::ZoneDetails {state: "FL", zone: "251", zone_numeric: 251, name: "Inland Hillsborough", wfo: "TBW" },
ForecastZone::FL254 => crate::ZoneDetails {state: "FL", zone: "254", zone_numeric: 254, name: "Inland Indian River", wfo: "MLB" },
ForecastZone::FL255 => crate::ZoneDetails {state: "FL", zone: "255", zone_numeric: 255, name: "Inland Manatee", wfo: "TBW" },
ForecastZone::FL259 => crate::ZoneDetails {state: "FL", zone: "259", zone_numeric: 259, name: "Inland St. Lucie", wfo: "MLB" },
ForecastZone::FL260 => crate::ZoneDetails {state: "FL", zone: "260", zone_numeric: 260, name: "Inland Sarasota", wfo: "TBW" },
ForecastZone::FL262 => crate::ZoneDetails {state: "FL", zone: "262", zone_numeric: 262, name: "Inland Charlotte", wfo: "TBW" },
ForecastZone::FL264 => crate::ZoneDetails {state: "FL", zone: "264", zone_numeric: 264, name: "Inland Martin", wfo: "MLB" },
ForecastZone::FL265 => crate::ZoneDetails {state: "FL", zone: "265", zone_numeric: 265, name: "Inland Lee", wfo: "TBW" },
ForecastZone::FL325 => crate::ZoneDetails {state: "FL", zone: "325", zone_numeric: 325, name: "South Central Duval", wfo: "JAX" },
ForecastZone::FL340 => crate::ZoneDetails {state: "FL", zone: "340", zone_numeric: 340, name: "Western Marion", wfo: "JAX" },
ForecastZone::FL347 => crate::ZoneDetails {state: "FL", zone: "347", zone_numeric: 347, name: "Mainland Northern Brevard", wfo: "MLB" },
ForecastZone::FL425 => crate::ZoneDetails {state: "FL", zone: "425", zone_numeric: 425, name: "Western Duval", wfo: "JAX" },
ForecastZone::FL447 => crate::ZoneDetails {state: "FL", zone: "447", zone_numeric: 447, name: "Northern Brevard Barrier Islands", wfo: "MLB" },
ForecastZone::FL547 => crate::ZoneDetails {state: "FL", zone: "547", zone_numeric: 547, name: "Inland Southern Brevard", wfo: "MLB" },
ForecastZone::FL647 => crate::ZoneDetails {state: "FL", zone: "647", zone_numeric: 647, name: "Mainland Southern Brevard", wfo: "MLB" },
ForecastZone::FL747 => crate::ZoneDetails {state: "FL", zone: "747", zone_numeric: 747, name: "Southern Brevard Barrier Islands", wfo: "MLB" },
ForecastZone::FM001 => crate::ZoneDetails {state: "FM", zone: "001", zone_numeric: 001, name: "Kosrae", wfo: "PQE" },
ForecastZone::FM011 => crate::ZoneDetails {state: "FM", zone: "011", zone_numeric: 011, name: "Pingelap", wfo: "PQE" },
ForecastZone::FM012 => crate::ZoneDetails {state: "FM", zone: "012", zone_numeric: 012, name: "Mwoakilloa", wfo: "PQE" },
ForecastZone::FM013 => crate::ZoneDetails {state: "FM", zone: "013", zone_numeric: 013, name: "Pohnpei", wfo: "PQE" },
ForecastZone::FM014 => crate::ZoneDetails {state: "FM", zone: "014", zone_numeric: 014, name: "Pakin", wfo: "PQE" },
ForecastZone::FM015 => crate::ZoneDetails {state: "FM", zone: "015", zone_numeric: 015, name: "Sapwuahfik", wfo: "PQE" },
ForecastZone::FM016 => crate::ZoneDetails {state: "FM", zone: "016", zone_numeric: 016, name: "Oroluk", wfo: "PQE" },
ForecastZone::FM017 => crate::ZoneDetails {state: "FM", zone: "017", zone_numeric: 017, name: "Nukuoro", wfo: "PQE" },
ForecastZone::FM018 => crate::ZoneDetails {state: "FM", zone: "018", zone_numeric: 018, name: "Kapingamarangi", wfo: "PQE" },
ForecastZone::FM021 => crate::ZoneDetails {state: "FM", zone: "021", zone_numeric: 021, name: "Lukunoch", wfo: "PQW" },
ForecastZone::FM022 => crate::ZoneDetails {state: "FM", zone: "022", zone_numeric: 022, name: "Losap", wfo: "PQW" },
ForecastZone::FM023 => crate::ZoneDetails {state: "FM", zone: "023", zone_numeric: 023, name: "Chuuk Lagoon", wfo: "PQW" },
ForecastZone::FM024 => crate::ZoneDetails {state: "FM", zone: "024", zone_numeric: 024, name: "Fananu", wfo: "PQW" },
ForecastZone::FM025 => crate::ZoneDetails {state: "FM", zone: "025", zone_numeric: 025, name: "Onoun", wfo: "PQW" },
ForecastZone::FM026 => crate::ZoneDetails {state: "FM", zone: "026", zone_numeric: 026, name: "Polowat", wfo: "PQW" },
ForecastZone::FM031 => crate::ZoneDetails {state: "FM", zone: "031", zone_numeric: 031, name: "Satawal", wfo: "PQW" },
ForecastZone::FM032 => crate::ZoneDetails {state: "FM", zone: "032", zone_numeric: 032, name: "Woleai", wfo: "PQW" },
ForecastZone::FM033 => crate::ZoneDetails {state: "FM", zone: "033", zone_numeric: 033, name: "Faraulep", wfo: "PQW" },
ForecastZone::FM034 => crate::ZoneDetails {state: "FM", zone: "034", zone_numeric: 034, name: "Eauripik", wfo: "PQW" },
ForecastZone::FM035 => crate::ZoneDetails {state: "FM", zone: "035", zone_numeric: 035, name: "Fais", wfo: "PQW" },
ForecastZone::FM036 => crate::ZoneDetails {state: "FM", zone: "036", zone_numeric: 036, name: "Ulithi", wfo: "PQW" },
ForecastZone::FM037 => crate::ZoneDetails {state: "FM", zone: "037", zone_numeric: 037, name: "Yap", wfo: "PQW" },
ForecastZone::FM038 => crate::ZoneDetails {state: "FM", zone: "038", zone_numeric: 038, name: "Ngulu", wfo: "PQW" },
ForecastZone::GA001 => crate::ZoneDetails {state: "GA", zone: "001", zone_numeric: 001, name: "Dade", wfo: "FFC" },
ForecastZone::GA002 => crate::ZoneDetails {state: "GA", zone: "002", zone_numeric: 002, name: "Walker", wfo: "FFC" },
ForecastZone::GA003 => crate::ZoneDetails {state: "GA", zone: "003", zone_numeric: 003, name: "Catoosa", wfo: "FFC" },
ForecastZone::GA004 => crate::ZoneDetails {state: "GA", zone: "004", zone_numeric: 004, name: "Whitfield", wfo: "FFC" },
ForecastZone::GA005 => crate::ZoneDetails {state: "GA", zone: "005", zone_numeric: 005, name: "Murray", wfo: "FFC" },
ForecastZone::GA006 => crate::ZoneDetails {state: "GA", zone: "006", zone_numeric: 006, name: "Fannin", wfo: "FFC" },
ForecastZone::GA007 => crate::ZoneDetails {state: "GA", zone: "007", zone_numeric: 007, name: "Gilmer", wfo: "FFC" },
ForecastZone::GA008 => crate::ZoneDetails {state: "GA", zone: "008", zone_numeric: 008, name: "Union", wfo: "FFC" },
ForecastZone::GA009 => crate::ZoneDetails {state: "GA", zone: "009", zone_numeric: 009, name: "Towns", wfo: "FFC" },
ForecastZone::GA010 => crate::ZoneDetails {state: "GA", zone: "010", zone_numeric: 010, name: "Rabun", wfo: "GSP" },
ForecastZone::GA011 => crate::ZoneDetails {state: "GA", zone: "011", zone_numeric: 011, name: "Chattooga", wfo: "FFC" },
ForecastZone::GA012 => crate::ZoneDetails {state: "GA", zone: "012", zone_numeric: 012, name: "Gordon", wfo: "FFC" },
ForecastZone::GA013 => crate::ZoneDetails {state: "GA", zone: "013", zone_numeric: 013, name: "Pickens", wfo: "FFC" },
ForecastZone::GA014 => crate::ZoneDetails {state: "GA", zone: "014", zone_numeric: 014, name: "Dawson", wfo: "FFC" },
ForecastZone::GA015 => crate::ZoneDetails {state: "GA", zone: "015", zone_numeric: 015, name: "Lumpkin", wfo: "FFC" },
ForecastZone::GA016 => crate::ZoneDetails {state: "GA", zone: "016", zone_numeric: 016, name: "White", wfo: "FFC" },
ForecastZone::GA017 => crate::ZoneDetails {state: "GA", zone: "017", zone_numeric: 017, name: "Habersham", wfo: "GSP" },
ForecastZone::GA018 => crate::ZoneDetails {state: "GA", zone: "018", zone_numeric: 018, name: "Stephens", wfo: "GSP" },
ForecastZone::GA019 => crate::ZoneDetails {state: "GA", zone: "019", zone_numeric: 019, name: "Floyd", wfo: "FFC" },
ForecastZone::GA020 => crate::ZoneDetails {state: "GA", zone: "020", zone_numeric: 020, name: "Bartow", wfo: "FFC" },
ForecastZone::GA021 => crate::ZoneDetails {state: "GA", zone: "021", zone_numeric: 021, name: "Cherokee", wfo: "FFC" },
ForecastZone::GA022 => crate::ZoneDetails {state: "GA", zone: "022", zone_numeric: 022, name: "Forsyth", wfo: "FFC" },
ForecastZone::GA023 => crate::ZoneDetails {state: "GA", zone: "023", zone_numeric: 023, name: "Hall", wfo: "FFC" },
ForecastZone::GA024 => crate::ZoneDetails {state: "GA", zone: "024", zone_numeric: 024, name: "Banks", wfo: "FFC" },
ForecastZone::GA025 => crate::ZoneDetails {state: "GA", zone: "025", zone_numeric: 025, name: "Jackson", wfo: "FFC" },
ForecastZone::GA026 => crate::ZoneDetails {state: "GA", zone: "026", zone_numeric: 026, name: "Franklin", wfo: "GSP" },
ForecastZone::GA027 => crate::ZoneDetails {state: "GA", zone: "027", zone_numeric: 027, name: "Madison", wfo: "FFC" },
ForecastZone::GA028 => crate::ZoneDetails {state: "GA", zone: "028", zone_numeric: 028, name: "Hart", wfo: "GSP" },
ForecastZone::GA029 => crate::ZoneDetails {state: "GA", zone: "029", zone_numeric: 029, name: "Elbert", wfo: "GSP" },
ForecastZone::GA030 => crate::ZoneDetails {state: "GA", zone: "030", zone_numeric: 030, name: "Polk", wfo: "FFC" },
ForecastZone::GA031 => crate::ZoneDetails {state: "GA", zone: "031", zone_numeric: 031, name: "Paulding", wfo: "FFC" },
ForecastZone::GA032 => crate::ZoneDetails {state: "GA", zone: "032", zone_numeric: 032, name: "Cobb", wfo: "FFC" },
ForecastZone::GA033 => crate::ZoneDetails {state: "GA", zone: "033", zone_numeric: 033, name: "North Fulton", wfo: "FFC" },
ForecastZone::GA034 => crate::ZoneDetails {state: "GA", zone: "034", zone_numeric: 034, name: "Gwinnett", wfo: "FFC" },
ForecastZone::GA035 => crate::ZoneDetails {state: "GA", zone: "035", zone_numeric: 035, name: "Barrow", wfo: "FFC" },
ForecastZone::GA036 => crate::ZoneDetails {state: "GA", zone: "036", zone_numeric: 036, name: "Clarke", wfo: "FFC" },
ForecastZone::GA037 => crate::ZoneDetails {state: "GA", zone: "037", zone_numeric: 037, name: "Oconee", wfo: "FFC" },
ForecastZone::GA038 => crate::ZoneDetails {state: "GA", zone: "038", zone_numeric: 038, name: "Oglethorpe", wfo: "FFC" },
ForecastZone::GA039 => crate::ZoneDetails {state: "GA", zone: "039", zone_numeric: 039, name: "Wilkes", wfo: "FFC" },
ForecastZone::GA040 => crate::ZoneDetails {state: "GA", zone: "040", zone_numeric: 040, name: "Lincoln", wfo: "CAE" },
ForecastZone::GA041 => crate::ZoneDetails {state: "GA", zone: "041", zone_numeric: 041, name: "Haralson", wfo: "FFC" },
ForecastZone::GA042 => crate::ZoneDetails {state: "GA", zone: "042", zone_numeric: 042, name: "Carroll", wfo: "FFC" },
ForecastZone::GA043 => crate::ZoneDetails {state: "GA", zone: "043", zone_numeric: 043, name: "Douglas", wfo: "FFC" },
ForecastZone::GA044 => crate::ZoneDetails {state: "GA", zone: "044", zone_numeric: 044, name: "South Fulton", wfo: "FFC" },
ForecastZone::GA045 => crate::ZoneDetails {state: "GA", zone: "045", zone_numeric: 045, name: "DeKalb", wfo: "FFC" },
ForecastZone::GA046 => crate::ZoneDetails {state: "GA", zone: "046", zone_numeric: 046, name: "Rockdale", wfo: "FFC" },
ForecastZone::GA047 => crate::ZoneDetails {state: "GA", zone: "047", zone_numeric: 047, name: "Walton", wfo: "FFC" },
ForecastZone::GA048 => crate::ZoneDetails {state: "GA", zone: "048", zone_numeric: 048, name: "Newton", wfo: "FFC" },
ForecastZone::GA049 => crate::ZoneDetails {state: "GA", zone: "049", zone_numeric: 049, name: "Morgan", wfo: "FFC" },
ForecastZone::GA050 => crate::ZoneDetails {state: "GA", zone: "050", zone_numeric: 050, name: "Greene", wfo: "FFC" },
ForecastZone::GA051 => crate::ZoneDetails {state: "GA", zone: "051", zone_numeric: 051, name: "Taliaferro", wfo: "FFC" },
ForecastZone::GA052 => crate::ZoneDetails {state: "GA", zone: "052", zone_numeric: 052, name: "Heard", wfo: "FFC" },
ForecastZone::GA053 => crate::ZoneDetails {state: "GA", zone: "053", zone_numeric: 053, name: "Coweta", wfo: "FFC" },
ForecastZone::GA054 => crate::ZoneDetails {state: "GA", zone: "054", zone_numeric: 054, name: "Fayette", wfo: "FFC" },
ForecastZone::GA055 => crate::ZoneDetails {state: "GA", zone: "055", zone_numeric: 055, name: "Clayton", wfo: "FFC" },
ForecastZone::GA056 => crate::ZoneDetails {state: "GA", zone: "056", zone_numeric: 056, name: "Spalding", wfo: "FFC" },
ForecastZone::GA057 => crate::ZoneDetails {state: "GA", zone: "057", zone_numeric: 057, name: "Henry", wfo: "FFC" },
ForecastZone::GA058 => crate::ZoneDetails {state: "GA", zone: "058", zone_numeric: 058, name: "Butts", wfo: "FFC" },
ForecastZone::GA059 => crate::ZoneDetails {state: "GA", zone: "059", zone_numeric: 059, name: "Jasper", wfo: "FFC" },
ForecastZone::GA060 => crate::ZoneDetails {state: "GA", zone: "060", zone_numeric: 060, name: "Putnam", wfo: "FFC" },
ForecastZone::GA061 => crate::ZoneDetails {state: "GA", zone: "061", zone_numeric: 061, name: "Hancock", wfo: "FFC" },
ForecastZone::GA062 => crate::ZoneDetails {state: "GA", zone: "062", zone_numeric: 062, name: "Warren", wfo: "FFC" },
ForecastZone::GA063 => crate::ZoneDetails {state: "GA", zone: "063", zone_numeric: 063, name: "McDuffie", wfo: "CAE" },
ForecastZone::GA064 => crate::ZoneDetails {state: "GA", zone: "064", zone_numeric: 064, name: "Columbia", wfo: "CAE" },
ForecastZone::GA065 => crate::ZoneDetails {state: "GA", zone: "065", zone_numeric: 065, name: "Richmond", wfo: "CAE" },
ForecastZone::GA066 => crate::ZoneDetails {state: "GA", zone: "066", zone_numeric: 066, name: "Troup", wfo: "FFC" },
ForecastZone::GA067 => crate::ZoneDetails {state: "GA", zone: "067", zone_numeric: 067, name: "Meriwether", wfo: "FFC" },
ForecastZone::GA068 => crate::ZoneDetails {state: "GA", zone: "068", zone_numeric: 068, name: "Pike", wfo: "FFC" },
ForecastZone::GA069 => crate::ZoneDetails {state: "GA", zone: "069", zone_numeric: 069, name: "Upson", wfo: "FFC" },
ForecastZone::GA070 => crate::ZoneDetails {state: "GA", zone: "070", zone_numeric: 070, name: "Lamar", wfo: "FFC" },
ForecastZone::GA071 => crate::ZoneDetails {state: "GA", zone: "071", zone_numeric: 071, name: "Monroe", wfo: "FFC" },
ForecastZone::GA072 => crate::ZoneDetails {state: "GA", zone: "072", zone_numeric: 072, name: "Jones", wfo: "FFC" },
ForecastZone::GA073 => crate::ZoneDetails {state: "GA", zone: "073", zone_numeric: 073, name: "Baldwin", wfo: "FFC" },
ForecastZone::GA074 => crate::ZoneDetails {state: "GA", zone: "074", zone_numeric: 074, name: "Washington", wfo: "FFC" },
ForecastZone::GA075 => crate::ZoneDetails {state: "GA", zone: "075", zone_numeric: 075, name: "Glascock", wfo: "FFC" },
ForecastZone::GA076 => crate::ZoneDetails {state: "GA", zone: "076", zone_numeric: 076, name: "Jefferson", wfo: "FFC" },
ForecastZone::GA077 => crate::ZoneDetails {state: "GA", zone: "077", zone_numeric: 077, name: "Burke", wfo: "CAE" },
ForecastZone::GA078 => crate::ZoneDetails {state: "GA", zone: "078", zone_numeric: 078, name: "Harris", wfo: "FFC" },
ForecastZone::GA079 => crate::ZoneDetails {state: "GA", zone: "079", zone_numeric: 079, name: "Talbot", wfo: "FFC" },
ForecastZone::GA080 => crate::ZoneDetails {state: "GA", zone: "080", zone_numeric: 080, name: "Taylor", wfo: "FFC" },
ForecastZone::GA081 => crate::ZoneDetails {state: "GA", zone: "081", zone_numeric: 081, name: "Crawford", wfo: "FFC" },
ForecastZone::GA082 => crate::ZoneDetails {state: "GA", zone: "082", zone_numeric: 082, name: "Bibb", wfo: "FFC" },
ForecastZone::GA083 => crate::ZoneDetails {state: "GA", zone: "083", zone_numeric: 083, name: "Twiggs", wfo: "FFC" },
ForecastZone::GA084 => crate::ZoneDetails {state: "GA", zone: "084", zone_numeric: 084, name: "Wilkinson", wfo: "FFC" },
ForecastZone::GA085 => crate::ZoneDetails {state: "GA", zone: "085", zone_numeric: 085, name: "Johnson", wfo: "FFC" },
ForecastZone::GA086 => crate::ZoneDetails {state: "GA", zone: "086", zone_numeric: 086, name: "Emanuel", wfo: "FFC" },
ForecastZone::GA087 => crate::ZoneDetails {state: "GA", zone: "087", zone_numeric: 087, name: "Jenkins", wfo: "CHS" },
ForecastZone::GA088 => crate::ZoneDetails {state: "GA", zone: "088", zone_numeric: 088, name: "Screven", wfo: "CHS" },
ForecastZone::GA089 => crate::ZoneDetails {state: "GA", zone: "089", zone_numeric: 089, name: "Muscogee", wfo: "FFC" },
ForecastZone::GA090 => crate::ZoneDetails {state: "GA", zone: "090", zone_numeric: 090, name: "Chattahoochee", wfo: "FFC" },
ForecastZone::GA091 => crate::ZoneDetails {state: "GA", zone: "091", zone_numeric: 091, name: "Marion", wfo: "FFC" },
ForecastZone::GA092 => crate::ZoneDetails {state: "GA", zone: "092", zone_numeric: 092, name: "Schley", wfo: "FFC" },
ForecastZone::GA093 => crate::ZoneDetails {state: "GA", zone: "093", zone_numeric: 093, name: "Macon", wfo: "FFC" },
ForecastZone::GA094 => crate::ZoneDetails {state: "GA", zone: "094", zone_numeric: 094, name: "Peach", wfo: "FFC" },
ForecastZone::GA095 => crate::ZoneDetails {state: "GA", zone: "095", zone_numeric: 095, name: "Houston", wfo: "FFC" },
ForecastZone::GA096 => crate::ZoneDetails {state: "GA", zone: "096", zone_numeric: 096, name: "Bleckley", wfo: "FFC" },
ForecastZone::GA097 => crate::ZoneDetails {state: "GA", zone: "097", zone_numeric: 097, name: "Laurens", wfo: "FFC" },
ForecastZone::GA098 => crate::ZoneDetails {state: "GA", zone: "098", zone_numeric: 098, name: "Treutlen", wfo: "FFC" },
ForecastZone::GA099 => crate::ZoneDetails {state: "GA", zone: "099", zone_numeric: 099, name: "Candler", wfo: "CHS" },
ForecastZone::GA100 => crate::ZoneDetails {state: "GA", zone: "100", zone_numeric: 100, name: "Bulloch", wfo: "CHS" },
ForecastZone::GA101 => crate::ZoneDetails {state: "GA", zone: "101", zone_numeric: 101, name: "Effingham", wfo: "CHS" },
ForecastZone::GA102 => crate::ZoneDetails {state: "GA", zone: "102", zone_numeric: 102, name: "Stewart", wfo: "FFC" },
ForecastZone::GA103 => crate::ZoneDetails {state: "GA", zone: "103", zone_numeric: 103, name: "Webster", wfo: "FFC" },
ForecastZone::GA104 => crate::ZoneDetails {state: "GA", zone: "104", zone_numeric: 104, name: "Sumter", wfo: "FFC" },
ForecastZone::GA105 => crate::ZoneDetails {state: "GA", zone: "105", zone_numeric: 105, name: "Dooly", wfo: "FFC" },
ForecastZone::GA106 => crate::ZoneDetails {state: "GA", zone: "106", zone_numeric: 106, name: "Crisp", wfo: "FFC" },
ForecastZone::GA107 => crate::ZoneDetails {state: "GA", zone: "107", zone_numeric: 107, name: "Pulaski", wfo: "FFC" },
ForecastZone::GA108 => crate::ZoneDetails {state: "GA", zone: "108", zone_numeric: 108, name: "Wilcox", wfo: "FFC" },
ForecastZone::GA109 => crate::ZoneDetails {state: "GA", zone: "109", zone_numeric: 109, name: "Dodge", wfo: "FFC" },
ForecastZone::GA110 => crate::ZoneDetails {state: "GA", zone: "110", zone_numeric: 110, name: "Telfair", wfo: "FFC" },
ForecastZone::GA111 => crate::ZoneDetails {state: "GA", zone: "111", zone_numeric: 111, name: "Wheeler", wfo: "FFC" },
ForecastZone::GA112 => crate::ZoneDetails {state: "GA", zone: "112", zone_numeric: 112, name: "Montgomery", wfo: "FFC" },
ForecastZone::GA113 => crate::ZoneDetails {state: "GA", zone: "113", zone_numeric: 113, name: "Toombs", wfo: "FFC" },
ForecastZone::GA114 => crate::ZoneDetails {state: "GA", zone: "114", zone_numeric: 114, name: "Tattnall", wfo: "CHS" },
ForecastZone::GA115 => crate::ZoneDetails {state: "GA", zone: "115", zone_numeric: 115, name: "Evans", wfo: "CHS" },
ForecastZone::GA116 => crate::ZoneDetails {state: "GA", zone: "116", zone_numeric: 116, name: "Inland Bryan", wfo: "CHS" },
ForecastZone::GA117 => crate::ZoneDetails {state: "GA", zone: "117", zone_numeric: 117, name: "Coastal Bryan", wfo: "CHS" },
ForecastZone::GA118 => crate::ZoneDetails {state: "GA", zone: "118", zone_numeric: 118, name: "Inland Chatham", wfo: "CHS" },
ForecastZone::GA119 => crate::ZoneDetails {state: "GA", zone: "119", zone_numeric: 119, name: "Coastal Chatham", wfo: "CHS" },
ForecastZone::GA120 => crate::ZoneDetails {state: "GA", zone: "120", zone_numeric: 120, name: "Quitman", wfo: "TAE" },
ForecastZone::GA121 => crate::ZoneDetails {state: "GA", zone: "121", zone_numeric: 121, name: "Clay", wfo: "TAE" },
ForecastZone::GA122 => crate::ZoneDetails {state: "GA", zone: "122", zone_numeric: 122, name: "Randolph", wfo: "TAE" },
ForecastZone::GA123 => crate::ZoneDetails {state: "GA", zone: "123", zone_numeric: 123, name: "Calhoun", wfo: "TAE" },
ForecastZone::GA124 => crate::ZoneDetails {state: "GA", zone: "124", zone_numeric: 124, name: "Terrell", wfo: "TAE" },
ForecastZone::GA125 => crate::ZoneDetails {state: "GA", zone: "125", zone_numeric: 125, name: "Dougherty", wfo: "TAE" },
ForecastZone::GA126 => crate::ZoneDetails {state: "GA", zone: "126", zone_numeric: 126, name: "Lee", wfo: "TAE" },
ForecastZone::GA127 => crate::ZoneDetails {state: "GA", zone: "127", zone_numeric: 127, name: "Worth", wfo: "TAE" },
ForecastZone::GA128 => crate::ZoneDetails {state: "GA", zone: "128", zone_numeric: 128, name: "Turner", wfo: "TAE" },
ForecastZone::GA129 => crate::ZoneDetails {state: "GA", zone: "129", zone_numeric: 129, name: "Tift", wfo: "TAE" },
ForecastZone::GA130 => crate::ZoneDetails {state: "GA", zone: "130", zone_numeric: 130, name: "Ben Hill", wfo: "TAE" },
ForecastZone::GA131 => crate::ZoneDetails {state: "GA", zone: "131", zone_numeric: 131, name: "Irwin", wfo: "TAE" },
ForecastZone::GA132 => crate::ZoneDetails {state: "GA", zone: "132", zone_numeric: 132, name: "Coffee", wfo: "JAX" },
ForecastZone::GA133 => crate::ZoneDetails {state: "GA", zone: "133", zone_numeric: 133, name: "Jeff Davis", wfo: "JAX" },
ForecastZone::GA134 => crate::ZoneDetails {state: "GA", zone: "134", zone_numeric: 134, name: "Bacon", wfo: "JAX" },
ForecastZone::GA135 => crate::ZoneDetails {state: "GA", zone: "135", zone_numeric: 135, name: "Appling", wfo: "JAX" },
ForecastZone::GA136 => crate::ZoneDetails {state: "GA", zone: "136", zone_numeric: 136, name: "Wayne", wfo: "JAX" },
ForecastZone::GA137 => crate::ZoneDetails {state: "GA", zone: "137", zone_numeric: 137, name: "Long", wfo: "CHS" },
ForecastZone::GA138 => crate::ZoneDetails {state: "GA", zone: "138", zone_numeric: 138, name: "Inland Liberty", wfo: "CHS" },
ForecastZone::GA139 => crate::ZoneDetails {state: "GA", zone: "139", zone_numeric: 139, name: "Coastal Liberty", wfo: "CHS" },
ForecastZone::GA140 => crate::ZoneDetails {state: "GA", zone: "140", zone_numeric: 140, name: "Inland McIntosh", wfo: "CHS" },
ForecastZone::GA141 => crate::ZoneDetails {state: "GA", zone: "141", zone_numeric: 141, name: "Coastal McIntosh", wfo: "CHS" },
ForecastZone::GA142 => crate::ZoneDetails {state: "GA", zone: "142", zone_numeric: 142, name: "Early", wfo: "TAE" },
ForecastZone::GA143 => crate::ZoneDetails {state: "GA", zone: "143", zone_numeric: 143, name: "Miller", wfo: "TAE" },
ForecastZone::GA144 => crate::ZoneDetails {state: "GA", zone: "144", zone_numeric: 144, name: "Baker", wfo: "TAE" },
ForecastZone::GA145 => crate::ZoneDetails {state: "GA", zone: "145", zone_numeric: 145, name: "Mitchell", wfo: "TAE" },
ForecastZone::GA146 => crate::ZoneDetails {state: "GA", zone: "146", zone_numeric: 146, name: "Colquitt", wfo: "TAE" },
ForecastZone::GA147 => crate::ZoneDetails {state: "GA", zone: "147", zone_numeric: 147, name: "Cook", wfo: "TAE" },
ForecastZone::GA148 => crate::ZoneDetails {state: "GA", zone: "148", zone_numeric: 148, name: "Berrien", wfo: "TAE" },
ForecastZone::GA149 => crate::ZoneDetails {state: "GA", zone: "149", zone_numeric: 149, name: "Atkinson", wfo: "JAX" },
ForecastZone::GA151 => crate::ZoneDetails {state: "GA", zone: "151", zone_numeric: 151, name: "Pierce", wfo: "JAX" },
ForecastZone::GA152 => crate::ZoneDetails {state: "GA", zone: "152", zone_numeric: 152, name: "Brantley", wfo: "JAX" },
ForecastZone::GA153 => crate::ZoneDetails {state: "GA", zone: "153", zone_numeric: 153, name: "Inland Glynn", wfo: "JAX" },
ForecastZone::GA154 => crate::ZoneDetails {state: "GA", zone: "154", zone_numeric: 154, name: "Coastal Glynn", wfo: "JAX" },
ForecastZone::GA155 => crate::ZoneDetails {state: "GA", zone: "155", zone_numeric: 155, name: "Seminole", wfo: "TAE" },
ForecastZone::GA156 => crate::ZoneDetails {state: "GA", zone: "156", zone_numeric: 156, name: "Decatur", wfo: "TAE" },
ForecastZone::GA157 => crate::ZoneDetails {state: "GA", zone: "157", zone_numeric: 157, name: "Grady", wfo: "TAE" },
ForecastZone::GA158 => crate::ZoneDetails {state: "GA", zone: "158", zone_numeric: 158, name: "Thomas", wfo: "TAE" },
ForecastZone::GA159 => crate::ZoneDetails {state: "GA", zone: "159", zone_numeric: 159, name: "Brooks", wfo: "TAE" },
ForecastZone::GA160 => crate::ZoneDetails {state: "GA", zone: "160", zone_numeric: 160, name: "Lowndes", wfo: "TAE" },
ForecastZone::GA161 => crate::ZoneDetails {state: "GA", zone: "161", zone_numeric: 161, name: "Lanier", wfo: "TAE" },
ForecastZone::GA162 => crate::ZoneDetails {state: "GA", zone: "162", zone_numeric: 162, name: "Echols", wfo: "JAX" },
ForecastZone::GA163 => crate::ZoneDetails {state: "GA", zone: "163", zone_numeric: 163, name: "Clinch", wfo: "JAX" },
ForecastZone::GA165 => crate::ZoneDetails {state: "GA", zone: "165", zone_numeric: 165, name: "Inland Camden", wfo: "JAX" },
ForecastZone::GA166 => crate::ZoneDetails {state: "GA", zone: "166", zone_numeric: 166, name: "Coastal Camden", wfo: "JAX" },
ForecastZone::GA250 => crate::ZoneDetails {state: "GA", zone: "250", zone_numeric: 250, name: "Northern Ware", wfo: "JAX" },
ForecastZone::GA264 => crate::ZoneDetails {state: "GA", zone: "264", zone_numeric: 264, name: "Northeastern Charlton", wfo: "JAX" },
ForecastZone::GA350 => crate::ZoneDetails {state: "GA", zone: "350", zone_numeric: 350, name: "Southern Ware", wfo: "JAX" },
ForecastZone::GA364 => crate::ZoneDetails {state: "GA", zone: "364", zone_numeric: 364, name: "Western Charlton", wfo: "JAX" },
ForecastZone::GU001 => crate::ZoneDetails {state: "GU", zone: "001", zone_numeric: 001, name: "Guam", wfo: "GUM" },
ForecastZone::HI001 => crate::ZoneDetails {state: "HI", zone: "001", zone_numeric: 001, name: "Niihau", wfo: "HFO" },
ForecastZone::HI003 => crate::ZoneDetails {state: "HI", zone: "003", zone_numeric: 003, name: "Kauai Southwest", wfo: "HFO" },
ForecastZone::HI004 => crate::ZoneDetails {state: "HI", zone: "004", zone_numeric: 004, name: "Kauai Mountains", wfo: "HFO" },
ForecastZone::HI006 => crate::ZoneDetails {state: "HI", zone: "006", zone_numeric: 006, name: "Waianae Coast", wfo: "HFO" },
ForecastZone::HI007 => crate::ZoneDetails {state: "HI", zone: "007", zone_numeric: 007, name: "Oahu North Shore", wfo: "HFO" },
ForecastZone::HI009 => crate::ZoneDetails {state: "HI", zone: "009", zone_numeric: 009, name: "Olomana", wfo: "HFO" },
ForecastZone::HI010 => crate::ZoneDetails {state: "HI", zone: "010", zone_numeric: 010, name: "Central Oahu", wfo: "HFO" },
ForecastZone::HI011 => crate::ZoneDetails {state: "HI", zone: "011", zone_numeric: 011, name: "Waianae Mountains", wfo: "HFO" },
ForecastZone::HI015 => crate::ZoneDetails {state: "HI", zone: "015", zone_numeric: 015, name: "Lanai Mauka", wfo: "HFO" },
ForecastZone::HI016 => crate::ZoneDetails {state: "HI", zone: "016", zone_numeric: 016, name: "Kahoolawe", wfo: "HFO" },
ForecastZone::HI017 => crate::ZoneDetails {state: "HI", zone: "017", zone_numeric: 017, name: "Maui Windward West", wfo: "HFO" },
ForecastZone::HI018 => crate::ZoneDetails {state: "HI", zone: "018", zone_numeric: 018, name: "Maui Leeward West", wfo: "HFO" },
ForecastZone::HI022 => crate::ZoneDetails {state: "HI", zone: "022", zone_numeric: 022, name: "Haleakala Summit", wfo: "HFO" },
ForecastZone::HI023 => crate::ZoneDetails {state: "HI", zone: "023", zone_numeric: 023, name: "Kona", wfo: "HFO" },
ForecastZone::HI026 => crate::ZoneDetails {state: "HI", zone: "026", zone_numeric: 026, name: "Kohala", wfo: "HFO" },
ForecastZone::HI027 => crate::ZoneDetails {state: "HI", zone: "027", zone_numeric: 027, name: "Big Island Interior", wfo: "HFO" },
ForecastZone::HI028 => crate::ZoneDetails {state: "HI", zone: "028", zone_numeric: 028, name: "Big Island Summit", wfo: "HFO" },
ForecastZone::HI029 => crate::ZoneDetails {state: "HI", zone: "029", zone_numeric: 029, name: "Kauai North", wfo: "HFO" },
ForecastZone::HI030 => crate::ZoneDetails {state: "HI", zone: "030", zone_numeric: 030, name: "Kauai East", wfo: "HFO" },
ForecastZone::HI031 => crate::ZoneDetails {state: "HI", zone: "031", zone_numeric: 031, name: "Kauai South", wfo: "HFO" },
ForecastZone::HI032 => crate::ZoneDetails {state: "HI", zone: "032", zone_numeric: 032, name: "East Honolulu", wfo: "HFO" },
ForecastZone::HI033 => crate::ZoneDetails {state: "HI", zone: "033", zone_numeric: 033, name: "Honolulu Metro", wfo: "HFO" },
ForecastZone::HI034 => crate::ZoneDetails {state: "HI", zone: "034", zone_numeric: 034, name: "Ewa Plain", wfo: "HFO" },
ForecastZone::HI035 => crate::ZoneDetails {state: "HI", zone: "035", zone_numeric: 035, name: "Koolau Windward", wfo: "HFO" },
ForecastZone::HI036 => crate::ZoneDetails {state: "HI", zone: "036", zone_numeric: 036, name: "Koolau Leeward", wfo: "HFO" },
ForecastZone::HI037 => crate::ZoneDetails {state: "HI", zone: "037", zone_numeric: 037, name: "Molokai Windward", wfo: "HFO" },
ForecastZone::HI038 => crate::ZoneDetails {state: "HI", zone: "038", zone_numeric: 038, name: "Molokai Southeast", wfo: "HFO" },
ForecastZone::HI039 => crate::ZoneDetails {state: "HI", zone: "039", zone_numeric: 039, name: "Molokai North", wfo: "HFO" },
ForecastZone::HI040 => crate::ZoneDetails {state: "HI", zone: "040", zone_numeric: 040, name: "Molokai West", wfo: "HFO" },
ForecastZone::HI041 => crate::ZoneDetails {state: "HI", zone: "041", zone_numeric: 041, name: "Molokai Leeward South", wfo: "HFO" },
ForecastZone::HI042 => crate::ZoneDetails {state: "HI", zone: "042", zone_numeric: 042, name: "Lanai Windward", wfo: "HFO" },
ForecastZone::HI043 => crate::ZoneDetails {state: "HI", zone: "043", zone_numeric: 043, name: "Lanai Leeward", wfo: "HFO" },
ForecastZone::HI044 => crate::ZoneDetails {state: "HI", zone: "044", zone_numeric: 044, name: "Lanai South", wfo: "HFO" },
ForecastZone::HI045 => crate::ZoneDetails {state: "HI", zone: "045", zone_numeric: 045, name: "Maui Central Valley North", wfo: "HFO" },
ForecastZone::HI046 => crate::ZoneDetails {state: "HI", zone: "046", zone_numeric: 046, name: "Maui Central Valley South", wfo: "HFO" },
ForecastZone::HI047 => crate::ZoneDetails {state: "HI", zone: "047", zone_numeric: 047, name: "Windward Haleakala", wfo: "HFO" },
ForecastZone::HI048 => crate::ZoneDetails {state: "HI", zone: "048", zone_numeric: 048, name: "Kipahulu", wfo: "HFO" },
ForecastZone::HI049 => crate::ZoneDetails {state: "HI", zone: "049", zone_numeric: 049, name: "South Maui/Upcountry", wfo: "HFO" },
ForecastZone::HI050 => crate::ZoneDetails {state: "HI", zone: "050", zone_numeric: 050, name: "South Haleakala", wfo: "HFO" },
ForecastZone::HI051 => crate::ZoneDetails {state: "HI", zone: "051", zone_numeric: 051, name: "Big Island South", wfo: "HFO" },
ForecastZone::HI052 => crate::ZoneDetails {state: "HI", zone: "052", zone_numeric: 052, name: "Big Island Southeast", wfo: "HFO" },
ForecastZone::HI053 => crate::ZoneDetails {state: "HI", zone: "053", zone_numeric: 053, name: "Big Island East", wfo: "HFO" },
ForecastZone::HI054 => crate::ZoneDetails {state: "HI", zone: "054", zone_numeric: 054, name: "Big Island North", wfo: "HFO" },
ForecastZone::IA001 => crate::ZoneDetails {state: "IA", zone: "001", zone_numeric: 001, name: "Lyon", wfo: "FSD" },
ForecastZone::IA002 => crate::ZoneDetails {state: "IA", zone: "002", zone_numeric: 002, name: "Osceola", wfo: "FSD" },
ForecastZone::IA003 => crate::ZoneDetails {state: "IA", zone: "003", zone_numeric: 003, name: "Dickinson", wfo: "FSD" },
ForecastZone::IA004 => crate::ZoneDetails {state: "IA", zone: "004", zone_numeric: 004, name: "Emmet", wfo: "DMX" },
ForecastZone::IA005 => crate::ZoneDetails {state: "IA", zone: "005", zone_numeric: 005, name: "Kossuth", wfo: "DMX" },
ForecastZone::IA006 => crate::ZoneDetails {state: "IA", zone: "006", zone_numeric: 006, name: "Winnebago", wfo: "DMX" },
ForecastZone::IA007 => crate::ZoneDetails {state: "IA", zone: "007", zone_numeric: 007, name: "Worth", wfo: "DMX" },
ForecastZone::IA008 => crate::ZoneDetails {state: "IA", zone: "008", zone_numeric: 008, name: "Mitchell", wfo: "ARX" },
ForecastZone::IA009 => crate::ZoneDetails {state: "IA", zone: "009", zone_numeric: 009, name: "Howard", wfo: "ARX" },
ForecastZone::IA010 => crate::ZoneDetails {state: "IA", zone: "010", zone_numeric: 010, name: "Winneshiek", wfo: "ARX" },
ForecastZone::IA011 => crate::ZoneDetails {state: "IA", zone: "011", zone_numeric: 011, name: "Allamakee", wfo: "ARX" },
ForecastZone::IA012 => crate::ZoneDetails {state: "IA", zone: "012", zone_numeric: 012, name: "Sioux", wfo: "FSD" },
ForecastZone::IA013 => crate::ZoneDetails {state: "IA", zone: "013", zone_numeric: 013, name: "O'Brien", wfo: "FSD" },
ForecastZone::IA014 => crate::ZoneDetails {state: "IA", zone: "014", zone_numeric: 014, name: "Clay", wfo: "FSD" },
ForecastZone::IA015 => crate::ZoneDetails {state: "IA", zone: "015", zone_numeric: 015, name: "Palo Alto", wfo: "DMX" },
ForecastZone::IA016 => crate::ZoneDetails {state: "IA", zone: "016", zone_numeric: 016, name: "Hancock", wfo: "DMX" },
ForecastZone::IA017 => crate::ZoneDetails {state: "IA", zone: "017", zone_numeric: 017, name: "Cerro Gordo", wfo: "DMX" },
ForecastZone::IA018 => crate::ZoneDetails {state: "IA", zone: "018", zone_numeric: 018, name: "Floyd", wfo: "ARX" },
ForecastZone::IA019 => crate::ZoneDetails {state: "IA", zone: "019", zone_numeric: 019, name: "Chickasaw", wfo: "ARX" },
ForecastZone::IA020 => crate::ZoneDetails {state: "IA", zone: "020", zone_numeric: 020, name: "Plymouth", wfo: "FSD" },
ForecastZone::IA021 => crate::ZoneDetails {state: "IA", zone: "021", zone_numeric: 021, name: "Cherokee", wfo: "FSD" },
ForecastZone::IA022 => crate::ZoneDetails {state: "IA", zone: "022", zone_numeric: 022, name: "Buena Vista", wfo: "FSD" },
ForecastZone::IA023 => crate::ZoneDetails {state: "IA", zone: "023", zone_numeric: 023, name: "Pocahontas", wfo: "DMX" },
ForecastZone::IA024 => crate::ZoneDetails {state: "IA", zone: "024", zone_numeric: 024, name: "Humboldt", wfo: "DMX" },
ForecastZone::IA025 => crate::ZoneDetails {state: "IA", zone: "025", zone_numeric: 025, name: "Wright", wfo: "DMX" },
ForecastZone::IA026 => crate::ZoneDetails {state: "IA", zone: "026", zone_numeric: 026, name: "Franklin", wfo: "DMX" },
ForecastZone::IA027 => crate::ZoneDetails {state: "IA", zone: "027", zone_numeric: 027, name: "Butler", wfo: "DMX" },
ForecastZone::IA028 => crate::ZoneDetails {state: "IA", zone: "028", zone_numeric: 028, name: "Bremer", wfo: "DMX" },
ForecastZone::IA029 => crate::ZoneDetails {state: "IA", zone: "029", zone_numeric: 029, name: "Fayette", wfo: "ARX" },
ForecastZone::IA030 => crate::ZoneDetails {state: "IA", zone: "030", zone_numeric: 030, name: "Clayton", wfo: "ARX" },
ForecastZone::IA031 => crate::ZoneDetails {state: "IA", zone: "031", zone_numeric: 031, name: "Woodbury", wfo: "FSD" },
ForecastZone::IA032 => crate::ZoneDetails {state: "IA", zone: "032", zone_numeric: 032, name: "Ida", wfo: "FSD" },
ForecastZone::IA033 => crate::ZoneDetails {state: "IA", zone: "033", zone_numeric: 033, name: "Sac", wfo: "DMX" },
ForecastZone::IA034 => crate::ZoneDetails {state: "IA", zone: "034", zone_numeric: 034, name: "Calhoun", wfo: "DMX" },
ForecastZone::IA035 => crate::ZoneDetails {state: "IA", zone: "035", zone_numeric: 035, name: "Webster", wfo: "DMX" },
ForecastZone::IA036 => crate::ZoneDetails {state: "IA", zone: "036", zone_numeric: 036, name: "Hamilton", wfo: "DMX" },
ForecastZone::IA037 => crate::ZoneDetails {state: "IA", zone: "037", zone_numeric: 037, name: "Hardin", wfo: "DMX" },
ForecastZone::IA038 => crate::ZoneDetails {state: "IA", zone: "038", zone_numeric: 038, name: "Grundy", wfo: "DMX" },
ForecastZone::IA039 => crate::ZoneDetails {state: "IA", zone: "039", zone_numeric: 039, name: "Black Hawk", wfo: "DMX" },
ForecastZone::IA040 => crate::ZoneDetails {state: "IA", zone: "040", zone_numeric: 040, name: "Buchanan", wfo: "DVN" },
ForecastZone::IA041 => crate::ZoneDetails {state: "IA", zone: "041", zone_numeric: 041, name: "Delaware", wfo: "DVN" },
ForecastZone::IA042 => crate::ZoneDetails {state: "IA", zone: "042", zone_numeric: 042, name: "Dubuque", wfo: "DVN" },
ForecastZone::IA043 => crate::ZoneDetails {state: "IA", zone: "043", zone_numeric: 043, name: "Monona", wfo: "OAX" },
ForecastZone::IA044 => crate::ZoneDetails {state: "IA", zone: "044", zone_numeric: 044, name: "Crawford", wfo: "DMX" },
ForecastZone::IA045 => crate::ZoneDetails {state: "IA", zone: "045", zone_numeric: 045, name: "Carroll", wfo: "DMX" },
ForecastZone::IA046 => crate::ZoneDetails {state: "IA", zone: "046", zone_numeric: 046, name: "Greene", wfo: "DMX" },
ForecastZone::IA047 => crate::ZoneDetails {state: "IA", zone: "047", zone_numeric: 047, name: "Boone", wfo: "DMX" },
ForecastZone::IA048 => crate::ZoneDetails {state: "IA", zone: "048", zone_numeric: 048, name: "Story", wfo: "DMX" },
ForecastZone::IA049 => crate::ZoneDetails {state: "IA", zone: "049", zone_numeric: 049, name: "Marshall", wfo: "DMX" },
ForecastZone::IA050 => crate::ZoneDetails {state: "IA", zone: "050", zone_numeric: 050, name: "Tama", wfo: "DMX" },
ForecastZone::IA051 => crate::ZoneDetails {state: "IA", zone: "051", zone_numeric: 051, name: "Benton", wfo: "DVN" },
ForecastZone::IA052 => crate::ZoneDetails {state: "IA", zone: "052", zone_numeric: 052, name: "Linn", wfo: "DVN" },
ForecastZone::IA053 => crate::ZoneDetails {state: "IA", zone: "053", zone_numeric: 053, name: "Jones", wfo: "DVN" },
ForecastZone::IA054 => crate::ZoneDetails {state: "IA", zone: "054", zone_numeric: 054, name: "Jackson", wfo: "DVN" },
ForecastZone::IA055 => crate::ZoneDetails {state: "IA", zone: "055", zone_numeric: 055, name: "Harrison", wfo: "OAX" },
ForecastZone::IA056 => crate::ZoneDetails {state: "IA", zone: "056", zone_numeric: 056, name: "Shelby", wfo: "OAX" },
ForecastZone::IA057 => crate::ZoneDetails {state: "IA", zone: "057", zone_numeric: 057, name: "Audubon", wfo: "DMX" },
ForecastZone::IA058 => crate::ZoneDetails {state: "IA", zone: "058", zone_numeric: 058, name: "Guthrie", wfo: "DMX" },
ForecastZone::IA059 => crate::ZoneDetails {state: "IA", zone: "059", zone_numeric: 059, name: "Dallas", wfo: "DMX" },
ForecastZone::IA060 => crate::ZoneDetails {state: "IA", zone: "060", zone_numeric: 060, name: "Polk", wfo: "DMX" },
ForecastZone::IA061 => crate::ZoneDetails {state: "IA", zone: "061", zone_numeric: 061, name: "Jasper", wfo: "DMX" },
ForecastZone::IA062 => crate::ZoneDetails {state: "IA", zone: "062", zone_numeric: 062, name: "Poweshiek", wfo: "DMX" },
ForecastZone::IA063 => crate::ZoneDetails {state: "IA", zone: "063", zone_numeric: 063, name: "Iowa", wfo: "DVN" },
ForecastZone::IA064 => crate::ZoneDetails {state: "IA", zone: "064", zone_numeric: 064, name: "Johnson", wfo: "DVN" },
ForecastZone::IA065 => crate::ZoneDetails {state: "IA", zone: "065", zone_numeric: 065, name: "Cedar", wfo: "DVN" },
ForecastZone::IA066 => crate::ZoneDetails {state: "IA", zone: "066", zone_numeric: 066, name: "Clinton", wfo: "DVN" },
ForecastZone::IA067 => crate::ZoneDetails {state: "IA", zone: "067", zone_numeric: 067, name: "Muscatine", wfo: "DVN" },
ForecastZone::IA068 => crate::ZoneDetails {state: "IA", zone: "068", zone_numeric: 068, name: "Scott", wfo: "DVN" },
ForecastZone::IA069 => crate::ZoneDetails {state: "IA", zone: "069", zone_numeric: 069, name: "Pottawattamie", wfo: "OAX" },
ForecastZone::IA070 => crate::ZoneDetails {state: "IA", zone: "070", zone_numeric: 070, name: "Cass", wfo: "DMX" },
ForecastZone::IA071 => crate::ZoneDetails {state: "IA", zone: "071", zone_numeric: 071, name: "Adair", wfo: "DMX" },
ForecastZone::IA072 => crate::ZoneDetails {state: "IA", zone: "072", zone_numeric: 072, name: "Madison", wfo: "DMX" },
ForecastZone::IA073 => crate::ZoneDetails {state: "IA", zone: "073", zone_numeric: 073, name: "Warren", wfo: "DMX" },
ForecastZone::IA074 => crate::ZoneDetails {state: "IA", zone: "074", zone_numeric: 074, name: "Marion", wfo: "DMX" },
ForecastZone::IA075 => crate::ZoneDetails {state: "IA", zone: "075", zone_numeric: 075, name: "Mahaska", wfo: "DMX" },
ForecastZone::IA076 => crate::ZoneDetails {state: "IA", zone: "076", zone_numeric: 076, name: "Keokuk", wfo: "DVN" },
ForecastZone::IA077 => crate::ZoneDetails {state: "IA", zone: "077", zone_numeric: 077, name: "Washington", wfo: "DVN" },
ForecastZone::IA078 => crate::ZoneDetails {state: "IA", zone: "078", zone_numeric: 078, name: "Louisa", wfo: "DVN" },
ForecastZone::IA079 => crate::ZoneDetails {state: "IA", zone: "079", zone_numeric: 079, name: "Mills", wfo: "OAX" },
ForecastZone::IA080 => crate::ZoneDetails {state: "IA", zone: "080", zone_numeric: 080, name: "Montgomery", wfo: "OAX" },
ForecastZone::IA081 => crate::ZoneDetails {state: "IA", zone: "081", zone_numeric: 081, name: "Adams", wfo: "DMX" },
ForecastZone::IA082 => crate::ZoneDetails {state: "IA", zone: "082", zone_numeric: 082, name: "Union", wfo: "DMX" },
ForecastZone::IA083 => crate::ZoneDetails {state: "IA", zone: "083", zone_numeric: 083, name: "Clarke", wfo: "DMX" },
ForecastZone::IA084 => crate::ZoneDetails {state: "IA", zone: "084", zone_numeric: 084, name: "Lucas", wfo: "DMX" },
ForecastZone::IA085 => crate::ZoneDetails {state: "IA", zone: "085", zone_numeric: 085, name: "Monroe", wfo: "DMX" },
ForecastZone::IA086 => crate::ZoneDetails {state: "IA", zone: "086", zone_numeric: 086, name: "Wapello", wfo: "DMX" },
ForecastZone::IA087 => crate::ZoneDetails {state: "IA", zone: "087", zone_numeric: 087, name: "Jefferson", wfo: "DVN" },
ForecastZone::IA088 => crate::ZoneDetails {state: "IA", zone: "088", zone_numeric: 088, name: "Henry", wfo: "DVN" },
ForecastZone::IA089 => crate::ZoneDetails {state: "IA", zone: "089", zone_numeric: 089, name: "Des Moines", wfo: "DVN" },
ForecastZone::IA090 => crate::ZoneDetails {state: "IA", zone: "090", zone_numeric: 090, name: "Fremont", wfo: "OAX" },
ForecastZone::IA091 => crate::ZoneDetails {state: "IA", zone: "091", zone_numeric: 091, name: "Page", wfo: "OAX" },
ForecastZone::IA092 => crate::ZoneDetails {state: "IA", zone: "092", zone_numeric: 092, name: "Taylor", wfo: "DMX" },
ForecastZone::IA093 => crate::ZoneDetails {state: "IA", zone: "093", zone_numeric: 093, name: "Ringgold", wfo: "DMX" },
ForecastZone::IA094 => crate::ZoneDetails {state: "IA", zone: "094", zone_numeric: 094, name: "Decatur", wfo: "DMX" },
ForecastZone::IA095 => crate::ZoneDetails {state: "IA", zone: "095", zone_numeric: 095, name: "Wayne", wfo: "DMX" },
ForecastZone::IA096 => crate::ZoneDetails {state: "IA", zone: "096", zone_numeric: 096, name: "Appanoose", wfo: "DMX" },
ForecastZone::IA097 => crate::ZoneDetails {state: "IA", zone: "097", zone_numeric: 097, name: "Davis", wfo: "DMX" },
ForecastZone::IA098 => crate::ZoneDetails {state: "IA", zone: "098", zone_numeric: 098, name: "Van Buren", wfo: "DVN" },
ForecastZone::IA099 => crate::ZoneDetails {state: "IA", zone: "099", zone_numeric: 099, name: "Lee", wfo: "DVN" },
ForecastZone::ID001 => crate::ZoneDetails {state: "ID", zone: "001", zone_numeric: 001, name: "Northern Panhandle", wfo: "OTX" },
ForecastZone::ID002 => crate::ZoneDetails {state: "ID", zone: "002", zone_numeric: 002, name: "Coeur d'Alene Area", wfo: "OTX" },
ForecastZone::ID003 => crate::ZoneDetails {state: "ID", zone: "003", zone_numeric: 003, name: "Idaho Palouse", wfo: "OTX" },
ForecastZone::ID004 => crate::ZoneDetails {state: "ID", zone: "004", zone_numeric: 004, name: "Central Panhandle Mountains", wfo: "OTX" },
ForecastZone::ID005 => crate::ZoneDetails {state: "ID", zone: "005", zone_numeric: 005, name: "Northern Clearwater Mountains", wfo: "MSO" },
ForecastZone::ID006 => crate::ZoneDetails {state: "ID", zone: "006", zone_numeric: 006, name: "Southern Clearwater Mountains", wfo: "MSO" },
ForecastZone::ID007 => crate::ZoneDetails {state: "ID", zone: "007", zone_numeric: 007, name: "Orofino/Grangeville Region", wfo: "MSO" },
ForecastZone::ID008 => crate::ZoneDetails {state: "ID", zone: "008", zone_numeric: 008, name: "Lower Hells Canyon/Salmon River Region", wfo: "MSO" },
ForecastZone::ID009 => crate::ZoneDetails {state: "ID", zone: "009", zone_numeric: 009, name: "Western Lemhi County", wfo: "MSO" },
ForecastZone::ID010 => crate::ZoneDetails {state: "ID", zone: "010", zone_numeric: 010, name: "Eastern Lemhi County", wfo: "MSO" },
ForecastZone::ID011 => crate::ZoneDetails {state: "ID", zone: "011", zone_numeric: 011, name: "West Central Mountains", wfo: "BOI" },
ForecastZone::ID012 => crate::ZoneDetails {state: "ID", zone: "012", zone_numeric: 012, name: "Lower Treasure Valley", wfo: "BOI" },
ForecastZone::ID013 => crate::ZoneDetails {state: "ID", zone: "013", zone_numeric: 013, name: "Boise Mountains", wfo: "BOI" },
ForecastZone::ID014 => crate::ZoneDetails {state: "ID", zone: "014", zone_numeric: 014, name: "Upper Treasure Valley", wfo: "BOI" },
ForecastZone::ID015 => crate::ZoneDetails {state: "ID", zone: "015", zone_numeric: 015, name: "Southwest Highlands", wfo: "BOI" },
ForecastZone::ID016 => crate::ZoneDetails {state: "ID", zone: "016", zone_numeric: 016, name: "Western Magic Valley", wfo: "BOI" },
ForecastZone::ID026 => crate::ZoneDetails {state: "ID", zone: "026", zone_numeric: 026, name: "Lewiston Area", wfo: "OTX" },
ForecastZone::ID027 => crate::ZoneDetails {state: "ID", zone: "027", zone_numeric: 027, name: "Lewis and Southern Nez Perce Counties", wfo: "OTX" },
ForecastZone::ID028 => crate::ZoneDetails {state: "ID", zone: "028", zone_numeric: 028, name: "Camas Prairie", wfo: "BOI" },
ForecastZone::ID029 => crate::ZoneDetails {state: "ID", zone: "029", zone_numeric: 029, name: "Owyhee Mountains", wfo: "BOI" },
ForecastZone::ID030 => crate::ZoneDetails {state: "ID", zone: "030", zone_numeric: 030, name: "Southern Twin  Falls County", wfo: "BOI" },
ForecastZone::ID033 => crate::ZoneDetails {state: "ID", zone: "033", zone_numeric: 033, name: "Upper Weiser River", wfo: "BOI" },
ForecastZone::ID051 => crate::ZoneDetails {state: "ID", zone: "051", zone_numeric: 051, name: "Shoshone/Lava Beds", wfo: "PIH" },
ForecastZone::ID052 => crate::ZoneDetails {state: "ID", zone: "052", zone_numeric: 052, name: "Arco/Mud Lake Desert", wfo: "PIH" },
ForecastZone::ID053 => crate::ZoneDetails {state: "ID", zone: "053", zone_numeric: 053, name: "Upper Snake River Plain", wfo: "PIH" },
ForecastZone::ID054 => crate::ZoneDetails {state: "ID", zone: "054", zone_numeric: 054, name: "Lower Snake River Plain", wfo: "PIH" },
ForecastZone::ID055 => crate::ZoneDetails {state: "ID", zone: "055", zone_numeric: 055, name: "Eastern Magic Valley", wfo: "PIH" },
ForecastZone::ID056 => crate::ZoneDetails {state: "ID", zone: "056", zone_numeric: 056, name: "Southern Hills/Albion Mountains", wfo: "PIH" },
ForecastZone::ID057 => crate::ZoneDetails {state: "ID", zone: "057", zone_numeric: 057, name: "Raft River Region", wfo: "PIH" },
ForecastZone::ID058 => crate::ZoneDetails {state: "ID", zone: "058", zone_numeric: 058, name: "Marsh and Arbon Highlands", wfo: "PIH" },
ForecastZone::ID059 => crate::ZoneDetails {state: "ID", zone: "059", zone_numeric: 059, name: "Franklin/Eastern Oneida Region", wfo: "PIH" },
ForecastZone::ID060 => crate::ZoneDetails {state: "ID", zone: "060", zone_numeric: 060, name: "Bear River Range", wfo: "PIH" },
ForecastZone::ID061 => crate::ZoneDetails {state: "ID", zone: "061", zone_numeric: 061, name: "Bear Lake Valley", wfo: "PIH" },
ForecastZone::ID062 => crate::ZoneDetails {state: "ID", zone: "062", zone_numeric: 062, name: "Blackfoot Mountains", wfo: "PIH" },
ForecastZone::ID063 => crate::ZoneDetails {state: "ID", zone: "063", zone_numeric: 063, name: "Caribou Range", wfo: "PIH" },
ForecastZone::ID064 => crate::ZoneDetails {state: "ID", zone: "064", zone_numeric: 064, name: "Big Hole Mountains", wfo: "PIH" },
ForecastZone::ID065 => crate::ZoneDetails {state: "ID", zone: "065", zone_numeric: 065, name: "Teton Valley", wfo: "PIH" },
ForecastZone::ID066 => crate::ZoneDetails {state: "ID", zone: "066", zone_numeric: 066, name: "Centennial Mountains/Island Park", wfo: "PIH" },
ForecastZone::ID067 => crate::ZoneDetails {state: "ID", zone: "067", zone_numeric: 067, name: "Beaverhead/Lemhi Highlands", wfo: "PIH" },
ForecastZone::ID068 => crate::ZoneDetails {state: "ID", zone: "068", zone_numeric: 068, name: "Lost River Valleys", wfo: "PIH" },
ForecastZone::ID069 => crate::ZoneDetails {state: "ID", zone: "069", zone_numeric: 069, name: "Lost River Range", wfo: "PIH" },
ForecastZone::ID070 => crate::ZoneDetails {state: "ID", zone: "070", zone_numeric: 070, name: "Challis/Pahsimeroi Valleys", wfo: "PIH" },
ForecastZone::ID071 => crate::ZoneDetails {state: "ID", zone: "071", zone_numeric: 071, name: "Frank Church Wilderness", wfo: "PIH" },
ForecastZone::ID072 => crate::ZoneDetails {state: "ID", zone: "072", zone_numeric: 072, name: "Sawtooth/Stanley Basin", wfo: "PIH" },
ForecastZone::ID073 => crate::ZoneDetails {state: "ID", zone: "073", zone_numeric: 073, name: "Sun Valley Region", wfo: "PIH" },
ForecastZone::ID074 => crate::ZoneDetails {state: "ID", zone: "074", zone_numeric: 074, name: "Big Lost Highlands/Copper Basin", wfo: "PIH" },
ForecastZone::ID075 => crate::ZoneDetails {state: "ID", zone: "075", zone_numeric: 075, name: "Wood River Foothills", wfo: "PIH" },
ForecastZone::IL001 => crate::ZoneDetails {state: "IL", zone: "001", zone_numeric: 001, name: "Jo Daviess", wfo: "DVN" },
ForecastZone::IL002 => crate::ZoneDetails {state: "IL", zone: "002", zone_numeric: 002, name: "Stephenson", wfo: "DVN" },
ForecastZone::IL003 => crate::ZoneDetails {state: "IL", zone: "003", zone_numeric: 003, name: "Winnebago", wfo: "LOT" },
ForecastZone::IL004 => crate::ZoneDetails {state: "IL", zone: "004", zone_numeric: 004, name: "Boone", wfo: "LOT" },
ForecastZone::IL005 => crate::ZoneDetails {state: "IL", zone: "005", zone_numeric: 005, name: "McHenry", wfo: "LOT" },
ForecastZone::IL006 => crate::ZoneDetails {state: "IL", zone: "006", zone_numeric: 006, name: "Lake", wfo: "LOT" },
ForecastZone::IL007 => crate::ZoneDetails {state: "IL", zone: "007", zone_numeric: 007, name: "Carroll", wfo: "DVN" },
ForecastZone::IL008 => crate::ZoneDetails {state: "IL", zone: "008", zone_numeric: 008, name: "Ogle", wfo: "LOT" },
ForecastZone::IL009 => crate::ZoneDetails {state: "IL", zone: "009", zone_numeric: 009, name: "Whiteside", wfo: "DVN" },
ForecastZone::IL010 => crate::ZoneDetails {state: "IL", zone: "010", zone_numeric: 010, name: "Lee", wfo: "LOT" },
ForecastZone::IL011 => crate::ZoneDetails {state: "IL", zone: "011", zone_numeric: 011, name: "De Kalb", wfo: "LOT" },
ForecastZone::IL012 => crate::ZoneDetails {state: "IL", zone: "012", zone_numeric: 012, name: "Kane", wfo: "LOT" },
ForecastZone::IL013 => crate::ZoneDetails {state: "IL", zone: "013", zone_numeric: 013, name: "DuPage", wfo: "LOT" },
ForecastZone::IL015 => crate::ZoneDetails {state: "IL", zone: "015", zone_numeric: 015, name: "Rock Island", wfo: "DVN" },
ForecastZone::IL016 => crate::ZoneDetails {state: "IL", zone: "016", zone_numeric: 016, name: "Henry", wfo: "DVN" },
ForecastZone::IL017 => crate::ZoneDetails {state: "IL", zone: "017", zone_numeric: 017, name: "Bureau", wfo: "DVN" },
ForecastZone::IL018 => crate::ZoneDetails {state: "IL", zone: "018", zone_numeric: 018, name: "Putnam", wfo: "DVN" },
ForecastZone::IL019 => crate::ZoneDetails {state: "IL", zone: "019", zone_numeric: 019, name: "La Salle", wfo: "LOT" },
ForecastZone::IL020 => crate::ZoneDetails {state: "IL", zone: "020", zone_numeric: 020, name: "Kendall", wfo: "LOT" },
ForecastZone::IL021 => crate::ZoneDetails {state: "IL", zone: "021", zone_numeric: 021, name: "Grundy", wfo: "LOT" },
ForecastZone::IL023 => crate::ZoneDetails {state: "IL", zone: "023", zone_numeric: 023, name: "Kankakee", wfo: "LOT" },
ForecastZone::IL024 => crate::ZoneDetails {state: "IL", zone: "024", zone_numeric: 024, name: "Mercer", wfo: "DVN" },
ForecastZone::IL025 => crate::ZoneDetails {state: "IL", zone: "025", zone_numeric: 025, name: "Henderson", wfo: "DVN" },
ForecastZone::IL026 => crate::ZoneDetails {state: "IL", zone: "026", zone_numeric: 026, name: "Warren", wfo: "DVN" },
ForecastZone::IL027 => crate::ZoneDetails {state: "IL", zone: "027", zone_numeric: 027, name: "Knox", wfo: "ILX" },
ForecastZone::IL028 => crate::ZoneDetails {state: "IL", zone: "028", zone_numeric: 028, name: "Stark", wfo: "ILX" },
ForecastZone::IL029 => crate::ZoneDetails {state: "IL", zone: "029", zone_numeric: 029, name: "Peoria", wfo: "ILX" },
ForecastZone::IL030 => crate::ZoneDetails {state: "IL", zone: "030", zone_numeric: 030, name: "Marshall", wfo: "ILX" },
ForecastZone::IL031 => crate::ZoneDetails {state: "IL", zone: "031", zone_numeric: 031, name: "Woodford", wfo: "ILX" },
ForecastZone::IL032 => crate::ZoneDetails {state: "IL", zone: "032", zone_numeric: 032, name: "Livingston", wfo: "LOT" },
ForecastZone::IL033 => crate::ZoneDetails {state: "IL", zone: "033", zone_numeric: 033, name: "Iroquois", wfo: "LOT" },
ForecastZone::IL034 => crate::ZoneDetails {state: "IL", zone: "034", zone_numeric: 034, name: "Hancock", wfo: "DVN" },
ForecastZone::IL035 => crate::ZoneDetails {state: "IL", zone: "035", zone_numeric: 035, name: "McDonough", wfo: "DVN" },
ForecastZone::IL036 => crate::ZoneDetails {state: "IL", zone: "036", zone_numeric: 036, name: "Fulton", wfo: "ILX" },
ForecastZone::IL037 => crate::ZoneDetails {state: "IL", zone: "037", zone_numeric: 037, name: "Tazewell", wfo: "ILX" },
ForecastZone::IL038 => crate::ZoneDetails {state: "IL", zone: "038", zone_numeric: 038, name: "McLean", wfo: "ILX" },
ForecastZone::IL039 => crate::ZoneDetails {state: "IL", zone: "039", zone_numeric: 039, name: "Ford", wfo: "LOT" },
ForecastZone::IL040 => crate::ZoneDetails {state: "IL", zone: "040", zone_numeric: 040, name: "Schuyler", wfo: "ILX" },
ForecastZone::IL041 => crate::ZoneDetails {state: "IL", zone: "041", zone_numeric: 041, name: "Mason", wfo: "ILX" },
ForecastZone::IL042 => crate::ZoneDetails {state: "IL", zone: "042", zone_numeric: 042, name: "Logan", wfo: "ILX" },
ForecastZone::IL043 => crate::ZoneDetails {state: "IL", zone: "043", zone_numeric: 043, name: "De Witt", wfo: "ILX" },
ForecastZone::IL044 => crate::ZoneDetails {state: "IL", zone: "044", zone_numeric: 044, name: "Piatt", wfo: "ILX" },
ForecastZone::IL045 => crate::ZoneDetails {state: "IL", zone: "045", zone_numeric: 045, name: "Champaign", wfo: "ILX" },
ForecastZone::IL046 => crate::ZoneDetails {state: "IL", zone: "046", zone_numeric: 046, name: "Vermilion", wfo: "ILX" },
ForecastZone::IL047 => crate::ZoneDetails {state: "IL", zone: "047", zone_numeric: 047, name: "Cass", wfo: "ILX" },
ForecastZone::IL048 => crate::ZoneDetails {state: "IL", zone: "048", zone_numeric: 048, name: "Menard", wfo: "ILX" },
ForecastZone::IL049 => crate::ZoneDetails {state: "IL", zone: "049", zone_numeric: 049, name: "Scott", wfo: "ILX" },
ForecastZone::IL050 => crate::ZoneDetails {state: "IL", zone: "050", zone_numeric: 050, name: "Morgan", wfo: "ILX" },
ForecastZone::IL051 => crate::ZoneDetails {state: "IL", zone: "051", zone_numeric: 051, name: "Sangamon", wfo: "ILX" },
ForecastZone::IL052 => crate::ZoneDetails {state: "IL", zone: "052", zone_numeric: 052, name: "Christian", wfo: "ILX" },
ForecastZone::IL053 => crate::ZoneDetails {state: "IL", zone: "053", zone_numeric: 053, name: "Macon", wfo: "ILX" },
ForecastZone::IL054 => crate::ZoneDetails {state: "IL", zone: "054", zone_numeric: 054, name: "Moultrie", wfo: "ILX" },
ForecastZone::IL055 => crate::ZoneDetails {state: "IL", zone: "055", zone_numeric: 055, name: "Douglas", wfo: "ILX" },
ForecastZone::IL056 => crate::ZoneDetails {state: "IL", zone: "056", zone_numeric: 056, name: "Coles", wfo: "ILX" },
ForecastZone::IL057 => crate::ZoneDetails {state: "IL", zone: "057", zone_numeric: 057, name: "Edgar", wfo: "ILX" },
ForecastZone::IL058 => crate::ZoneDetails {state: "IL", zone: "058", zone_numeric: 058, name: "Greene", wfo: "LSX" },
ForecastZone::IL059 => crate::ZoneDetails {state: "IL", zone: "059", zone_numeric: 059, name: "Macoupin", wfo: "LSX" },
ForecastZone::IL060 => crate::ZoneDetails {state: "IL", zone: "060", zone_numeric: 060, name: "Montgomery", wfo: "LSX" },
ForecastZone::IL061 => crate::ZoneDetails {state: "IL", zone: "061", zone_numeric: 061, name: "Shelby", wfo: "ILX" },
ForecastZone::IL062 => crate::ZoneDetails {state: "IL", zone: "062", zone_numeric: 062, name: "Cumberland", wfo: "ILX" },
ForecastZone::IL063 => crate::ZoneDetails {state: "IL", zone: "063", zone_numeric: 063, name: "Clark", wfo: "ILX" },
ForecastZone::IL064 => crate::ZoneDetails {state: "IL", zone: "064", zone_numeric: 064, name: "Bond", wfo: "LSX" },
ForecastZone::IL065 => crate::ZoneDetails {state: "IL", zone: "065", zone_numeric: 065, name: "Fayette", wfo: "LSX" },
ForecastZone::IL066 => crate::ZoneDetails {state: "IL", zone: "066", zone_numeric: 066, name: "Effingham", wfo: "ILX" },
ForecastZone::IL067 => crate::ZoneDetails {state: "IL", zone: "067", zone_numeric: 067, name: "Jasper", wfo: "ILX" },
ForecastZone::IL068 => crate::ZoneDetails {state: "IL", zone: "068", zone_numeric: 068, name: "Crawford", wfo: "ILX" },
ForecastZone::IL069 => crate::ZoneDetails {state: "IL", zone: "069", zone_numeric: 069, name: "Clinton", wfo: "LSX" },
ForecastZone::IL070 => crate::ZoneDetails {state: "IL", zone: "070", zone_numeric: 070, name: "Marion", wfo: "LSX" },
ForecastZone::IL071 => crate::ZoneDetails {state: "IL", zone: "071", zone_numeric: 071, name: "Clay", wfo: "ILX" },
ForecastZone::IL072 => crate::ZoneDetails {state: "IL", zone: "072", zone_numeric: 072, name: "Richland", wfo: "ILX" },
ForecastZone::IL073 => crate::ZoneDetails {state: "IL", zone: "073", zone_numeric: 073, name: "Lawrence", wfo: "ILX" },
ForecastZone::IL074 => crate::ZoneDetails {state: "IL", zone: "074", zone_numeric: 074, name: "Washington", wfo: "LSX" },
ForecastZone::IL075 => crate::ZoneDetails {state: "IL", zone: "075", zone_numeric: 075, name: "Jefferson", wfo: "PAH" },
ForecastZone::IL076 => crate::ZoneDetails {state: "IL", zone: "076", zone_numeric: 076, name: "Wayne", wfo: "PAH" },
ForecastZone::IL077 => crate::ZoneDetails {state: "IL", zone: "077", zone_numeric: 077, name: "Edwards", wfo: "PAH" },
ForecastZone::IL078 => crate::ZoneDetails {state: "IL", zone: "078", zone_numeric: 078, name: "Wabash", wfo: "PAH" },
ForecastZone::IL079 => crate::ZoneDetails {state: "IL", zone: "079", zone_numeric: 079, name: "Randolph", wfo: "LSX" },
ForecastZone::IL080 => crate::ZoneDetails {state: "IL", zone: "080", zone_numeric: 080, name: "Perry", wfo: "PAH" },
ForecastZone::IL081 => crate::ZoneDetails {state: "IL", zone: "081", zone_numeric: 081, name: "Franklin", wfo: "PAH" },
ForecastZone::IL082 => crate::ZoneDetails {state: "IL", zone: "082", zone_numeric: 082, name: "Hamilton", wfo: "PAH" },
ForecastZone::IL083 => crate::ZoneDetails {state: "IL", zone: "083", zone_numeric: 083, name: "White", wfo: "PAH" },
ForecastZone::IL084 => crate::ZoneDetails {state: "IL", zone: "084", zone_numeric: 084, name: "Jackson", wfo: "PAH" },
ForecastZone::IL085 => crate::ZoneDetails {state: "IL", zone: "085", zone_numeric: 085, name: "Williamson", wfo: "PAH" },
ForecastZone::IL086 => crate::ZoneDetails {state: "IL", zone: "086", zone_numeric: 086, name: "Saline", wfo: "PAH" },
ForecastZone::IL087 => crate::ZoneDetails {state: "IL", zone: "087", zone_numeric: 087, name: "Gallatin", wfo: "PAH" },
ForecastZone::IL088 => crate::ZoneDetails {state: "IL", zone: "088", zone_numeric: 088, name: "Union", wfo: "PAH" },
ForecastZone::IL089 => crate::ZoneDetails {state: "IL", zone: "089", zone_numeric: 089, name: "Johnson", wfo: "PAH" },
ForecastZone::IL090 => crate::ZoneDetails {state: "IL", zone: "090", zone_numeric: 090, name: "Pope", wfo: "PAH" },
ForecastZone::IL091 => crate::ZoneDetails {state: "IL", zone: "091", zone_numeric: 091, name: "Hardin", wfo: "PAH" },
ForecastZone::IL092 => crate::ZoneDetails {state: "IL", zone: "092", zone_numeric: 092, name: "Alexander", wfo: "PAH" },
ForecastZone::IL093 => crate::ZoneDetails {state: "IL", zone: "093", zone_numeric: 093, name: "Pulaski", wfo: "PAH" },
ForecastZone::IL094 => crate::ZoneDetails {state: "IL", zone: "094", zone_numeric: 094, name: "Massac", wfo: "PAH" },
ForecastZone::IL095 => crate::ZoneDetails {state: "IL", zone: "095", zone_numeric: 095, name: "Adams", wfo: "LSX" },
ForecastZone::IL096 => crate::ZoneDetails {state: "IL", zone: "096", zone_numeric: 096, name: "Brown", wfo: "LSX" },
ForecastZone::IL097 => crate::ZoneDetails {state: "IL", zone: "097", zone_numeric: 097, name: "Pike", wfo: "LSX" },
ForecastZone::IL098 => crate::ZoneDetails {state: "IL", zone: "098", zone_numeric: 098, name: "Calhoun", wfo: "LSX" },
ForecastZone::IL099 => crate::ZoneDetails {state: "IL", zone: "099", zone_numeric: 099, name: "Jersey", wfo: "LSX" },
ForecastZone::IL100 => crate::ZoneDetails {state: "IL", zone: "100", zone_numeric: 100, name: "Madison", wfo: "LSX" },
ForecastZone::IL101 => crate::ZoneDetails {state: "IL", zone: "101", zone_numeric: 101, name: "St. Clair", wfo: "LSX" },
ForecastZone::IL102 => crate::ZoneDetails {state: "IL", zone: "102", zone_numeric: 102, name: "Monroe", wfo: "LSX" },
ForecastZone::IL103 => crate::ZoneDetails {state: "IL", zone: "103", zone_numeric: 103, name: "Northern Cook", wfo: "LOT" },
ForecastZone::IL104 => crate::ZoneDetails {state: "IL", zone: "104", zone_numeric: 104, name: "Central Cook", wfo: "LOT" },
ForecastZone::IL105 => crate::ZoneDetails {state: "IL", zone: "105", zone_numeric: 105, name: "Southern Cook", wfo: "LOT" },
ForecastZone::IL106 => crate::ZoneDetails {state: "IL", zone: "106", zone_numeric: 106, name: "Northern Will", wfo: "LOT" },
ForecastZone::IL107 => crate::ZoneDetails {state: "IL", zone: "107", zone_numeric: 107, name: "Southern Will", wfo: "LOT" },
ForecastZone::IL108 => crate::ZoneDetails {state: "IL", zone: "108", zone_numeric: 108, name: "Eastern Will", wfo: "LOT" },
ForecastZone::IN001 => crate::ZoneDetails {state: "IN", zone: "001", zone_numeric: 001, name: "Lake", wfo: "LOT" },
ForecastZone::IN002 => crate::ZoneDetails {state: "IN", zone: "002", zone_numeric: 002, name: "Porter", wfo: "LOT" },
ForecastZone::IN003 => crate::ZoneDetails {state: "IN", zone: "003", zone_numeric: 003, name: "La Porte", wfo: "IWX" },
ForecastZone::IN004 => crate::ZoneDetails {state: "IN", zone: "004", zone_numeric: 004, name: "St. Joseph", wfo: "IWX" },
ForecastZone::IN005 => crate::ZoneDetails {state: "IN", zone: "005", zone_numeric: 005, name: "Elkhart", wfo: "IWX" },
ForecastZone::IN006 => crate::ZoneDetails {state: "IN", zone: "006", zone_numeric: 006, name: "Lagrange", wfo: "IWX" },
ForecastZone::IN007 => crate::ZoneDetails {state: "IN", zone: "007", zone_numeric: 007, name: "Steuben", wfo: "IWX" },
ForecastZone::IN008 => crate::ZoneDetails {state: "IN", zone: "008", zone_numeric: 008, name: "Noble", wfo: "IWX" },
ForecastZone::IN009 => crate::ZoneDetails {state: "IN", zone: "009", zone_numeric: 009, name: "De Kalb", wfo: "IWX" },
ForecastZone::IN010 => crate::ZoneDetails {state: "IN", zone: "010", zone_numeric: 010, name: "Newton", wfo: "LOT" },
ForecastZone::IN011 => crate::ZoneDetails {state: "IN", zone: "011", zone_numeric: 011, name: "Jasper", wfo: "LOT" },
ForecastZone::IN012 => crate::ZoneDetails {state: "IN", zone: "012", zone_numeric: 012, name: "Starke", wfo: "IWX" },
ForecastZone::IN013 => crate::ZoneDetails {state: "IN", zone: "013", zone_numeric: 013, name: "Pulaski", wfo: "IWX" },
ForecastZone::IN014 => crate::ZoneDetails {state: "IN", zone: "014", zone_numeric: 014, name: "Marshall", wfo: "IWX" },
ForecastZone::IN015 => crate::ZoneDetails {state: "IN", zone: "015", zone_numeric: 015, name: "Fulton", wfo: "IWX" },
ForecastZone::IN016 => crate::ZoneDetails {state: "IN", zone: "016", zone_numeric: 016, name: "Kosciusko", wfo: "IWX" },
ForecastZone::IN017 => crate::ZoneDetails {state: "IN", zone: "017", zone_numeric: 017, name: "Whitley", wfo: "IWX" },
ForecastZone::IN018 => crate::ZoneDetails {state: "IN", zone: "018", zone_numeric: 018, name: "Allen", wfo: "IWX" },
ForecastZone::IN019 => crate::ZoneDetails {state: "IN", zone: "019", zone_numeric: 019, name: "Benton", wfo: "LOT" },
ForecastZone::IN020 => crate::ZoneDetails {state: "IN", zone: "020", zone_numeric: 020, name: "White", wfo: "IWX" },
ForecastZone::IN021 => crate::ZoneDetails {state: "IN", zone: "021", zone_numeric: 021, name: "Carroll", wfo: "IND" },
ForecastZone::IN022 => crate::ZoneDetails {state: "IN", zone: "022", zone_numeric: 022, name: "Cass", wfo: "IWX" },
ForecastZone::IN023 => crate::ZoneDetails {state: "IN", zone: "023", zone_numeric: 023, name: "Miami", wfo: "IWX" },
ForecastZone::IN024 => crate::ZoneDetails {state: "IN", zone: "024", zone_numeric: 024, name: "Wabash", wfo: "IWX" },
ForecastZone::IN025 => crate::ZoneDetails {state: "IN", zone: "025", zone_numeric: 025, name: "Huntington", wfo: "IWX" },
ForecastZone::IN026 => crate::ZoneDetails {state: "IN", zone: "026", zone_numeric: 026, name: "Wells", wfo: "IWX" },
ForecastZone::IN027 => crate::ZoneDetails {state: "IN", zone: "027", zone_numeric: 027, name: "Adams", wfo: "IWX" },
ForecastZone::IN028 => crate::ZoneDetails {state: "IN", zone: "028", zone_numeric: 028, name: "Warren", wfo: "IND" },
ForecastZone::IN029 => crate::ZoneDetails {state: "IN", zone: "029", zone_numeric: 029, name: "Tippecanoe", wfo: "IND" },
ForecastZone::IN030 => crate::ZoneDetails {state: "IN", zone: "030", zone_numeric: 030, name: "Clinton", wfo: "IND" },
ForecastZone::IN031 => crate::ZoneDetails {state: "IN", zone: "031", zone_numeric: 031, name: "Howard", wfo: "IND" },
ForecastZone::IN032 => crate::ZoneDetails {state: "IN", zone: "032", zone_numeric: 032, name: "Grant", wfo: "IWX" },
ForecastZone::IN033 => crate::ZoneDetails {state: "IN", zone: "033", zone_numeric: 033, name: "Blackford", wfo: "IWX" },
ForecastZone::IN034 => crate::ZoneDetails {state: "IN", zone: "034", zone_numeric: 034, name: "Jay", wfo: "IWX" },
ForecastZone::IN035 => crate::ZoneDetails {state: "IN", zone: "035", zone_numeric: 035, name: "Fountain", wfo: "IND" },
ForecastZone::IN036 => crate::ZoneDetails {state: "IN", zone: "036", zone_numeric: 036, name: "Montgomery", wfo: "IND" },
ForecastZone::IN037 => crate::ZoneDetails {state: "IN", zone: "037", zone_numeric: 037, name: "Boone", wfo: "IND" },
ForecastZone::IN038 => crate::ZoneDetails {state: "IN", zone: "038", zone_numeric: 038, name: "Tipton", wfo: "IND" },
ForecastZone::IN039 => crate::ZoneDetails {state: "IN", zone: "039", zone_numeric: 039, name: "Hamilton", wfo: "IND" },
ForecastZone::IN040 => crate::ZoneDetails {state: "IN", zone: "040", zone_numeric: 040, name: "Madison", wfo: "IND" },
ForecastZone::IN041 => crate::ZoneDetails {state: "IN", zone: "041", zone_numeric: 041, name: "Delaware", wfo: "IND" },
ForecastZone::IN042 => crate::ZoneDetails {state: "IN", zone: "042", zone_numeric: 042, name: "Randolph", wfo: "IND" },
ForecastZone::IN043 => crate::ZoneDetails {state: "IN", zone: "043", zone_numeric: 043, name: "Vermillion", wfo: "IND" },
ForecastZone::IN044 => crate::ZoneDetails {state: "IN", zone: "044", zone_numeric: 044, name: "Parke", wfo: "IND" },
ForecastZone::IN045 => crate::ZoneDetails {state: "IN", zone: "045", zone_numeric: 045, name: "Putnam", wfo: "IND" },
ForecastZone::IN046 => crate::ZoneDetails {state: "IN", zone: "046", zone_numeric: 046, name: "Hendricks", wfo: "IND" },
ForecastZone::IN047 => crate::ZoneDetails {state: "IN", zone: "047", zone_numeric: 047, name: "Marion", wfo: "IND" },
ForecastZone::IN048 => crate::ZoneDetails {state: "IN", zone: "048", zone_numeric: 048, name: "Hancock", wfo: "IND" },
ForecastZone::IN049 => crate::ZoneDetails {state: "IN", zone: "049", zone_numeric: 049, name: "Henry", wfo: "IND" },
ForecastZone::IN050 => crate::ZoneDetails {state: "IN", zone: "050", zone_numeric: 050, name: "Wayne", wfo: "ILN" },
ForecastZone::IN051 => crate::ZoneDetails {state: "IN", zone: "051", zone_numeric: 051, name: "Vigo", wfo: "IND" },
ForecastZone::IN052 => crate::ZoneDetails {state: "IN", zone: "052", zone_numeric: 052, name: "Clay", wfo: "IND" },
ForecastZone::IN053 => crate::ZoneDetails {state: "IN", zone: "053", zone_numeric: 053, name: "Owen", wfo: "IND" },
ForecastZone::IN054 => crate::ZoneDetails {state: "IN", zone: "054", zone_numeric: 054, name: "Morgan", wfo: "IND" },
ForecastZone::IN055 => crate::ZoneDetails {state: "IN", zone: "055", zone_numeric: 055, name: "Johnson", wfo: "IND" },
ForecastZone::IN056 => crate::ZoneDetails {state: "IN", zone: "056", zone_numeric: 056, name: "Shelby", wfo: "IND" },
ForecastZone::IN057 => crate::ZoneDetails {state: "IN", zone: "057", zone_numeric: 057, name: "Rush", wfo: "IND" },
ForecastZone::IN058 => crate::ZoneDetails {state: "IN", zone: "058", zone_numeric: 058, name: "Fayette", wfo: "ILN" },
ForecastZone::IN059 => crate::ZoneDetails {state: "IN", zone: "059", zone_numeric: 059, name: "Union", wfo: "ILN" },
ForecastZone::IN060 => crate::ZoneDetails {state: "IN", zone: "060", zone_numeric: 060, name: "Sullivan", wfo: "IND" },
ForecastZone::IN061 => crate::ZoneDetails {state: "IN", zone: "061", zone_numeric: 061, name: "Greene", wfo: "IND" },
ForecastZone::IN062 => crate::ZoneDetails {state: "IN", zone: "062", zone_numeric: 062, name: "Monroe", wfo: "IND" },
ForecastZone::IN063 => crate::ZoneDetails {state: "IN", zone: "063", zone_numeric: 063, name: "Brown", wfo: "IND" },
ForecastZone::IN064 => crate::ZoneDetails {state: "IN", zone: "064", zone_numeric: 064, name: "Bartholomew", wfo: "IND" },
ForecastZone::IN065 => crate::ZoneDetails {state: "IN", zone: "065", zone_numeric: 065, name: "Decatur", wfo: "IND" },
ForecastZone::IN066 => crate::ZoneDetails {state: "IN", zone: "066", zone_numeric: 066, name: "Franklin", wfo: "ILN" },
ForecastZone::IN067 => crate::ZoneDetails {state: "IN", zone: "067", zone_numeric: 067, name: "Knox", wfo: "IND" },
ForecastZone::IN068 => crate::ZoneDetails {state: "IN", zone: "068", zone_numeric: 068, name: "Daviess", wfo: "IND" },
ForecastZone::IN069 => crate::ZoneDetails {state: "IN", zone: "069", zone_numeric: 069, name: "Martin", wfo: "IND" },
ForecastZone::IN070 => crate::ZoneDetails {state: "IN", zone: "070", zone_numeric: 070, name: "Lawrence", wfo: "IND" },
ForecastZone::IN071 => crate::ZoneDetails {state: "IN", zone: "071", zone_numeric: 071, name: "Jackson", wfo: "IND" },
ForecastZone::IN072 => crate::ZoneDetails {state: "IN", zone: "072", zone_numeric: 072, name: "Jennings", wfo: "IND" },
ForecastZone::IN073 => crate::ZoneDetails {state: "IN", zone: "073", zone_numeric: 073, name: "Ripley", wfo: "ILN" },
ForecastZone::IN074 => crate::ZoneDetails {state: "IN", zone: "074", zone_numeric: 074, name: "Dearborn", wfo: "ILN" },
ForecastZone::IN075 => crate::ZoneDetails {state: "IN", zone: "075", zone_numeric: 075, name: "Ohio", wfo: "ILN" },
ForecastZone::IN076 => crate::ZoneDetails {state: "IN", zone: "076", zone_numeric: 076, name: "Orange", wfo: "LMK" },
ForecastZone::IN077 => crate::ZoneDetails {state: "IN", zone: "077", zone_numeric: 077, name: "Washington", wfo: "LMK" },
ForecastZone::IN078 => crate::ZoneDetails {state: "IN", zone: "078", zone_numeric: 078, name: "Scott", wfo: "LMK" },
ForecastZone::IN079 => crate::ZoneDetails {state: "IN", zone: "079", zone_numeric: 079, name: "Jefferson", wfo: "LMK" },
ForecastZone::IN080 => crate::ZoneDetails {state: "IN", zone: "080", zone_numeric: 080, name: "Switzerland", wfo: "ILN" },
ForecastZone::IN081 => crate::ZoneDetails {state: "IN", zone: "081", zone_numeric: 081, name: "Gibson", wfo: "PAH" },
ForecastZone::IN082 => crate::ZoneDetails {state: "IN", zone: "082", zone_numeric: 082, name: "Pike", wfo: "PAH" },
ForecastZone::IN083 => crate::ZoneDetails {state: "IN", zone: "083", zone_numeric: 083, name: "Dubois", wfo: "LMK" },
ForecastZone::IN084 => crate::ZoneDetails {state: "IN", zone: "084", zone_numeric: 084, name: "Crawford", wfo: "LMK" },
ForecastZone::IN085 => crate::ZoneDetails {state: "IN", zone: "085", zone_numeric: 085, name: "Posey", wfo: "PAH" },
ForecastZone::IN086 => crate::ZoneDetails {state: "IN", zone: "086", zone_numeric: 086, name: "Vanderburgh", wfo: "PAH" },
ForecastZone::IN087 => crate::ZoneDetails {state: "IN", zone: "087", zone_numeric: 087, name: "Warrick", wfo: "PAH" },
ForecastZone::IN088 => crate::ZoneDetails {state: "IN", zone: "088", zone_numeric: 088, name: "Spencer", wfo: "PAH" },
ForecastZone::IN089 => crate::ZoneDetails {state: "IN", zone: "089", zone_numeric: 089, name: "Perry", wfo: "LMK" },
ForecastZone::IN090 => crate::ZoneDetails {state: "IN", zone: "090", zone_numeric: 090, name: "Harrison", wfo: "LMK" },
ForecastZone::IN091 => crate::ZoneDetails {state: "IN", zone: "091", zone_numeric: 091, name: "Floyd", wfo: "LMK" },
ForecastZone::IN092 => crate::ZoneDetails {state: "IN", zone: "092", zone_numeric: 092, name: "Clark", wfo: "LMK" },
ForecastZone::KS001 => crate::ZoneDetails {state: "KS", zone: "001", zone_numeric: 001, name: "Cheyenne", wfo: "GLD" },
ForecastZone::KS002 => crate::ZoneDetails {state: "KS", zone: "002", zone_numeric: 002, name: "Rawlins", wfo: "GLD" },
ForecastZone::KS003 => crate::ZoneDetails {state: "KS", zone: "003", zone_numeric: 003, name: "Decatur", wfo: "GLD" },
ForecastZone::KS004 => crate::ZoneDetails {state: "KS", zone: "004", zone_numeric: 004, name: "Norton", wfo: "GLD" },
ForecastZone::KS005 => crate::ZoneDetails {state: "KS", zone: "005", zone_numeric: 005, name: "Phillips", wfo: "GID" },
ForecastZone::KS006 => crate::ZoneDetails {state: "KS", zone: "006", zone_numeric: 006, name: "Smith", wfo: "GID" },
ForecastZone::KS007 => crate::ZoneDetails {state: "KS", zone: "007", zone_numeric: 007, name: "Jewell", wfo: "GID" },
ForecastZone::KS008 => crate::ZoneDetails {state: "KS", zone: "008", zone_numeric: 008, name: "Republic", wfo: "TOP" },
ForecastZone::KS009 => crate::ZoneDetails {state: "KS", zone: "009", zone_numeric: 009, name: "Washington", wfo: "TOP" },
ForecastZone::KS010 => crate::ZoneDetails {state: "KS", zone: "010", zone_numeric: 010, name: "Marshall", wfo: "TOP" },
ForecastZone::KS011 => crate::ZoneDetails {state: "KS", zone: "011", zone_numeric: 011, name: "Nemaha", wfo: "TOP" },
ForecastZone::KS012 => crate::ZoneDetails {state: "KS", zone: "012", zone_numeric: 012, name: "Brown", wfo: "TOP" },
ForecastZone::KS013 => crate::ZoneDetails {state: "KS", zone: "013", zone_numeric: 013, name: "Sherman", wfo: "GLD" },
ForecastZone::KS014 => crate::ZoneDetails {state: "KS", zone: "014", zone_numeric: 014, name: "Thomas", wfo: "GLD" },
ForecastZone::KS015 => crate::ZoneDetails {state: "KS", zone: "015", zone_numeric: 015, name: "Sheridan", wfo: "GLD" },
ForecastZone::KS016 => crate::ZoneDetails {state: "KS", zone: "016", zone_numeric: 016, name: "Graham", wfo: "GLD" },
ForecastZone::KS017 => crate::ZoneDetails {state: "KS", zone: "017", zone_numeric: 017, name: "Rooks", wfo: "GID" },
ForecastZone::KS018 => crate::ZoneDetails {state: "KS", zone: "018", zone_numeric: 018, name: "Osborne", wfo: "GID" },
ForecastZone::KS019 => crate::ZoneDetails {state: "KS", zone: "019", zone_numeric: 019, name: "Mitchell", wfo: "GID" },
ForecastZone::KS020 => crate::ZoneDetails {state: "KS", zone: "020", zone_numeric: 020, name: "Cloud", wfo: "TOP" },
ForecastZone::KS021 => crate::ZoneDetails {state: "KS", zone: "021", zone_numeric: 021, name: "Clay", wfo: "TOP" },
ForecastZone::KS022 => crate::ZoneDetails {state: "KS", zone: "022", zone_numeric: 022, name: "Riley", wfo: "TOP" },
ForecastZone::KS023 => crate::ZoneDetails {state: "KS", zone: "023", zone_numeric: 023, name: "Pottawatomie", wfo: "TOP" },
ForecastZone::KS024 => crate::ZoneDetails {state: "KS", zone: "024", zone_numeric: 024, name: "Jackson", wfo: "TOP" },
ForecastZone::KS025 => crate::ZoneDetails {state: "KS", zone: "025", zone_numeric: 025, name: "Atchison", wfo: "EAX" },
ForecastZone::KS026 => crate::ZoneDetails {state: "KS", zone: "026", zone_numeric: 026, name: "Jefferson", wfo: "TOP" },
ForecastZone::KS027 => crate::ZoneDetails {state: "KS", zone: "027", zone_numeric: 027, name: "Wallace", wfo: "GLD" },
ForecastZone::KS028 => crate::ZoneDetails {state: "KS", zone: "028", zone_numeric: 028, name: "Logan", wfo: "GLD" },
ForecastZone::KS029 => crate::ZoneDetails {state: "KS", zone: "029", zone_numeric: 029, name: "Gove", wfo: "GLD" },
ForecastZone::KS030 => crate::ZoneDetails {state: "KS", zone: "030", zone_numeric: 030, name: "Trego", wfo: "DDC" },
ForecastZone::KS031 => crate::ZoneDetails {state: "KS", zone: "031", zone_numeric: 031, name: "Ellis", wfo: "DDC" },
ForecastZone::KS032 => crate::ZoneDetails {state: "KS", zone: "032", zone_numeric: 032, name: "Russell", wfo: "ICT" },
ForecastZone::KS033 => crate::ZoneDetails {state: "KS", zone: "033", zone_numeric: 033, name: "Lincoln", wfo: "ICT" },
ForecastZone::KS034 => crate::ZoneDetails {state: "KS", zone: "034", zone_numeric: 034, name: "Ottawa", wfo: "TOP" },
ForecastZone::KS035 => crate::ZoneDetails {state: "KS", zone: "035", zone_numeric: 035, name: "Dickinson", wfo: "TOP" },
ForecastZone::KS036 => crate::ZoneDetails {state: "KS", zone: "036", zone_numeric: 036, name: "Geary", wfo: "TOP" },
ForecastZone::KS037 => crate::ZoneDetails {state: "KS", zone: "037", zone_numeric: 037, name: "Morris", wfo: "TOP" },
ForecastZone::KS038 => crate::ZoneDetails {state: "KS", zone: "038", zone_numeric: 038, name: "Wabaunsee", wfo: "TOP" },
ForecastZone::KS039 => crate::ZoneDetails {state: "KS", zone: "039", zone_numeric: 039, name: "Shawnee", wfo: "TOP" },
ForecastZone::KS040 => crate::ZoneDetails {state: "KS", zone: "040", zone_numeric: 040, name: "Douglas", wfo: "TOP" },
ForecastZone::KS041 => crate::ZoneDetails {state: "KS", zone: "041", zone_numeric: 041, name: "Greeley", wfo: "GLD" },
ForecastZone::KS042 => crate::ZoneDetails {state: "KS", zone: "042", zone_numeric: 042, name: "Wichita", wfo: "GLD" },
ForecastZone::KS043 => crate::ZoneDetails {state: "KS", zone: "043", zone_numeric: 043, name: "Scott", wfo: "DDC" },
ForecastZone::KS044 => crate::ZoneDetails {state: "KS", zone: "044", zone_numeric: 044, name: "Lane", wfo: "DDC" },
ForecastZone::KS045 => crate::ZoneDetails {state: "KS", zone: "045", zone_numeric: 045, name: "Ness", wfo: "DDC" },
ForecastZone::KS046 => crate::ZoneDetails {state: "KS", zone: "046", zone_numeric: 046, name: "Rush", wfo: "DDC" },
ForecastZone::KS047 => crate::ZoneDetails {state: "KS", zone: "047", zone_numeric: 047, name: "Barton", wfo: "ICT" },
ForecastZone::KS048 => crate::ZoneDetails {state: "KS", zone: "048", zone_numeric: 048, name: "Ellsworth", wfo: "ICT" },
ForecastZone::KS049 => crate::ZoneDetails {state: "KS", zone: "049", zone_numeric: 049, name: "Saline", wfo: "ICT" },
ForecastZone::KS050 => crate::ZoneDetails {state: "KS", zone: "050", zone_numeric: 050, name: "Rice", wfo: "ICT" },
ForecastZone::KS051 => crate::ZoneDetails {state: "KS", zone: "051", zone_numeric: 051, name: "McPherson", wfo: "ICT" },
ForecastZone::KS052 => crate::ZoneDetails {state: "KS", zone: "052", zone_numeric: 052, name: "Marion", wfo: "ICT" },
ForecastZone::KS053 => crate::ZoneDetails {state: "KS", zone: "053", zone_numeric: 053, name: "Chase", wfo: "ICT" },
ForecastZone::KS054 => crate::ZoneDetails {state: "KS", zone: "054", zone_numeric: 054, name: "Lyon", wfo: "TOP" },
ForecastZone::KS055 => crate::ZoneDetails {state: "KS", zone: "055", zone_numeric: 055, name: "Osage", wfo: "TOP" },
ForecastZone::KS056 => crate::ZoneDetails {state: "KS", zone: "056", zone_numeric: 056, name: "Franklin", wfo: "TOP" },
ForecastZone::KS057 => crate::ZoneDetails {state: "KS", zone: "057", zone_numeric: 057, name: "Miami", wfo: "EAX" },
ForecastZone::KS058 => crate::ZoneDetails {state: "KS", zone: "058", zone_numeric: 058, name: "Coffey", wfo: "TOP" },
ForecastZone::KS059 => crate::ZoneDetails {state: "KS", zone: "059", zone_numeric: 059, name: "Anderson", wfo: "TOP" },
ForecastZone::KS060 => crate::ZoneDetails {state: "KS", zone: "060", zone_numeric: 060, name: "Linn", wfo: "EAX" },
ForecastZone::KS061 => crate::ZoneDetails {state: "KS", zone: "061", zone_numeric: 061, name: "Hamilton", wfo: "DDC" },
ForecastZone::KS062 => crate::ZoneDetails {state: "KS", zone: "062", zone_numeric: 062, name: "Kearny", wfo: "DDC" },
ForecastZone::KS063 => crate::ZoneDetails {state: "KS", zone: "063", zone_numeric: 063, name: "Finney", wfo: "DDC" },
ForecastZone::KS064 => crate::ZoneDetails {state: "KS", zone: "064", zone_numeric: 064, name: "Hodgeman", wfo: "DDC" },
ForecastZone::KS065 => crate::ZoneDetails {state: "KS", zone: "065", zone_numeric: 065, name: "Pawnee", wfo: "DDC" },
ForecastZone::KS066 => crate::ZoneDetails {state: "KS", zone: "066", zone_numeric: 066, name: "Stafford", wfo: "DDC" },
ForecastZone::KS067 => crate::ZoneDetails {state: "KS", zone: "067", zone_numeric: 067, name: "Reno", wfo: "ICT" },
ForecastZone::KS068 => crate::ZoneDetails {state: "KS", zone: "068", zone_numeric: 068, name: "Harvey", wfo: "ICT" },
ForecastZone::KS069 => crate::ZoneDetails {state: "KS", zone: "069", zone_numeric: 069, name: "Butler", wfo: "ICT" },
ForecastZone::KS070 => crate::ZoneDetails {state: "KS", zone: "070", zone_numeric: 070, name: "Greenwood", wfo: "ICT" },
ForecastZone::KS071 => crate::ZoneDetails {state: "KS", zone: "071", zone_numeric: 071, name: "Woodson", wfo: "ICT" },
ForecastZone::KS072 => crate::ZoneDetails {state: "KS", zone: "072", zone_numeric: 072, name: "Allen", wfo: "ICT" },
ForecastZone::KS073 => crate::ZoneDetails {state: "KS", zone: "073", zone_numeric: 073, name: "Bourbon", wfo: "SGF" },
ForecastZone::KS074 => crate::ZoneDetails {state: "KS", zone: "074", zone_numeric: 074, name: "Stanton", wfo: "DDC" },
ForecastZone::KS075 => crate::ZoneDetails {state: "KS", zone: "075", zone_numeric: 075, name: "Grant", wfo: "DDC" },
ForecastZone::KS076 => crate::ZoneDetails {state: "KS", zone: "076", zone_numeric: 076, name: "Haskell", wfo: "DDC" },
ForecastZone::KS077 => crate::ZoneDetails {state: "KS", zone: "077", zone_numeric: 077, name: "Gray", wfo: "DDC" },
ForecastZone::KS078 => crate::ZoneDetails {state: "KS", zone: "078", zone_numeric: 078, name: "Ford", wfo: "DDC" },
ForecastZone::KS079 => crate::ZoneDetails {state: "KS", zone: "079", zone_numeric: 079, name: "Edwards", wfo: "DDC" },
ForecastZone::KS080 => crate::ZoneDetails {state: "KS", zone: "080", zone_numeric: 080, name: "Kiowa", wfo: "DDC" },
ForecastZone::KS081 => crate::ZoneDetails {state: "KS", zone: "081", zone_numeric: 081, name: "Pratt", wfo: "DDC" },
ForecastZone::KS082 => crate::ZoneDetails {state: "KS", zone: "082", zone_numeric: 082, name: "Kingman", wfo: "ICT" },
ForecastZone::KS083 => crate::ZoneDetails {state: "KS", zone: "083", zone_numeric: 083, name: "Sedgwick", wfo: "ICT" },
ForecastZone::KS084 => crate::ZoneDetails {state: "KS", zone: "084", zone_numeric: 084, name: "Morton", wfo: "DDC" },
ForecastZone::KS085 => crate::ZoneDetails {state: "KS", zone: "085", zone_numeric: 085, name: "Stevens", wfo: "DDC" },
ForecastZone::KS086 => crate::ZoneDetails {state: "KS", zone: "086", zone_numeric: 086, name: "Seward", wfo: "DDC" },
ForecastZone::KS087 => crate::ZoneDetails {state: "KS", zone: "087", zone_numeric: 087, name: "Meade", wfo: "DDC" },
ForecastZone::KS088 => crate::ZoneDetails {state: "KS", zone: "088", zone_numeric: 088, name: "Clark", wfo: "DDC" },
ForecastZone::KS089 => crate::ZoneDetails {state: "KS", zone: "089", zone_numeric: 089, name: "Comanche", wfo: "DDC" },
ForecastZone::KS090 => crate::ZoneDetails {state: "KS", zone: "090", zone_numeric: 090, name: "Barber", wfo: "DDC" },
ForecastZone::KS091 => crate::ZoneDetails {state: "KS", zone: "091", zone_numeric: 091, name: "Harper", wfo: "ICT" },
ForecastZone::KS092 => crate::ZoneDetails {state: "KS", zone: "092", zone_numeric: 092, name: "Sumner", wfo: "ICT" },
ForecastZone::KS093 => crate::ZoneDetails {state: "KS", zone: "093", zone_numeric: 093, name: "Cowley", wfo: "ICT" },
ForecastZone::KS094 => crate::ZoneDetails {state: "KS", zone: "094", zone_numeric: 094, name: "Elk", wfo: "ICT" },
ForecastZone::KS095 => crate::ZoneDetails {state: "KS", zone: "095", zone_numeric: 095, name: "Wilson", wfo: "ICT" },
ForecastZone::KS096 => crate::ZoneDetails {state: "KS", zone: "096", zone_numeric: 096, name: "Neosho", wfo: "ICT" },
ForecastZone::KS097 => crate::ZoneDetails {state: "KS", zone: "097", zone_numeric: 097, name: "Crawford", wfo: "SGF" },
ForecastZone::KS098 => crate::ZoneDetails {state: "KS", zone: "098", zone_numeric: 098, name: "Chautauqua", wfo: "ICT" },
ForecastZone::KS099 => crate::ZoneDetails {state: "KS", zone: "099", zone_numeric: 099, name: "Montgomery", wfo: "ICT" },
ForecastZone::KS100 => crate::ZoneDetails {state: "KS", zone: "100", zone_numeric: 100, name: "Labette", wfo: "ICT" },
ForecastZone::KS101 => crate::ZoneDetails {state: "KS", zone: "101", zone_numeric: 101, name: "Cherokee", wfo: "SGF" },
ForecastZone::KS102 => crate::ZoneDetails {state: "KS", zone: "102", zone_numeric: 102, name: "Doniphan", wfo: "EAX" },
ForecastZone::KS103 => crate::ZoneDetails {state: "KS", zone: "103", zone_numeric: 103, name: "Leavenworth", wfo: "EAX" },
ForecastZone::KS104 => crate::ZoneDetails {state: "KS", zone: "104", zone_numeric: 104, name: "Wyandotte", wfo: "EAX" },
ForecastZone::KS105 => crate::ZoneDetails {state: "KS", zone: "105", zone_numeric: 105, name: "Johnson", wfo: "EAX" },
ForecastZone::KY001 => crate::ZoneDetails {state: "KY", zone: "001", zone_numeric: 001, name: "Fulton", wfo: "PAH" },
ForecastZone::KY002 => crate::ZoneDetails {state: "KY", zone: "002", zone_numeric: 002, name: "Hickman", wfo: "PAH" },
ForecastZone::KY003 => crate::ZoneDetails {state: "KY", zone: "003", zone_numeric: 003, name: "Carlisle", wfo: "PAH" },
ForecastZone::KY004 => crate::ZoneDetails {state: "KY", zone: "004", zone_numeric: 004, name: "Ballard", wfo: "PAH" },
ForecastZone::KY005 => crate::ZoneDetails {state: "KY", zone: "005", zone_numeric: 005, name: "McCracken", wfo: "PAH" },
ForecastZone::KY006 => crate::ZoneDetails {state: "KY", zone: "006", zone_numeric: 006, name: "Graves", wfo: "PAH" },
ForecastZone::KY007 => crate::ZoneDetails {state: "KY", zone: "007", zone_numeric: 007, name: "Livingston", wfo: "PAH" },
ForecastZone::KY008 => crate::ZoneDetails {state: "KY", zone: "008", zone_numeric: 008, name: "Marshall", wfo: "PAH" },
ForecastZone::KY009 => crate::ZoneDetails {state: "KY", zone: "009", zone_numeric: 009, name: "Calloway", wfo: "PAH" },
ForecastZone::KY010 => crate::ZoneDetails {state: "KY", zone: "010", zone_numeric: 010, name: "Crittenden", wfo: "PAH" },
ForecastZone::KY011 => crate::ZoneDetails {state: "KY", zone: "011", zone_numeric: 011, name: "Lyon", wfo: "PAH" },
ForecastZone::KY012 => crate::ZoneDetails {state: "KY", zone: "012", zone_numeric: 012, name: "Trigg", wfo: "PAH" },
ForecastZone::KY013 => crate::ZoneDetails {state: "KY", zone: "013", zone_numeric: 013, name: "Caldwell", wfo: "PAH" },
ForecastZone::KY014 => crate::ZoneDetails {state: "KY", zone: "014", zone_numeric: 014, name: "Union", wfo: "PAH" },
ForecastZone::KY015 => crate::ZoneDetails {state: "KY", zone: "015", zone_numeric: 015, name: "Webster", wfo: "PAH" },
ForecastZone::KY016 => crate::ZoneDetails {state: "KY", zone: "016", zone_numeric: 016, name: "Hopkins", wfo: "PAH" },
ForecastZone::KY017 => crate::ZoneDetails {state: "KY", zone: "017", zone_numeric: 017, name: "Christian", wfo: "PAH" },
ForecastZone::KY018 => crate::ZoneDetails {state: "KY", zone: "018", zone_numeric: 018, name: "Henderson", wfo: "PAH" },
ForecastZone::KY019 => crate::ZoneDetails {state: "KY", zone: "019", zone_numeric: 019, name: "Daviess", wfo: "PAH" },
ForecastZone::KY020 => crate::ZoneDetails {state: "KY", zone: "020", zone_numeric: 020, name: "McLean", wfo: "PAH" },
ForecastZone::KY021 => crate::ZoneDetails {state: "KY", zone: "021", zone_numeric: 021, name: "Muhlenberg", wfo: "PAH" },
ForecastZone::KY022 => crate::ZoneDetails {state: "KY", zone: "022", zone_numeric: 022, name: "Todd", wfo: "PAH" },
ForecastZone::KY023 => crate::ZoneDetails {state: "KY", zone: "023", zone_numeric: 023, name: "Hancock", wfo: "LMK" },
ForecastZone::KY024 => crate::ZoneDetails {state: "KY", zone: "024", zone_numeric: 024, name: "Breckinridge", wfo: "LMK" },
ForecastZone::KY025 => crate::ZoneDetails {state: "KY", zone: "025", zone_numeric: 025, name: "Meade", wfo: "LMK" },
ForecastZone::KY026 => crate::ZoneDetails {state: "KY", zone: "026", zone_numeric: 026, name: "Ohio", wfo: "LMK" },
ForecastZone::KY027 => crate::ZoneDetails {state: "KY", zone: "027", zone_numeric: 027, name: "Grayson", wfo: "LMK" },
ForecastZone::KY028 => crate::ZoneDetails {state: "KY", zone: "028", zone_numeric: 028, name: "Hardin", wfo: "LMK" },
ForecastZone::KY029 => crate::ZoneDetails {state: "KY", zone: "029", zone_numeric: 029, name: "Bullitt", wfo: "LMK" },
ForecastZone::KY030 => crate::ZoneDetails {state: "KY", zone: "030", zone_numeric: 030, name: "Jefferson", wfo: "LMK" },
ForecastZone::KY031 => crate::ZoneDetails {state: "KY", zone: "031", zone_numeric: 031, name: "Oldham", wfo: "LMK" },
ForecastZone::KY032 => crate::ZoneDetails {state: "KY", zone: "032", zone_numeric: 032, name: "Trimble", wfo: "LMK" },
ForecastZone::KY033 => crate::ZoneDetails {state: "KY", zone: "033", zone_numeric: 033, name: "Henry", wfo: "LMK" },
ForecastZone::KY034 => crate::ZoneDetails {state: "KY", zone: "034", zone_numeric: 034, name: "Shelby", wfo: "LMK" },
ForecastZone::KY035 => crate::ZoneDetails {state: "KY", zone: "035", zone_numeric: 035, name: "Franklin", wfo: "LMK" },
ForecastZone::KY036 => crate::ZoneDetails {state: "KY", zone: "036", zone_numeric: 036, name: "Scott", wfo: "LMK" },
ForecastZone::KY037 => crate::ZoneDetails {state: "KY", zone: "037", zone_numeric: 037, name: "Harrison", wfo: "LMK" },
ForecastZone::KY038 => crate::ZoneDetails {state: "KY", zone: "038", zone_numeric: 038, name: "Spencer", wfo: "LMK" },
ForecastZone::KY039 => crate::ZoneDetails {state: "KY", zone: "039", zone_numeric: 039, name: "Anderson", wfo: "LMK" },
ForecastZone::KY040 => crate::ZoneDetails {state: "KY", zone: "040", zone_numeric: 040, name: "Woodford", wfo: "LMK" },
ForecastZone::KY041 => crate::ZoneDetails {state: "KY", zone: "041", zone_numeric: 041, name: "Fayette", wfo: "LMK" },
ForecastZone::KY042 => crate::ZoneDetails {state: "KY", zone: "042", zone_numeric: 042, name: "Bourbon", wfo: "LMK" },
ForecastZone::KY043 => crate::ZoneDetails {state: "KY", zone: "043", zone_numeric: 043, name: "Nicholas", wfo: "LMK" },
ForecastZone::KY044 => crate::ZoneDetails {state: "KY", zone: "044", zone_numeric: 044, name: "Fleming", wfo: "JKL" },
ForecastZone::KY045 => crate::ZoneDetails {state: "KY", zone: "045", zone_numeric: 045, name: "Nelson", wfo: "LMK" },
ForecastZone::KY046 => crate::ZoneDetails {state: "KY", zone: "046", zone_numeric: 046, name: "Washington", wfo: "LMK" },
ForecastZone::KY047 => crate::ZoneDetails {state: "KY", zone: "047", zone_numeric: 047, name: "Mercer", wfo: "LMK" },
ForecastZone::KY048 => crate::ZoneDetails {state: "KY", zone: "048", zone_numeric: 048, name: "Jessamine", wfo: "LMK" },
ForecastZone::KY049 => crate::ZoneDetails {state: "KY", zone: "049", zone_numeric: 049, name: "Clark", wfo: "LMK" },
ForecastZone::KY050 => crate::ZoneDetails {state: "KY", zone: "050", zone_numeric: 050, name: "Montgomery", wfo: "JKL" },
ForecastZone::KY051 => crate::ZoneDetails {state: "KY", zone: "051", zone_numeric: 051, name: "Bath", wfo: "JKL" },
ForecastZone::KY052 => crate::ZoneDetails {state: "KY", zone: "052", zone_numeric: 052, name: "Rowan", wfo: "JKL" },
ForecastZone::KY053 => crate::ZoneDetails {state: "KY", zone: "053", zone_numeric: 053, name: "Larue", wfo: "LMK" },
ForecastZone::KY054 => crate::ZoneDetails {state: "KY", zone: "054", zone_numeric: 054, name: "Marion", wfo: "LMK" },
ForecastZone::KY055 => crate::ZoneDetails {state: "KY", zone: "055", zone_numeric: 055, name: "Boyle", wfo: "LMK" },
ForecastZone::KY056 => crate::ZoneDetails {state: "KY", zone: "056", zone_numeric: 056, name: "Garrard", wfo: "LMK" },
ForecastZone::KY057 => crate::ZoneDetails {state: "KY", zone: "057", zone_numeric: 057, name: "Madison", wfo: "LMK" },
ForecastZone::KY058 => crate::ZoneDetails {state: "KY", zone: "058", zone_numeric: 058, name: "Estill", wfo: "JKL" },
ForecastZone::KY059 => crate::ZoneDetails {state: "KY", zone: "059", zone_numeric: 059, name: "Powell", wfo: "JKL" },
ForecastZone::KY060 => crate::ZoneDetails {state: "KY", zone: "060", zone_numeric: 060, name: "Menifee", wfo: "JKL" },
ForecastZone::KY061 => crate::ZoneDetails {state: "KY", zone: "061", zone_numeric: 061, name: "Butler", wfo: "LMK" },
ForecastZone::KY062 => crate::ZoneDetails {state: "KY", zone: "062", zone_numeric: 062, name: "Edmonson", wfo: "LMK" },
ForecastZone::KY063 => crate::ZoneDetails {state: "KY", zone: "063", zone_numeric: 063, name: "Hart", wfo: "LMK" },
ForecastZone::KY064 => crate::ZoneDetails {state: "KY", zone: "064", zone_numeric: 064, name: "Green", wfo: "LMK" },
ForecastZone::KY065 => crate::ZoneDetails {state: "KY", zone: "065", zone_numeric: 065, name: "Taylor", wfo: "LMK" },
ForecastZone::KY066 => crate::ZoneDetails {state: "KY", zone: "066", zone_numeric: 066, name: "Casey", wfo: "LMK" },
ForecastZone::KY067 => crate::ZoneDetails {state: "KY", zone: "067", zone_numeric: 067, name: "Lincoln", wfo: "LMK" },
ForecastZone::KY068 => crate::ZoneDetails {state: "KY", zone: "068", zone_numeric: 068, name: "Rockcastle", wfo: "JKL" },
ForecastZone::KY069 => crate::ZoneDetails {state: "KY", zone: "069", zone_numeric: 069, name: "Jackson", wfo: "JKL" },
ForecastZone::KY070 => crate::ZoneDetails {state: "KY", zone: "070", zone_numeric: 070, name: "Logan", wfo: "LMK" },
ForecastZone::KY071 => crate::ZoneDetails {state: "KY", zone: "071", zone_numeric: 071, name: "Warren", wfo: "LMK" },
ForecastZone::KY072 => crate::ZoneDetails {state: "KY", zone: "072", zone_numeric: 072, name: "Simpson", wfo: "LMK" },
ForecastZone::KY073 => crate::ZoneDetails {state: "KY", zone: "073", zone_numeric: 073, name: "Allen", wfo: "LMK" },
ForecastZone::KY074 => crate::ZoneDetails {state: "KY", zone: "074", zone_numeric: 074, name: "Barren", wfo: "LMK" },
ForecastZone::KY075 => crate::ZoneDetails {state: "KY", zone: "075", zone_numeric: 075, name: "Monroe", wfo: "LMK" },
ForecastZone::KY076 => crate::ZoneDetails {state: "KY", zone: "076", zone_numeric: 076, name: "Metcalfe", wfo: "LMK" },
ForecastZone::KY077 => crate::ZoneDetails {state: "KY", zone: "077", zone_numeric: 077, name: "Adair", wfo: "LMK" },
ForecastZone::KY078 => crate::ZoneDetails {state: "KY", zone: "078", zone_numeric: 078, name: "Russell", wfo: "LMK" },
ForecastZone::KY079 => crate::ZoneDetails {state: "KY", zone: "079", zone_numeric: 079, name: "Pulaski", wfo: "JKL" },
ForecastZone::KY080 => crate::ZoneDetails {state: "KY", zone: "080", zone_numeric: 080, name: "Laurel", wfo: "JKL" },
ForecastZone::KY081 => crate::ZoneDetails {state: "KY", zone: "081", zone_numeric: 081, name: "Cumberland", wfo: "LMK" },
ForecastZone::KY082 => crate::ZoneDetails {state: "KY", zone: "082", zone_numeric: 082, name: "Clinton", wfo: "LMK" },
ForecastZone::KY083 => crate::ZoneDetails {state: "KY", zone: "083", zone_numeric: 083, name: "Wayne", wfo: "JKL" },
ForecastZone::KY084 => crate::ZoneDetails {state: "KY", zone: "084", zone_numeric: 084, name: "McCreary", wfo: "JKL" },
ForecastZone::KY085 => crate::ZoneDetails {state: "KY", zone: "085", zone_numeric: 085, name: "Whitley", wfo: "JKL" },
ForecastZone::KY086 => crate::ZoneDetails {state: "KY", zone: "086", zone_numeric: 086, name: "Knox", wfo: "JKL" },
ForecastZone::KY087 => crate::ZoneDetails {state: "KY", zone: "087", zone_numeric: 087, name: "Bell", wfo: "JKL" },
ForecastZone::KY088 => crate::ZoneDetails {state: "KY", zone: "088", zone_numeric: 088, name: "Harlan", wfo: "JKL" },
ForecastZone::KY089 => crate::ZoneDetails {state: "KY", zone: "089", zone_numeric: 089, name: "Carroll", wfo: "ILN" },
ForecastZone::KY090 => crate::ZoneDetails {state: "KY", zone: "090", zone_numeric: 090, name: "Gallatin", wfo: "ILN" },
ForecastZone::KY091 => crate::ZoneDetails {state: "KY", zone: "091", zone_numeric: 091, name: "Boone", wfo: "ILN" },
ForecastZone::KY092 => crate::ZoneDetails {state: "KY", zone: "092", zone_numeric: 092, name: "Kenton", wfo: "ILN" },
ForecastZone::KY093 => crate::ZoneDetails {state: "KY", zone: "093", zone_numeric: 093, name: "Campbell", wfo: "ILN" },
ForecastZone::KY094 => crate::ZoneDetails {state: "KY", zone: "094", zone_numeric: 094, name: "Owen", wfo: "ILN" },
ForecastZone::KY095 => crate::ZoneDetails {state: "KY", zone: "095", zone_numeric: 095, name: "Grant", wfo: "ILN" },
ForecastZone::KY096 => crate::ZoneDetails {state: "KY", zone: "096", zone_numeric: 096, name: "Pendleton", wfo: "ILN" },
ForecastZone::KY097 => crate::ZoneDetails {state: "KY", zone: "097", zone_numeric: 097, name: "Bracken", wfo: "ILN" },
ForecastZone::KY098 => crate::ZoneDetails {state: "KY", zone: "098", zone_numeric: 098, name: "Robertson", wfo: "ILN" },
ForecastZone::KY099 => crate::ZoneDetails {state: "KY", zone: "099", zone_numeric: 099, name: "Mason", wfo: "ILN" },
ForecastZone::KY100 => crate::ZoneDetails {state: "KY", zone: "100", zone_numeric: 100, name: "Lewis", wfo: "ILN" },
ForecastZone::KY101 => crate::ZoneDetails {state: "KY", zone: "101", zone_numeric: 101, name: "Greenup", wfo: "RLX" },
ForecastZone::KY102 => crate::ZoneDetails {state: "KY", zone: "102", zone_numeric: 102, name: "Carter", wfo: "RLX" },
ForecastZone::KY103 => crate::ZoneDetails {state: "KY", zone: "103", zone_numeric: 103, name: "Boyd", wfo: "RLX" },
ForecastZone::KY104 => crate::ZoneDetails {state: "KY", zone: "104", zone_numeric: 104, name: "Elliott", wfo: "JKL" },
ForecastZone::KY105 => crate::ZoneDetails {state: "KY", zone: "105", zone_numeric: 105, name: "Lawrence", wfo: "RLX" },
ForecastZone::KY106 => crate::ZoneDetails {state: "KY", zone: "106", zone_numeric: 106, name: "Morgan", wfo: "JKL" },
ForecastZone::KY107 => crate::ZoneDetails {state: "KY", zone: "107", zone_numeric: 107, name: "Johnson", wfo: "JKL" },
ForecastZone::KY108 => crate::ZoneDetails {state: "KY", zone: "108", zone_numeric: 108, name: "Wolfe", wfo: "JKL" },
ForecastZone::KY109 => crate::ZoneDetails {state: "KY", zone: "109", zone_numeric: 109, name: "Magoffin", wfo: "JKL" },
ForecastZone::KY110 => crate::ZoneDetails {state: "KY", zone: "110", zone_numeric: 110, name: "Floyd", wfo: "JKL" },
ForecastZone::KY111 => crate::ZoneDetails {state: "KY", zone: "111", zone_numeric: 111, name: "Lee", wfo: "JKL" },
ForecastZone::KY112 => crate::ZoneDetails {state: "KY", zone: "112", zone_numeric: 112, name: "Breathitt", wfo: "JKL" },
ForecastZone::KY113 => crate::ZoneDetails {state: "KY", zone: "113", zone_numeric: 113, name: "Knott", wfo: "JKL" },
ForecastZone::KY114 => crate::ZoneDetails {state: "KY", zone: "114", zone_numeric: 114, name: "Owsley", wfo: "JKL" },
ForecastZone::KY115 => crate::ZoneDetails {state: "KY", zone: "115", zone_numeric: 115, name: "Perry", wfo: "JKL" },
ForecastZone::KY116 => crate::ZoneDetails {state: "KY", zone: "116", zone_numeric: 116, name: "Clay", wfo: "JKL" },
ForecastZone::KY117 => crate::ZoneDetails {state: "KY", zone: "117", zone_numeric: 117, name: "Leslie", wfo: "JKL" },
ForecastZone::KY118 => crate::ZoneDetails {state: "KY", zone: "118", zone_numeric: 118, name: "Letcher", wfo: "JKL" },
ForecastZone::KY119 => crate::ZoneDetails {state: "KY", zone: "119", zone_numeric: 119, name: "Martin", wfo: "JKL" },
ForecastZone::KY120 => crate::ZoneDetails {state: "KY", zone: "120", zone_numeric: 120, name: "Pike", wfo: "JKL" },
ForecastZone::LA001 => crate::ZoneDetails {state: "LA", zone: "001", zone_numeric: 001, name: "Caddo", wfo: "SHV" },
ForecastZone::LA002 => crate::ZoneDetails {state: "LA", zone: "002", zone_numeric: 002, name: "Bossier", wfo: "SHV" },
ForecastZone::LA003 => crate::ZoneDetails {state: "LA", zone: "003", zone_numeric: 003, name: "Webster", wfo: "SHV" },
ForecastZone::LA004 => crate::ZoneDetails {state: "LA", zone: "004", zone_numeric: 004, name: "Claiborne", wfo: "SHV" },
ForecastZone::LA005 => crate::ZoneDetails {state: "LA", zone: "005", zone_numeric: 005, name: "Lincoln", wfo: "SHV" },
ForecastZone::LA006 => crate::ZoneDetails {state: "LA", zone: "006", zone_numeric: 006, name: "Union", wfo: "SHV" },
ForecastZone::LA007 => crate::ZoneDetails {state: "LA", zone: "007", zone_numeric: 007, name: "Morehouse", wfo: "JAN" },
ForecastZone::LA008 => crate::ZoneDetails {state: "LA", zone: "008", zone_numeric: 008, name: "West Carroll", wfo: "JAN" },
ForecastZone::LA009 => crate::ZoneDetails {state: "LA", zone: "009", zone_numeric: 009, name: "East Carroll", wfo: "JAN" },
ForecastZone::LA010 => crate::ZoneDetails {state: "LA", zone: "010", zone_numeric: 010, name: "De Soto", wfo: "SHV" },
ForecastZone::LA011 => crate::ZoneDetails {state: "LA", zone: "011", zone_numeric: 011, name: "Red River", wfo: "SHV" },
ForecastZone::LA012 => crate::ZoneDetails {state: "LA", zone: "012", zone_numeric: 012, name: "Bienville", wfo: "SHV" },
ForecastZone::LA013 => crate::ZoneDetails {state: "LA", zone: "013", zone_numeric: 013, name: "Jackson", wfo: "SHV" },
ForecastZone::LA014 => crate::ZoneDetails {state: "LA", zone: "014", zone_numeric: 014, name: "Ouachita", wfo: "SHV" },
ForecastZone::LA015 => crate::ZoneDetails {state: "LA", zone: "015", zone_numeric: 015, name: "Richland", wfo: "JAN" },
ForecastZone::LA016 => crate::ZoneDetails {state: "LA", zone: "016", zone_numeric: 016, name: "Madison", wfo: "JAN" },
ForecastZone::LA017 => crate::ZoneDetails {state: "LA", zone: "017", zone_numeric: 017, name: "Sabine", wfo: "SHV" },
ForecastZone::LA018 => crate::ZoneDetails {state: "LA", zone: "018", zone_numeric: 018, name: "Natchitoches", wfo: "SHV" },
ForecastZone::LA019 => crate::ZoneDetails {state: "LA", zone: "019", zone_numeric: 019, name: "Winn", wfo: "SHV" },
ForecastZone::LA020 => crate::ZoneDetails {state: "LA", zone: "020", zone_numeric: 020, name: "Grant", wfo: "SHV" },
ForecastZone::LA021 => crate::ZoneDetails {state: "LA", zone: "021", zone_numeric: 021, name: "Caldwell", wfo: "SHV" },
ForecastZone::LA022 => crate::ZoneDetails {state: "LA", zone: "022", zone_numeric: 022, name: "La Salle", wfo: "SHV" },
ForecastZone::LA023 => crate::ZoneDetails {state: "LA", zone: "023", zone_numeric: 023, name: "Franklin", wfo: "JAN" },
ForecastZone::LA024 => crate::ZoneDetails {state: "LA", zone: "024", zone_numeric: 024, name: "Catahoula", wfo: "JAN" },
ForecastZone::LA025 => crate::ZoneDetails {state: "LA", zone: "025", zone_numeric: 025, name: "Tensas", wfo: "JAN" },
ForecastZone::LA026 => crate::ZoneDetails {state: "LA", zone: "026", zone_numeric: 026, name: "Concordia", wfo: "JAN" },
ForecastZone::LA027 => crate::ZoneDetails {state: "LA", zone: "027", zone_numeric: 027, name: "Vernon", wfo: "LCH" },
ForecastZone::LA028 => crate::ZoneDetails {state: "LA", zone: "028", zone_numeric: 028, name: "Rapides", wfo: "LCH" },
ForecastZone::LA029 => crate::ZoneDetails {state: "LA", zone: "029", zone_numeric: 029, name: "Avoyelles", wfo: "LCH" },
ForecastZone::LA030 => crate::ZoneDetails {state: "LA", zone: "030", zone_numeric: 030, name: "Beauregard", wfo: "LCH" },
ForecastZone::LA031 => crate::ZoneDetails {state: "LA", zone: "031", zone_numeric: 031, name: "Allen", wfo: "LCH" },
ForecastZone::LA032 => crate::ZoneDetails {state: "LA", zone: "032", zone_numeric: 032, name: "Evangeline", wfo: "LCH" },
ForecastZone::LA033 => crate::ZoneDetails {state: "LA", zone: "033", zone_numeric: 033, name: "St. Landry", wfo: "LCH" },
ForecastZone::LA034 => crate::ZoneDetails {state: "LA", zone: "034", zone_numeric: 034, name: "Pointe Coupee", wfo: "LIX" },
ForecastZone::LA035 => crate::ZoneDetails {state: "LA", zone: "035", zone_numeric: 035, name: "West Feliciana", wfo: "LIX" },
ForecastZone::LA036 => crate::ZoneDetails {state: "LA", zone: "036", zone_numeric: 036, name: "East Feliciana", wfo: "LIX" },
ForecastZone::LA037 => crate::ZoneDetails {state: "LA", zone: "037", zone_numeric: 037, name: "St. Helena", wfo: "LIX" },
ForecastZone::LA039 => crate::ZoneDetails {state: "LA", zone: "039", zone_numeric: 039, name: "Washington", wfo: "LIX" },
ForecastZone::LA041 => crate::ZoneDetails {state: "LA", zone: "041", zone_numeric: 041, name: "Calcasieu", wfo: "LCH" },
ForecastZone::LA042 => crate::ZoneDetails {state: "LA", zone: "042", zone_numeric: 042, name: "Jefferson Davis", wfo: "LCH" },
ForecastZone::LA043 => crate::ZoneDetails {state: "LA", zone: "043", zone_numeric: 043, name: "Acadia", wfo: "LCH" },
ForecastZone::LA044 => crate::ZoneDetails {state: "LA", zone: "044", zone_numeric: 044, name: "Lafayette", wfo: "LCH" },
ForecastZone::LA045 => crate::ZoneDetails {state: "LA", zone: "045", zone_numeric: 045, name: "Upper St. Martin", wfo: "LCH" },
ForecastZone::LA046 => crate::ZoneDetails {state: "LA", zone: "046", zone_numeric: 046, name: "Iberville", wfo: "LIX" },
ForecastZone::LA047 => crate::ZoneDetails {state: "LA", zone: "047", zone_numeric: 047, name: "West Baton Rouge", wfo: "LIX" },
ForecastZone::LA048 => crate::ZoneDetails {state: "LA", zone: "048", zone_numeric: 048, name: "East Baton Rouge", wfo: "LIX" },
ForecastZone::LA052 => crate::ZoneDetails {state: "LA", zone: "052", zone_numeric: 052, name: "Vermilion", wfo: "LCH" },
ForecastZone::LA053 => crate::ZoneDetails {state: "LA", zone: "053", zone_numeric: 053, name: "Iberia", wfo: "LCH" },
ForecastZone::LA054 => crate::ZoneDetails {state: "LA", zone: "054", zone_numeric: 054, name: "St. Mary", wfo: "LCH" },
ForecastZone::LA055 => crate::ZoneDetails {state: "LA", zone: "055", zone_numeric: 055, name: "Lower St. Martin", wfo: "LCH" },
ForecastZone::LA056 => crate::ZoneDetails {state: "LA", zone: "056", zone_numeric: 056, name: "Assumption", wfo: "LIX" },
ForecastZone::LA057 => crate::ZoneDetails {state: "LA", zone: "057", zone_numeric: 057, name: "St. James", wfo: "LIX" },
ForecastZone::LA058 => crate::ZoneDetails {state: "LA", zone: "058", zone_numeric: 058, name: "St. John The Baptist", wfo: "LIX" },
ForecastZone::LA059 => crate::ZoneDetails {state: "LA", zone: "059", zone_numeric: 059, name: "Upper Lafourche", wfo: "LIX" },
ForecastZone::LA060 => crate::ZoneDetails {state: "LA", zone: "060", zone_numeric: 060, name: "St. Charles", wfo: "LIX" },
ForecastZone::LA064 => crate::ZoneDetails {state: "LA", zone: "064", zone_numeric: 064, name: "Upper St. Bernard", wfo: "LIX" },
ForecastZone::LA065 => crate::ZoneDetails {state: "LA", zone: "065", zone_numeric: 065, name: "Upper Terrebonne", wfo: "LIX" },
ForecastZone::LA066 => crate::ZoneDetails {state: "LA", zone: "066", zone_numeric: 066, name: "Lower Terrebonne", wfo: "LIX" },
ForecastZone::LA067 => crate::ZoneDetails {state: "LA", zone: "067", zone_numeric: 067, name: "Lower Lafourche", wfo: "LIX" },
ForecastZone::LA068 => crate::ZoneDetails {state: "LA", zone: "068", zone_numeric: 068, name: "Coastal Jefferson", wfo: "LIX" },
ForecastZone::LA069 => crate::ZoneDetails {state: "LA", zone: "069", zone_numeric: 069, name: "Lower Plaquemines", wfo: "LIX" },
ForecastZone::LA070 => crate::ZoneDetails {state: "LA", zone: "070", zone_numeric: 070, name: "Lower St. Bernard", wfo: "LIX" },
ForecastZone::LA071 => crate::ZoneDetails {state: "LA", zone: "071", zone_numeric: 071, name: "Northern Tangipahoa", wfo: "LIX" },
ForecastZone::LA073 => crate::ZoneDetails {state: "LA", zone: "073", zone_numeric: 073, name: "West Cameron", wfo: "LCH" },
ForecastZone::LA074 => crate::ZoneDetails {state: "LA", zone: "074", zone_numeric: 074, name: "East Cameron", wfo: "LCH" },
ForecastZone::LA076 => crate::ZoneDetails {state: "LA", zone: "076", zone_numeric: 076, name: "Southeast St. Tammany", wfo: "LIX" },
ForecastZone::LA077 => crate::ZoneDetails {state: "LA", zone: "077", zone_numeric: 077, name: "Western Orleans", wfo: "LIX" },
ForecastZone::LA078 => crate::ZoneDetails {state: "LA", zone: "078", zone_numeric: 078, name: "Eastern Orleans", wfo: "LIX" },
ForecastZone::LA079 => crate::ZoneDetails {state: "LA", zone: "079", zone_numeric: 079, name: "Northern St. Tammany", wfo: "LIX" },
ForecastZone::LA080 => crate::ZoneDetails {state: "LA", zone: "080", zone_numeric: 080, name: "Southwestern St. Tammany", wfo: "LIX" },
ForecastZone::LA081 => crate::ZoneDetails {state: "LA", zone: "081", zone_numeric: 081, name: "Central Tangipahoa", wfo: "LIX" },
ForecastZone::LA082 => crate::ZoneDetails {state: "LA", zone: "082", zone_numeric: 082, name: "Lower Tangipahoa", wfo: "LIX" },
ForecastZone::LA083 => crate::ZoneDetails {state: "LA", zone: "083", zone_numeric: 083, name: "Northern Livingston", wfo: "LIX" },
ForecastZone::LA084 => crate::ZoneDetails {state: "LA", zone: "084", zone_numeric: 084, name: "Southern Livingston", wfo: "LIX" },
ForecastZone::LA085 => crate::ZoneDetails {state: "LA", zone: "085", zone_numeric: 085, name: "Western Ascension", wfo: "LIX" },
ForecastZone::LA086 => crate::ZoneDetails {state: "LA", zone: "086", zone_numeric: 086, name: "Eastern Ascension", wfo: "LIX" },
ForecastZone::LA087 => crate::ZoneDetails {state: "LA", zone: "087", zone_numeric: 087, name: "Upper Jefferson", wfo: "LIX" },
ForecastZone::LA088 => crate::ZoneDetails {state: "LA", zone: "088", zone_numeric: 088, name: "Lower Jefferson", wfo: "LIX" },
ForecastZone::LA089 => crate::ZoneDetails {state: "LA", zone: "089", zone_numeric: 089, name: "Upper Plaquemines", wfo: "LIX" },
ForecastZone::LA090 => crate::ZoneDetails {state: "LA", zone: "090", zone_numeric: 090, name: "Central Plaquemines", wfo: "LIX" },
ForecastZone::MA001 => crate::ZoneDetails {state: "MA", zone: "001", zone_numeric: 001, name: "Northern Berkshire", wfo: "ALY" },
ForecastZone::MA002 => crate::ZoneDetails {state: "MA", zone: "002", zone_numeric: 002, name: "Western Franklin", wfo: "BOX" },
ForecastZone::MA003 => crate::ZoneDetails {state: "MA", zone: "003", zone_numeric: 003, name: "Eastern Franklin", wfo: "BOX" },
ForecastZone::MA004 => crate::ZoneDetails {state: "MA", zone: "004", zone_numeric: 004, name: "Northern Worcester", wfo: "BOX" },
ForecastZone::MA005 => crate::ZoneDetails {state: "MA", zone: "005", zone_numeric: 005, name: "Central Middlesex County", wfo: "BOX" },
ForecastZone::MA006 => crate::ZoneDetails {state: "MA", zone: "006", zone_numeric: 006, name: "Western Essex", wfo: "BOX" },
ForecastZone::MA007 => crate::ZoneDetails {state: "MA", zone: "007", zone_numeric: 007, name: "Eastern Essex", wfo: "BOX" },
ForecastZone::MA008 => crate::ZoneDetails {state: "MA", zone: "008", zone_numeric: 008, name: "Western Hampshire", wfo: "BOX" },
ForecastZone::MA009 => crate::ZoneDetails {state: "MA", zone: "009", zone_numeric: 009, name: "Western Hampden", wfo: "BOX" },
ForecastZone::MA010 => crate::ZoneDetails {state: "MA", zone: "010", zone_numeric: 010, name: "Eastern Hampshire", wfo: "BOX" },
ForecastZone::MA011 => crate::ZoneDetails {state: "MA", zone: "011", zone_numeric: 011, name: "Eastern Hampden", wfo: "BOX" },
ForecastZone::MA012 => crate::ZoneDetails {state: "MA", zone: "012", zone_numeric: 012, name: "Southern Worcester", wfo: "BOX" },
ForecastZone::MA013 => crate::ZoneDetails {state: "MA", zone: "013", zone_numeric: 013, name: "Western Norfolk", wfo: "BOX" },
ForecastZone::MA014 => crate::ZoneDetails {state: "MA", zone: "014", zone_numeric: 014, name: "Southeast Middlesex", wfo: "BOX" },
ForecastZone::MA015 => crate::ZoneDetails {state: "MA", zone: "015", zone_numeric: 015, name: "Suffolk", wfo: "BOX" },
ForecastZone::MA016 => crate::ZoneDetails {state: "MA", zone: "016", zone_numeric: 016, name: "Eastern Norfolk", wfo: "BOX" },
ForecastZone::MA017 => crate::ZoneDetails {state: "MA", zone: "017", zone_numeric: 017, name: "Northern Bristol", wfo: "BOX" },
ForecastZone::MA018 => crate::ZoneDetails {state: "MA", zone: "018", zone_numeric: 018, name: "Western Plymouth", wfo: "BOX" },
ForecastZone::MA019 => crate::ZoneDetails {state: "MA", zone: "019", zone_numeric: 019, name: "Eastern Plymouth", wfo: "BOX" },
ForecastZone::MA020 => crate::ZoneDetails {state: "MA", zone: "020", zone_numeric: 020, name: "Southern Bristol", wfo: "BOX" },
ForecastZone::MA021 => crate::ZoneDetails {state: "MA", zone: "021", zone_numeric: 021, name: "Southern Plymouth", wfo: "BOX" },
ForecastZone::MA022 => crate::ZoneDetails {state: "MA", zone: "022", zone_numeric: 022, name: "Barnstable", wfo: "BOX" },
ForecastZone::MA023 => crate::ZoneDetails {state: "MA", zone: "023", zone_numeric: 023, name: "Dukes", wfo: "BOX" },
ForecastZone::MA024 => crate::ZoneDetails {state: "MA", zone: "024", zone_numeric: 024, name: "Nantucket", wfo: "BOX" },
ForecastZone::MA025 => crate::ZoneDetails {state: "MA", zone: "025", zone_numeric: 025, name: "Southern Berkshire", wfo: "ALY" },
ForecastZone::MA026 => crate::ZoneDetails {state: "MA", zone: "026", zone_numeric: 026, name: "Northwest Middlesex County", wfo: "BOX" },
ForecastZone::MD001 => crate::ZoneDetails {state: "MD", zone: "001", zone_numeric: 001, name: "Garrett", wfo: "LWX" },
ForecastZone::MD003 => crate::ZoneDetails {state: "MD", zone: "003", zone_numeric: 003, name: "Washington", wfo: "LWX" },
ForecastZone::MD004 => crate::ZoneDetails {state: "MD", zone: "004", zone_numeric: 004, name: "Frederick", wfo: "LWX" },
ForecastZone::MD005 => crate::ZoneDetails {state: "MD", zone: "005", zone_numeric: 005, name: "Carroll", wfo: "LWX" },
ForecastZone::MD006 => crate::ZoneDetails {state: "MD", zone: "006", zone_numeric: 006, name: "Northern Baltimore", wfo: "LWX" },
ForecastZone::MD008 => crate::ZoneDetails {state: "MD", zone: "008", zone_numeric: 008, name: "Cecil", wfo: "LWX" },
ForecastZone::MD011 => crate::ZoneDetails {state: "MD", zone: "011", zone_numeric: 011, name: "Southern Baltimore", wfo: "LWX" },
ForecastZone::MD012 => crate::ZoneDetails {state: "MD", zone: "012", zone_numeric: 012, name: "Kent", wfo: "PHI" },
ForecastZone::MD013 => crate::ZoneDetails {state: "MD", zone: "013", zone_numeric: 013, name: "Prince Georges", wfo: "LWX" },
ForecastZone::MD014 => crate::ZoneDetails {state: "MD", zone: "014", zone_numeric: 014, name: "Anne Arundel", wfo: "LWX" },
ForecastZone::MD015 => crate::ZoneDetails {state: "MD", zone: "015", zone_numeric: 015, name: "Queen Anne's", wfo: "PHI" },
ForecastZone::MD016 => crate::ZoneDetails {state: "MD", zone: "016", zone_numeric: 016, name: "Charles", wfo: "LWX" },
ForecastZone::MD017 => crate::ZoneDetails {state: "MD", zone: "017", zone_numeric: 017, name: "St. Marys", wfo: "LWX" },
ForecastZone::MD018 => crate::ZoneDetails {state: "MD", zone: "018", zone_numeric: 018, name: "Calvert", wfo: "LWX" },
ForecastZone::MD019 => crate::ZoneDetails {state: "MD", zone: "019", zone_numeric: 019, name: "Talbot", wfo: "PHI" },
ForecastZone::MD020 => crate::ZoneDetails {state: "MD", zone: "020", zone_numeric: 020, name: "Caroline", wfo: "PHI" },
ForecastZone::MD021 => crate::ZoneDetails {state: "MD", zone: "021", zone_numeric: 021, name: "Dorchester", wfo: "AKQ" },
ForecastZone::MD022 => crate::ZoneDetails {state: "MD", zone: "022", zone_numeric: 022, name: "Wicomico", wfo: "AKQ" },
ForecastZone::MD023 => crate::ZoneDetails {state: "MD", zone: "023", zone_numeric: 023, name: "Somerset", wfo: "AKQ" },
ForecastZone::MD024 => crate::ZoneDetails {state: "MD", zone: "024", zone_numeric: 024, name: "Inland Worcester", wfo: "AKQ" },
ForecastZone::MD025 => crate::ZoneDetails {state: "MD", zone: "025", zone_numeric: 025, name: "Maryland Beaches", wfo: "AKQ" },
ForecastZone::MD501 => crate::ZoneDetails {state: "MD", zone: "501", zone_numeric: 501, name: "Extreme Western Allegany", wfo: "LWX" },
ForecastZone::MD502 => crate::ZoneDetails {state: "MD", zone: "502", zone_numeric: 502, name: "Central and Eastern Allegany", wfo: "LWX" },
ForecastZone::MD503 => crate::ZoneDetails {state: "MD", zone: "503", zone_numeric: 503, name: "Northwest Montgomery", wfo: "LWX" },
ForecastZone::MD504 => crate::ZoneDetails {state: "MD", zone: "504", zone_numeric: 504, name: "Central and Southeast Montgomery", wfo: "LWX" },
ForecastZone::MD505 => crate::ZoneDetails {state: "MD", zone: "505", zone_numeric: 505, name: "Northwest Howard", wfo: "LWX" },
ForecastZone::MD506 => crate::ZoneDetails {state: "MD", zone: "506", zone_numeric: 506, name: "Central and Southeast Howard", wfo: "LWX" },
ForecastZone::MD507 => crate::ZoneDetails {state: "MD", zone: "507", zone_numeric: 507, name: "Northwest Harford", wfo: "LWX" },
ForecastZone::MD508 => crate::ZoneDetails {state: "MD", zone: "508", zone_numeric: 508, name: "Southeast Harford", wfo: "LWX" },
ForecastZone::ME001 => crate::ZoneDetails {state: "ME", zone: "001", zone_numeric: 001, name: "Northwest Aroostook", wfo: "CAR" },
ForecastZone::ME002 => crate::ZoneDetails {state: "ME", zone: "002", zone_numeric: 002, name: "Northeast Aroostook", wfo: "CAR" },
ForecastZone::ME003 => crate::ZoneDetails {state: "ME", zone: "003", zone_numeric: 003, name: "Northern Somerset", wfo: "CAR" },
ForecastZone::ME004 => crate::ZoneDetails {state: "ME", zone: "004", zone_numeric: 004, name: "Northern Piscataquis", wfo: "CAR" },
ForecastZone::ME005 => crate::ZoneDetails {state: "ME", zone: "005", zone_numeric: 005, name: "Northern Penobscot", wfo: "CAR" },
ForecastZone::ME006 => crate::ZoneDetails {state: "ME", zone: "006", zone_numeric: 006, name: "Southeast Aroostook", wfo: "CAR" },
ForecastZone::ME007 => crate::ZoneDetails {state: "ME", zone: "007", zone_numeric: 007, name: "Northern Oxford", wfo: "GYX" },
ForecastZone::ME008 => crate::ZoneDetails {state: "ME", zone: "008", zone_numeric: 008, name: "Northern Franklin", wfo: "GYX" },
ForecastZone::ME009 => crate::ZoneDetails {state: "ME", zone: "009", zone_numeric: 009, name: "Central Somerset", wfo: "GYX" },
ForecastZone::ME010 => crate::ZoneDetails {state: "ME", zone: "010", zone_numeric: 010, name: "Central Piscataquis", wfo: "CAR" },
ForecastZone::ME011 => crate::ZoneDetails {state: "ME", zone: "011", zone_numeric: 011, name: "Central Penobscot", wfo: "CAR" },
ForecastZone::ME012 => crate::ZoneDetails {state: "ME", zone: "012", zone_numeric: 012, name: "Southern Oxford", wfo: "GYX" },
ForecastZone::ME013 => crate::ZoneDetails {state: "ME", zone: "013", zone_numeric: 013, name: "Southern Franklin", wfo: "GYX" },
ForecastZone::ME014 => crate::ZoneDetails {state: "ME", zone: "014", zone_numeric: 014, name: "Southern Somerset", wfo: "GYX" },
ForecastZone::ME015 => crate::ZoneDetails {state: "ME", zone: "015", zone_numeric: 015, name: "Southern Penobscot", wfo: "CAR" },
ForecastZone::ME016 => crate::ZoneDetails {state: "ME", zone: "016", zone_numeric: 016, name: "Interior Hancock", wfo: "CAR" },
ForecastZone::ME017 => crate::ZoneDetails {state: "ME", zone: "017", zone_numeric: 017, name: "Central Washington", wfo: "CAR" },
ForecastZone::ME018 => crate::ZoneDetails {state: "ME", zone: "018", zone_numeric: 018, name: "Interior York", wfo: "GYX" },
ForecastZone::ME019 => crate::ZoneDetails {state: "ME", zone: "019", zone_numeric: 019, name: "Central Interior Cumberland", wfo: "GYX" },
ForecastZone::ME020 => crate::ZoneDetails {state: "ME", zone: "020", zone_numeric: 020, name: "Androscoggin", wfo: "GYX" },
ForecastZone::ME021 => crate::ZoneDetails {state: "ME", zone: "021", zone_numeric: 021, name: "Kennebec", wfo: "GYX" },
ForecastZone::ME022 => crate::ZoneDetails {state: "ME", zone: "022", zone_numeric: 022, name: "Interior Waldo", wfo: "GYX" },
ForecastZone::ME023 => crate::ZoneDetails {state: "ME", zone: "023", zone_numeric: 023, name: "Coastal York", wfo: "GYX" },
ForecastZone::ME024 => crate::ZoneDetails {state: "ME", zone: "024", zone_numeric: 024, name: "Coastal Cumberland", wfo: "GYX" },
ForecastZone::ME025 => crate::ZoneDetails {state: "ME", zone: "025", zone_numeric: 025, name: "Sagadahoc", wfo: "GYX" },
ForecastZone::ME026 => crate::ZoneDetails {state: "ME", zone: "026", zone_numeric: 026, name: "Lincoln", wfo: "GYX" },
ForecastZone::ME027 => crate::ZoneDetails {state: "ME", zone: "027", zone_numeric: 027, name: "Knox", wfo: "GYX" },
ForecastZone::ME028 => crate::ZoneDetails {state: "ME", zone: "028", zone_numeric: 028, name: "Coastal Waldo", wfo: "GYX" },
ForecastZone::ME029 => crate::ZoneDetails {state: "ME", zone: "029", zone_numeric: 029, name: "Coastal Hancock", wfo: "CAR" },
ForecastZone::ME030 => crate::ZoneDetails {state: "ME", zone: "030", zone_numeric: 030, name: "Coastal Washington", wfo: "CAR" },
ForecastZone::ME031 => crate::ZoneDetails {state: "ME", zone: "031", zone_numeric: 031, name: "Southern Piscataquis", wfo: "CAR" },
ForecastZone::ME032 => crate::ZoneDetails {state: "ME", zone: "032", zone_numeric: 032, name: "Northern Washington", wfo: "CAR" },
ForecastZone::ME033 => crate::ZoneDetails {state: "ME", zone: "033", zone_numeric: 033, name: "Interior Cumberland Highlands", wfo: "GYX" },
ForecastZone::MH001 => crate::ZoneDetails {state: "MH", zone: "001", zone_numeric: 001, name: "Arno", wfo: "PQE" },
ForecastZone::MH002 => crate::ZoneDetails {state: "MH", zone: "002", zone_numeric: 002, name: "Majuro", wfo: "PQE" },
ForecastZone::MH003 => crate::ZoneDetails {state: "MH", zone: "003", zone_numeric: 003, name: "Wotje", wfo: "PQE" },
ForecastZone::MH004 => crate::ZoneDetails {state: "MH", zone: "004", zone_numeric: 004, name: "Mili", wfo: "PQE" },
ForecastZone::MH005 => crate::ZoneDetails {state: "MH", zone: "005", zone_numeric: 005, name: "Utrok", wfo: "PQE" },
ForecastZone::MH006 => crate::ZoneDetails {state: "MH", zone: "006", zone_numeric: 006, name: "Jaluit", wfo: "PQE" },
ForecastZone::MH007 => crate::ZoneDetails {state: "MH", zone: "007", zone_numeric: 007, name: "Ailinglaplap", wfo: "PQE" },
ForecastZone::MH008 => crate::ZoneDetails {state: "MH", zone: "008", zone_numeric: 008, name: "Kwajalein", wfo: "PQE" },
ForecastZone::MH009 => crate::ZoneDetails {state: "MH", zone: "009", zone_numeric: 009, name: "Ujae", wfo: "PQE" },
ForecastZone::MH010 => crate::ZoneDetails {state: "MH", zone: "010", zone_numeric: 010, name: "Enewetak", wfo: "PQE" },
ForecastZone::MI001 => crate::ZoneDetails {state: "MI", zone: "001", zone_numeric: 001, name: "Keweenaw", wfo: "MQT" },
ForecastZone::MI002 => crate::ZoneDetails {state: "MI", zone: "002", zone_numeric: 002, name: "Ontonagon", wfo: "MQT" },
ForecastZone::MI003 => crate::ZoneDetails {state: "MI", zone: "003", zone_numeric: 003, name: "Houghton", wfo: "MQT" },
ForecastZone::MI004 => crate::ZoneDetails {state: "MI", zone: "004", zone_numeric: 004, name: "Baraga", wfo: "MQT" },
ForecastZone::MI005 => crate::ZoneDetails {state: "MI", zone: "005", zone_numeric: 005, name: "Marquette", wfo: "MQT" },
ForecastZone::MI006 => crate::ZoneDetails {state: "MI", zone: "006", zone_numeric: 006, name: "Alger", wfo: "MQT" },
ForecastZone::MI007 => crate::ZoneDetails {state: "MI", zone: "007", zone_numeric: 007, name: "Luce", wfo: "MQT" },
ForecastZone::MI009 => crate::ZoneDetails {state: "MI", zone: "009", zone_numeric: 009, name: "Gogebic", wfo: "MQT" },
ForecastZone::MI010 => crate::ZoneDetails {state: "MI", zone: "010", zone_numeric: 010, name: "Iron", wfo: "MQT" },
ForecastZone::MI011 => crate::ZoneDetails {state: "MI", zone: "011", zone_numeric: 011, name: "Dickinson", wfo: "MQT" },
ForecastZone::MI012 => crate::ZoneDetails {state: "MI", zone: "012", zone_numeric: 012, name: "Menominee", wfo: "MQT" },
ForecastZone::MI013 => crate::ZoneDetails {state: "MI", zone: "013", zone_numeric: 013, name: "Delta", wfo: "MQT" },
ForecastZone::MI014 => crate::ZoneDetails {state: "MI", zone: "014", zone_numeric: 014, name: "Southern Schoolcraft", wfo: "MQT" },
ForecastZone::MI016 => crate::ZoneDetails {state: "MI", zone: "016", zone_numeric: 016, name: "Emmet", wfo: "APX" },
ForecastZone::MI017 => crate::ZoneDetails {state: "MI", zone: "017", zone_numeric: 017, name: "Cheboygan", wfo: "APX" },
ForecastZone::MI018 => crate::ZoneDetails {state: "MI", zone: "018", zone_numeric: 018, name: "Presque Isle", wfo: "APX" },
ForecastZone::MI020 => crate::ZoneDetails {state: "MI", zone: "020", zone_numeric: 020, name: "Leelanau", wfo: "APX" },
ForecastZone::MI021 => crate::ZoneDetails {state: "MI", zone: "021", zone_numeric: 021, name: "Antrim", wfo: "APX" },
ForecastZone::MI022 => crate::ZoneDetails {state: "MI", zone: "022", zone_numeric: 022, name: "Otsego", wfo: "APX" },
ForecastZone::MI023 => crate::ZoneDetails {state: "MI", zone: "023", zone_numeric: 023, name: "Montmorency", wfo: "APX" },
ForecastZone::MI024 => crate::ZoneDetails {state: "MI", zone: "024", zone_numeric: 024, name: "Alpena", wfo: "APX" },
ForecastZone::MI025 => crate::ZoneDetails {state: "MI", zone: "025", zone_numeric: 025, name: "Benzie", wfo: "APX" },
ForecastZone::MI026 => crate::ZoneDetails {state: "MI", zone: "026", zone_numeric: 026, name: "Grand Traverse", wfo: "APX" },
ForecastZone::MI027 => crate::ZoneDetails {state: "MI", zone: "027", zone_numeric: 027, name: "Kalkaska", wfo: "APX" },
ForecastZone::MI028 => crate::ZoneDetails {state: "MI", zone: "028", zone_numeric: 028, name: "Crawford", wfo: "APX" },
ForecastZone::MI029 => crate::ZoneDetails {state: "MI", zone: "029", zone_numeric: 029, name: "Oscoda", wfo: "APX" },
ForecastZone::MI030 => crate::ZoneDetails {state: "MI", zone: "030", zone_numeric: 030, name: "Alcona", wfo: "APX" },
ForecastZone::MI031 => crate::ZoneDetails {state: "MI", zone: "031", zone_numeric: 031, name: "Manistee", wfo: "APX" },
ForecastZone::MI032 => crate::ZoneDetails {state: "MI", zone: "032", zone_numeric: 032, name: "Wexford", wfo: "APX" },
ForecastZone::MI033 => crate::ZoneDetails {state: "MI", zone: "033", zone_numeric: 033, name: "Missaukee", wfo: "APX" },
ForecastZone::MI034 => crate::ZoneDetails {state: "MI", zone: "034", zone_numeric: 034, name: "Roscommon", wfo: "APX" },
ForecastZone::MI035 => crate::ZoneDetails {state: "MI", zone: "035", zone_numeric: 035, name: "Ogemaw", wfo: "APX" },
ForecastZone::MI036 => crate::ZoneDetails {state: "MI", zone: "036", zone_numeric: 036, name: "Iosco", wfo: "APX" },
ForecastZone::MI037 => crate::ZoneDetails {state: "MI", zone: "037", zone_numeric: 037, name: "Mason", wfo: "GRR" },
ForecastZone::MI038 => crate::ZoneDetails {state: "MI", zone: "038", zone_numeric: 038, name: "Lake", wfo: "GRR" },
ForecastZone::MI039 => crate::ZoneDetails {state: "MI", zone: "039", zone_numeric: 039, name: "Osceola", wfo: "GRR" },
ForecastZone::MI040 => crate::ZoneDetails {state: "MI", zone: "040", zone_numeric: 040, name: "Clare", wfo: "GRR" },
ForecastZone::MI041 => crate::ZoneDetails {state: "MI", zone: "041", zone_numeric: 041, name: "Gladwin", wfo: "APX" },
ForecastZone::MI042 => crate::ZoneDetails {state: "MI", zone: "042", zone_numeric: 042, name: "Arenac", wfo: "APX" },
ForecastZone::MI043 => crate::ZoneDetails {state: "MI", zone: "043", zone_numeric: 043, name: "Oceana", wfo: "GRR" },
ForecastZone::MI044 => crate::ZoneDetails {state: "MI", zone: "044", zone_numeric: 044, name: "Newaygo", wfo: "GRR" },
ForecastZone::MI045 => crate::ZoneDetails {state: "MI", zone: "045", zone_numeric: 045, name: "Mecosta", wfo: "GRR" },
ForecastZone::MI046 => crate::ZoneDetails {state: "MI", zone: "046", zone_numeric: 046, name: "Isabella", wfo: "GRR" },
ForecastZone::MI047 => crate::ZoneDetails {state: "MI", zone: "047", zone_numeric: 047, name: "Midland", wfo: "DTX" },
ForecastZone::MI048 => crate::ZoneDetails {state: "MI", zone: "048", zone_numeric: 048, name: "Bay", wfo: "DTX" },
ForecastZone::MI049 => crate::ZoneDetails {state: "MI", zone: "049", zone_numeric: 049, name: "Huron", wfo: "DTX" },
ForecastZone::MI050 => crate::ZoneDetails {state: "MI", zone: "050", zone_numeric: 050, name: "Muskegon", wfo: "GRR" },
ForecastZone::MI051 => crate::ZoneDetails {state: "MI", zone: "051", zone_numeric: 051, name: "Montcalm", wfo: "GRR" },
ForecastZone::MI052 => crate::ZoneDetails {state: "MI", zone: "052", zone_numeric: 052, name: "Gratiot", wfo: "GRR" },
ForecastZone::MI053 => crate::ZoneDetails {state: "MI", zone: "053", zone_numeric: 053, name: "Saginaw", wfo: "DTX" },
ForecastZone::MI054 => crate::ZoneDetails {state: "MI", zone: "054", zone_numeric: 054, name: "Tuscola", wfo: "DTX" },
ForecastZone::MI055 => crate::ZoneDetails {state: "MI", zone: "055", zone_numeric: 055, name: "Sanilac", wfo: "DTX" },
ForecastZone::MI056 => crate::ZoneDetails {state: "MI", zone: "056", zone_numeric: 056, name: "Ottawa", wfo: "GRR" },
ForecastZone::MI057 => crate::ZoneDetails {state: "MI", zone: "057", zone_numeric: 057, name: "Kent", wfo: "GRR" },
ForecastZone::MI058 => crate::ZoneDetails {state: "MI", zone: "058", zone_numeric: 058, name: "Ionia", wfo: "GRR" },
ForecastZone::MI059 => crate::ZoneDetails {state: "MI", zone: "059", zone_numeric: 059, name: "Clinton", wfo: "GRR" },
ForecastZone::MI060 => crate::ZoneDetails {state: "MI", zone: "060", zone_numeric: 060, name: "Shiawassee", wfo: "DTX" },
ForecastZone::MI061 => crate::ZoneDetails {state: "MI", zone: "061", zone_numeric: 061, name: "Genesee", wfo: "DTX" },
ForecastZone::MI062 => crate::ZoneDetails {state: "MI", zone: "062", zone_numeric: 062, name: "Lapeer", wfo: "DTX" },
ForecastZone::MI063 => crate::ZoneDetails {state: "MI", zone: "063", zone_numeric: 063, name: "St. Clair", wfo: "DTX" },
ForecastZone::MI064 => crate::ZoneDetails {state: "MI", zone: "064", zone_numeric: 064, name: "Allegan", wfo: "GRR" },
ForecastZone::MI065 => crate::ZoneDetails {state: "MI", zone: "065", zone_numeric: 065, name: "Barry", wfo: "GRR" },
ForecastZone::MI066 => crate::ZoneDetails {state: "MI", zone: "066", zone_numeric: 066, name: "Eaton", wfo: "GRR" },
ForecastZone::MI067 => crate::ZoneDetails {state: "MI", zone: "067", zone_numeric: 067, name: "Ingham", wfo: "GRR" },
ForecastZone::MI068 => crate::ZoneDetails {state: "MI", zone: "068", zone_numeric: 068, name: "Livingston", wfo: "DTX" },
ForecastZone::MI069 => crate::ZoneDetails {state: "MI", zone: "069", zone_numeric: 069, name: "Oakland", wfo: "DTX" },
ForecastZone::MI070 => crate::ZoneDetails {state: "MI", zone: "070", zone_numeric: 070, name: "Macomb", wfo: "DTX" },
ForecastZone::MI071 => crate::ZoneDetails {state: "MI", zone: "071", zone_numeric: 071, name: "Van Buren", wfo: "GRR" },
ForecastZone::MI072 => crate::ZoneDetails {state: "MI", zone: "072", zone_numeric: 072, name: "Kalamazoo", wfo: "GRR" },
ForecastZone::MI073 => crate::ZoneDetails {state: "MI", zone: "073", zone_numeric: 073, name: "Calhoun", wfo: "GRR" },
ForecastZone::MI074 => crate::ZoneDetails {state: "MI", zone: "074", zone_numeric: 074, name: "Jackson", wfo: "GRR" },
ForecastZone::MI075 => crate::ZoneDetails {state: "MI", zone: "075", zone_numeric: 075, name: "Washtenaw", wfo: "DTX" },
ForecastZone::MI076 => crate::ZoneDetails {state: "MI", zone: "076", zone_numeric: 076, name: "Wayne", wfo: "DTX" },
ForecastZone::MI077 => crate::ZoneDetails {state: "MI", zone: "077", zone_numeric: 077, name: "Berrien", wfo: "IWX" },
ForecastZone::MI078 => crate::ZoneDetails {state: "MI", zone: "078", zone_numeric: 078, name: "Cass", wfo: "IWX" },
ForecastZone::MI079 => crate::ZoneDetails {state: "MI", zone: "079", zone_numeric: 079, name: "St. Joseph", wfo: "IWX" },
ForecastZone::MI080 => crate::ZoneDetails {state: "MI", zone: "080", zone_numeric: 080, name: "Branch", wfo: "IWX" },
ForecastZone::MI081 => crate::ZoneDetails {state: "MI", zone: "081", zone_numeric: 081, name: "Hillsdale", wfo: "IWX" },
ForecastZone::MI082 => crate::ZoneDetails {state: "MI", zone: "082", zone_numeric: 082, name: "Lenawee", wfo: "DTX" },
ForecastZone::MI083 => crate::ZoneDetails {state: "MI", zone: "083", zone_numeric: 083, name: "Monroe", wfo: "DTX" },
ForecastZone::MI084 => crate::ZoneDetails {state: "MI", zone: "084", zone_numeric: 084, name: "Southern Houghton", wfo: "MQT" },
ForecastZone::MI085 => crate::ZoneDetails {state: "MI", zone: "085", zone_numeric: 085, name: "Northern Schoolcraft", wfo: "MQT" },
ForecastZone::MI086 => crate::ZoneDetails {state: "MI", zone: "086", zone_numeric: 086, name: "Western Chippewa", wfo: "APX" },
ForecastZone::MI087 => crate::ZoneDetails {state: "MI", zone: "087", zone_numeric: 087, name: "Central Chippewa", wfo: "APX" },
ForecastZone::MI088 => crate::ZoneDetails {state: "MI", zone: "088", zone_numeric: 088, name: "Southeast Chippewa", wfo: "APX" },
ForecastZone::MI095 => crate::ZoneDetails {state: "MI", zone: "095", zone_numeric: 095, name: "Western Mackinac", wfo: "APX" },
ForecastZone::MI096 => crate::ZoneDetails {state: "MI", zone: "096", zone_numeric: 096, name: "Eastern Mackinac", wfo: "APX" },
ForecastZone::MI097 => crate::ZoneDetails {state: "MI", zone: "097", zone_numeric: 097, name: "Mackinac Island/Bois Blanc Island", wfo: "APX" },
ForecastZone::MI098 => crate::ZoneDetails {state: "MI", zone: "098", zone_numeric: 098, name: "Beaver Island and surrounding islands", wfo: "APX" },
ForecastZone::MI099 => crate::ZoneDetails {state: "MI", zone: "099", zone_numeric: 099, name: "Charlevoix", wfo: "APX" },
ForecastZone::MN001 => crate::ZoneDetails {state: "MN", zone: "001", zone_numeric: 001, name: "West Polk", wfo: "FGF" },
ForecastZone::MN002 => crate::ZoneDetails {state: "MN", zone: "002", zone_numeric: 002, name: "Norman", wfo: "FGF" },
ForecastZone::MN003 => crate::ZoneDetails {state: "MN", zone: "003", zone_numeric: 003, name: "Clay", wfo: "FGF" },
ForecastZone::MN004 => crate::ZoneDetails {state: "MN", zone: "004", zone_numeric: 004, name: "Kittson", wfo: "FGF" },
ForecastZone::MN005 => crate::ZoneDetails {state: "MN", zone: "005", zone_numeric: 005, name: "Roseau", wfo: "FGF" },
ForecastZone::MN006 => crate::ZoneDetails {state: "MN", zone: "006", zone_numeric: 006, name: "Lake Of The Woods", wfo: "FGF" },
ForecastZone::MN007 => crate::ZoneDetails {state: "MN", zone: "007", zone_numeric: 007, name: "West Marshall", wfo: "FGF" },
ForecastZone::MN008 => crate::ZoneDetails {state: "MN", zone: "008", zone_numeric: 008, name: "East Marshall", wfo: "FGF" },
ForecastZone::MN009 => crate::ZoneDetails {state: "MN", zone: "009", zone_numeric: 009, name: "North Beltrami", wfo: "FGF" },
ForecastZone::MN010 => crate::ZoneDetails {state: "MN", zone: "010", zone_numeric: 010, name: "Koochiching", wfo: "DLH" },
ForecastZone::MN011 => crate::ZoneDetails {state: "MN", zone: "011", zone_numeric: 011, name: "North St. Louis", wfo: "DLH" },
ForecastZone::MN012 => crate::ZoneDetails {state: "MN", zone: "012", zone_numeric: 012, name: "Northern Cook/Northern Lake", wfo: "DLH" },
ForecastZone::MN013 => crate::ZoneDetails {state: "MN", zone: "013", zone_numeric: 013, name: "Pennington", wfo: "FGF" },
ForecastZone::MN014 => crate::ZoneDetails {state: "MN", zone: "014", zone_numeric: 014, name: "Red Lake", wfo: "FGF" },
ForecastZone::MN015 => crate::ZoneDetails {state: "MN", zone: "015", zone_numeric: 015, name: "East Polk", wfo: "FGF" },
ForecastZone::MN016 => crate::ZoneDetails {state: "MN", zone: "016", zone_numeric: 016, name: "North Clearwater", wfo: "FGF" },
ForecastZone::MN017 => crate::ZoneDetails {state: "MN", zone: "017", zone_numeric: 017, name: "South Beltrami", wfo: "FGF" },
ForecastZone::MN018 => crate::ZoneDetails {state: "MN", zone: "018", zone_numeric: 018, name: "North Itasca", wfo: "DLH" },
ForecastZone::MN019 => crate::ZoneDetails {state: "MN", zone: "019", zone_numeric: 019, name: "Central St. Louis", wfo: "DLH" },
ForecastZone::MN020 => crate::ZoneDetails {state: "MN", zone: "020", zone_numeric: 020, name: "Southern Lake/North Shore", wfo: "DLH" },
ForecastZone::MN021 => crate::ZoneDetails {state: "MN", zone: "021", zone_numeric: 021, name: "Southern Cook/North Shore", wfo: "DLH" },
ForecastZone::MN022 => crate::ZoneDetails {state: "MN", zone: "022", zone_numeric: 022, name: "Mahnomen", wfo: "FGF" },
ForecastZone::MN023 => crate::ZoneDetails {state: "MN", zone: "023", zone_numeric: 023, name: "South Clearwater", wfo: "FGF" },
ForecastZone::MN024 => crate::ZoneDetails {state: "MN", zone: "024", zone_numeric: 024, name: "Hubbard", wfo: "FGF" },
ForecastZone::MN025 => crate::ZoneDetails {state: "MN", zone: "025", zone_numeric: 025, name: "North Cass", wfo: "DLH" },
ForecastZone::MN026 => crate::ZoneDetails {state: "MN", zone: "026", zone_numeric: 026, name: "South Itasca", wfo: "DLH" },
ForecastZone::MN027 => crate::ZoneDetails {state: "MN", zone: "027", zone_numeric: 027, name: "West Becker", wfo: "FGF" },
ForecastZone::MN028 => crate::ZoneDetails {state: "MN", zone: "028", zone_numeric: 028, name: "East Becker", wfo: "FGF" },
ForecastZone::MN029 => crate::ZoneDetails {state: "MN", zone: "029", zone_numeric: 029, name: "Wilkin", wfo: "FGF" },
ForecastZone::MN030 => crate::ZoneDetails {state: "MN", zone: "030", zone_numeric: 030, name: "West Otter Tail", wfo: "FGF" },
ForecastZone::MN031 => crate::ZoneDetails {state: "MN", zone: "031", zone_numeric: 031, name: "East Otter Tail", wfo: "FGF" },
ForecastZone::MN032 => crate::ZoneDetails {state: "MN", zone: "032", zone_numeric: 032, name: "Wadena", wfo: "FGF" },
ForecastZone::MN033 => crate::ZoneDetails {state: "MN", zone: "033", zone_numeric: 033, name: "South Cass", wfo: "DLH" },
ForecastZone::MN034 => crate::ZoneDetails {state: "MN", zone: "034", zone_numeric: 034, name: "Crow Wing", wfo: "DLH" },
ForecastZone::MN035 => crate::ZoneDetails {state: "MN", zone: "035", zone_numeric: 035, name: "Northern Aitkin", wfo: "DLH" },
ForecastZone::MN036 => crate::ZoneDetails {state: "MN", zone: "036", zone_numeric: 036, name: "South Aitkin", wfo: "DLH" },
ForecastZone::MN037 => crate::ZoneDetails {state: "MN", zone: "037", zone_numeric: 037, name: "Carlton/South St. Louis", wfo: "DLH" },
ForecastZone::MN038 => crate::ZoneDetails {state: "MN", zone: "038", zone_numeric: 038, name: "Pine", wfo: "DLH" },
ForecastZone::MN039 => crate::ZoneDetails {state: "MN", zone: "039", zone_numeric: 039, name: "Traverse", wfo: "ABR" },
ForecastZone::MN040 => crate::ZoneDetails {state: "MN", zone: "040", zone_numeric: 040, name: "Grant", wfo: "FGF" },
ForecastZone::MN041 => crate::ZoneDetails {state: "MN", zone: "041", zone_numeric: 041, name: "Douglas", wfo: "MPX" },
ForecastZone::MN042 => crate::ZoneDetails {state: "MN", zone: "042", zone_numeric: 042, name: "Todd", wfo: "MPX" },
ForecastZone::MN043 => crate::ZoneDetails {state: "MN", zone: "043", zone_numeric: 043, name: "Morrison", wfo: "MPX" },
ForecastZone::MN044 => crate::ZoneDetails {state: "MN", zone: "044", zone_numeric: 044, name: "Mille Lacs", wfo: "MPX" },
ForecastZone::MN045 => crate::ZoneDetails {state: "MN", zone: "045", zone_numeric: 045, name: "Kanabec", wfo: "MPX" },
ForecastZone::MN046 => crate::ZoneDetails {state: "MN", zone: "046", zone_numeric: 046, name: "Big Stone", wfo: "ABR" },
ForecastZone::MN047 => crate::ZoneDetails {state: "MN", zone: "047", zone_numeric: 047, name: "Stevens", wfo: "MPX" },
ForecastZone::MN048 => crate::ZoneDetails {state: "MN", zone: "048", zone_numeric: 048, name: "Pope", wfo: "MPX" },
ForecastZone::MN049 => crate::ZoneDetails {state: "MN", zone: "049", zone_numeric: 049, name: "Stearns", wfo: "MPX" },
ForecastZone::MN050 => crate::ZoneDetails {state: "MN", zone: "050", zone_numeric: 050, name: "Benton", wfo: "MPX" },
ForecastZone::MN051 => crate::ZoneDetails {state: "MN", zone: "051", zone_numeric: 051, name: "Sherburne", wfo: "MPX" },
ForecastZone::MN052 => crate::ZoneDetails {state: "MN", zone: "052", zone_numeric: 052, name: "Isanti", wfo: "MPX" },
ForecastZone::MN053 => crate::ZoneDetails {state: "MN", zone: "053", zone_numeric: 053, name: "Chisago", wfo: "MPX" },
ForecastZone::MN054 => crate::ZoneDetails {state: "MN", zone: "054", zone_numeric: 054, name: "Lac Qui Parle", wfo: "MPX" },
ForecastZone::MN055 => crate::ZoneDetails {state: "MN", zone: "055", zone_numeric: 055, name: "Swift", wfo: "MPX" },
ForecastZone::MN056 => crate::ZoneDetails {state: "MN", zone: "056", zone_numeric: 056, name: "Chippewa", wfo: "MPX" },
ForecastZone::MN057 => crate::ZoneDetails {state: "MN", zone: "057", zone_numeric: 057, name: "Kandiyohi", wfo: "MPX" },
ForecastZone::MN058 => crate::ZoneDetails {state: "MN", zone: "058", zone_numeric: 058, name: "Meeker", wfo: "MPX" },
ForecastZone::MN059 => crate::ZoneDetails {state: "MN", zone: "059", zone_numeric: 059, name: "Wright", wfo: "MPX" },
ForecastZone::MN060 => crate::ZoneDetails {state: "MN", zone: "060", zone_numeric: 060, name: "Hennepin", wfo: "MPX" },
ForecastZone::MN061 => crate::ZoneDetails {state: "MN", zone: "061", zone_numeric: 061, name: "Anoka", wfo: "MPX" },
ForecastZone::MN062 => crate::ZoneDetails {state: "MN", zone: "062", zone_numeric: 062, name: "Ramsey", wfo: "MPX" },
ForecastZone::MN063 => crate::ZoneDetails {state: "MN", zone: "063", zone_numeric: 063, name: "Washington", wfo: "MPX" },
ForecastZone::MN064 => crate::ZoneDetails {state: "MN", zone: "064", zone_numeric: 064, name: "Yellow Medicine", wfo: "MPX" },
ForecastZone::MN065 => crate::ZoneDetails {state: "MN", zone: "065", zone_numeric: 065, name: "Renville", wfo: "MPX" },
ForecastZone::MN066 => crate::ZoneDetails {state: "MN", zone: "066", zone_numeric: 066, name: "McLeod", wfo: "MPX" },
ForecastZone::MN067 => crate::ZoneDetails {state: "MN", zone: "067", zone_numeric: 067, name: "Sibley", wfo: "MPX" },
ForecastZone::MN068 => crate::ZoneDetails {state: "MN", zone: "068", zone_numeric: 068, name: "Carver", wfo: "MPX" },
ForecastZone::MN069 => crate::ZoneDetails {state: "MN", zone: "069", zone_numeric: 069, name: "Scott", wfo: "MPX" },
ForecastZone::MN070 => crate::ZoneDetails {state: "MN", zone: "070", zone_numeric: 070, name: "Dakota", wfo: "MPX" },
ForecastZone::MN071 => crate::ZoneDetails {state: "MN", zone: "071", zone_numeric: 071, name: "Lincoln", wfo: "FSD" },
ForecastZone::MN072 => crate::ZoneDetails {state: "MN", zone: "072", zone_numeric: 072, name: "Lyon", wfo: "FSD" },
ForecastZone::MN073 => crate::ZoneDetails {state: "MN", zone: "073", zone_numeric: 073, name: "Redwood", wfo: "MPX" },
ForecastZone::MN074 => crate::ZoneDetails {state: "MN", zone: "074", zone_numeric: 074, name: "Brown", wfo: "MPX" },
ForecastZone::MN075 => crate::ZoneDetails {state: "MN", zone: "075", zone_numeric: 075, name: "Nicollet", wfo: "MPX" },
ForecastZone::MN076 => crate::ZoneDetails {state: "MN", zone: "076", zone_numeric: 076, name: "Le Sueur", wfo: "MPX" },
ForecastZone::MN077 => crate::ZoneDetails {state: "MN", zone: "077", zone_numeric: 077, name: "Rice", wfo: "MPX" },
ForecastZone::MN078 => crate::ZoneDetails {state: "MN", zone: "078", zone_numeric: 078, name: "Goodhue", wfo: "MPX" },
ForecastZone::MN079 => crate::ZoneDetails {state: "MN", zone: "079", zone_numeric: 079, name: "Wabasha", wfo: "ARX" },
ForecastZone::MN080 => crate::ZoneDetails {state: "MN", zone: "080", zone_numeric: 080, name: "Murray", wfo: "FSD" },
ForecastZone::MN081 => crate::ZoneDetails {state: "MN", zone: "081", zone_numeric: 081, name: "Cottonwood", wfo: "FSD" },
ForecastZone::MN082 => crate::ZoneDetails {state: "MN", zone: "082", zone_numeric: 082, name: "Watonwan", wfo: "MPX" },
ForecastZone::MN083 => crate::ZoneDetails {state: "MN", zone: "083", zone_numeric: 083, name: "Blue Earth", wfo: "MPX" },
ForecastZone::MN084 => crate::ZoneDetails {state: "MN", zone: "084", zone_numeric: 084, name: "Waseca", wfo: "MPX" },
ForecastZone::MN085 => crate::ZoneDetails {state: "MN", zone: "085", zone_numeric: 085, name: "Steele", wfo: "MPX" },
ForecastZone::MN086 => crate::ZoneDetails {state: "MN", zone: "086", zone_numeric: 086, name: "Dodge", wfo: "ARX" },
ForecastZone::MN087 => crate::ZoneDetails {state: "MN", zone: "087", zone_numeric: 087, name: "Olmsted", wfo: "ARX" },
ForecastZone::MN088 => crate::ZoneDetails {state: "MN", zone: "088", zone_numeric: 088, name: "Winona", wfo: "ARX" },
ForecastZone::MN089 => crate::ZoneDetails {state: "MN", zone: "089", zone_numeric: 089, name: "Nobles", wfo: "FSD" },
ForecastZone::MN090 => crate::ZoneDetails {state: "MN", zone: "090", zone_numeric: 090, name: "Jackson", wfo: "FSD" },
ForecastZone::MN091 => crate::ZoneDetails {state: "MN", zone: "091", zone_numeric: 091, name: "Martin", wfo: "MPX" },
ForecastZone::MN092 => crate::ZoneDetails {state: "MN", zone: "092", zone_numeric: 092, name: "Faribault", wfo: "MPX" },
ForecastZone::MN093 => crate::ZoneDetails {state: "MN", zone: "093", zone_numeric: 093, name: "Freeborn", wfo: "MPX" },
ForecastZone::MN094 => crate::ZoneDetails {state: "MN", zone: "094", zone_numeric: 094, name: "Mower", wfo: "ARX" },
ForecastZone::MN095 => crate::ZoneDetails {state: "MN", zone: "095", zone_numeric: 095, name: "Fillmore", wfo: "ARX" },
ForecastZone::MN096 => crate::ZoneDetails {state: "MN", zone: "096", zone_numeric: 096, name: "Houston", wfo: "ARX" },
ForecastZone::MN097 => crate::ZoneDetails {state: "MN", zone: "097", zone_numeric: 097, name: "Pipestone", wfo: "FSD" },
ForecastZone::MN098 => crate::ZoneDetails {state: "MN", zone: "098", zone_numeric: 098, name: "Rock", wfo: "FSD" },
ForecastZone::MO001 => crate::ZoneDetails {state: "MO", zone: "001", zone_numeric: 001, name: "Atchison", wfo: "EAX" },
ForecastZone::MO002 => crate::ZoneDetails {state: "MO", zone: "002", zone_numeric: 002, name: "Nodaway", wfo: "EAX" },
ForecastZone::MO003 => crate::ZoneDetails {state: "MO", zone: "003", zone_numeric: 003, name: "Worth", wfo: "EAX" },
ForecastZone::MO004 => crate::ZoneDetails {state: "MO", zone: "004", zone_numeric: 004, name: "Gentry", wfo: "EAX" },
ForecastZone::MO005 => crate::ZoneDetails {state: "MO", zone: "005", zone_numeric: 005, name: "Harrison", wfo: "EAX" },
ForecastZone::MO006 => crate::ZoneDetails {state: "MO", zone: "006", zone_numeric: 006, name: "Mercer", wfo: "EAX" },
ForecastZone::MO007 => crate::ZoneDetails {state: "MO", zone: "007", zone_numeric: 007, name: "Putnam", wfo: "EAX" },
ForecastZone::MO008 => crate::ZoneDetails {state: "MO", zone: "008", zone_numeric: 008, name: "Schuyler", wfo: "EAX" },
ForecastZone::MO009 => crate::ZoneDetails {state: "MO", zone: "009", zone_numeric: 009, name: "Scotland", wfo: "DVN" },
ForecastZone::MO010 => crate::ZoneDetails {state: "MO", zone: "010", zone_numeric: 010, name: "Clark", wfo: "DVN" },
ForecastZone::MO011 => crate::ZoneDetails {state: "MO", zone: "011", zone_numeric: 011, name: "Holt", wfo: "EAX" },
ForecastZone::MO012 => crate::ZoneDetails {state: "MO", zone: "012", zone_numeric: 012, name: "Andrew", wfo: "EAX" },
ForecastZone::MO013 => crate::ZoneDetails {state: "MO", zone: "013", zone_numeric: 013, name: "De Kalb", wfo: "EAX" },
ForecastZone::MO014 => crate::ZoneDetails {state: "MO", zone: "014", zone_numeric: 014, name: "Daviess", wfo: "EAX" },
ForecastZone::MO015 => crate::ZoneDetails {state: "MO", zone: "015", zone_numeric: 015, name: "Grundy", wfo: "EAX" },
ForecastZone::MO016 => crate::ZoneDetails {state: "MO", zone: "016", zone_numeric: 016, name: "Sullivan", wfo: "EAX" },
ForecastZone::MO017 => crate::ZoneDetails {state: "MO", zone: "017", zone_numeric: 017, name: "Adair", wfo: "EAX" },
ForecastZone::MO018 => crate::ZoneDetails {state: "MO", zone: "018", zone_numeric: 018, name: "Knox", wfo: "LSX" },
ForecastZone::MO019 => crate::ZoneDetails {state: "MO", zone: "019", zone_numeric: 019, name: "Lewis", wfo: "LSX" },
ForecastZone::MO020 => crate::ZoneDetails {state: "MO", zone: "020", zone_numeric: 020, name: "Buchanan", wfo: "EAX" },
ForecastZone::MO021 => crate::ZoneDetails {state: "MO", zone: "021", zone_numeric: 021, name: "Clinton", wfo: "EAX" },
ForecastZone::MO022 => crate::ZoneDetails {state: "MO", zone: "022", zone_numeric: 022, name: "Caldwell", wfo: "EAX" },
ForecastZone::MO023 => crate::ZoneDetails {state: "MO", zone: "023", zone_numeric: 023, name: "Livingston", wfo: "EAX" },
ForecastZone::MO024 => crate::ZoneDetails {state: "MO", zone: "024", zone_numeric: 024, name: "Linn", wfo: "EAX" },
ForecastZone::MO025 => crate::ZoneDetails {state: "MO", zone: "025", zone_numeric: 025, name: "Macon", wfo: "EAX" },
ForecastZone::MO026 => crate::ZoneDetails {state: "MO", zone: "026", zone_numeric: 026, name: "Shelby", wfo: "LSX" },
ForecastZone::MO027 => crate::ZoneDetails {state: "MO", zone: "027", zone_numeric: 027, name: "Marion", wfo: "LSX" },
ForecastZone::MO028 => crate::ZoneDetails {state: "MO", zone: "028", zone_numeric: 028, name: "Platte", wfo: "EAX" },
ForecastZone::MO029 => crate::ZoneDetails {state: "MO", zone: "029", zone_numeric: 029, name: "Clay", wfo: "EAX" },
ForecastZone::MO030 => crate::ZoneDetails {state: "MO", zone: "030", zone_numeric: 030, name: "Ray", wfo: "EAX" },
ForecastZone::MO031 => crate::ZoneDetails {state: "MO", zone: "031", zone_numeric: 031, name: "Carroll", wfo: "EAX" },
ForecastZone::MO032 => crate::ZoneDetails {state: "MO", zone: "032", zone_numeric: 032, name: "Chariton", wfo: "EAX" },
ForecastZone::MO033 => crate::ZoneDetails {state: "MO", zone: "033", zone_numeric: 033, name: "Randolph", wfo: "EAX" },
ForecastZone::MO034 => crate::ZoneDetails {state: "MO", zone: "034", zone_numeric: 034, name: "Monroe", wfo: "LSX" },
ForecastZone::MO035 => crate::ZoneDetails {state: "MO", zone: "035", zone_numeric: 035, name: "Ralls", wfo: "LSX" },
ForecastZone::MO036 => crate::ZoneDetails {state: "MO", zone: "036", zone_numeric: 036, name: "Pike", wfo: "LSX" },
ForecastZone::MO037 => crate::ZoneDetails {state: "MO", zone: "037", zone_numeric: 037, name: "Jackson", wfo: "EAX" },
ForecastZone::MO038 => crate::ZoneDetails {state: "MO", zone: "038", zone_numeric: 038, name: "Lafayette", wfo: "EAX" },
ForecastZone::MO039 => crate::ZoneDetails {state: "MO", zone: "039", zone_numeric: 039, name: "Saline", wfo: "EAX" },
ForecastZone::MO040 => crate::ZoneDetails {state: "MO", zone: "040", zone_numeric: 040, name: "Howard", wfo: "EAX" },
ForecastZone::MO041 => crate::ZoneDetails {state: "MO", zone: "041", zone_numeric: 041, name: "Boone", wfo: "LSX" },
ForecastZone::MO042 => crate::ZoneDetails {state: "MO", zone: "042", zone_numeric: 042, name: "Audrain", wfo: "LSX" },
ForecastZone::MO043 => crate::ZoneDetails {state: "MO", zone: "043", zone_numeric: 043, name: "Cass", wfo: "EAX" },
ForecastZone::MO044 => crate::ZoneDetails {state: "MO", zone: "044", zone_numeric: 044, name: "Johnson", wfo: "EAX" },
ForecastZone::MO045 => crate::ZoneDetails {state: "MO", zone: "045", zone_numeric: 045, name: "Pettis", wfo: "EAX" },
ForecastZone::MO046 => crate::ZoneDetails {state: "MO", zone: "046", zone_numeric: 046, name: "Cooper", wfo: "EAX" },
ForecastZone::MO047 => crate::ZoneDetails {state: "MO", zone: "047", zone_numeric: 047, name: "Moniteau", wfo: "LSX" },
ForecastZone::MO048 => crate::ZoneDetails {state: "MO", zone: "048", zone_numeric: 048, name: "Cole", wfo: "LSX" },
ForecastZone::MO049 => crate::ZoneDetails {state: "MO", zone: "049", zone_numeric: 049, name: "Osage", wfo: "LSX" },
ForecastZone::MO050 => crate::ZoneDetails {state: "MO", zone: "050", zone_numeric: 050, name: "Callaway", wfo: "LSX" },
ForecastZone::MO051 => crate::ZoneDetails {state: "MO", zone: "051", zone_numeric: 051, name: "Montgomery", wfo: "LSX" },
ForecastZone::MO052 => crate::ZoneDetails {state: "MO", zone: "052", zone_numeric: 052, name: "Lincoln", wfo: "LSX" },
ForecastZone::MO053 => crate::ZoneDetails {state: "MO", zone: "053", zone_numeric: 053, name: "Bates", wfo: "EAX" },
ForecastZone::MO054 => crate::ZoneDetails {state: "MO", zone: "054", zone_numeric: 054, name: "Henry", wfo: "EAX" },
ForecastZone::MO055 => crate::ZoneDetails {state: "MO", zone: "055", zone_numeric: 055, name: "Benton", wfo: "SGF" },
ForecastZone::MO056 => crate::ZoneDetails {state: "MO", zone: "056", zone_numeric: 056, name: "Morgan", wfo: "SGF" },
ForecastZone::MO057 => crate::ZoneDetails {state: "MO", zone: "057", zone_numeric: 057, name: "Miller", wfo: "SGF" },
ForecastZone::MO058 => crate::ZoneDetails {state: "MO", zone: "058", zone_numeric: 058, name: "Maries", wfo: "SGF" },
ForecastZone::MO059 => crate::ZoneDetails {state: "MO", zone: "059", zone_numeric: 059, name: "Gasconade", wfo: "LSX" },
ForecastZone::MO060 => crate::ZoneDetails {state: "MO", zone: "060", zone_numeric: 060, name: "Warren", wfo: "LSX" },
ForecastZone::MO061 => crate::ZoneDetails {state: "MO", zone: "061", zone_numeric: 061, name: "St. Charles", wfo: "LSX" },
ForecastZone::MO062 => crate::ZoneDetails {state: "MO", zone: "062", zone_numeric: 062, name: "Franklin", wfo: "LSX" },
ForecastZone::MO063 => crate::ZoneDetails {state: "MO", zone: "063", zone_numeric: 063, name: "St. Louis", wfo: "LSX" },
ForecastZone::MO064 => crate::ZoneDetails {state: "MO", zone: "064", zone_numeric: 064, name: "St. Louis City", wfo: "LSX" },
ForecastZone::MO065 => crate::ZoneDetails {state: "MO", zone: "065", zone_numeric: 065, name: "Jefferson", wfo: "LSX" },
ForecastZone::MO066 => crate::ZoneDetails {state: "MO", zone: "066", zone_numeric: 066, name: "Vernon", wfo: "SGF" },
ForecastZone::MO067 => crate::ZoneDetails {state: "MO", zone: "067", zone_numeric: 067, name: "St. Clair", wfo: "SGF" },
ForecastZone::MO068 => crate::ZoneDetails {state: "MO", zone: "068", zone_numeric: 068, name: "Hickory", wfo: "SGF" },
ForecastZone::MO069 => crate::ZoneDetails {state: "MO", zone: "069", zone_numeric: 069, name: "Camden", wfo: "SGF" },
ForecastZone::MO070 => crate::ZoneDetails {state: "MO", zone: "070", zone_numeric: 070, name: "Pulaski", wfo: "SGF" },
ForecastZone::MO071 => crate::ZoneDetails {state: "MO", zone: "071", zone_numeric: 071, name: "Phelps", wfo: "SGF" },
ForecastZone::MO072 => crate::ZoneDetails {state: "MO", zone: "072", zone_numeric: 072, name: "Crawford", wfo: "LSX" },
ForecastZone::MO073 => crate::ZoneDetails {state: "MO", zone: "073", zone_numeric: 073, name: "Washington", wfo: "LSX" },
ForecastZone::MO074 => crate::ZoneDetails {state: "MO", zone: "074", zone_numeric: 074, name: "St. Francois", wfo: "LSX" },
ForecastZone::MO075 => crate::ZoneDetails {state: "MO", zone: "075", zone_numeric: 075, name: "Ste. Genevieve", wfo: "LSX" },
ForecastZone::MO076 => crate::ZoneDetails {state: "MO", zone: "076", zone_numeric: 076, name: "Perry", wfo: "PAH" },
ForecastZone::MO077 => crate::ZoneDetails {state: "MO", zone: "077", zone_numeric: 077, name: "Barton", wfo: "SGF" },
ForecastZone::MO078 => crate::ZoneDetails {state: "MO", zone: "078", zone_numeric: 078, name: "Cedar", wfo: "SGF" },
ForecastZone::MO079 => crate::ZoneDetails {state: "MO", zone: "079", zone_numeric: 079, name: "Polk", wfo: "SGF" },
ForecastZone::MO080 => crate::ZoneDetails {state: "MO", zone: "080", zone_numeric: 080, name: "Dallas", wfo: "SGF" },
ForecastZone::MO081 => crate::ZoneDetails {state: "MO", zone: "081", zone_numeric: 081, name: "Laclede", wfo: "SGF" },
ForecastZone::MO082 => crate::ZoneDetails {state: "MO", zone: "082", zone_numeric: 082, name: "Texas", wfo: "SGF" },
ForecastZone::MO083 => crate::ZoneDetails {state: "MO", zone: "083", zone_numeric: 083, name: "Dent", wfo: "SGF" },
ForecastZone::MO084 => crate::ZoneDetails {state: "MO", zone: "084", zone_numeric: 084, name: "Iron", wfo: "LSX" },
ForecastZone::MO085 => crate::ZoneDetails {state: "MO", zone: "085", zone_numeric: 085, name: "Madison", wfo: "LSX" },
ForecastZone::MO086 => crate::ZoneDetails {state: "MO", zone: "086", zone_numeric: 086, name: "Bollinger", wfo: "PAH" },
ForecastZone::MO087 => crate::ZoneDetails {state: "MO", zone: "087", zone_numeric: 087, name: "Cape Girardeau", wfo: "PAH" },
ForecastZone::MO088 => crate::ZoneDetails {state: "MO", zone: "088", zone_numeric: 088, name: "Jasper", wfo: "SGF" },
ForecastZone::MO089 => crate::ZoneDetails {state: "MO", zone: "089", zone_numeric: 089, name: "Dade", wfo: "SGF" },
ForecastZone::MO090 => crate::ZoneDetails {state: "MO", zone: "090", zone_numeric: 090, name: "Greene", wfo: "SGF" },
ForecastZone::MO091 => crate::ZoneDetails {state: "MO", zone: "091", zone_numeric: 091, name: "Webster", wfo: "SGF" },
ForecastZone::MO092 => crate::ZoneDetails {state: "MO", zone: "092", zone_numeric: 092, name: "Wright", wfo: "SGF" },
ForecastZone::MO093 => crate::ZoneDetails {state: "MO", zone: "093", zone_numeric: 093, name: "Newton", wfo: "SGF" },
ForecastZone::MO094 => crate::ZoneDetails {state: "MO", zone: "094", zone_numeric: 094, name: "Lawrence", wfo: "SGF" },
ForecastZone::MO095 => crate::ZoneDetails {state: "MO", zone: "095", zone_numeric: 095, name: "Christian", wfo: "SGF" },
ForecastZone::MO096 => crate::ZoneDetails {state: "MO", zone: "096", zone_numeric: 096, name: "Douglas", wfo: "SGF" },
ForecastZone::MO097 => crate::ZoneDetails {state: "MO", zone: "097", zone_numeric: 097, name: "Howell", wfo: "SGF" },
ForecastZone::MO098 => crate::ZoneDetails {state: "MO", zone: "098", zone_numeric: 098, name: "Shannon", wfo: "SGF" },
ForecastZone::MO099 => crate::ZoneDetails {state: "MO", zone: "099", zone_numeric: 099, name: "Reynolds", wfo: "LSX" },
ForecastZone::MO100 => crate::ZoneDetails {state: "MO", zone: "100", zone_numeric: 100, name: "Wayne", wfo: "PAH" },
ForecastZone::MO101 => crate::ZoneDetails {state: "MO", zone: "101", zone_numeric: 101, name: "McDonald", wfo: "SGF" },
ForecastZone::MO102 => crate::ZoneDetails {state: "MO", zone: "102", zone_numeric: 102, name: "Barry", wfo: "SGF" },
ForecastZone::MO103 => crate::ZoneDetails {state: "MO", zone: "103", zone_numeric: 103, name: "Stone", wfo: "SGF" },
ForecastZone::MO104 => crate::ZoneDetails {state: "MO", zone: "104", zone_numeric: 104, name: "Taney", wfo: "SGF" },
ForecastZone::MO105 => crate::ZoneDetails {state: "MO", zone: "105", zone_numeric: 105, name: "Ozark", wfo: "SGF" },
ForecastZone::MO106 => crate::ZoneDetails {state: "MO", zone: "106", zone_numeric: 106, name: "Oregon", wfo: "SGF" },
ForecastZone::MO107 => crate::ZoneDetails {state: "MO", zone: "107", zone_numeric: 107, name: "Carter", wfo: "PAH" },
ForecastZone::MO108 => crate::ZoneDetails {state: "MO", zone: "108", zone_numeric: 108, name: "Ripley", wfo: "PAH" },
ForecastZone::MO109 => crate::ZoneDetails {state: "MO", zone: "109", zone_numeric: 109, name: "Butler", wfo: "PAH" },
ForecastZone::MO110 => crate::ZoneDetails {state: "MO", zone: "110", zone_numeric: 110, name: "Stoddard", wfo: "PAH" },
ForecastZone::MO111 => crate::ZoneDetails {state: "MO", zone: "111", zone_numeric: 111, name: "Scott", wfo: "PAH" },
ForecastZone::MO112 => crate::ZoneDetails {state: "MO", zone: "112", zone_numeric: 112, name: "Mississippi", wfo: "PAH" },
ForecastZone::MO113 => crate::ZoneDetails {state: "MO", zone: "113", zone_numeric: 113, name: "Dunklin", wfo: "MEG" },
ForecastZone::MO114 => crate::ZoneDetails {state: "MO", zone: "114", zone_numeric: 114, name: "New Madrid", wfo: "PAH" },
ForecastZone::MO115 => crate::ZoneDetails {state: "MO", zone: "115", zone_numeric: 115, name: "Pemiscot", wfo: "MEG" },
ForecastZone::MP001 => crate::ZoneDetails {state: "MP", zone: "001", zone_numeric: 001, name: "Rota", wfo: "GUM" },
ForecastZone::MP002 => crate::ZoneDetails {state: "MP", zone: "002", zone_numeric: 002, name: "Tinian", wfo: "GUM" },
ForecastZone::MP003 => crate::ZoneDetails {state: "MP", zone: "003", zone_numeric: 003, name: "Saipan", wfo: "GUM" },
ForecastZone::MP004 => crate::ZoneDetails {state: "MP", zone: "004", zone_numeric: 004, name: "Anatahan", wfo: "GUM" },
ForecastZone::MP005 => crate::ZoneDetails {state: "MP", zone: "005", zone_numeric: 005, name: "Alamagan", wfo: "GUM" },
ForecastZone::MP006 => crate::ZoneDetails {state: "MP", zone: "006", zone_numeric: 006, name: "Pagan", wfo: "GUM" },
ForecastZone::MP007 => crate::ZoneDetails {state: "MP", zone: "007", zone_numeric: 007, name: "Agrihan", wfo: "GUM" },
ForecastZone::MS001 => crate::ZoneDetails {state: "MS", zone: "001", zone_numeric: 001, name: "DeSoto", wfo: "MEG" },
ForecastZone::MS002 => crate::ZoneDetails {state: "MS", zone: "002", zone_numeric: 002, name: "Marshall", wfo: "MEG" },
ForecastZone::MS003 => crate::ZoneDetails {state: "MS", zone: "003", zone_numeric: 003, name: "Benton", wfo: "MEG" },
ForecastZone::MS004 => crate::ZoneDetails {state: "MS", zone: "004", zone_numeric: 004, name: "Tippah", wfo: "MEG" },
ForecastZone::MS005 => crate::ZoneDetails {state: "MS", zone: "005", zone_numeric: 005, name: "Alcorn", wfo: "MEG" },
ForecastZone::MS006 => crate::ZoneDetails {state: "MS", zone: "006", zone_numeric: 006, name: "Tishomingo", wfo: "MEG" },
ForecastZone::MS007 => crate::ZoneDetails {state: "MS", zone: "007", zone_numeric: 007, name: "Tunica", wfo: "MEG" },
ForecastZone::MS008 => crate::ZoneDetails {state: "MS", zone: "008", zone_numeric: 008, name: "Tate", wfo: "MEG" },
ForecastZone::MS009 => crate::ZoneDetails {state: "MS", zone: "009", zone_numeric: 009, name: "Prentiss", wfo: "MEG" },
ForecastZone::MS010 => crate::ZoneDetails {state: "MS", zone: "010", zone_numeric: 010, name: "Coahoma", wfo: "MEG" },
ForecastZone::MS011 => crate::ZoneDetails {state: "MS", zone: "011", zone_numeric: 011, name: "Quitman", wfo: "MEG" },
ForecastZone::MS012 => crate::ZoneDetails {state: "MS", zone: "012", zone_numeric: 012, name: "Panola", wfo: "MEG" },
ForecastZone::MS013 => crate::ZoneDetails {state: "MS", zone: "013", zone_numeric: 013, name: "Lafayette", wfo: "MEG" },
ForecastZone::MS014 => crate::ZoneDetails {state: "MS", zone: "014", zone_numeric: 014, name: "Union", wfo: "MEG" },
ForecastZone::MS015 => crate::ZoneDetails {state: "MS", zone: "015", zone_numeric: 015, name: "Pontotoc", wfo: "MEG" },
ForecastZone::MS016 => crate::ZoneDetails {state: "MS", zone: "016", zone_numeric: 016, name: "Lee", wfo: "MEG" },
ForecastZone::MS017 => crate::ZoneDetails {state: "MS", zone: "017", zone_numeric: 017, name: "Itawamba", wfo: "MEG" },
ForecastZone::MS018 => crate::ZoneDetails {state: "MS", zone: "018", zone_numeric: 018, name: "Bolivar", wfo: "JAN" },
ForecastZone::MS019 => crate::ZoneDetails {state: "MS", zone: "019", zone_numeric: 019, name: "Sunflower", wfo: "JAN" },
ForecastZone::MS020 => crate::ZoneDetails {state: "MS", zone: "020", zone_numeric: 020, name: "Tallahatchie", wfo: "MEG" },
ForecastZone::MS021 => crate::ZoneDetails {state: "MS", zone: "021", zone_numeric: 021, name: "Yalobusha", wfo: "MEG" },
ForecastZone::MS022 => crate::ZoneDetails {state: "MS", zone: "022", zone_numeric: 022, name: "Calhoun", wfo: "MEG" },
ForecastZone::MS023 => crate::ZoneDetails {state: "MS", zone: "023", zone_numeric: 023, name: "Chickasaw", wfo: "MEG" },
ForecastZone::MS024 => crate::ZoneDetails {state: "MS", zone: "024", zone_numeric: 024, name: "Monroe", wfo: "MEG" },
ForecastZone::MS025 => crate::ZoneDetails {state: "MS", zone: "025", zone_numeric: 025, name: "Leflore", wfo: "JAN" },
ForecastZone::MS026 => crate::ZoneDetails {state: "MS", zone: "026", zone_numeric: 026, name: "Grenada", wfo: "JAN" },
ForecastZone::MS027 => crate::ZoneDetails {state: "MS", zone: "027", zone_numeric: 027, name: "Carroll", wfo: "JAN" },
ForecastZone::MS028 => crate::ZoneDetails {state: "MS", zone: "028", zone_numeric: 028, name: "Montgomery", wfo: "JAN" },
ForecastZone::MS029 => crate::ZoneDetails {state: "MS", zone: "029", zone_numeric: 029, name: "Webster", wfo: "JAN" },
ForecastZone::MS030 => crate::ZoneDetails {state: "MS", zone: "030", zone_numeric: 030, name: "Clay", wfo: "JAN" },
ForecastZone::MS031 => crate::ZoneDetails {state: "MS", zone: "031", zone_numeric: 031, name: "Lowndes", wfo: "JAN" },
ForecastZone::MS032 => crate::ZoneDetails {state: "MS", zone: "032", zone_numeric: 032, name: "Choctaw", wfo: "JAN" },
ForecastZone::MS033 => crate::ZoneDetails {state: "MS", zone: "033", zone_numeric: 033, name: "Oktibbeha", wfo: "JAN" },
ForecastZone::MS034 => crate::ZoneDetails {state: "MS", zone: "034", zone_numeric: 034, name: "Washington", wfo: "JAN" },
ForecastZone::MS035 => crate::ZoneDetails {state: "MS", zone: "035", zone_numeric: 035, name: "Humphreys", wfo: "JAN" },
ForecastZone::MS036 => crate::ZoneDetails {state: "MS", zone: "036", zone_numeric: 036, name: "Holmes", wfo: "JAN" },
ForecastZone::MS037 => crate::ZoneDetails {state: "MS", zone: "037", zone_numeric: 037, name: "Attala", wfo: "JAN" },
ForecastZone::MS038 => crate::ZoneDetails {state: "MS", zone: "038", zone_numeric: 038, name: "Winston", wfo: "JAN" },
ForecastZone::MS039 => crate::ZoneDetails {state: "MS", zone: "039", zone_numeric: 039, name: "Noxubee", wfo: "JAN" },
ForecastZone::MS040 => crate::ZoneDetails {state: "MS", zone: "040", zone_numeric: 040, name: "Issaquena", wfo: "JAN" },
ForecastZone::MS041 => crate::ZoneDetails {state: "MS", zone: "041", zone_numeric: 041, name: "Sharkey", wfo: "JAN" },
ForecastZone::MS042 => crate::ZoneDetails {state: "MS", zone: "042", zone_numeric: 042, name: "Yazoo", wfo: "JAN" },
ForecastZone::MS043 => crate::ZoneDetails {state: "MS", zone: "043", zone_numeric: 043, name: "Madison", wfo: "JAN" },
ForecastZone::MS044 => crate::ZoneDetails {state: "MS", zone: "044", zone_numeric: 044, name: "Leake", wfo: "JAN" },
ForecastZone::MS045 => crate::ZoneDetails {state: "MS", zone: "045", zone_numeric: 045, name: "Neshoba", wfo: "JAN" },
ForecastZone::MS046 => crate::ZoneDetails {state: "MS", zone: "046", zone_numeric: 046, name: "Kemper", wfo: "JAN" },
ForecastZone::MS047 => crate::ZoneDetails {state: "MS", zone: "047", zone_numeric: 047, name: "Warren", wfo: "JAN" },
ForecastZone::MS048 => crate::ZoneDetails {state: "MS", zone: "048", zone_numeric: 048, name: "Hinds", wfo: "JAN" },
ForecastZone::MS049 => crate::ZoneDetails {state: "MS", zone: "049", zone_numeric: 049, name: "Rankin", wfo: "JAN" },
ForecastZone::MS050 => crate::ZoneDetails {state: "MS", zone: "050", zone_numeric: 050, name: "Scott", wfo: "JAN" },
ForecastZone::MS051 => crate::ZoneDetails {state: "MS", zone: "051", zone_numeric: 051, name: "Newton", wfo: "JAN" },
ForecastZone::MS052 => crate::ZoneDetails {state: "MS", zone: "052", zone_numeric: 052, name: "Lauderdale", wfo: "JAN" },
ForecastZone::MS053 => crate::ZoneDetails {state: "MS", zone: "053", zone_numeric: 053, name: "Claiborne", wfo: "JAN" },
ForecastZone::MS054 => crate::ZoneDetails {state: "MS", zone: "054", zone_numeric: 054, name: "Copiah", wfo: "JAN" },
ForecastZone::MS055 => crate::ZoneDetails {state: "MS", zone: "055", zone_numeric: 055, name: "Simpson", wfo: "JAN" },
ForecastZone::MS056 => crate::ZoneDetails {state: "MS", zone: "056", zone_numeric: 056, name: "Smith", wfo: "JAN" },
ForecastZone::MS057 => crate::ZoneDetails {state: "MS", zone: "057", zone_numeric: 057, name: "Jasper", wfo: "JAN" },
ForecastZone::MS058 => crate::ZoneDetails {state: "MS", zone: "058", zone_numeric: 058, name: "Clarke", wfo: "JAN" },
ForecastZone::MS059 => crate::ZoneDetails {state: "MS", zone: "059", zone_numeric: 059, name: "Jefferson", wfo: "JAN" },
ForecastZone::MS060 => crate::ZoneDetails {state: "MS", zone: "060", zone_numeric: 060, name: "Adams", wfo: "JAN" },
ForecastZone::MS061 => crate::ZoneDetails {state: "MS", zone: "061", zone_numeric: 061, name: "Franklin", wfo: "JAN" },
ForecastZone::MS062 => crate::ZoneDetails {state: "MS", zone: "062", zone_numeric: 062, name: "Lincoln", wfo: "JAN" },
ForecastZone::MS063 => crate::ZoneDetails {state: "MS", zone: "063", zone_numeric: 063, name: "Lawrence", wfo: "JAN" },
ForecastZone::MS064 => crate::ZoneDetails {state: "MS", zone: "064", zone_numeric: 064, name: "Jefferson Davis", wfo: "JAN" },
ForecastZone::MS065 => crate::ZoneDetails {state: "MS", zone: "065", zone_numeric: 065, name: "Covington", wfo: "JAN" },
ForecastZone::MS066 => crate::ZoneDetails {state: "MS", zone: "066", zone_numeric: 066, name: "Jones", wfo: "JAN" },
ForecastZone::MS067 => crate::ZoneDetails {state: "MS", zone: "067", zone_numeric: 067, name: "Wayne", wfo: "MOB" },
ForecastZone::MS068 => crate::ZoneDetails {state: "MS", zone: "068", zone_numeric: 068, name: "Wilkinson", wfo: "LIX" },
ForecastZone::MS069 => crate::ZoneDetails {state: "MS", zone: "069", zone_numeric: 069, name: "Amite", wfo: "LIX" },
ForecastZone::MS070 => crate::ZoneDetails {state: "MS", zone: "070", zone_numeric: 070, name: "Pike", wfo: "LIX" },
ForecastZone::MS071 => crate::ZoneDetails {state: "MS", zone: "071", zone_numeric: 071, name: "Walthall", wfo: "LIX" },
ForecastZone::MS072 => crate::ZoneDetails {state: "MS", zone: "072", zone_numeric: 072, name: "Marion", wfo: "JAN" },
ForecastZone::MS073 => crate::ZoneDetails {state: "MS", zone: "073", zone_numeric: 073, name: "Lamar", wfo: "JAN" },
ForecastZone::MS074 => crate::ZoneDetails {state: "MS", zone: "074", zone_numeric: 074, name: "Forrest", wfo: "JAN" },
ForecastZone::MS075 => crate::ZoneDetails {state: "MS", zone: "075", zone_numeric: 075, name: "Perry", wfo: "MOB" },
ForecastZone::MS076 => crate::ZoneDetails {state: "MS", zone: "076", zone_numeric: 076, name: "Greene", wfo: "MOB" },
ForecastZone::MS077 => crate::ZoneDetails {state: "MS", zone: "077", zone_numeric: 077, name: "Pearl River", wfo: "LIX" },
ForecastZone::MS078 => crate::ZoneDetails {state: "MS", zone: "078", zone_numeric: 078, name: "Stone", wfo: "MOB" },
ForecastZone::MS079 => crate::ZoneDetails {state: "MS", zone: "079", zone_numeric: 079, name: "George", wfo: "MOB" },
ForecastZone::MS080 => crate::ZoneDetails {state: "MS", zone: "080", zone_numeric: 080, name: "Hancock", wfo: "LIX" },
ForecastZone::MS081 => crate::ZoneDetails {state: "MS", zone: "081", zone_numeric: 081, name: "Harrison", wfo: "LIX" },
ForecastZone::MS082 => crate::ZoneDetails {state: "MS", zone: "082", zone_numeric: 082, name: "Jackson", wfo: "LIX" },
ForecastZone::MT001 => crate::ZoneDetails {state: "MT", zone: "001", zone_numeric: 001, name: "Kootenai/Cabinet Region", wfo: "MSO" },
ForecastZone::MT002 => crate::ZoneDetails {state: "MT", zone: "002", zone_numeric: 002, name: "West Glacier Region", wfo: "MSO" },
ForecastZone::MT003 => crate::ZoneDetails {state: "MT", zone: "003", zone_numeric: 003, name: "Flathead/Mission Valleys", wfo: "MSO" },
ForecastZone::MT004 => crate::ZoneDetails {state: "MT", zone: "004", zone_numeric: 004, name: "Lower Clark Fork Region", wfo: "MSO" },
ForecastZone::MT005 => crate::ZoneDetails {state: "MT", zone: "005", zone_numeric: 005, name: "Missoula/Bitterroot Valleys", wfo: "MSO" },
ForecastZone::MT006 => crate::ZoneDetails {state: "MT", zone: "006", zone_numeric: 006, name: "Bitterroot/Sapphire Mountains", wfo: "MSO" },
ForecastZone::MT007 => crate::ZoneDetails {state: "MT", zone: "007", zone_numeric: 007, name: "Butte/Blackfoot Region", wfo: "MSO" },
ForecastZone::MT008 => crate::ZoneDetails {state: "MT", zone: "008", zone_numeric: 008, name: "Beaverhead", wfo: "TFX" },
ForecastZone::MT009 => crate::ZoneDetails {state: "MT", zone: "009", zone_numeric: 009, name: "Northern Rocky Mountain Front", wfo: "TFX" },
ForecastZone::MT010 => crate::ZoneDetails {state: "MT", zone: "010", zone_numeric: 010, name: "Eastern Glacier", wfo: "TFX" },
ForecastZone::MT011 => crate::ZoneDetails {state: "MT", zone: "011", zone_numeric: 011, name: "Hill", wfo: "TFX" },
ForecastZone::MT012 => crate::ZoneDetails {state: "MT", zone: "012", zone_numeric: 012, name: "Cascade", wfo: "TFX" },
ForecastZone::MT013 => crate::ZoneDetails {state: "MT", zone: "013", zone_numeric: 013, name: "Chouteau", wfo: "TFX" },
ForecastZone::MT014 => crate::ZoneDetails {state: "MT", zone: "014", zone_numeric: 014, name: "Central and Southern Lewis and Clark", wfo: "TFX" },
ForecastZone::MT015 => crate::ZoneDetails {state: "MT", zone: "015", zone_numeric: 015, name: "Madison", wfo: "TFX" },
ForecastZone::MT016 => crate::ZoneDetails {state: "MT", zone: "016", zone_numeric: 016, name: "Central and Southeast Phillips", wfo: "GGW" },
ForecastZone::MT017 => crate::ZoneDetails {state: "MT", zone: "017", zone_numeric: 017, name: "Central and Southern Valley", wfo: "GGW" },
ForecastZone::MT018 => crate::ZoneDetails {state: "MT", zone: "018", zone_numeric: 018, name: "Daniels", wfo: "GGW" },
ForecastZone::MT019 => crate::ZoneDetails {state: "MT", zone: "019", zone_numeric: 019, name: "Sheridan", wfo: "GGW" },
ForecastZone::MT020 => crate::ZoneDetails {state: "MT", zone: "020", zone_numeric: 020, name: "Western Roosevelt", wfo: "GGW" },
ForecastZone::MT021 => crate::ZoneDetails {state: "MT", zone: "021", zone_numeric: 021, name: "Petroleum", wfo: "GGW" },
ForecastZone::MT022 => crate::ZoneDetails {state: "MT", zone: "022", zone_numeric: 022, name: "Garfield", wfo: "GGW" },
ForecastZone::MT023 => crate::ZoneDetails {state: "MT", zone: "023", zone_numeric: 023, name: "McCone", wfo: "GGW" },
ForecastZone::MT024 => crate::ZoneDetails {state: "MT", zone: "024", zone_numeric: 024, name: "Richland", wfo: "GGW" },
ForecastZone::MT025 => crate::ZoneDetails {state: "MT", zone: "025", zone_numeric: 025, name: "Dawson", wfo: "GGW" },
ForecastZone::MT026 => crate::ZoneDetails {state: "MT", zone: "026", zone_numeric: 026, name: "Prairie", wfo: "GGW" },
ForecastZone::MT027 => crate::ZoneDetails {state: "MT", zone: "027", zone_numeric: 027, name: "Wibaux", wfo: "GGW" },
ForecastZone::MT029 => crate::ZoneDetails {state: "MT", zone: "029", zone_numeric: 029, name: "Musselshell", wfo: "BYZ" },
ForecastZone::MT030 => crate::ZoneDetails {state: "MT", zone: "030", zone_numeric: 030, name: "Treasure", wfo: "BYZ" },
ForecastZone::MT031 => crate::ZoneDetails {state: "MT", zone: "031", zone_numeric: 031, name: "Northern Rosebud", wfo: "BYZ" },
ForecastZone::MT032 => crate::ZoneDetails {state: "MT", zone: "032", zone_numeric: 032, name: "Custer", wfo: "BYZ" },
ForecastZone::MT033 => crate::ZoneDetails {state: "MT", zone: "033", zone_numeric: 033, name: "Fallon", wfo: "BYZ" },
ForecastZone::MT034 => crate::ZoneDetails {state: "MT", zone: "034", zone_numeric: 034, name: "Northern Stillwater", wfo: "BYZ" },
ForecastZone::MT036 => crate::ZoneDetails {state: "MT", zone: "036", zone_numeric: 036, name: "Powder River", wfo: "BYZ" },
ForecastZone::MT037 => crate::ZoneDetails {state: "MT", zone: "037", zone_numeric: 037, name: "Carter", wfo: "BYZ" },
ForecastZone::MT040 => crate::ZoneDetails {state: "MT", zone: "040", zone_numeric: 040, name: "Northern Park", wfo: "BYZ" },
ForecastZone::MT042 => crate::ZoneDetails {state: "MT", zone: "042", zone_numeric: 042, name: "Golden Valley", wfo: "BYZ" },
ForecastZone::MT043 => crate::ZoneDetails {state: "MT", zone: "043", zone_numeric: 043, name: "Potomac/Seeley Lake Region", wfo: "MSO" },
ForecastZone::MT044 => crate::ZoneDetails {state: "MT", zone: "044", zone_numeric: 044, name: "Toole", wfo: "TFX" },
ForecastZone::MT045 => crate::ZoneDetails {state: "MT", zone: "045", zone_numeric: 045, name: "Liberty", wfo: "TFX" },
ForecastZone::MT046 => crate::ZoneDetails {state: "MT", zone: "046", zone_numeric: 046, name: "Eastern Pondera", wfo: "TFX" },
ForecastZone::MT047 => crate::ZoneDetails {state: "MT", zone: "047", zone_numeric: 047, name: "Blaine", wfo: "TFX" },
ForecastZone::MT048 => crate::ZoneDetails {state: "MT", zone: "048", zone_numeric: 048, name: "Southern Rocky Mountain Front", wfo: "TFX" },
ForecastZone::MT049 => crate::ZoneDetails {state: "MT", zone: "049", zone_numeric: 049, name: "Eastern Teton", wfo: "TFX" },
ForecastZone::MT050 => crate::ZoneDetails {state: "MT", zone: "050", zone_numeric: 050, name: "Judith Basin", wfo: "TFX" },
ForecastZone::MT051 => crate::ZoneDetails {state: "MT", zone: "051", zone_numeric: 051, name: "Fergus", wfo: "TFX" },
ForecastZone::MT052 => crate::ZoneDetails {state: "MT", zone: "052", zone_numeric: 052, name: "Jefferson", wfo: "TFX" },
ForecastZone::MT053 => crate::ZoneDetails {state: "MT", zone: "053", zone_numeric: 053, name: "Broadwater", wfo: "TFX" },
ForecastZone::MT054 => crate::ZoneDetails {state: "MT", zone: "054", zone_numeric: 054, name: "Meagher", wfo: "TFX" },
ForecastZone::MT055 => crate::ZoneDetails {state: "MT", zone: "055", zone_numeric: 055, name: "Gallatin", wfo: "TFX" },
ForecastZone::MT056 => crate::ZoneDetails {state: "MT", zone: "056", zone_numeric: 056, name: "Red Lodge Foothills", wfo: "BYZ" },
ForecastZone::MT057 => crate::ZoneDetails {state: "MT", zone: "057", zone_numeric: 057, name: "Northern Big Horn", wfo: "BYZ" },
ForecastZone::MT058 => crate::ZoneDetails {state: "MT", zone: "058", zone_numeric: 058, name: "Southern Rosebud", wfo: "BYZ" },
ForecastZone::MT059 => crate::ZoneDetails {state: "MT", zone: "059", zone_numeric: 059, name: "Northern Phillips", wfo: "GGW" },
ForecastZone::MT060 => crate::ZoneDetails {state: "MT", zone: "060", zone_numeric: 060, name: "Southwest Phillips", wfo: "GGW" },
ForecastZone::MT061 => crate::ZoneDetails {state: "MT", zone: "061", zone_numeric: 061, name: "Northern Valley", wfo: "GGW" },
ForecastZone::MT062 => crate::ZoneDetails {state: "MT", zone: "062", zone_numeric: 062, name: "Eastern Roosevelt", wfo: "GGW" },
ForecastZone::MT063 => crate::ZoneDetails {state: "MT", zone: "063", zone_numeric: 063, name: "Judith Gap", wfo: "BYZ" },
ForecastZone::MT064 => crate::ZoneDetails {state: "MT", zone: "064", zone_numeric: 064, name: "Paradise Valley", wfo: "BYZ" },
ForecastZone::MT065 => crate::ZoneDetails {state: "MT", zone: "065", zone_numeric: 065, name: "Livingston Area", wfo: "BYZ" },
ForecastZone::MT066 => crate::ZoneDetails {state: "MT", zone: "066", zone_numeric: 066, name: "Beartooth Foothills", wfo: "BYZ" },
ForecastZone::MT067 => crate::ZoneDetails {state: "MT", zone: "067", zone_numeric: 067, name: "Absaroka/Beartooth Mountains", wfo: "BYZ" },
ForecastZone::MT068 => crate::ZoneDetails {state: "MT", zone: "068", zone_numeric: 068, name: "Crazy Mountains", wfo: "BYZ" },
ForecastZone::MT138 => crate::ZoneDetails {state: "MT", zone: "138", zone_numeric: 138, name: "Southern Big Horn", wfo: "BYZ" },
ForecastZone::MT139 => crate::ZoneDetails {state: "MT", zone: "139", zone_numeric: 139, name: "Southeastern Carbon", wfo: "BYZ" },
ForecastZone::MT141 => crate::ZoneDetails {state: "MT", zone: "141", zone_numeric: 141, name: "Northern Sweet Grass", wfo: "BYZ" },
ForecastZone::MT169 => crate::ZoneDetails {state: "MT", zone: "169", zone_numeric: 169, name: "Bighorn Canyon", wfo: "BYZ" },
ForecastZone::MT170 => crate::ZoneDetails {state: "MT", zone: "170", zone_numeric: 170, name: "Northern Carbon", wfo: "BYZ" },
ForecastZone::MT171 => crate::ZoneDetails {state: "MT", zone: "171", zone_numeric: 171, name: "Pryor/Northern Bighorn Mountains", wfo: "BYZ" },
ForecastZone::MT172 => crate::ZoneDetails {state: "MT", zone: "172", zone_numeric: 172, name: "Melville Foothills", wfo: "BYZ" },
ForecastZone::MT173 => crate::ZoneDetails {state: "MT", zone: "173", zone_numeric: 173, name: "Northeastern Yellowstone", wfo: "BYZ" },
ForecastZone::MT228 => crate::ZoneDetails {state: "MT", zone: "228", zone_numeric: 228, name: "Southern Wheatland", wfo: "BYZ" },
ForecastZone::MT235 => crate::ZoneDetails {state: "MT", zone: "235", zone_numeric: 235, name: "Southwestern Yellowstone", wfo: "BYZ" },
ForecastZone::NC001 => crate::ZoneDetails {state: "NC", zone: "001", zone_numeric: 001, name: "Ashe", wfo: "RNK" },
ForecastZone::NC002 => crate::ZoneDetails {state: "NC", zone: "002", zone_numeric: 002, name: "Alleghany", wfo: "RNK" },
ForecastZone::NC003 => crate::ZoneDetails {state: "NC", zone: "003", zone_numeric: 003, name: "Surry", wfo: "RNK" },
ForecastZone::NC004 => crate::ZoneDetails {state: "NC", zone: "004", zone_numeric: 004, name: "Stokes", wfo: "RNK" },
ForecastZone::NC005 => crate::ZoneDetails {state: "NC", zone: "005", zone_numeric: 005, name: "Rockingham", wfo: "RNK" },
ForecastZone::NC006 => crate::ZoneDetails {state: "NC", zone: "006", zone_numeric: 006, name: "Caswell", wfo: "RNK" },
ForecastZone::NC007 => crate::ZoneDetails {state: "NC", zone: "007", zone_numeric: 007, name: "Person", wfo: "RAH" },
ForecastZone::NC008 => crate::ZoneDetails {state: "NC", zone: "008", zone_numeric: 008, name: "Granville", wfo: "RAH" },
ForecastZone::NC009 => crate::ZoneDetails {state: "NC", zone: "009", zone_numeric: 009, name: "Vance", wfo: "RAH" },
ForecastZone::NC010 => crate::ZoneDetails {state: "NC", zone: "010", zone_numeric: 010, name: "Warren", wfo: "RAH" },
ForecastZone::NC011 => crate::ZoneDetails {state: "NC", zone: "011", zone_numeric: 011, name: "Halifax", wfo: "RAH" },
ForecastZone::NC012 => crate::ZoneDetails {state: "NC", zone: "012", zone_numeric: 012, name: "Northampton", wfo: "AKQ" },
ForecastZone::NC013 => crate::ZoneDetails {state: "NC", zone: "013", zone_numeric: 013, name: "Hertford", wfo: "AKQ" },
ForecastZone::NC014 => crate::ZoneDetails {state: "NC", zone: "014", zone_numeric: 014, name: "Gates", wfo: "AKQ" },
ForecastZone::NC015 => crate::ZoneDetails {state: "NC", zone: "015", zone_numeric: 015, name: "Pasquotank", wfo: "AKQ" },
ForecastZone::NC016 => crate::ZoneDetails {state: "NC", zone: "016", zone_numeric: 016, name: "Camden", wfo: "AKQ" },
ForecastZone::NC017 => crate::ZoneDetails {state: "NC", zone: "017", zone_numeric: 017, name: "Western Currituck", wfo: "AKQ" },
ForecastZone::NC018 => crate::ZoneDetails {state: "NC", zone: "018", zone_numeric: 018, name: "Watauga", wfo: "RNK" },
ForecastZone::NC019 => crate::ZoneDetails {state: "NC", zone: "019", zone_numeric: 019, name: "Wilkes", wfo: "RNK" },
ForecastZone::NC020 => crate::ZoneDetails {state: "NC", zone: "020", zone_numeric: 020, name: "Yadkin", wfo: "RNK" },
ForecastZone::NC021 => crate::ZoneDetails {state: "NC", zone: "021", zone_numeric: 021, name: "Forsyth", wfo: "RAH" },
ForecastZone::NC022 => crate::ZoneDetails {state: "NC", zone: "022", zone_numeric: 022, name: "Guilford", wfo: "RAH" },
ForecastZone::NC023 => crate::ZoneDetails {state: "NC", zone: "023", zone_numeric: 023, name: "Alamance", wfo: "RAH" },
ForecastZone::NC024 => crate::ZoneDetails {state: "NC", zone: "024", zone_numeric: 024, name: "Orange", wfo: "RAH" },
ForecastZone::NC025 => crate::ZoneDetails {state: "NC", zone: "025", zone_numeric: 025, name: "Durham", wfo: "RAH" },
ForecastZone::NC026 => crate::ZoneDetails {state: "NC", zone: "026", zone_numeric: 026, name: "Franklin", wfo: "RAH" },
ForecastZone::NC027 => crate::ZoneDetails {state: "NC", zone: "027", zone_numeric: 027, name: "Nash", wfo: "RAH" },
ForecastZone::NC028 => crate::ZoneDetails {state: "NC", zone: "028", zone_numeric: 028, name: "Edgecombe", wfo: "RAH" },
ForecastZone::NC029 => crate::ZoneDetails {state: "NC", zone: "029", zone_numeric: 029, name: "Martin", wfo: "MHX" },
ForecastZone::NC030 => crate::ZoneDetails {state: "NC", zone: "030", zone_numeric: 030, name: "Bertie", wfo: "AKQ" },
ForecastZone::NC031 => crate::ZoneDetails {state: "NC", zone: "031", zone_numeric: 031, name: "Chowan", wfo: "AKQ" },
ForecastZone::NC032 => crate::ZoneDetails {state: "NC", zone: "032", zone_numeric: 032, name: "Perquimans", wfo: "AKQ" },
ForecastZone::NC033 => crate::ZoneDetails {state: "NC", zone: "033", zone_numeric: 033, name: "Avery", wfo: "GSP" },
ForecastZone::NC035 => crate::ZoneDetails {state: "NC", zone: "035", zone_numeric: 035, name: "Alexander", wfo: "GSP" },
ForecastZone::NC036 => crate::ZoneDetails {state: "NC", zone: "036", zone_numeric: 036, name: "Iredell", wfo: "GSP" },
ForecastZone::NC037 => crate::ZoneDetails {state: "NC", zone: "037", zone_numeric: 037, name: "Davie", wfo: "GSP" },
ForecastZone::NC038 => crate::ZoneDetails {state: "NC", zone: "038", zone_numeric: 038, name: "Davidson", wfo: "RAH" },
ForecastZone::NC039 => crate::ZoneDetails {state: "NC", zone: "039", zone_numeric: 039, name: "Randolph", wfo: "RAH" },
ForecastZone::NC040 => crate::ZoneDetails {state: "NC", zone: "040", zone_numeric: 040, name: "Chatham", wfo: "RAH" },
ForecastZone::NC041 => crate::ZoneDetails {state: "NC", zone: "041", zone_numeric: 041, name: "Wake", wfo: "RAH" },
ForecastZone::NC042 => crate::ZoneDetails {state: "NC", zone: "042", zone_numeric: 042, name: "Johnston", wfo: "RAH" },
ForecastZone::NC043 => crate::ZoneDetails {state: "NC", zone: "043", zone_numeric: 043, name: "Wilson", wfo: "RAH" },
ForecastZone::NC044 => crate::ZoneDetails {state: "NC", zone: "044", zone_numeric: 044, name: "Pitt", wfo: "MHX" },
ForecastZone::NC045 => crate::ZoneDetails {state: "NC", zone: "045", zone_numeric: 045, name: "Washington", wfo: "MHX" },
ForecastZone::NC046 => crate::ZoneDetails {state: "NC", zone: "046", zone_numeric: 046, name: "Tyrrell", wfo: "MHX" },
ForecastZone::NC047 => crate::ZoneDetails {state: "NC", zone: "047", zone_numeric: 047, name: "Mainland Dare", wfo: "MHX" },
ForecastZone::NC048 => crate::ZoneDetails {state: "NC", zone: "048", zone_numeric: 048, name: "Madison", wfo: "GSP" },
ForecastZone::NC049 => crate::ZoneDetails {state: "NC", zone: "049", zone_numeric: 049, name: "Yancey", wfo: "GSP" },
ForecastZone::NC050 => crate::ZoneDetails {state: "NC", zone: "050", zone_numeric: 050, name: "Mitchell", wfo: "GSP" },
ForecastZone::NC051 => crate::ZoneDetails {state: "NC", zone: "051", zone_numeric: 051, name: "Swain", wfo: "GSP" },
ForecastZone::NC052 => crate::ZoneDetails {state: "NC", zone: "052", zone_numeric: 052, name: "Haywood", wfo: "GSP" },
ForecastZone::NC053 => crate::ZoneDetails {state: "NC", zone: "053", zone_numeric: 053, name: "Buncombe", wfo: "GSP" },
ForecastZone::NC056 => crate::ZoneDetails {state: "NC", zone: "056", zone_numeric: 056, name: "Catawba", wfo: "GSP" },
ForecastZone::NC057 => crate::ZoneDetails {state: "NC", zone: "057", zone_numeric: 057, name: "Rowan", wfo: "GSP" },
ForecastZone::NC058 => crate::ZoneDetails {state: "NC", zone: "058", zone_numeric: 058, name: "Graham", wfo: "GSP" },
ForecastZone::NC059 => crate::ZoneDetails {state: "NC", zone: "059", zone_numeric: 059, name: "Northern Jackson", wfo: "GSP" },
ForecastZone::NC060 => crate::ZoneDetails {state: "NC", zone: "060", zone_numeric: 060, name: "Cherokee", wfo: "MRX" },
ForecastZone::NC061 => crate::ZoneDetails {state: "NC", zone: "061", zone_numeric: 061, name: "Clay", wfo: "MRX" },
ForecastZone::NC062 => crate::ZoneDetails {state: "NC", zone: "062", zone_numeric: 062, name: "Macon", wfo: "GSP" },
ForecastZone::NC063 => crate::ZoneDetails {state: "NC", zone: "063", zone_numeric: 063, name: "Southern Jackson", wfo: "GSP" },
ForecastZone::NC064 => crate::ZoneDetails {state: "NC", zone: "064", zone_numeric: 064, name: "Transylvania", wfo: "GSP" },
ForecastZone::NC065 => crate::ZoneDetails {state: "NC", zone: "065", zone_numeric: 065, name: "Henderson", wfo: "GSP" },
ForecastZone::NC068 => crate::ZoneDetails {state: "NC", zone: "068", zone_numeric: 068, name: "Cleveland", wfo: "GSP" },
ForecastZone::NC069 => crate::ZoneDetails {state: "NC", zone: "069", zone_numeric: 069, name: "Lincoln", wfo: "GSP" },
ForecastZone::NC070 => crate::ZoneDetails {state: "NC", zone: "070", zone_numeric: 070, name: "Gaston", wfo: "GSP" },
ForecastZone::NC071 => crate::ZoneDetails {state: "NC", zone: "071", zone_numeric: 071, name: "Mecklenburg", wfo: "GSP" },
ForecastZone::NC072 => crate::ZoneDetails {state: "NC", zone: "072", zone_numeric: 072, name: "Cabarrus", wfo: "GSP" },
ForecastZone::NC073 => crate::ZoneDetails {state: "NC", zone: "073", zone_numeric: 073, name: "Stanly", wfo: "RAH" },
ForecastZone::NC074 => crate::ZoneDetails {state: "NC", zone: "074", zone_numeric: 074, name: "Montgomery", wfo: "RAH" },
ForecastZone::NC075 => crate::ZoneDetails {state: "NC", zone: "075", zone_numeric: 075, name: "Moore", wfo: "RAH" },
ForecastZone::NC076 => crate::ZoneDetails {state: "NC", zone: "076", zone_numeric: 076, name: "Lee", wfo: "RAH" },
ForecastZone::NC077 => crate::ZoneDetails {state: "NC", zone: "077", zone_numeric: 077, name: "Harnett", wfo: "RAH" },
ForecastZone::NC078 => crate::ZoneDetails {state: "NC", zone: "078", zone_numeric: 078, name: "Wayne", wfo: "RAH" },
ForecastZone::NC079 => crate::ZoneDetails {state: "NC", zone: "079", zone_numeric: 079, name: "Greene", wfo: "MHX" },
ForecastZone::NC080 => crate::ZoneDetails {state: "NC", zone: "080", zone_numeric: 080, name: "Beaufort", wfo: "MHX" },
ForecastZone::NC081 => crate::ZoneDetails {state: "NC", zone: "081", zone_numeric: 081, name: "Mainland Hyde", wfo: "MHX" },
ForecastZone::NC082 => crate::ZoneDetails {state: "NC", zone: "082", zone_numeric: 082, name: "Union", wfo: "GSP" },
ForecastZone::NC083 => crate::ZoneDetails {state: "NC", zone: "083", zone_numeric: 083, name: "Anson", wfo: "RAH" },
ForecastZone::NC084 => crate::ZoneDetails {state: "NC", zone: "084", zone_numeric: 084, name: "Richmond", wfo: "RAH" },
ForecastZone::NC085 => crate::ZoneDetails {state: "NC", zone: "085", zone_numeric: 085, name: "Scotland", wfo: "RAH" },
ForecastZone::NC086 => crate::ZoneDetails {state: "NC", zone: "086", zone_numeric: 086, name: "Hoke", wfo: "RAH" },
ForecastZone::NC087 => crate::ZoneDetails {state: "NC", zone: "087", zone_numeric: 087, name: "Robeson", wfo: "ILM" },
ForecastZone::NC088 => crate::ZoneDetails {state: "NC", zone: "088", zone_numeric: 088, name: "Cumberland", wfo: "RAH" },
ForecastZone::NC089 => crate::ZoneDetails {state: "NC", zone: "089", zone_numeric: 089, name: "Sampson", wfo: "RAH" },
ForecastZone::NC090 => crate::ZoneDetails {state: "NC", zone: "090", zone_numeric: 090, name: "Duplin", wfo: "MHX" },
ForecastZone::NC091 => crate::ZoneDetails {state: "NC", zone: "091", zone_numeric: 091, name: "Lenoir", wfo: "MHX" },
ForecastZone::NC092 => crate::ZoneDetails {state: "NC", zone: "092", zone_numeric: 092, name: "Jones", wfo: "MHX" },
ForecastZone::NC094 => crate::ZoneDetails {state: "NC", zone: "094", zone_numeric: 094, name: "Pamlico", wfo: "MHX" },
ForecastZone::NC096 => crate::ZoneDetails {state: "NC", zone: "096", zone_numeric: 096, name: "Bladen", wfo: "ILM" },
ForecastZone::NC099 => crate::ZoneDetails {state: "NC", zone: "099", zone_numeric: 099, name: "Columbus", wfo: "ILM" },
ForecastZone::NC102 => crate::ZoneDetails {state: "NC", zone: "102", zone_numeric: 102, name: "Eastern Currituck", wfo: "AKQ" },
ForecastZone::NC105 => crate::ZoneDetails {state: "NC", zone: "105", zone_numeric: 105, name: "Inland Pender", wfo: "ILM" },
ForecastZone::NC106 => crate::ZoneDetails {state: "NC", zone: "106", zone_numeric: 106, name: "Coastal Pender", wfo: "ILM" },
ForecastZone::NC107 => crate::ZoneDetails {state: "NC", zone: "107", zone_numeric: 107, name: "Inland New Hanover", wfo: "ILM" },
ForecastZone::NC108 => crate::ZoneDetails {state: "NC", zone: "108", zone_numeric: 108, name: "Coastal New Hanover", wfo: "ILM" },
ForecastZone::NC109 => crate::ZoneDetails {state: "NC", zone: "109", zone_numeric: 109, name: "Inland Brunswick", wfo: "ILM" },
ForecastZone::NC110 => crate::ZoneDetails {state: "NC", zone: "110", zone_numeric: 110, name: "Coastal Brunswick", wfo: "ILM" },
ForecastZone::NC193 => crate::ZoneDetails {state: "NC", zone: "193", zone_numeric: 193, name: "Northern Craven", wfo: "MHX" },
ForecastZone::NC194 => crate::ZoneDetails {state: "NC", zone: "194", zone_numeric: 194, name: "Southern Craven", wfo: "MHX" },
ForecastZone::NC195 => crate::ZoneDetails {state: "NC", zone: "195", zone_numeric: 195, name: "West Carteret", wfo: "MHX" },
ForecastZone::NC196 => crate::ZoneDetails {state: "NC", zone: "196", zone_numeric: 196, name: "East Carteret", wfo: "MHX" },
ForecastZone::NC198 => crate::ZoneDetails {state: "NC", zone: "198", zone_numeric: 198, name: "Inland Onslow", wfo: "MHX" },
ForecastZone::NC199 => crate::ZoneDetails {state: "NC", zone: "199", zone_numeric: 199, name: "Coastal Onslow", wfo: "MHX" },
ForecastZone::NC203 => crate::ZoneDetails {state: "NC", zone: "203", zone_numeric: 203, name: "Northern Outer Banks", wfo: "MHX" },
ForecastZone::NC204 => crate::ZoneDetails {state: "NC", zone: "204", zone_numeric: 204, name: "Ocracoke Island", wfo: "MHX" },
ForecastZone::NC205 => crate::ZoneDetails {state: "NC", zone: "205", zone_numeric: 205, name: "Hatteras Island", wfo: "MHX" },
ForecastZone::NC501 => crate::ZoneDetails {state: "NC", zone: "501", zone_numeric: 501, name: "Caldwell Mountains", wfo: "GSP" },
ForecastZone::NC502 => crate::ZoneDetails {state: "NC", zone: "502", zone_numeric: 502, name: "Greater Caldwell", wfo: "GSP" },
ForecastZone::NC503 => crate::ZoneDetails {state: "NC", zone: "503", zone_numeric: 503, name: "Burke Mountains", wfo: "GSP" },
ForecastZone::NC504 => crate::ZoneDetails {state: "NC", zone: "504", zone_numeric: 504, name: "Greater Burke", wfo: "GSP" },
ForecastZone::NC505 => crate::ZoneDetails {state: "NC", zone: "505", zone_numeric: 505, name: "McDowell Mountains", wfo: "GSP" },
ForecastZone::NC506 => crate::ZoneDetails {state: "NC", zone: "506", zone_numeric: 506, name: "Eastern McDowell", wfo: "GSP" },
ForecastZone::NC507 => crate::ZoneDetails {state: "NC", zone: "507", zone_numeric: 507, name: "Rutherford Mountains", wfo: "GSP" },
ForecastZone::NC508 => crate::ZoneDetails {state: "NC", zone: "508", zone_numeric: 508, name: "Greater Rutherford", wfo: "GSP" },
ForecastZone::NC509 => crate::ZoneDetails {state: "NC", zone: "509", zone_numeric: 509, name: "Polk Mountains", wfo: "GSP" },
ForecastZone::NC510 => crate::ZoneDetails {state: "NC", zone: "510", zone_numeric: 510, name: "Eastern Polk", wfo: "GSP" },
ForecastZone::ND001 => crate::ZoneDetails {state: "ND", zone: "001", zone_numeric: 001, name: "Divide", wfo: "BIS" },
ForecastZone::ND002 => crate::ZoneDetails {state: "ND", zone: "002", zone_numeric: 002, name: "Burke", wfo: "BIS" },
ForecastZone::ND003 => crate::ZoneDetails {state: "ND", zone: "003", zone_numeric: 003, name: "Renville", wfo: "BIS" },
ForecastZone::ND004 => crate::ZoneDetails {state: "ND", zone: "004", zone_numeric: 004, name: "Bottineau", wfo: "BIS" },
ForecastZone::ND005 => crate::ZoneDetails {state: "ND", zone: "005", zone_numeric: 005, name: "Rolette", wfo: "BIS" },
ForecastZone::ND006 => crate::ZoneDetails {state: "ND", zone: "006", zone_numeric: 006, name: "Towner", wfo: "FGF" },
ForecastZone::ND007 => crate::ZoneDetails {state: "ND", zone: "007", zone_numeric: 007, name: "Cavalier", wfo: "FGF" },
ForecastZone::ND008 => crate::ZoneDetails {state: "ND", zone: "008", zone_numeric: 008, name: "Pembina", wfo: "FGF" },
ForecastZone::ND009 => crate::ZoneDetails {state: "ND", zone: "009", zone_numeric: 009, name: "Williams", wfo: "BIS" },
ForecastZone::ND010 => crate::ZoneDetails {state: "ND", zone: "010", zone_numeric: 010, name: "Mountrail", wfo: "BIS" },
ForecastZone::ND011 => crate::ZoneDetails {state: "ND", zone: "011", zone_numeric: 011, name: "Ward", wfo: "BIS" },
ForecastZone::ND012 => crate::ZoneDetails {state: "ND", zone: "012", zone_numeric: 012, name: "McHenry", wfo: "BIS" },
ForecastZone::ND013 => crate::ZoneDetails {state: "ND", zone: "013", zone_numeric: 013, name: "Pierce", wfo: "BIS" },
ForecastZone::ND014 => crate::ZoneDetails {state: "ND", zone: "014", zone_numeric: 014, name: "Benson", wfo: "FGF" },
ForecastZone::ND015 => crate::ZoneDetails {state: "ND", zone: "015", zone_numeric: 015, name: "Ramsey", wfo: "FGF" },
ForecastZone::ND016 => crate::ZoneDetails {state: "ND", zone: "016", zone_numeric: 016, name: "Eastern Walsh County", wfo: "FGF" },
ForecastZone::ND017 => crate::ZoneDetails {state: "ND", zone: "017", zone_numeric: 017, name: "McKenzie", wfo: "BIS" },
ForecastZone::ND018 => crate::ZoneDetails {state: "ND", zone: "018", zone_numeric: 018, name: "Dunn", wfo: "BIS" },
ForecastZone::ND019 => crate::ZoneDetails {state: "ND", zone: "019", zone_numeric: 019, name: "Mercer", wfo: "BIS" },
ForecastZone::ND020 => crate::ZoneDetails {state: "ND", zone: "020", zone_numeric: 020, name: "Oliver", wfo: "BIS" },
ForecastZone::ND021 => crate::ZoneDetails {state: "ND", zone: "021", zone_numeric: 021, name: "McLean", wfo: "BIS" },
ForecastZone::ND022 => crate::ZoneDetails {state: "ND", zone: "022", zone_numeric: 022, name: "Sheridan", wfo: "BIS" },
ForecastZone::ND023 => crate::ZoneDetails {state: "ND", zone: "023", zone_numeric: 023, name: "Wells", wfo: "BIS" },
ForecastZone::ND024 => crate::ZoneDetails {state: "ND", zone: "024", zone_numeric: 024, name: "Eddy", wfo: "FGF" },
ForecastZone::ND025 => crate::ZoneDetails {state: "ND", zone: "025", zone_numeric: 025, name: "Foster", wfo: "BIS" },
ForecastZone::ND026 => crate::ZoneDetails {state: "ND", zone: "026", zone_numeric: 026, name: "Nelson", wfo: "FGF" },
ForecastZone::ND027 => crate::ZoneDetails {state: "ND", zone: "027", zone_numeric: 027, name: "Grand Forks", wfo: "FGF" },
ForecastZone::ND028 => crate::ZoneDetails {state: "ND", zone: "028", zone_numeric: 028, name: "Griggs", wfo: "FGF" },
ForecastZone::ND029 => crate::ZoneDetails {state: "ND", zone: "029", zone_numeric: 029, name: "Steele", wfo: "FGF" },
ForecastZone::ND030 => crate::ZoneDetails {state: "ND", zone: "030", zone_numeric: 030, name: "Traill", wfo: "FGF" },
ForecastZone::ND031 => crate::ZoneDetails {state: "ND", zone: "031", zone_numeric: 031, name: "Golden Valley", wfo: "BIS" },
ForecastZone::ND032 => crate::ZoneDetails {state: "ND", zone: "032", zone_numeric: 032, name: "Billings", wfo: "BIS" },
ForecastZone::ND033 => crate::ZoneDetails {state: "ND", zone: "033", zone_numeric: 033, name: "Stark", wfo: "BIS" },
ForecastZone::ND034 => crate::ZoneDetails {state: "ND", zone: "034", zone_numeric: 034, name: "Morton", wfo: "BIS" },
ForecastZone::ND035 => crate::ZoneDetails {state: "ND", zone: "035", zone_numeric: 035, name: "Burleigh", wfo: "BIS" },
ForecastZone::ND036 => crate::ZoneDetails {state: "ND", zone: "036", zone_numeric: 036, name: "Kidder", wfo: "BIS" },
ForecastZone::ND037 => crate::ZoneDetails {state: "ND", zone: "037", zone_numeric: 037, name: "Stutsman", wfo: "BIS" },
ForecastZone::ND038 => crate::ZoneDetails {state: "ND", zone: "038", zone_numeric: 038, name: "Barnes", wfo: "FGF" },
ForecastZone::ND039 => crate::ZoneDetails {state: "ND", zone: "039", zone_numeric: 039, name: "Cass", wfo: "FGF" },
ForecastZone::ND040 => crate::ZoneDetails {state: "ND", zone: "040", zone_numeric: 040, name: "Slope", wfo: "BIS" },
ForecastZone::ND041 => crate::ZoneDetails {state: "ND", zone: "041", zone_numeric: 041, name: "Hettinger", wfo: "BIS" },
ForecastZone::ND042 => crate::ZoneDetails {state: "ND", zone: "042", zone_numeric: 042, name: "Grant", wfo: "BIS" },
ForecastZone::ND043 => crate::ZoneDetails {state: "ND", zone: "043", zone_numeric: 043, name: "Bowman", wfo: "BIS" },
ForecastZone::ND044 => crate::ZoneDetails {state: "ND", zone: "044", zone_numeric: 044, name: "Adams", wfo: "BIS" },
ForecastZone::ND045 => crate::ZoneDetails {state: "ND", zone: "045", zone_numeric: 045, name: "Sioux", wfo: "BIS" },
ForecastZone::ND046 => crate::ZoneDetails {state: "ND", zone: "046", zone_numeric: 046, name: "Emmons", wfo: "BIS" },
ForecastZone::ND047 => crate::ZoneDetails {state: "ND", zone: "047", zone_numeric: 047, name: "Logan", wfo: "BIS" },
ForecastZone::ND048 => crate::ZoneDetails {state: "ND", zone: "048", zone_numeric: 048, name: "La Moure", wfo: "BIS" },
ForecastZone::ND049 => crate::ZoneDetails {state: "ND", zone: "049", zone_numeric: 049, name: "Ransom", wfo: "FGF" },
ForecastZone::ND050 => crate::ZoneDetails {state: "ND", zone: "050", zone_numeric: 050, name: "McIntosh", wfo: "BIS" },
ForecastZone::ND051 => crate::ZoneDetails {state: "ND", zone: "051", zone_numeric: 051, name: "Dickey", wfo: "BIS" },
ForecastZone::ND052 => crate::ZoneDetails {state: "ND", zone: "052", zone_numeric: 052, name: "Sargent", wfo: "FGF" },
ForecastZone::ND053 => crate::ZoneDetails {state: "ND", zone: "053", zone_numeric: 053, name: "Richland", wfo: "FGF" },
ForecastZone::ND054 => crate::ZoneDetails {state: "ND", zone: "054", zone_numeric: 054, name: "Western Walsh County", wfo: "FGF" },
ForecastZone::NE002 => crate::ZoneDetails {state: "NE", zone: "002", zone_numeric: 002, name: "Dawes", wfo: "CYS" },
ForecastZone::NE003 => crate::ZoneDetails {state: "NE", zone: "003", zone_numeric: 003, name: "Box Butte", wfo: "CYS" },
ForecastZone::NE004 => crate::ZoneDetails {state: "NE", zone: "004", zone_numeric: 004, name: "Sheridan", wfo: "LBF" },
ForecastZone::NE005 => crate::ZoneDetails {state: "NE", zone: "005", zone_numeric: 005, name: "Eastern Cherry", wfo: "LBF" },
ForecastZone::NE006 => crate::ZoneDetails {state: "NE", zone: "006", zone_numeric: 006, name: "Keya Paha", wfo: "LBF" },
ForecastZone::NE007 => crate::ZoneDetails {state: "NE", zone: "007", zone_numeric: 007, name: "Boyd", wfo: "LBF" },
ForecastZone::NE008 => crate::ZoneDetails {state: "NE", zone: "008", zone_numeric: 008, name: "Brown", wfo: "LBF" },
ForecastZone::NE009 => crate::ZoneDetails {state: "NE", zone: "009", zone_numeric: 009, name: "Rock", wfo: "LBF" },
ForecastZone::NE010 => crate::ZoneDetails {state: "NE", zone: "010", zone_numeric: 010, name: "Holt", wfo: "LBF" },
ForecastZone::NE011 => crate::ZoneDetails {state: "NE", zone: "011", zone_numeric: 011, name: "Knox", wfo: "OAX" },
ForecastZone::NE012 => crate::ZoneDetails {state: "NE", zone: "012", zone_numeric: 012, name: "Cedar", wfo: "OAX" },
ForecastZone::NE013 => crate::ZoneDetails {state: "NE", zone: "013", zone_numeric: 013, name: "Dixon", wfo: "FSD" },
ForecastZone::NE014 => crate::ZoneDetails {state: "NE", zone: "014", zone_numeric: 014, name: "Dakota", wfo: "FSD" },
ForecastZone::NE015 => crate::ZoneDetails {state: "NE", zone: "015", zone_numeric: 015, name: "Thurston", wfo: "OAX" },
ForecastZone::NE016 => crate::ZoneDetails {state: "NE", zone: "016", zone_numeric: 016, name: "Antelope", wfo: "OAX" },
ForecastZone::NE017 => crate::ZoneDetails {state: "NE", zone: "017", zone_numeric: 017, name: "Pierce", wfo: "OAX" },
ForecastZone::NE018 => crate::ZoneDetails {state: "NE", zone: "018", zone_numeric: 018, name: "Wayne", wfo: "OAX" },
ForecastZone::NE019 => crate::ZoneDetails {state: "NE", zone: "019", zone_numeric: 019, name: "Scotts Bluff", wfo: "CYS" },
ForecastZone::NE020 => crate::ZoneDetails {state: "NE", zone: "020", zone_numeric: 020, name: "Banner", wfo: "CYS" },
ForecastZone::NE021 => crate::ZoneDetails {state: "NE", zone: "021", zone_numeric: 021, name: "Morrill", wfo: "CYS" },
ForecastZone::NE022 => crate::ZoneDetails {state: "NE", zone: "022", zone_numeric: 022, name: "Garden", wfo: "LBF" },
ForecastZone::NE023 => crate::ZoneDetails {state: "NE", zone: "023", zone_numeric: 023, name: "Grant", wfo: "LBF" },
ForecastZone::NE024 => crate::ZoneDetails {state: "NE", zone: "024", zone_numeric: 024, name: "Hooker", wfo: "LBF" },
ForecastZone::NE025 => crate::ZoneDetails {state: "NE", zone: "025", zone_numeric: 025, name: "Thomas", wfo: "LBF" },
ForecastZone::NE026 => crate::ZoneDetails {state: "NE", zone: "026", zone_numeric: 026, name: "Blaine", wfo: "LBF" },
ForecastZone::NE027 => crate::ZoneDetails {state: "NE", zone: "027", zone_numeric: 027, name: "Loup", wfo: "LBF" },
ForecastZone::NE028 => crate::ZoneDetails {state: "NE", zone: "028", zone_numeric: 028, name: "Garfield", wfo: "LBF" },
ForecastZone::NE029 => crate::ZoneDetails {state: "NE", zone: "029", zone_numeric: 029, name: "Wheeler", wfo: "LBF" },
ForecastZone::NE030 => crate::ZoneDetails {state: "NE", zone: "030", zone_numeric: 030, name: "Boone", wfo: "OAX" },
ForecastZone::NE031 => crate::ZoneDetails {state: "NE", zone: "031", zone_numeric: 031, name: "Madison", wfo: "OAX" },
ForecastZone::NE032 => crate::ZoneDetails {state: "NE", zone: "032", zone_numeric: 032, name: "Stanton", wfo: "OAX" },
ForecastZone::NE033 => crate::ZoneDetails {state: "NE", zone: "033", zone_numeric: 033, name: "Cuming", wfo: "OAX" },
ForecastZone::NE034 => crate::ZoneDetails {state: "NE", zone: "034", zone_numeric: 034, name: "Burt", wfo: "OAX" },
ForecastZone::NE035 => crate::ZoneDetails {state: "NE", zone: "035", zone_numeric: 035, name: "Arthur", wfo: "LBF" },
ForecastZone::NE036 => crate::ZoneDetails {state: "NE", zone: "036", zone_numeric: 036, name: "McPherson", wfo: "LBF" },
ForecastZone::NE037 => crate::ZoneDetails {state: "NE", zone: "037", zone_numeric: 037, name: "Logan", wfo: "LBF" },
ForecastZone::NE038 => crate::ZoneDetails {state: "NE", zone: "038", zone_numeric: 038, name: "Custer", wfo: "LBF" },
ForecastZone::NE039 => crate::ZoneDetails {state: "NE", zone: "039", zone_numeric: 039, name: "Valley", wfo: "GID" },
ForecastZone::NE040 => crate::ZoneDetails {state: "NE", zone: "040", zone_numeric: 040, name: "Greeley", wfo: "GID" },
ForecastZone::NE041 => crate::ZoneDetails {state: "NE", zone: "041", zone_numeric: 041, name: "Nance", wfo: "GID" },
ForecastZone::NE042 => crate::ZoneDetails {state: "NE", zone: "042", zone_numeric: 042, name: "Platte", wfo: "OAX" },
ForecastZone::NE043 => crate::ZoneDetails {state: "NE", zone: "043", zone_numeric: 043, name: "Colfax", wfo: "OAX" },
ForecastZone::NE044 => crate::ZoneDetails {state: "NE", zone: "044", zone_numeric: 044, name: "Dodge", wfo: "OAX" },
ForecastZone::NE045 => crate::ZoneDetails {state: "NE", zone: "045", zone_numeric: 045, name: "Washington", wfo: "OAX" },
ForecastZone::NE046 => crate::ZoneDetails {state: "NE", zone: "046", zone_numeric: 046, name: "Sherman", wfo: "GID" },
ForecastZone::NE047 => crate::ZoneDetails {state: "NE", zone: "047", zone_numeric: 047, name: "Howard", wfo: "GID" },
ForecastZone::NE048 => crate::ZoneDetails {state: "NE", zone: "048", zone_numeric: 048, name: "Merrick", wfo: "GID" },
ForecastZone::NE049 => crate::ZoneDetails {state: "NE", zone: "049", zone_numeric: 049, name: "Polk", wfo: "GID" },
ForecastZone::NE050 => crate::ZoneDetails {state: "NE", zone: "050", zone_numeric: 050, name: "Butler", wfo: "OAX" },
ForecastZone::NE051 => crate::ZoneDetails {state: "NE", zone: "051", zone_numeric: 051, name: "Saunders", wfo: "OAX" },
ForecastZone::NE052 => crate::ZoneDetails {state: "NE", zone: "052", zone_numeric: 052, name: "Douglas", wfo: "OAX" },
ForecastZone::NE053 => crate::ZoneDetails {state: "NE", zone: "053", zone_numeric: 053, name: "Sarpy", wfo: "OAX" },
ForecastZone::NE054 => crate::ZoneDetails {state: "NE", zone: "054", zone_numeric: 054, name: "Kimball", wfo: "CYS" },
ForecastZone::NE055 => crate::ZoneDetails {state: "NE", zone: "055", zone_numeric: 055, name: "Cheyenne", wfo: "CYS" },
ForecastZone::NE056 => crate::ZoneDetails {state: "NE", zone: "056", zone_numeric: 056, name: "Deuel", wfo: "LBF" },
ForecastZone::NE057 => crate::ZoneDetails {state: "NE", zone: "057", zone_numeric: 057, name: "Keith", wfo: "LBF" },
ForecastZone::NE058 => crate::ZoneDetails {state: "NE", zone: "058", zone_numeric: 058, name: "Perkins", wfo: "LBF" },
ForecastZone::NE059 => crate::ZoneDetails {state: "NE", zone: "059", zone_numeric: 059, name: "Lincoln", wfo: "LBF" },
ForecastZone::NE060 => crate::ZoneDetails {state: "NE", zone: "060", zone_numeric: 060, name: "Dawson", wfo: "GID" },
ForecastZone::NE061 => crate::ZoneDetails {state: "NE", zone: "061", zone_numeric: 061, name: "Buffalo", wfo: "GID" },
ForecastZone::NE062 => crate::ZoneDetails {state: "NE", zone: "062", zone_numeric: 062, name: "Hall", wfo: "GID" },
ForecastZone::NE063 => crate::ZoneDetails {state: "NE", zone: "063", zone_numeric: 063, name: "Hamilton", wfo: "GID" },
ForecastZone::NE064 => crate::ZoneDetails {state: "NE", zone: "064", zone_numeric: 064, name: "York", wfo: "GID" },
ForecastZone::NE065 => crate::ZoneDetails {state: "NE", zone: "065", zone_numeric: 065, name: "Seward", wfo: "OAX" },
ForecastZone::NE066 => crate::ZoneDetails {state: "NE", zone: "066", zone_numeric: 066, name: "Lancaster", wfo: "OAX" },
ForecastZone::NE067 => crate::ZoneDetails {state: "NE", zone: "067", zone_numeric: 067, name: "Cass", wfo: "OAX" },
ForecastZone::NE068 => crate::ZoneDetails {state: "NE", zone: "068", zone_numeric: 068, name: "Otoe", wfo: "OAX" },
ForecastZone::NE069 => crate::ZoneDetails {state: "NE", zone: "069", zone_numeric: 069, name: "Chase", wfo: "LBF" },
ForecastZone::NE070 => crate::ZoneDetails {state: "NE", zone: "070", zone_numeric: 070, name: "Hayes", wfo: "LBF" },
ForecastZone::NE071 => crate::ZoneDetails {state: "NE", zone: "071", zone_numeric: 071, name: "Frontier", wfo: "LBF" },
ForecastZone::NE072 => crate::ZoneDetails {state: "NE", zone: "072", zone_numeric: 072, name: "Gosper", wfo: "GID" },
ForecastZone::NE073 => crate::ZoneDetails {state: "NE", zone: "073", zone_numeric: 073, name: "Phelps", wfo: "GID" },
ForecastZone::NE074 => crate::ZoneDetails {state: "NE", zone: "074", zone_numeric: 074, name: "Kearney", wfo: "GID" },
ForecastZone::NE075 => crate::ZoneDetails {state: "NE", zone: "075", zone_numeric: 075, name: "Adams", wfo: "GID" },
ForecastZone::NE076 => crate::ZoneDetails {state: "NE", zone: "076", zone_numeric: 076, name: "Clay", wfo: "GID" },
ForecastZone::NE077 => crate::ZoneDetails {state: "NE", zone: "077", zone_numeric: 077, name: "Fillmore", wfo: "GID" },
ForecastZone::NE078 => crate::ZoneDetails {state: "NE", zone: "078", zone_numeric: 078, name: "Saline", wfo: "OAX" },
ForecastZone::NE079 => crate::ZoneDetails {state: "NE", zone: "079", zone_numeric: 079, name: "Dundy", wfo: "GLD" },
ForecastZone::NE080 => crate::ZoneDetails {state: "NE", zone: "080", zone_numeric: 080, name: "Hitchcock", wfo: "GLD" },
ForecastZone::NE081 => crate::ZoneDetails {state: "NE", zone: "081", zone_numeric: 081, name: "Red Willow", wfo: "GLD" },
ForecastZone::NE082 => crate::ZoneDetails {state: "NE", zone: "082", zone_numeric: 082, name: "Furnas", wfo: "GID" },
ForecastZone::NE083 => crate::ZoneDetails {state: "NE", zone: "083", zone_numeric: 083, name: "Harlan", wfo: "GID" },
ForecastZone::NE084 => crate::ZoneDetails {state: "NE", zone: "084", zone_numeric: 084, name: "Franklin", wfo: "GID" },
ForecastZone::NE085 => crate::ZoneDetails {state: "NE", zone: "085", zone_numeric: 085, name: "Webster", wfo: "GID" },
ForecastZone::NE086 => crate::ZoneDetails {state: "NE", zone: "086", zone_numeric: 086, name: "Nuckolls", wfo: "GID" },
ForecastZone::NE087 => crate::ZoneDetails {state: "NE", zone: "087", zone_numeric: 087, name: "Thayer", wfo: "GID" },
ForecastZone::NE088 => crate::ZoneDetails {state: "NE", zone: "088", zone_numeric: 088, name: "Jefferson", wfo: "OAX" },
ForecastZone::NE089 => crate::ZoneDetails {state: "NE", zone: "089", zone_numeric: 089, name: "Gage", wfo: "OAX" },
ForecastZone::NE090 => crate::ZoneDetails {state: "NE", zone: "090", zone_numeric: 090, name: "Johnson", wfo: "OAX" },
ForecastZone::NE091 => crate::ZoneDetails {state: "NE", zone: "091", zone_numeric: 091, name: "Nemaha", wfo: "OAX" },
ForecastZone::NE092 => crate::ZoneDetails {state: "NE", zone: "092", zone_numeric: 092, name: "Pawnee", wfo: "OAX" },
ForecastZone::NE093 => crate::ZoneDetails {state: "NE", zone: "093", zone_numeric: 093, name: "Richardson", wfo: "OAX" },
ForecastZone::NE094 => crate::ZoneDetails {state: "NE", zone: "094", zone_numeric: 094, name: "Western Cherry", wfo: "LBF" },
ForecastZone::NE095 => crate::ZoneDetails {state: "NE", zone: "095", zone_numeric: 095, name: "North Sioux", wfo: "CYS" },
ForecastZone::NE096 => crate::ZoneDetails {state: "NE", zone: "096", zone_numeric: 096, name: "South Sioux", wfo: "CYS" },
ForecastZone::NH001 => crate::ZoneDetails {state: "NH", zone: "001", zone_numeric: 001, name: "Northern Coos", wfo: "GYX" },
ForecastZone::NH002 => crate::ZoneDetails {state: "NH", zone: "002", zone_numeric: 002, name: "Southern Coos", wfo: "GYX" },
ForecastZone::NH003 => crate::ZoneDetails {state: "NH", zone: "003", zone_numeric: 003, name: "Northern Grafton", wfo: "GYX" },
ForecastZone::NH004 => crate::ZoneDetails {state: "NH", zone: "004", zone_numeric: 004, name: "Northern Carroll", wfo: "GYX" },
ForecastZone::NH005 => crate::ZoneDetails {state: "NH", zone: "005", zone_numeric: 005, name: "Southern Grafton", wfo: "GYX" },
ForecastZone::NH006 => crate::ZoneDetails {state: "NH", zone: "006", zone_numeric: 006, name: "Southern Carroll", wfo: "GYX" },
ForecastZone::NH007 => crate::ZoneDetails {state: "NH", zone: "007", zone_numeric: 007, name: "Sullivan", wfo: "GYX" },
ForecastZone::NH008 => crate::ZoneDetails {state: "NH", zone: "008", zone_numeric: 008, name: "Merrimack", wfo: "GYX" },
ForecastZone::NH009 => crate::ZoneDetails {state: "NH", zone: "009", zone_numeric: 009, name: "Belknap", wfo: "GYX" },
ForecastZone::NH010 => crate::ZoneDetails {state: "NH", zone: "010", zone_numeric: 010, name: "Strafford", wfo: "GYX" },
ForecastZone::NH011 => crate::ZoneDetails {state: "NH", zone: "011", zone_numeric: 011, name: "Cheshire", wfo: "GYX" },
ForecastZone::NH012 => crate::ZoneDetails {state: "NH", zone: "012", zone_numeric: 012, name: "Eastern Hillsborough", wfo: "GYX" },
ForecastZone::NH013 => crate::ZoneDetails {state: "NH", zone: "013", zone_numeric: 013, name: "Interior Rockingham", wfo: "GYX" },
ForecastZone::NH014 => crate::ZoneDetails {state: "NH", zone: "014", zone_numeric: 014, name: "Coastal Rockingham", wfo: "GYX" },
ForecastZone::NH015 => crate::ZoneDetails {state: "NH", zone: "015", zone_numeric: 015, name: "Western And Central Hillsborough", wfo: "GYX" },
ForecastZone::NJ001 => crate::ZoneDetails {state: "NJ", zone: "001", zone_numeric: 001, name: "Sussex", wfo: "PHI" },
ForecastZone::NJ002 => crate::ZoneDetails {state: "NJ", zone: "002", zone_numeric: 002, name: "Western Passaic", wfo: "OKX" },
ForecastZone::NJ004 => crate::ZoneDetails {state: "NJ", zone: "004", zone_numeric: 004, name: "Eastern Passaic", wfo: "OKX" },
ForecastZone::NJ006 => crate::ZoneDetails {state: "NJ", zone: "006", zone_numeric: 006, name: "Hudson", wfo: "OKX" },
ForecastZone::NJ007 => crate::ZoneDetails {state: "NJ", zone: "007", zone_numeric: 007, name: "Warren", wfo: "PHI" },
ForecastZone::NJ008 => crate::ZoneDetails {state: "NJ", zone: "008", zone_numeric: 008, name: "Morris", wfo: "PHI" },
ForecastZone::NJ009 => crate::ZoneDetails {state: "NJ", zone: "009", zone_numeric: 009, name: "Hunterdon", wfo: "PHI" },
ForecastZone::NJ010 => crate::ZoneDetails {state: "NJ", zone: "010", zone_numeric: 010, name: "Somerset", wfo: "PHI" },
ForecastZone::NJ012 => crate::ZoneDetails {state: "NJ", zone: "012", zone_numeric: 012, name: "Middlesex", wfo: "PHI" },
ForecastZone::NJ013 => crate::ZoneDetails {state: "NJ", zone: "013", zone_numeric: 013, name: "Western Monmouth", wfo: "PHI" },
ForecastZone::NJ014 => crate::ZoneDetails {state: "NJ", zone: "014", zone_numeric: 014, name: "Eastern Monmouth", wfo: "PHI" },
ForecastZone::NJ015 => crate::ZoneDetails {state: "NJ", zone: "015", zone_numeric: 015, name: "Mercer", wfo: "PHI" },
ForecastZone::NJ016 => crate::ZoneDetails {state: "NJ", zone: "016", zone_numeric: 016, name: "Salem", wfo: "PHI" },
ForecastZone::NJ017 => crate::ZoneDetails {state: "NJ", zone: "017", zone_numeric: 017, name: "Gloucester", wfo: "PHI" },
ForecastZone::NJ018 => crate::ZoneDetails {state: "NJ", zone: "018", zone_numeric: 018, name: "Camden", wfo: "PHI" },
ForecastZone::NJ019 => crate::ZoneDetails {state: "NJ", zone: "019", zone_numeric: 019, name: "Northwestern Burlington", wfo: "PHI" },
ForecastZone::NJ020 => crate::ZoneDetails {state: "NJ", zone: "020", zone_numeric: 020, name: "Ocean", wfo: "PHI" },
ForecastZone::NJ021 => crate::ZoneDetails {state: "NJ", zone: "021", zone_numeric: 021, name: "Cumberland", wfo: "PHI" },
ForecastZone::NJ022 => crate::ZoneDetails {state: "NJ", zone: "022", zone_numeric: 022, name: "Atlantic", wfo: "PHI" },
ForecastZone::NJ023 => crate::ZoneDetails {state: "NJ", zone: "023", zone_numeric: 023, name: "Cape May", wfo: "PHI" },
ForecastZone::NJ024 => crate::ZoneDetails {state: "NJ", zone: "024", zone_numeric: 024, name: "Atlantic Coastal Cape May", wfo: "PHI" },
ForecastZone::NJ025 => crate::ZoneDetails {state: "NJ", zone: "025", zone_numeric: 025, name: "Coastal Atlantic", wfo: "PHI" },
ForecastZone::NJ026 => crate::ZoneDetails {state: "NJ", zone: "026", zone_numeric: 026, name: "Coastal Ocean", wfo: "PHI" },
ForecastZone::NJ027 => crate::ZoneDetails {state: "NJ", zone: "027", zone_numeric: 027, name: "Southeastern Burlington", wfo: "PHI" },
ForecastZone::NJ103 => crate::ZoneDetails {state: "NJ", zone: "103", zone_numeric: 103, name: "Western Bergen", wfo: "OKX" },
ForecastZone::NJ104 => crate::ZoneDetails {state: "NJ", zone: "104", zone_numeric: 104, name: "Eastern Bergen", wfo: "OKX" },
ForecastZone::NJ105 => crate::ZoneDetails {state: "NJ", zone: "105", zone_numeric: 105, name: "Western Essex", wfo: "OKX" },
ForecastZone::NJ106 => crate::ZoneDetails {state: "NJ", zone: "106", zone_numeric: 106, name: "Eastern Essex", wfo: "OKX" },
ForecastZone::NJ107 => crate::ZoneDetails {state: "NJ", zone: "107", zone_numeric: 107, name: "Western Union", wfo: "OKX" },
ForecastZone::NJ108 => crate::ZoneDetails {state: "NJ", zone: "108", zone_numeric: 108, name: "Eastern Union", wfo: "OKX" },
ForecastZone::NM027 => crate::ZoneDetails {state: "NM", zone: "027", zone_numeric: 027, name: "Guadalupe Mountains of Eddy County", wfo: "MAF" },
ForecastZone::NM028 => crate::ZoneDetails {state: "NM", zone: "028", zone_numeric: 028, name: "Eddy County Plains", wfo: "MAF" },
ForecastZone::NM029 => crate::ZoneDetails {state: "NM", zone: "029", zone_numeric: 029, name: "Northern Lea County", wfo: "MAF" },
ForecastZone::NM033 => crate::ZoneDetails {state: "NM", zone: "033", zone_numeric: 033, name: "Central Lea County", wfo: "MAF" },
ForecastZone::NM034 => crate::ZoneDetails {state: "NM", zone: "034", zone_numeric: 034, name: "Southern Lea County", wfo: "MAF" },
ForecastZone::NM201 => crate::ZoneDetails {state: "NM", zone: "201", zone_numeric: 201, name: "Northwest Plateau", wfo: "ABQ" },
ForecastZone::NM202 => crate::ZoneDetails {state: "NM", zone: "202", zone_numeric: 202, name: "Chuska Mountains", wfo: "ABQ" },
ForecastZone::NM203 => crate::ZoneDetails {state: "NM", zone: "203", zone_numeric: 203, name: "Far Northwest Highlands", wfo: "ABQ" },
ForecastZone::NM204 => crate::ZoneDetails {state: "NM", zone: "204", zone_numeric: 204, name: "Northwest Highlands", wfo: "ABQ" },
ForecastZone::NM205 => crate::ZoneDetails {state: "NM", zone: "205", zone_numeric: 205, name: "West Central Plateau", wfo: "ABQ" },
ForecastZone::NM206 => crate::ZoneDetails {state: "NM", zone: "206", zone_numeric: 206, name: "West Central Mountains", wfo: "ABQ" },
ForecastZone::NM207 => crate::ZoneDetails {state: "NM", zone: "207", zone_numeric: 207, name: "West Central Highlands", wfo: "ABQ" },
ForecastZone::NM208 => crate::ZoneDetails {state: "NM", zone: "208", zone_numeric: 208, name: "Southwest Mountains", wfo: "ABQ" },
ForecastZone::NM209 => crate::ZoneDetails {state: "NM", zone: "209", zone_numeric: 209, name: "San Francisco River Valley", wfo: "ABQ" },
ForecastZone::NM210 => crate::ZoneDetails {state: "NM", zone: "210", zone_numeric: 210, name: "Tusas Mountains Including Chama", wfo: "ABQ" },
ForecastZone::NM211 => crate::ZoneDetails {state: "NM", zone: "211", zone_numeric: 211, name: "Jemez Mountains", wfo: "ABQ" },
ForecastZone::NM212 => crate::ZoneDetails {state: "NM", zone: "212", zone_numeric: 212, name: "Glorieta Mesa Including Glorieta Pass", wfo: "ABQ" },
ForecastZone::NM213 => crate::ZoneDetails {state: "NM", zone: "213", zone_numeric: 213, name: "Northern Sangre de Cristo Mountains", wfo: "ABQ" },
ForecastZone::NM214 => crate::ZoneDetails {state: "NM", zone: "214", zone_numeric: 214, name: "Southern Sangre de Cristo Mountains", wfo: "ABQ" },
ForecastZone::NM215 => crate::ZoneDetails {state: "NM", zone: "215", zone_numeric: 215, name: "East Slopes Sangre de Cristo Mountains", wfo: "ABQ" },
ForecastZone::NM216 => crate::ZoneDetails {state: "NM", zone: "216", zone_numeric: 216, name: "Upper Rio Grande Valley", wfo: "ABQ" },
ForecastZone::NM217 => crate::ZoneDetails {state: "NM", zone: "217", zone_numeric: 217, name: "Espanola Valley", wfo: "ABQ" },
ForecastZone::NM218 => crate::ZoneDetails {state: "NM", zone: "218", zone_numeric: 218, name: "Santa Fe Metro Area", wfo: "ABQ" },
ForecastZone::NM219 => crate::ZoneDetails {state: "NM", zone: "219", zone_numeric: 219, name: "Middle Rio Grande Valley/Albuquerque Metro Area", wfo: "ABQ" },
ForecastZone::NM220 => crate::ZoneDetails {state: "NM", zone: "220", zone_numeric: 220, name: "Lower Rio Grande Valley", wfo: "ABQ" },
ForecastZone::NM221 => crate::ZoneDetails {state: "NM", zone: "221", zone_numeric: 221, name: "Sandia/Manzano Mountains Including Edgewood", wfo: "ABQ" },
ForecastZone::NM222 => crate::ZoneDetails {state: "NM", zone: "222", zone_numeric: 222, name: "Estancia Valley", wfo: "ABQ" },
ForecastZone::NM223 => crate::ZoneDetails {state: "NM", zone: "223", zone_numeric: 223, name: "Central Highlands", wfo: "ABQ" },
ForecastZone::NM224 => crate::ZoneDetails {state: "NM", zone: "224", zone_numeric: 224, name: "South Central Highlands", wfo: "ABQ" },
ForecastZone::NM225 => crate::ZoneDetails {state: "NM", zone: "225", zone_numeric: 225, name: "Upper Tularosa Valley", wfo: "ABQ" },
ForecastZone::NM226 => crate::ZoneDetails {state: "NM", zone: "226", zone_numeric: 226, name: "South Central Mountains", wfo: "ABQ" },
ForecastZone::NM227 => crate::ZoneDetails {state: "NM", zone: "227", zone_numeric: 227, name: "Johnson and Bartlett Mesas Including Raton Pass", wfo: "ABQ" },
ForecastZone::NM228 => crate::ZoneDetails {state: "NM", zone: "228", zone_numeric: 228, name: "Far Northeast Highlands", wfo: "ABQ" },
ForecastZone::NM229 => crate::ZoneDetails {state: "NM", zone: "229", zone_numeric: 229, name: "Northeast Highlands", wfo: "ABQ" },
ForecastZone::NM230 => crate::ZoneDetails {state: "NM", zone: "230", zone_numeric: 230, name: "Union County", wfo: "ABQ" },
ForecastZone::NM231 => crate::ZoneDetails {state: "NM", zone: "231", zone_numeric: 231, name: "Harding County", wfo: "ABQ" },
ForecastZone::NM232 => crate::ZoneDetails {state: "NM", zone: "232", zone_numeric: 232, name: "Eastern San Miguel County", wfo: "ABQ" },
ForecastZone::NM233 => crate::ZoneDetails {state: "NM", zone: "233", zone_numeric: 233, name: "Guadalupe County", wfo: "ABQ" },
ForecastZone::NM234 => crate::ZoneDetails {state: "NM", zone: "234", zone_numeric: 234, name: "Quay County", wfo: "ABQ" },
ForecastZone::NM235 => crate::ZoneDetails {state: "NM", zone: "235", zone_numeric: 235, name: "Curry County", wfo: "ABQ" },
ForecastZone::NM236 => crate::ZoneDetails {state: "NM", zone: "236", zone_numeric: 236, name: "Roosevelt County", wfo: "ABQ" },
ForecastZone::NM237 => crate::ZoneDetails {state: "NM", zone: "237", zone_numeric: 237, name: "De Baca County", wfo: "ABQ" },
ForecastZone::NM238 => crate::ZoneDetails {state: "NM", zone: "238", zone_numeric: 238, name: "Chaves County Plains", wfo: "ABQ" },
ForecastZone::NM239 => crate::ZoneDetails {state: "NM", zone: "239", zone_numeric: 239, name: "Eastern Lincoln County", wfo: "ABQ" },
ForecastZone::NM240 => crate::ZoneDetails {state: "NM", zone: "240", zone_numeric: 240, name: "Southwest Chaves County", wfo: "ABQ" },
ForecastZone::NM241 => crate::ZoneDetails {state: "NM", zone: "241", zone_numeric: 241, name: "San Agustin Plains and Adjacent Lowlands", wfo: "ABQ" },
ForecastZone::NM401 => crate::ZoneDetails {state: "NM", zone: "401", zone_numeric: 401, name: "Upper Gila River Valley", wfo: "EPZ" },
ForecastZone::NM402 => crate::ZoneDetails {state: "NM", zone: "402", zone_numeric: 402, name: "Southern Gila Highlands/Black Range", wfo: "EPZ" },
ForecastZone::NM403 => crate::ZoneDetails {state: "NM", zone: "403", zone_numeric: 403, name: "Southern Gila Foothills/Mimbres Valley", wfo: "EPZ" },
ForecastZone::NM404 => crate::ZoneDetails {state: "NM", zone: "404", zone_numeric: 404, name: "Southwest Desert/Lower Gila River Valley", wfo: "EPZ" },
ForecastZone::NM405 => crate::ZoneDetails {state: "NM", zone: "405", zone_numeric: 405, name: "Lowlands of the Bootheel", wfo: "EPZ" },
ForecastZone::NM406 => crate::ZoneDetails {state: "NM", zone: "406", zone_numeric: 406, name: "Uplands of the Bootheel", wfo: "EPZ" },
ForecastZone::NM407 => crate::ZoneDetails {state: "NM", zone: "407", zone_numeric: 407, name: "Southwest Desert/Mimbres Basin", wfo: "EPZ" },
ForecastZone::NM408 => crate::ZoneDetails {state: "NM", zone: "408", zone_numeric: 408, name: "Eastern Black Range Foothills", wfo: "EPZ" },
ForecastZone::NM409 => crate::ZoneDetails {state: "NM", zone: "409", zone_numeric: 409, name: "Sierra County Lakes", wfo: "EPZ" },
ForecastZone::NM410 => crate::ZoneDetails {state: "NM", zone: "410", zone_numeric: 410, name: "Northern Dona Ana County", wfo: "EPZ" },
ForecastZone::NM411 => crate::ZoneDetails {state: "NM", zone: "411", zone_numeric: 411, name: "Southern Dona Ana County/Mesilla Valley", wfo: "EPZ" },
ForecastZone::NM412 => crate::ZoneDetails {state: "NM", zone: "412", zone_numeric: 412, name: "Central Tularosa Basin", wfo: "EPZ" },
ForecastZone::NM413 => crate::ZoneDetails {state: "NM", zone: "413", zone_numeric: 413, name: "Southern Tularosa Basin", wfo: "EPZ" },
ForecastZone::NM414 => crate::ZoneDetails {state: "NM", zone: "414", zone_numeric: 414, name: "West Slopes Sacramento Mountains Below 7500 Feet", wfo: "EPZ" },
ForecastZone::NM415 => crate::ZoneDetails {state: "NM", zone: "415", zone_numeric: 415, name: "Sacramento Mountains Above 7500 Feet", wfo: "EPZ" },
ForecastZone::NM416 => crate::ZoneDetails {state: "NM", zone: "416", zone_numeric: 416, name: "East Slopes Sacramento Mountains Below 7500 Feet", wfo: "EPZ" },
ForecastZone::NM417 => crate::ZoneDetails {state: "NM", zone: "417", zone_numeric: 417, name: "Otero Mesa", wfo: "EPZ" },
ForecastZone::NV001 => crate::ZoneDetails {state: "NV", zone: "001", zone_numeric: 001, name: "Mineral and Southern Lyon Counties", wfo: "REV" },
ForecastZone::NV002 => crate::ZoneDetails {state: "NV", zone: "002", zone_numeric: 002, name: "Greater Lake Tahoe Area", wfo: "REV" },
ForecastZone::NV003 => crate::ZoneDetails {state: "NV", zone: "003", zone_numeric: 003, name: "Greater Reno-Carson City-Minden Area", wfo: "REV" },
ForecastZone::NV004 => crate::ZoneDetails {state: "NV", zone: "004", zone_numeric: 004, name: "Western Nevada Basin and Range including Pyramid Lake", wfo: "REV" },
ForecastZone::NV005 => crate::ZoneDetails {state: "NV", zone: "005", zone_numeric: 005, name: "Northern Washoe County", wfo: "REV" },
ForecastZone::NV014 => crate::ZoneDetails {state: "NV", zone: "014", zone_numeric: 014, name: "Esmeralda and Central Nye County", wfo: "VEF" },
ForecastZone::NV015 => crate::ZoneDetails {state: "NV", zone: "015", zone_numeric: 015, name: "Lincoln County", wfo: "VEF" },
ForecastZone::NV016 => crate::ZoneDetails {state: "NV", zone: "016", zone_numeric: 016, name: "Northeast Clark County", wfo: "VEF" },
ForecastZone::NV017 => crate::ZoneDetails {state: "NV", zone: "017", zone_numeric: 017, name: "Western Clark and Southern Nye County", wfo: "VEF" },
ForecastZone::NV018 => crate::ZoneDetails {state: "NV", zone: "018", zone_numeric: 018, name: "Sheep Range", wfo: "VEF" },
ForecastZone::NV019 => crate::ZoneDetails {state: "NV", zone: "019", zone_numeric: 019, name: "Spring Mountains-Red Rock Canyon", wfo: "VEF" },
ForecastZone::NV020 => crate::ZoneDetails {state: "NV", zone: "020", zone_numeric: 020, name: "Las Vegas Valley", wfo: "VEF" },
ForecastZone::NV021 => crate::ZoneDetails {state: "NV", zone: "021", zone_numeric: 021, name: "Lake Mead National Recreation Area", wfo: "VEF" },
ForecastZone::NV022 => crate::ZoneDetails {state: "NV", zone: "022", zone_numeric: 022, name: "Southern Clark County", wfo: "VEF" },
ForecastZone::NV030 => crate::ZoneDetails {state: "NV", zone: "030", zone_numeric: 030, name: "Humboldt County", wfo: "LKN" },
ForecastZone::NV031 => crate::ZoneDetails {state: "NV", zone: "031", zone_numeric: 031, name: "Northern Elko County", wfo: "LKN" },
ForecastZone::NV033 => crate::ZoneDetails {state: "NV", zone: "033", zone_numeric: 033, name: "Southeastern Elko County", wfo: "LKN" },
ForecastZone::NV034 => crate::ZoneDetails {state: "NV", zone: "034", zone_numeric: 034, name: "Ruby Mountains and East Humboldt Range", wfo: "LKN" },
ForecastZone::NV035 => crate::ZoneDetails {state: "NV", zone: "035", zone_numeric: 035, name: "White Pine County", wfo: "LKN" },
ForecastZone::NV036 => crate::ZoneDetails {state: "NV", zone: "036", zone_numeric: 036, name: "Northern Lander County and Northern Eureka County", wfo: "LKN" },
ForecastZone::NV037 => crate::ZoneDetails {state: "NV", zone: "037", zone_numeric: 037, name: "Southern Lander County and Southern Eureka County", wfo: "LKN" },
ForecastZone::NV038 => crate::ZoneDetails {state: "NV", zone: "038", zone_numeric: 038, name: "Southwest Elko County", wfo: "LKN" },
ForecastZone::NV039 => crate::ZoneDetails {state: "NV", zone: "039", zone_numeric: 039, name: "South Central Elko County", wfo: "LKN" },
ForecastZone::NV040 => crate::ZoneDetails {state: "NV", zone: "040", zone_numeric: 040, name: "Northwestern Nye County", wfo: "LKN" },
ForecastZone::NV041 => crate::ZoneDetails {state: "NV", zone: "041", zone_numeric: 041, name: "Northeastern Nye County", wfo: "LKN" },
ForecastZone::NY001 => crate::ZoneDetails {state: "NY", zone: "001", zone_numeric: 001, name: "Niagara", wfo: "BUF" },
ForecastZone::NY002 => crate::ZoneDetails {state: "NY", zone: "002", zone_numeric: 002, name: "Orleans", wfo: "BUF" },
ForecastZone::NY003 => crate::ZoneDetails {state: "NY", zone: "003", zone_numeric: 003, name: "Monroe", wfo: "BUF" },
ForecastZone::NY004 => crate::ZoneDetails {state: "NY", zone: "004", zone_numeric: 004, name: "Wayne", wfo: "BUF" },
ForecastZone::NY005 => crate::ZoneDetails {state: "NY", zone: "005", zone_numeric: 005, name: "Northern Cayuga", wfo: "BUF" },
ForecastZone::NY006 => crate::ZoneDetails {state: "NY", zone: "006", zone_numeric: 006, name: "Oswego", wfo: "BUF" },
ForecastZone::NY007 => crate::ZoneDetails {state: "NY", zone: "007", zone_numeric: 007, name: "Jefferson", wfo: "BUF" },
ForecastZone::NY008 => crate::ZoneDetails {state: "NY", zone: "008", zone_numeric: 008, name: "Lewis", wfo: "BUF" },
ForecastZone::NY009 => crate::ZoneDetails {state: "NY", zone: "009", zone_numeric: 009, name: "Northern Oneida", wfo: "BGM" },
ForecastZone::NY010 => crate::ZoneDetails {state: "NY", zone: "010", zone_numeric: 010, name: "Northern Erie", wfo: "BUF" },
ForecastZone::NY011 => crate::ZoneDetails {state: "NY", zone: "011", zone_numeric: 011, name: "Genesee", wfo: "BUF" },
ForecastZone::NY012 => crate::ZoneDetails {state: "NY", zone: "012", zone_numeric: 012, name: "Wyoming", wfo: "BUF" },
ForecastZone::NY013 => crate::ZoneDetails {state: "NY", zone: "013", zone_numeric: 013, name: "Livingston", wfo: "BUF" },
ForecastZone::NY014 => crate::ZoneDetails {state: "NY", zone: "014", zone_numeric: 014, name: "Ontario", wfo: "BUF" },
ForecastZone::NY015 => crate::ZoneDetails {state: "NY", zone: "015", zone_numeric: 015, name: "Yates", wfo: "BGM" },
ForecastZone::NY016 => crate::ZoneDetails {state: "NY", zone: "016", zone_numeric: 016, name: "Seneca", wfo: "BGM" },
ForecastZone::NY017 => crate::ZoneDetails {state: "NY", zone: "017", zone_numeric: 017, name: "Southern Cayuga", wfo: "BGM" },
ForecastZone::NY018 => crate::ZoneDetails {state: "NY", zone: "018", zone_numeric: 018, name: "Onondaga", wfo: "BGM" },
ForecastZone::NY019 => crate::ZoneDetails {state: "NY", zone: "019", zone_numeric: 019, name: "Chautauqua", wfo: "BUF" },
ForecastZone::NY020 => crate::ZoneDetails {state: "NY", zone: "020", zone_numeric: 020, name: "Cattaraugus", wfo: "BUF" },
ForecastZone::NY021 => crate::ZoneDetails {state: "NY", zone: "021", zone_numeric: 021, name: "Allegany", wfo: "BUF" },
ForecastZone::NY022 => crate::ZoneDetails {state: "NY", zone: "022", zone_numeric: 022, name: "Steuben", wfo: "BGM" },
ForecastZone::NY023 => crate::ZoneDetails {state: "NY", zone: "023", zone_numeric: 023, name: "Schuyler", wfo: "BGM" },
ForecastZone::NY024 => crate::ZoneDetails {state: "NY", zone: "024", zone_numeric: 024, name: "Chemung", wfo: "BGM" },
ForecastZone::NY025 => crate::ZoneDetails {state: "NY", zone: "025", zone_numeric: 025, name: "Tompkins", wfo: "BGM" },
ForecastZone::NY026 => crate::ZoneDetails {state: "NY", zone: "026", zone_numeric: 026, name: "Northern St. Lawrence", wfo: "BTV" },
ForecastZone::NY027 => crate::ZoneDetails {state: "NY", zone: "027", zone_numeric: 027, name: "Northern Franklin", wfo: "BTV" },
ForecastZone::NY028 => crate::ZoneDetails {state: "NY", zone: "028", zone_numeric: 028, name: "Eastern Clinton", wfo: "BTV" },
ForecastZone::NY029 => crate::ZoneDetails {state: "NY", zone: "029", zone_numeric: 029, name: "Southeastern St. Lawrence", wfo: "BTV" },
ForecastZone::NY030 => crate::ZoneDetails {state: "NY", zone: "030", zone_numeric: 030, name: "Southern Franklin", wfo: "BTV" },
ForecastZone::NY031 => crate::ZoneDetails {state: "NY", zone: "031", zone_numeric: 031, name: "Western Clinton", wfo: "BTV" },
ForecastZone::NY032 => crate::ZoneDetails {state: "NY", zone: "032", zone_numeric: 032, name: "Northern Herkimer", wfo: "ALY" },
ForecastZone::NY033 => crate::ZoneDetails {state: "NY", zone: "033", zone_numeric: 033, name: "Hamilton", wfo: "ALY" },
ForecastZone::NY034 => crate::ZoneDetails {state: "NY", zone: "034", zone_numeric: 034, name: "Western Essex", wfo: "BTV" },
ForecastZone::NY035 => crate::ZoneDetails {state: "NY", zone: "035", zone_numeric: 035, name: "Eastern Essex", wfo: "BTV" },
ForecastZone::NY036 => crate::ZoneDetails {state: "NY", zone: "036", zone_numeric: 036, name: "Madison", wfo: "BGM" },
ForecastZone::NY037 => crate::ZoneDetails {state: "NY", zone: "037", zone_numeric: 037, name: "Southern Oneida", wfo: "BGM" },
ForecastZone::NY038 => crate::ZoneDetails {state: "NY", zone: "038", zone_numeric: 038, name: "Southern Herkimer", wfo: "ALY" },
ForecastZone::NY039 => crate::ZoneDetails {state: "NY", zone: "039", zone_numeric: 039, name: "Southern Fulton", wfo: "ALY" },
ForecastZone::NY040 => crate::ZoneDetails {state: "NY", zone: "040", zone_numeric: 040, name: "Montgomery", wfo: "ALY" },
ForecastZone::NY041 => crate::ZoneDetails {state: "NY", zone: "041", zone_numeric: 041, name: "Northern Saratoga", wfo: "ALY" },
ForecastZone::NY042 => crate::ZoneDetails {state: "NY", zone: "042", zone_numeric: 042, name: "Northern Warren", wfo: "ALY" },
ForecastZone::NY043 => crate::ZoneDetails {state: "NY", zone: "043", zone_numeric: 043, name: "Northern Washington", wfo: "ALY" },
ForecastZone::NY044 => crate::ZoneDetails {state: "NY", zone: "044", zone_numeric: 044, name: "Cortland", wfo: "BGM" },
ForecastZone::NY045 => crate::ZoneDetails {state: "NY", zone: "045", zone_numeric: 045, name: "Chenango", wfo: "BGM" },
ForecastZone::NY046 => crate::ZoneDetails {state: "NY", zone: "046", zone_numeric: 046, name: "Otsego", wfo: "BGM" },
ForecastZone::NY047 => crate::ZoneDetails {state: "NY", zone: "047", zone_numeric: 047, name: "Schoharie", wfo: "ALY" },
ForecastZone::NY048 => crate::ZoneDetails {state: "NY", zone: "048", zone_numeric: 048, name: "Western Schenectady", wfo: "ALY" },
ForecastZone::NY049 => crate::ZoneDetails {state: "NY", zone: "049", zone_numeric: 049, name: "Eastern Schenectady", wfo: "ALY" },
ForecastZone::NY050 => crate::ZoneDetails {state: "NY", zone: "050", zone_numeric: 050, name: "Southern Saratoga", wfo: "ALY" },
ForecastZone::NY051 => crate::ZoneDetails {state: "NY", zone: "051", zone_numeric: 051, name: "Western Albany", wfo: "ALY" },
ForecastZone::NY052 => crate::ZoneDetails {state: "NY", zone: "052", zone_numeric: 052, name: "Eastern Albany", wfo: "ALY" },
ForecastZone::NY053 => crate::ZoneDetails {state: "NY", zone: "053", zone_numeric: 053, name: "Western Rensselaer", wfo: "ALY" },
ForecastZone::NY054 => crate::ZoneDetails {state: "NY", zone: "054", zone_numeric: 054, name: "Eastern Rensselaer", wfo: "ALY" },
ForecastZone::NY055 => crate::ZoneDetails {state: "NY", zone: "055", zone_numeric: 055, name: "Tioga", wfo: "BGM" },
ForecastZone::NY056 => crate::ZoneDetails {state: "NY", zone: "056", zone_numeric: 056, name: "Broome", wfo: "BGM" },
ForecastZone::NY057 => crate::ZoneDetails {state: "NY", zone: "057", zone_numeric: 057, name: "Delaware", wfo: "BGM" },
ForecastZone::NY058 => crate::ZoneDetails {state: "NY", zone: "058", zone_numeric: 058, name: "Western Greene", wfo: "ALY" },
ForecastZone::NY059 => crate::ZoneDetails {state: "NY", zone: "059", zone_numeric: 059, name: "Eastern Greene", wfo: "ALY" },
ForecastZone::NY060 => crate::ZoneDetails {state: "NY", zone: "060", zone_numeric: 060, name: "Western Columbia", wfo: "ALY" },
ForecastZone::NY061 => crate::ZoneDetails {state: "NY", zone: "061", zone_numeric: 061, name: "Eastern Columbia", wfo: "ALY" },
ForecastZone::NY062 => crate::ZoneDetails {state: "NY", zone: "062", zone_numeric: 062, name: "Sullivan", wfo: "BGM" },
ForecastZone::NY063 => crate::ZoneDetails {state: "NY", zone: "063", zone_numeric: 063, name: "Western Ulster", wfo: "ALY" },
ForecastZone::NY064 => crate::ZoneDetails {state: "NY", zone: "064", zone_numeric: 064, name: "Eastern Ulster", wfo: "ALY" },
ForecastZone::NY065 => crate::ZoneDetails {state: "NY", zone: "065", zone_numeric: 065, name: "Western Dutchess", wfo: "ALY" },
ForecastZone::NY066 => crate::ZoneDetails {state: "NY", zone: "066", zone_numeric: 066, name: "Eastern Dutchess", wfo: "ALY" },
ForecastZone::NY067 => crate::ZoneDetails {state: "NY", zone: "067", zone_numeric: 067, name: "Orange", wfo: "OKX" },
ForecastZone::NY068 => crate::ZoneDetails {state: "NY", zone: "068", zone_numeric: 068, name: "Putnam", wfo: "OKX" },
ForecastZone::NY069 => crate::ZoneDetails {state: "NY", zone: "069", zone_numeric: 069, name: "Rockland", wfo: "OKX" },
ForecastZone::NY070 => crate::ZoneDetails {state: "NY", zone: "070", zone_numeric: 070, name: "Northern Westchester", wfo: "OKX" },
ForecastZone::NY071 => crate::ZoneDetails {state: "NY", zone: "071", zone_numeric: 071, name: "Southern Westchester", wfo: "OKX" },
ForecastZone::NY072 => crate::ZoneDetails {state: "NY", zone: "072", zone_numeric: 072, name: "New York (Manhattan)", wfo: "OKX" },
ForecastZone::NY073 => crate::ZoneDetails {state: "NY", zone: "073", zone_numeric: 073, name: "Bronx", wfo: "OKX" },
ForecastZone::NY074 => crate::ZoneDetails {state: "NY", zone: "074", zone_numeric: 074, name: "Richmond (Staten Is.)", wfo: "OKX" },
ForecastZone::NY075 => crate::ZoneDetails {state: "NY", zone: "075", zone_numeric: 075, name: "Kings (Brooklyn)", wfo: "OKX" },
ForecastZone::NY078 => crate::ZoneDetails {state: "NY", zone: "078", zone_numeric: 078, name: "Northwest Suffolk", wfo: "OKX" },
ForecastZone::NY079 => crate::ZoneDetails {state: "NY", zone: "079", zone_numeric: 079, name: "Northeast Suffolk", wfo: "OKX" },
ForecastZone::NY080 => crate::ZoneDetails {state: "NY", zone: "080", zone_numeric: 080, name: "Southwest Suffolk", wfo: "OKX" },
ForecastZone::NY081 => crate::ZoneDetails {state: "NY", zone: "081", zone_numeric: 081, name: "Southeast Suffolk", wfo: "OKX" },
ForecastZone::NY082 => crate::ZoneDetails {state: "NY", zone: "082", zone_numeric: 082, name: "Northern Fulton", wfo: "ALY" },
ForecastZone::NY083 => crate::ZoneDetails {state: "NY", zone: "083", zone_numeric: 083, name: "Southeast Warren", wfo: "ALY" },
ForecastZone::NY084 => crate::ZoneDetails {state: "NY", zone: "084", zone_numeric: 084, name: "Southern Washington", wfo: "ALY" },
ForecastZone::NY085 => crate::ZoneDetails {state: "NY", zone: "085", zone_numeric: 085, name: "Southern Erie", wfo: "BUF" },
ForecastZone::NY087 => crate::ZoneDetails {state: "NY", zone: "087", zone_numeric: 087, name: "Southwestern St. Lawrence", wfo: "BTV" },
ForecastZone::NY176 => crate::ZoneDetails {state: "NY", zone: "176", zone_numeric: 176, name: "Northern Queens", wfo: "OKX" },
ForecastZone::NY177 => crate::ZoneDetails {state: "NY", zone: "177", zone_numeric: 177, name: "Northern Nassau", wfo: "OKX" },
ForecastZone::NY178 => crate::ZoneDetails {state: "NY", zone: "178", zone_numeric: 178, name: "Southern Queens", wfo: "OKX" },
ForecastZone::NY179 => crate::ZoneDetails {state: "NY", zone: "179", zone_numeric: 179, name: "Southern Nassau", wfo: "OKX" },
ForecastZone::OH001 => crate::ZoneDetails {state: "OH", zone: "001", zone_numeric: 001, name: "Williams", wfo: "IWX" },
ForecastZone::OH002 => crate::ZoneDetails {state: "OH", zone: "002", zone_numeric: 002, name: "Fulton", wfo: "IWX" },
ForecastZone::OH003 => crate::ZoneDetails {state: "OH", zone: "003", zone_numeric: 003, name: "Lucas", wfo: "CLE" },
ForecastZone::OH004 => crate::ZoneDetails {state: "OH", zone: "004", zone_numeric: 004, name: "Defiance", wfo: "IWX" },
ForecastZone::OH005 => crate::ZoneDetails {state: "OH", zone: "005", zone_numeric: 005, name: "Henry", wfo: "IWX" },
ForecastZone::OH006 => crate::ZoneDetails {state: "OH", zone: "006", zone_numeric: 006, name: "Wood", wfo: "CLE" },
ForecastZone::OH007 => crate::ZoneDetails {state: "OH", zone: "007", zone_numeric: 007, name: "Ottawa", wfo: "CLE" },
ForecastZone::OH008 => crate::ZoneDetails {state: "OH", zone: "008", zone_numeric: 008, name: "Sandusky", wfo: "CLE" },
ForecastZone::OH009 => crate::ZoneDetails {state: "OH", zone: "009", zone_numeric: 009, name: "Erie", wfo: "CLE" },
ForecastZone::OH010 => crate::ZoneDetails {state: "OH", zone: "010", zone_numeric: 010, name: "Lorain", wfo: "CLE" },
ForecastZone::OH011 => crate::ZoneDetails {state: "OH", zone: "011", zone_numeric: 011, name: "Cuyahoga", wfo: "CLE" },
ForecastZone::OH012 => crate::ZoneDetails {state: "OH", zone: "012", zone_numeric: 012, name: "Lake", wfo: "CLE" },
ForecastZone::OH013 => crate::ZoneDetails {state: "OH", zone: "013", zone_numeric: 013, name: "Geauga", wfo: "CLE" },
ForecastZone::OH014 => crate::ZoneDetails {state: "OH", zone: "014", zone_numeric: 014, name: "Ashtabula Inland", wfo: "CLE" },
ForecastZone::OH015 => crate::ZoneDetails {state: "OH", zone: "015", zone_numeric: 015, name: "Paulding", wfo: "IWX" },
ForecastZone::OH016 => crate::ZoneDetails {state: "OH", zone: "016", zone_numeric: 016, name: "Putnam", wfo: "IWX" },
ForecastZone::OH017 => crate::ZoneDetails {state: "OH", zone: "017", zone_numeric: 017, name: "Hancock", wfo: "CLE" },
ForecastZone::OH018 => crate::ZoneDetails {state: "OH", zone: "018", zone_numeric: 018, name: "Seneca", wfo: "CLE" },
ForecastZone::OH019 => crate::ZoneDetails {state: "OH", zone: "019", zone_numeric: 019, name: "Huron", wfo: "CLE" },
ForecastZone::OH020 => crate::ZoneDetails {state: "OH", zone: "020", zone_numeric: 020, name: "Medina", wfo: "CLE" },
ForecastZone::OH021 => crate::ZoneDetails {state: "OH", zone: "021", zone_numeric: 021, name: "Summit", wfo: "CLE" },
ForecastZone::OH022 => crate::ZoneDetails {state: "OH", zone: "022", zone_numeric: 022, name: "Portage", wfo: "CLE" },
ForecastZone::OH023 => crate::ZoneDetails {state: "OH", zone: "023", zone_numeric: 023, name: "Trumbull", wfo: "CLE" },
ForecastZone::OH024 => crate::ZoneDetails {state: "OH", zone: "024", zone_numeric: 024, name: "Van Wert", wfo: "IWX" },
ForecastZone::OH025 => crate::ZoneDetails {state: "OH", zone: "025", zone_numeric: 025, name: "Allen", wfo: "IWX" },
ForecastZone::OH026 => crate::ZoneDetails {state: "OH", zone: "026", zone_numeric: 026, name: "Hardin", wfo: "ILN" },
ForecastZone::OH027 => crate::ZoneDetails {state: "OH", zone: "027", zone_numeric: 027, name: "Wyandot", wfo: "CLE" },
ForecastZone::OH028 => crate::ZoneDetails {state: "OH", zone: "028", zone_numeric: 028, name: "Crawford", wfo: "CLE" },
ForecastZone::OH029 => crate::ZoneDetails {state: "OH", zone: "029", zone_numeric: 029, name: "Richland", wfo: "CLE" },
ForecastZone::OH030 => crate::ZoneDetails {state: "OH", zone: "030", zone_numeric: 030, name: "Ashland", wfo: "CLE" },
ForecastZone::OH031 => crate::ZoneDetails {state: "OH", zone: "031", zone_numeric: 031, name: "Wayne", wfo: "CLE" },
ForecastZone::OH032 => crate::ZoneDetails {state: "OH", zone: "032", zone_numeric: 032, name: "Stark", wfo: "CLE" },
ForecastZone::OH033 => crate::ZoneDetails {state: "OH", zone: "033", zone_numeric: 033, name: "Mahoning", wfo: "CLE" },
ForecastZone::OH034 => crate::ZoneDetails {state: "OH", zone: "034", zone_numeric: 034, name: "Mercer", wfo: "ILN" },
ForecastZone::OH035 => crate::ZoneDetails {state: "OH", zone: "035", zone_numeric: 035, name: "Auglaize", wfo: "ILN" },
ForecastZone::OH036 => crate::ZoneDetails {state: "OH", zone: "036", zone_numeric: 036, name: "Marion", wfo: "CLE" },
ForecastZone::OH037 => crate::ZoneDetails {state: "OH", zone: "037", zone_numeric: 037, name: "Morrow", wfo: "CLE" },
ForecastZone::OH038 => crate::ZoneDetails {state: "OH", zone: "038", zone_numeric: 038, name: "Holmes", wfo: "CLE" },
ForecastZone::OH039 => crate::ZoneDetails {state: "OH", zone: "039", zone_numeric: 039, name: "Tuscarawas", wfo: "PBZ" },
ForecastZone::OH040 => crate::ZoneDetails {state: "OH", zone: "040", zone_numeric: 040, name: "Carroll", wfo: "PBZ" },
ForecastZone::OH041 => crate::ZoneDetails {state: "OH", zone: "041", zone_numeric: 041, name: "Columbiana", wfo: "PBZ" },
ForecastZone::OH042 => crate::ZoneDetails {state: "OH", zone: "042", zone_numeric: 042, name: "Darke", wfo: "ILN" },
ForecastZone::OH043 => crate::ZoneDetails {state: "OH", zone: "043", zone_numeric: 043, name: "Shelby", wfo: "ILN" },
ForecastZone::OH044 => crate::ZoneDetails {state: "OH", zone: "044", zone_numeric: 044, name: "Logan", wfo: "ILN" },
ForecastZone::OH045 => crate::ZoneDetails {state: "OH", zone: "045", zone_numeric: 045, name: "Union", wfo: "ILN" },
ForecastZone::OH046 => crate::ZoneDetails {state: "OH", zone: "046", zone_numeric: 046, name: "Delaware", wfo: "ILN" },
ForecastZone::OH047 => crate::ZoneDetails {state: "OH", zone: "047", zone_numeric: 047, name: "Knox", wfo: "CLE" },
ForecastZone::OH048 => crate::ZoneDetails {state: "OH", zone: "048", zone_numeric: 048, name: "Coshocton", wfo: "PBZ" },
ForecastZone::OH049 => crate::ZoneDetails {state: "OH", zone: "049", zone_numeric: 049, name: "Harrison", wfo: "PBZ" },
ForecastZone::OH050 => crate::ZoneDetails {state: "OH", zone: "050", zone_numeric: 050, name: "Jefferson", wfo: "PBZ" },
ForecastZone::OH051 => crate::ZoneDetails {state: "OH", zone: "051", zone_numeric: 051, name: "Miami", wfo: "ILN" },
ForecastZone::OH052 => crate::ZoneDetails {state: "OH", zone: "052", zone_numeric: 052, name: "Champaign", wfo: "ILN" },
ForecastZone::OH053 => crate::ZoneDetails {state: "OH", zone: "053", zone_numeric: 053, name: "Clark", wfo: "ILN" },
ForecastZone::OH054 => crate::ZoneDetails {state: "OH", zone: "054", zone_numeric: 054, name: "Madison", wfo: "ILN" },
ForecastZone::OH055 => crate::ZoneDetails {state: "OH", zone: "055", zone_numeric: 055, name: "Franklin", wfo: "ILN" },
ForecastZone::OH056 => crate::ZoneDetails {state: "OH", zone: "056", zone_numeric: 056, name: "Licking", wfo: "ILN" },
ForecastZone::OH057 => crate::ZoneDetails {state: "OH", zone: "057", zone_numeric: 057, name: "Muskingum", wfo: "PBZ" },
ForecastZone::OH058 => crate::ZoneDetails {state: "OH", zone: "058", zone_numeric: 058, name: "Guernsey", wfo: "PBZ" },
ForecastZone::OH059 => crate::ZoneDetails {state: "OH", zone: "059", zone_numeric: 059, name: "Belmont", wfo: "PBZ" },
ForecastZone::OH060 => crate::ZoneDetails {state: "OH", zone: "060", zone_numeric: 060, name: "Preble", wfo: "ILN" },
ForecastZone::OH061 => crate::ZoneDetails {state: "OH", zone: "061", zone_numeric: 061, name: "Montgomery", wfo: "ILN" },
ForecastZone::OH062 => crate::ZoneDetails {state: "OH", zone: "062", zone_numeric: 062, name: "Greene", wfo: "ILN" },
ForecastZone::OH063 => crate::ZoneDetails {state: "OH", zone: "063", zone_numeric: 063, name: "Fayette", wfo: "ILN" },
ForecastZone::OH064 => crate::ZoneDetails {state: "OH", zone: "064", zone_numeric: 064, name: "Pickaway", wfo: "ILN" },
ForecastZone::OH065 => crate::ZoneDetails {state: "OH", zone: "065", zone_numeric: 065, name: "Fairfield", wfo: "ILN" },
ForecastZone::OH066 => crate::ZoneDetails {state: "OH", zone: "066", zone_numeric: 066, name: "Perry", wfo: "RLX" },
ForecastZone::OH067 => crate::ZoneDetails {state: "OH", zone: "067", zone_numeric: 067, name: "Morgan", wfo: "RLX" },
ForecastZone::OH068 => crate::ZoneDetails {state: "OH", zone: "068", zone_numeric: 068, name: "Noble", wfo: "PBZ" },
ForecastZone::OH069 => crate::ZoneDetails {state: "OH", zone: "069", zone_numeric: 069, name: "Monroe", wfo: "PBZ" },
ForecastZone::OH070 => crate::ZoneDetails {state: "OH", zone: "070", zone_numeric: 070, name: "Butler", wfo: "ILN" },
ForecastZone::OH071 => crate::ZoneDetails {state: "OH", zone: "071", zone_numeric: 071, name: "Warren", wfo: "ILN" },
ForecastZone::OH072 => crate::ZoneDetails {state: "OH", zone: "072", zone_numeric: 072, name: "Clinton", wfo: "ILN" },
ForecastZone::OH073 => crate::ZoneDetails {state: "OH", zone: "073", zone_numeric: 073, name: "Ross", wfo: "ILN" },
ForecastZone::OH074 => crate::ZoneDetails {state: "OH", zone: "074", zone_numeric: 074, name: "Hocking", wfo: "ILN" },
ForecastZone::OH075 => crate::ZoneDetails {state: "OH", zone: "075", zone_numeric: 075, name: "Athens", wfo: "RLX" },
ForecastZone::OH076 => crate::ZoneDetails {state: "OH", zone: "076", zone_numeric: 076, name: "Washington", wfo: "RLX" },
ForecastZone::OH077 => crate::ZoneDetails {state: "OH", zone: "077", zone_numeric: 077, name: "Hamilton", wfo: "ILN" },
ForecastZone::OH078 => crate::ZoneDetails {state: "OH", zone: "078", zone_numeric: 078, name: "Clermont", wfo: "ILN" },
ForecastZone::OH079 => crate::ZoneDetails {state: "OH", zone: "079", zone_numeric: 079, name: "Brown", wfo: "ILN" },
ForecastZone::OH080 => crate::ZoneDetails {state: "OH", zone: "080", zone_numeric: 080, name: "Highland", wfo: "ILN" },
ForecastZone::OH081 => crate::ZoneDetails {state: "OH", zone: "081", zone_numeric: 081, name: "Adams", wfo: "ILN" },
ForecastZone::OH082 => crate::ZoneDetails {state: "OH", zone: "082", zone_numeric: 082, name: "Pike", wfo: "ILN" },
ForecastZone::OH083 => crate::ZoneDetails {state: "OH", zone: "083", zone_numeric: 083, name: "Jackson", wfo: "RLX" },
ForecastZone::OH084 => crate::ZoneDetails {state: "OH", zone: "084", zone_numeric: 084, name: "Vinton", wfo: "RLX" },
ForecastZone::OH085 => crate::ZoneDetails {state: "OH", zone: "085", zone_numeric: 085, name: "Meigs", wfo: "RLX" },
ForecastZone::OH086 => crate::ZoneDetails {state: "OH", zone: "086", zone_numeric: 086, name: "Gallia", wfo: "RLX" },
ForecastZone::OH087 => crate::ZoneDetails {state: "OH", zone: "087", zone_numeric: 087, name: "Lawrence", wfo: "RLX" },
ForecastZone::OH088 => crate::ZoneDetails {state: "OH", zone: "088", zone_numeric: 088, name: "Scioto", wfo: "ILN" },
ForecastZone::OH089 => crate::ZoneDetails {state: "OH", zone: "089", zone_numeric: 089, name: "Ashtabula Lakeshore", wfo: "CLE" },
ForecastZone::OK001 => crate::ZoneDetails {state: "OK", zone: "001", zone_numeric: 001, name: "Cimarron", wfo: "AMA" },
ForecastZone::OK002 => crate::ZoneDetails {state: "OK", zone: "002", zone_numeric: 002, name: "Texas", wfo: "AMA" },
ForecastZone::OK003 => crate::ZoneDetails {state: "OK", zone: "003", zone_numeric: 003, name: "Beaver", wfo: "AMA" },
ForecastZone::OK004 => crate::ZoneDetails {state: "OK", zone: "004", zone_numeric: 004, name: "Harper", wfo: "OUN" },
ForecastZone::OK005 => crate::ZoneDetails {state: "OK", zone: "005", zone_numeric: 005, name: "Woods", wfo: "OUN" },
ForecastZone::OK006 => crate::ZoneDetails {state: "OK", zone: "006", zone_numeric: 006, name: "Alfalfa", wfo: "OUN" },
ForecastZone::OK007 => crate::ZoneDetails {state: "OK", zone: "007", zone_numeric: 007, name: "Grant", wfo: "OUN" },
ForecastZone::OK008 => crate::ZoneDetails {state: "OK", zone: "008", zone_numeric: 008, name: "Kay", wfo: "OUN" },
ForecastZone::OK009 => crate::ZoneDetails {state: "OK", zone: "009", zone_numeric: 009, name: "Ellis", wfo: "OUN" },
ForecastZone::OK010 => crate::ZoneDetails {state: "OK", zone: "010", zone_numeric: 010, name: "Woodward", wfo: "OUN" },
ForecastZone::OK011 => crate::ZoneDetails {state: "OK", zone: "011", zone_numeric: 011, name: "Major", wfo: "OUN" },
ForecastZone::OK012 => crate::ZoneDetails {state: "OK", zone: "012", zone_numeric: 012, name: "Garfield", wfo: "OUN" },
ForecastZone::OK013 => crate::ZoneDetails {state: "OK", zone: "013", zone_numeric: 013, name: "Noble", wfo: "OUN" },
ForecastZone::OK014 => crate::ZoneDetails {state: "OK", zone: "014", zone_numeric: 014, name: "Roger Mills", wfo: "OUN" },
ForecastZone::OK015 => crate::ZoneDetails {state: "OK", zone: "015", zone_numeric: 015, name: "Dewey", wfo: "OUN" },
ForecastZone::OK016 => crate::ZoneDetails {state: "OK", zone: "016", zone_numeric: 016, name: "Custer", wfo: "OUN" },
ForecastZone::OK017 => crate::ZoneDetails {state: "OK", zone: "017", zone_numeric: 017, name: "Blaine", wfo: "OUN" },
ForecastZone::OK018 => crate::ZoneDetails {state: "OK", zone: "018", zone_numeric: 018, name: "Kingfisher", wfo: "OUN" },
ForecastZone::OK019 => crate::ZoneDetails {state: "OK", zone: "019", zone_numeric: 019, name: "Logan", wfo: "OUN" },
ForecastZone::OK020 => crate::ZoneDetails {state: "OK", zone: "020", zone_numeric: 020, name: "Payne", wfo: "OUN" },
ForecastZone::OK021 => crate::ZoneDetails {state: "OK", zone: "021", zone_numeric: 021, name: "Beckham", wfo: "OUN" },
ForecastZone::OK022 => crate::ZoneDetails {state: "OK", zone: "022", zone_numeric: 022, name: "Washita", wfo: "OUN" },
ForecastZone::OK023 => crate::ZoneDetails {state: "OK", zone: "023", zone_numeric: 023, name: "Caddo", wfo: "OUN" },
ForecastZone::OK024 => crate::ZoneDetails {state: "OK", zone: "024", zone_numeric: 024, name: "Canadian", wfo: "OUN" },
ForecastZone::OK025 => crate::ZoneDetails {state: "OK", zone: "025", zone_numeric: 025, name: "Oklahoma", wfo: "OUN" },
ForecastZone::OK026 => crate::ZoneDetails {state: "OK", zone: "026", zone_numeric: 026, name: "Lincoln", wfo: "OUN" },
ForecastZone::OK027 => crate::ZoneDetails {state: "OK", zone: "027", zone_numeric: 027, name: "Grady", wfo: "OUN" },
ForecastZone::OK028 => crate::ZoneDetails {state: "OK", zone: "028", zone_numeric: 028, name: "McClain", wfo: "OUN" },
ForecastZone::OK029 => crate::ZoneDetails {state: "OK", zone: "029", zone_numeric: 029, name: "Cleveland", wfo: "OUN" },
ForecastZone::OK030 => crate::ZoneDetails {state: "OK", zone: "030", zone_numeric: 030, name: "Pottawatomie", wfo: "OUN" },
ForecastZone::OK031 => crate::ZoneDetails {state: "OK", zone: "031", zone_numeric: 031, name: "Seminole", wfo: "OUN" },
ForecastZone::OK032 => crate::ZoneDetails {state: "OK", zone: "032", zone_numeric: 032, name: "Hughes", wfo: "OUN" },
ForecastZone::OK033 => crate::ZoneDetails {state: "OK", zone: "033", zone_numeric: 033, name: "Harmon", wfo: "OUN" },
ForecastZone::OK034 => crate::ZoneDetails {state: "OK", zone: "034", zone_numeric: 034, name: "Greer", wfo: "OUN" },
ForecastZone::OK035 => crate::ZoneDetails {state: "OK", zone: "035", zone_numeric: 035, name: "Kiowa", wfo: "OUN" },
ForecastZone::OK036 => crate::ZoneDetails {state: "OK", zone: "036", zone_numeric: 036, name: "Jackson", wfo: "OUN" },
ForecastZone::OK037 => crate::ZoneDetails {state: "OK", zone: "037", zone_numeric: 037, name: "Tillman", wfo: "OUN" },
ForecastZone::OK038 => crate::ZoneDetails {state: "OK", zone: "038", zone_numeric: 038, name: "Comanche", wfo: "OUN" },
ForecastZone::OK039 => crate::ZoneDetails {state: "OK", zone: "039", zone_numeric: 039, name: "Stephens", wfo: "OUN" },
ForecastZone::OK040 => crate::ZoneDetails {state: "OK", zone: "040", zone_numeric: 040, name: "Garvin", wfo: "OUN" },
ForecastZone::OK041 => crate::ZoneDetails {state: "OK", zone: "041", zone_numeric: 041, name: "Murray", wfo: "OUN" },
ForecastZone::OK042 => crate::ZoneDetails {state: "OK", zone: "042", zone_numeric: 042, name: "Pontotoc", wfo: "OUN" },
ForecastZone::OK043 => crate::ZoneDetails {state: "OK", zone: "043", zone_numeric: 043, name: "Coal", wfo: "OUN" },
ForecastZone::OK044 => crate::ZoneDetails {state: "OK", zone: "044", zone_numeric: 044, name: "Cotton", wfo: "OUN" },
ForecastZone::OK045 => crate::ZoneDetails {state: "OK", zone: "045", zone_numeric: 045, name: "Jefferson", wfo: "OUN" },
ForecastZone::OK046 => crate::ZoneDetails {state: "OK", zone: "046", zone_numeric: 046, name: "Carter", wfo: "OUN" },
ForecastZone::OK047 => crate::ZoneDetails {state: "OK", zone: "047", zone_numeric: 047, name: "Johnston", wfo: "OUN" },
ForecastZone::OK048 => crate::ZoneDetails {state: "OK", zone: "048", zone_numeric: 048, name: "Atoka", wfo: "OUN" },
ForecastZone::OK049 => crate::ZoneDetails {state: "OK", zone: "049", zone_numeric: 049, name: "Pushmataha", wfo: "TSA" },
ForecastZone::OK050 => crate::ZoneDetails {state: "OK", zone: "050", zone_numeric: 050, name: "Love", wfo: "OUN" },
ForecastZone::OK051 => crate::ZoneDetails {state: "OK", zone: "051", zone_numeric: 051, name: "Marshall", wfo: "OUN" },
ForecastZone::OK052 => crate::ZoneDetails {state: "OK", zone: "052", zone_numeric: 052, name: "Bryan", wfo: "OUN" },
ForecastZone::OK053 => crate::ZoneDetails {state: "OK", zone: "053", zone_numeric: 053, name: "Choctaw", wfo: "TSA" },
ForecastZone::OK054 => crate::ZoneDetails {state: "OK", zone: "054", zone_numeric: 054, name: "Osage", wfo: "TSA" },
ForecastZone::OK055 => crate::ZoneDetails {state: "OK", zone: "055", zone_numeric: 055, name: "Washington", wfo: "TSA" },
ForecastZone::OK056 => crate::ZoneDetails {state: "OK", zone: "056", zone_numeric: 056, name: "Nowata", wfo: "TSA" },
ForecastZone::OK057 => crate::ZoneDetails {state: "OK", zone: "057", zone_numeric: 057, name: "Craig", wfo: "TSA" },
ForecastZone::OK058 => crate::ZoneDetails {state: "OK", zone: "058", zone_numeric: 058, name: "Ottawa", wfo: "TSA" },
ForecastZone::OK059 => crate::ZoneDetails {state: "OK", zone: "059", zone_numeric: 059, name: "Pawnee", wfo: "TSA" },
ForecastZone::OK060 => crate::ZoneDetails {state: "OK", zone: "060", zone_numeric: 060, name: "Tulsa", wfo: "TSA" },
ForecastZone::OK061 => crate::ZoneDetails {state: "OK", zone: "061", zone_numeric: 061, name: "Rogers", wfo: "TSA" },
ForecastZone::OK062 => crate::ZoneDetails {state: "OK", zone: "062", zone_numeric: 062, name: "Mayes", wfo: "TSA" },
ForecastZone::OK063 => crate::ZoneDetails {state: "OK", zone: "063", zone_numeric: 063, name: "Delaware", wfo: "TSA" },
ForecastZone::OK064 => crate::ZoneDetails {state: "OK", zone: "064", zone_numeric: 064, name: "Creek", wfo: "TSA" },
ForecastZone::OK065 => crate::ZoneDetails {state: "OK", zone: "065", zone_numeric: 065, name: "Okfuskee", wfo: "TSA" },
ForecastZone::OK066 => crate::ZoneDetails {state: "OK", zone: "066", zone_numeric: 066, name: "Okmulgee", wfo: "TSA" },
ForecastZone::OK067 => crate::ZoneDetails {state: "OK", zone: "067", zone_numeric: 067, name: "Wagoner", wfo: "TSA" },
ForecastZone::OK068 => crate::ZoneDetails {state: "OK", zone: "068", zone_numeric: 068, name: "Cherokee", wfo: "TSA" },
ForecastZone::OK069 => crate::ZoneDetails {state: "OK", zone: "069", zone_numeric: 069, name: "Adair", wfo: "TSA" },
ForecastZone::OK070 => crate::ZoneDetails {state: "OK", zone: "070", zone_numeric: 070, name: "Muskogee", wfo: "TSA" },
ForecastZone::OK071 => crate::ZoneDetails {state: "OK", zone: "071", zone_numeric: 071, name: "McIntosh", wfo: "TSA" },
ForecastZone::OK072 => crate::ZoneDetails {state: "OK", zone: "072", zone_numeric: 072, name: "Sequoyah", wfo: "TSA" },
ForecastZone::OK073 => crate::ZoneDetails {state: "OK", zone: "073", zone_numeric: 073, name: "Pittsburg", wfo: "TSA" },
ForecastZone::OK074 => crate::ZoneDetails {state: "OK", zone: "074", zone_numeric: 074, name: "Haskell", wfo: "TSA" },
ForecastZone::OK075 => crate::ZoneDetails {state: "OK", zone: "075", zone_numeric: 075, name: "Latimer", wfo: "TSA" },
ForecastZone::OK076 => crate::ZoneDetails {state: "OK", zone: "076", zone_numeric: 076, name: "Le Flore", wfo: "TSA" },
ForecastZone::OK077 => crate::ZoneDetails {state: "OK", zone: "077", zone_numeric: 077, name: "McCurtain", wfo: "SHV" },
ForecastZone::OR001 => crate::ZoneDetails {state: "OR", zone: "001", zone_numeric: 001, name: "North Oregon Coast", wfo: "PQR" },
ForecastZone::OR002 => crate::ZoneDetails {state: "OR", zone: "002", zone_numeric: 002, name: "Central Oregon Coast", wfo: "PQR" },
ForecastZone::OR003 => crate::ZoneDetails {state: "OR", zone: "003", zone_numeric: 003, name: "Coast Range of Northwest Oregon", wfo: "PQR" },
ForecastZone::OR004 => crate::ZoneDetails {state: "OR", zone: "004", zone_numeric: 004, name: "Central Coast Range of Western Oregon", wfo: "PQR" },
ForecastZone::OR005 => crate::ZoneDetails {state: "OR", zone: "005", zone_numeric: 005, name: "Lower Columbia", wfo: "PQR" },
ForecastZone::OR006 => crate::ZoneDetails {state: "OR", zone: "006", zone_numeric: 006, name: "Greater Portland Metro Area", wfo: "PQR" },
ForecastZone::OR007 => crate::ZoneDetails {state: "OR", zone: "007", zone_numeric: 007, name: "Central Willamette Valley", wfo: "PQR" },
ForecastZone::OR008 => crate::ZoneDetails {state: "OR", zone: "008", zone_numeric: 008, name: "South  Willamette Valley", wfo: "PQR" },
ForecastZone::OR010 => crate::ZoneDetails {state: "OR", zone: "010", zone_numeric: 010, name: "Northern Oregon Cascade Foothills", wfo: "PQR" },
ForecastZone::OR011 => crate::ZoneDetails {state: "OR", zone: "011", zone_numeric: 011, name: "Northern Oregon Cascades", wfo: "PQR" },
ForecastZone::OR012 => crate::ZoneDetails {state: "OR", zone: "012", zone_numeric: 012, name: "Cascade Foothills in Lane County", wfo: "PQR" },
ForecastZone::OR013 => crate::ZoneDetails {state: "OR", zone: "013", zone_numeric: 013, name: "Cascades in Lane County", wfo: "PQR" },
ForecastZone::OR014 => crate::ZoneDetails {state: "OR", zone: "014", zone_numeric: 014, name: "Upper Hood River Valley", wfo: "PQR" },
ForecastZone::OR015 => crate::ZoneDetails {state: "OR", zone: "015", zone_numeric: 015, name: "Western Columbia River Gorge", wfo: "PQR" },
ForecastZone::OR016 => crate::ZoneDetails {state: "OR", zone: "016", zone_numeric: 016, name: "Central Columbia River Gorge", wfo: "PQR" },
ForecastZone::OR021 => crate::ZoneDetails {state: "OR", zone: "021", zone_numeric: 021, name: "South Central Oregon Coast", wfo: "MFR" },
ForecastZone::OR022 => crate::ZoneDetails {state: "OR", zone: "022", zone_numeric: 022, name: "Curry County Coast", wfo: "MFR" },
ForecastZone::OR023 => crate::ZoneDetails {state: "OR", zone: "023", zone_numeric: 023, name: "Central Douglas County", wfo: "MFR" },
ForecastZone::OR024 => crate::ZoneDetails {state: "OR", zone: "024", zone_numeric: 024, name: "Eastern Curry County and Josephine County", wfo: "MFR" },
ForecastZone::OR025 => crate::ZoneDetails {state: "OR", zone: "025", zone_numeric: 025, name: "Eastern Douglas County Foothills", wfo: "MFR" },
ForecastZone::OR026 => crate::ZoneDetails {state: "OR", zone: "026", zone_numeric: 026, name: "Jackson County", wfo: "MFR" },
ForecastZone::OR027 => crate::ZoneDetails {state: "OR", zone: "027", zone_numeric: 027, name: "South Central Oregon Cascades", wfo: "MFR" },
ForecastZone::OR028 => crate::ZoneDetails {state: "OR", zone: "028", zone_numeric: 028, name: "Siskiyou Mountains and Southern Oregon Cascades", wfo: "MFR" },
ForecastZone::OR029 => crate::ZoneDetails {state: "OR", zone: "029", zone_numeric: 029, name: "Klamath Basin", wfo: "MFR" },
ForecastZone::OR030 => crate::ZoneDetails {state: "OR", zone: "030", zone_numeric: 030, name: "Northern and Eastern Klamath County and Western Lake County", wfo: "MFR" },
ForecastZone::OR031 => crate::ZoneDetails {state: "OR", zone: "031", zone_numeric: 031, name: "Central and Eastern Lake County", wfo: "MFR" },
ForecastZone::OR041 => crate::ZoneDetails {state: "OR", zone: "041", zone_numeric: 041, name: "Eastern Columbia River Gorge of Oregon", wfo: "PDT" },
ForecastZone::OR044 => crate::ZoneDetails {state: "OR", zone: "044", zone_numeric: 044, name: "Lower Columbia Basin of Oregon", wfo: "PDT" },
ForecastZone::OR049 => crate::ZoneDetails {state: "OR", zone: "049", zone_numeric: 049, name: "Grande Ronde Valley", wfo: "PDT" },
ForecastZone::OR050 => crate::ZoneDetails {state: "OR", zone: "050", zone_numeric: 050, name: "Wallowa County", wfo: "PDT" },
ForecastZone::OR061 => crate::ZoneDetails {state: "OR", zone: "061", zone_numeric: 061, name: "Harney County", wfo: "BOI" },
ForecastZone::OR062 => crate::ZoneDetails {state: "OR", zone: "062", zone_numeric: 062, name: "Baker County", wfo: "BOI" },
ForecastZone::OR063 => crate::ZoneDetails {state: "OR", zone: "063", zone_numeric: 063, name: "Malheur County", wfo: "BOI" },
ForecastZone::OR064 => crate::ZoneDetails {state: "OR", zone: "064", zone_numeric: 064, name: "Oregon Lower Treasure Valley", wfo: "BOI" },
ForecastZone::OR502 => crate::ZoneDetails {state: "OR", zone: "502", zone_numeric: 502, name: "Northern Blue Mountains of Oregon", wfo: "PDT" },
ForecastZone::OR503 => crate::ZoneDetails {state: "OR", zone: "503", zone_numeric: 503, name: "Southern Blue Mountains of Oregon", wfo: "PDT" },
ForecastZone::OR505 => crate::ZoneDetails {state: "OR", zone: "505", zone_numeric: 505, name: "John Day Basin", wfo: "PDT" },
ForecastZone::OR506 => crate::ZoneDetails {state: "OR", zone: "506", zone_numeric: 506, name: "Ochoco-John Day Highlands", wfo: "PDT" },
ForecastZone::OR507 => crate::ZoneDetails {state: "OR", zone: "507", zone_numeric: 507, name: "Foothills of the Northern Blue Mountains of Oregon", wfo: "PDT" },
ForecastZone::OR508 => crate::ZoneDetails {state: "OR", zone: "508", zone_numeric: 508, name: "Foothills of the Southern Blue Mountains of Oregon", wfo: "PDT" },
ForecastZone::OR509 => crate::ZoneDetails {state: "OR", zone: "509", zone_numeric: 509, name: "East Slopes of the Oregon Cascades", wfo: "PDT" },
ForecastZone::OR510 => crate::ZoneDetails {state: "OR", zone: "510", zone_numeric: 510, name: "North Central Oregon", wfo: "PDT" },
ForecastZone::OR511 => crate::ZoneDetails {state: "OR", zone: "511", zone_numeric: 511, name: "Central Oregon", wfo: "PDT" },
ForecastZone::PA001 => crate::ZoneDetails {state: "PA", zone: "001", zone_numeric: 001, name: "Northern Erie", wfo: "CLE" },
ForecastZone::PA002 => crate::ZoneDetails {state: "PA", zone: "002", zone_numeric: 002, name: "Southern Erie", wfo: "CLE" },
ForecastZone::PA003 => crate::ZoneDetails {state: "PA", zone: "003", zone_numeric: 003, name: "Crawford", wfo: "CLE" },
ForecastZone::PA004 => crate::ZoneDetails {state: "PA", zone: "004", zone_numeric: 004, name: "Warren", wfo: "CTP" },
ForecastZone::PA005 => crate::ZoneDetails {state: "PA", zone: "005", zone_numeric: 005, name: "McKean", wfo: "CTP" },
ForecastZone::PA006 => crate::ZoneDetails {state: "PA", zone: "006", zone_numeric: 006, name: "Potter", wfo: "CTP" },
ForecastZone::PA007 => crate::ZoneDetails {state: "PA", zone: "007", zone_numeric: 007, name: "Mercer", wfo: "PBZ" },
ForecastZone::PA008 => crate::ZoneDetails {state: "PA", zone: "008", zone_numeric: 008, name: "Venango", wfo: "PBZ" },
ForecastZone::PA009 => crate::ZoneDetails {state: "PA", zone: "009", zone_numeric: 009, name: "Forest", wfo: "PBZ" },
ForecastZone::PA010 => crate::ZoneDetails {state: "PA", zone: "010", zone_numeric: 010, name: "Elk", wfo: "CTP" },
ForecastZone::PA011 => crate::ZoneDetails {state: "PA", zone: "011", zone_numeric: 011, name: "Cameron", wfo: "CTP" },
ForecastZone::PA012 => crate::ZoneDetails {state: "PA", zone: "012", zone_numeric: 012, name: "Northern Clinton", wfo: "CTP" },
ForecastZone::PA013 => crate::ZoneDetails {state: "PA", zone: "013", zone_numeric: 013, name: "Lawrence", wfo: "PBZ" },
ForecastZone::PA014 => crate::ZoneDetails {state: "PA", zone: "014", zone_numeric: 014, name: "Butler", wfo: "PBZ" },
ForecastZone::PA015 => crate::ZoneDetails {state: "PA", zone: "015", zone_numeric: 015, name: "Clarion", wfo: "PBZ" },
ForecastZone::PA016 => crate::ZoneDetails {state: "PA", zone: "016", zone_numeric: 016, name: "Jefferson", wfo: "PBZ" },
ForecastZone::PA017 => crate::ZoneDetails {state: "PA", zone: "017", zone_numeric: 017, name: "Clearfield", wfo: "CTP" },
ForecastZone::PA018 => crate::ZoneDetails {state: "PA", zone: "018", zone_numeric: 018, name: "Northern Centre", wfo: "CTP" },
ForecastZone::PA019 => crate::ZoneDetails {state: "PA", zone: "019", zone_numeric: 019, name: "Southern Centre", wfo: "CTP" },
ForecastZone::PA020 => crate::ZoneDetails {state: "PA", zone: "020", zone_numeric: 020, name: "Beaver", wfo: "PBZ" },
ForecastZone::PA021 => crate::ZoneDetails {state: "PA", zone: "021", zone_numeric: 021, name: "Allegheny", wfo: "PBZ" },
ForecastZone::PA022 => crate::ZoneDetails {state: "PA", zone: "022", zone_numeric: 022, name: "Armstrong", wfo: "PBZ" },
ForecastZone::PA023 => crate::ZoneDetails {state: "PA", zone: "023", zone_numeric: 023, name: "Indiana", wfo: "PBZ" },
ForecastZone::PA024 => crate::ZoneDetails {state: "PA", zone: "024", zone_numeric: 024, name: "Cambria", wfo: "CTP" },
ForecastZone::PA025 => crate::ZoneDetails {state: "PA", zone: "025", zone_numeric: 025, name: "Blair", wfo: "CTP" },
ForecastZone::PA026 => crate::ZoneDetails {state: "PA", zone: "026", zone_numeric: 026, name: "Huntingdon", wfo: "CTP" },
ForecastZone::PA027 => crate::ZoneDetails {state: "PA", zone: "027", zone_numeric: 027, name: "Mifflin", wfo: "CTP" },
ForecastZone::PA028 => crate::ZoneDetails {state: "PA", zone: "028", zone_numeric: 028, name: "Juniata", wfo: "CTP" },
ForecastZone::PA029 => crate::ZoneDetails {state: "PA", zone: "029", zone_numeric: 029, name: "Washington", wfo: "PBZ" },
ForecastZone::PA031 => crate::ZoneDetails {state: "PA", zone: "031", zone_numeric: 031, name: "Greene", wfo: "PBZ" },
ForecastZone::PA033 => crate::ZoneDetails {state: "PA", zone: "033", zone_numeric: 033, name: "Somerset", wfo: "CTP" },
ForecastZone::PA034 => crate::ZoneDetails {state: "PA", zone: "034", zone_numeric: 034, name: "Bedford", wfo: "CTP" },
ForecastZone::PA035 => crate::ZoneDetails {state: "PA", zone: "035", zone_numeric: 035, name: "Fulton", wfo: "CTP" },
ForecastZone::PA036 => crate::ZoneDetails {state: "PA", zone: "036", zone_numeric: 036, name: "Franklin", wfo: "CTP" },
ForecastZone::PA037 => crate::ZoneDetails {state: "PA", zone: "037", zone_numeric: 037, name: "Tioga", wfo: "CTP" },
ForecastZone::PA038 => crate::ZoneDetails {state: "PA", zone: "038", zone_numeric: 038, name: "Bradford", wfo: "BGM" },
ForecastZone::PA039 => crate::ZoneDetails {state: "PA", zone: "039", zone_numeric: 039, name: "Susquehanna", wfo: "BGM" },
ForecastZone::PA040 => crate::ZoneDetails {state: "PA", zone: "040", zone_numeric: 040, name: "Northern Wayne", wfo: "BGM" },
ForecastZone::PA041 => crate::ZoneDetails {state: "PA", zone: "041", zone_numeric: 041, name: "Northern Lycoming", wfo: "CTP" },
ForecastZone::PA042 => crate::ZoneDetails {state: "PA", zone: "042", zone_numeric: 042, name: "Sullivan", wfo: "CTP" },
ForecastZone::PA043 => crate::ZoneDetails {state: "PA", zone: "043", zone_numeric: 043, name: "Wyoming", wfo: "BGM" },
ForecastZone::PA044 => crate::ZoneDetails {state: "PA", zone: "044", zone_numeric: 044, name: "Lackawanna", wfo: "BGM" },
ForecastZone::PA045 => crate::ZoneDetails {state: "PA", zone: "045", zone_numeric: 045, name: "Southern Clinton", wfo: "CTP" },
ForecastZone::PA046 => crate::ZoneDetails {state: "PA", zone: "046", zone_numeric: 046, name: "Southern Lycoming", wfo: "CTP" },
ForecastZone::PA047 => crate::ZoneDetails {state: "PA", zone: "047", zone_numeric: 047, name: "Luzerne", wfo: "BGM" },
ForecastZone::PA048 => crate::ZoneDetails {state: "PA", zone: "048", zone_numeric: 048, name: "Pike", wfo: "BGM" },
ForecastZone::PA049 => crate::ZoneDetails {state: "PA", zone: "049", zone_numeric: 049, name: "Union", wfo: "CTP" },
ForecastZone::PA050 => crate::ZoneDetails {state: "PA", zone: "050", zone_numeric: 050, name: "Snyder", wfo: "CTP" },
ForecastZone::PA051 => crate::ZoneDetails {state: "PA", zone: "051", zone_numeric: 051, name: "Montour", wfo: "CTP" },
ForecastZone::PA052 => crate::ZoneDetails {state: "PA", zone: "052", zone_numeric: 052, name: "Northumberland", wfo: "CTP" },
ForecastZone::PA053 => crate::ZoneDetails {state: "PA", zone: "053", zone_numeric: 053, name: "Columbia", wfo: "CTP" },
ForecastZone::PA054 => crate::ZoneDetails {state: "PA", zone: "054", zone_numeric: 054, name: "Carbon", wfo: "PHI" },
ForecastZone::PA055 => crate::ZoneDetails {state: "PA", zone: "055", zone_numeric: 055, name: "Monroe", wfo: "PHI" },
ForecastZone::PA056 => crate::ZoneDetails {state: "PA", zone: "056", zone_numeric: 056, name: "Perry", wfo: "CTP" },
ForecastZone::PA057 => crate::ZoneDetails {state: "PA", zone: "057", zone_numeric: 057, name: "Dauphin", wfo: "CTP" },
ForecastZone::PA058 => crate::ZoneDetails {state: "PA", zone: "058", zone_numeric: 058, name: "Schuylkill", wfo: "CTP" },
ForecastZone::PA059 => crate::ZoneDetails {state: "PA", zone: "059", zone_numeric: 059, name: "Lebanon", wfo: "CTP" },
ForecastZone::PA060 => crate::ZoneDetails {state: "PA", zone: "060", zone_numeric: 060, name: "Berks", wfo: "PHI" },
ForecastZone::PA061 => crate::ZoneDetails {state: "PA", zone: "061", zone_numeric: 061, name: "Lehigh", wfo: "PHI" },
ForecastZone::PA062 => crate::ZoneDetails {state: "PA", zone: "062", zone_numeric: 062, name: "Northampton", wfo: "PHI" },
ForecastZone::PA063 => crate::ZoneDetails {state: "PA", zone: "063", zone_numeric: 063, name: "Cumberland", wfo: "CTP" },
ForecastZone::PA064 => crate::ZoneDetails {state: "PA", zone: "064", zone_numeric: 064, name: "Adams", wfo: "CTP" },
ForecastZone::PA065 => crate::ZoneDetails {state: "PA", zone: "065", zone_numeric: 065, name: "York", wfo: "CTP" },
ForecastZone::PA066 => crate::ZoneDetails {state: "PA", zone: "066", zone_numeric: 066, name: "Lancaster", wfo: "CTP" },
ForecastZone::PA070 => crate::ZoneDetails {state: "PA", zone: "070", zone_numeric: 070, name: "Delaware", wfo: "PHI" },
ForecastZone::PA071 => crate::ZoneDetails {state: "PA", zone: "071", zone_numeric: 071, name: "Philadelphia", wfo: "PHI" },
ForecastZone::PA072 => crate::ZoneDetails {state: "PA", zone: "072", zone_numeric: 072, name: "Southern Wayne", wfo: "BGM" },
ForecastZone::PA073 => crate::ZoneDetails {state: "PA", zone: "073", zone_numeric: 073, name: "Westmoreland", wfo: "PBZ" },
ForecastZone::PA074 => crate::ZoneDetails {state: "PA", zone: "074", zone_numeric: 074, name: "Westmoreland Ridges", wfo: "PBZ" },
ForecastZone::PA075 => crate::ZoneDetails {state: "PA", zone: "075", zone_numeric: 075, name: "Fayette", wfo: "PBZ" },
ForecastZone::PA076 => crate::ZoneDetails {state: "PA", zone: "076", zone_numeric: 076, name: "Fayette Ridges", wfo: "PBZ" },
ForecastZone::PA101 => crate::ZoneDetails {state: "PA", zone: "101", zone_numeric: 101, name: "Western Chester", wfo: "PHI" },
ForecastZone::PA102 => crate::ZoneDetails {state: "PA", zone: "102", zone_numeric: 102, name: "Eastern Chester", wfo: "PHI" },
ForecastZone::PA103 => crate::ZoneDetails {state: "PA", zone: "103", zone_numeric: 103, name: "Western Montgomery", wfo: "PHI" },
ForecastZone::PA104 => crate::ZoneDetails {state: "PA", zone: "104", zone_numeric: 104, name: "Eastern Montgomery", wfo: "PHI" },
ForecastZone::PA105 => crate::ZoneDetails {state: "PA", zone: "105", zone_numeric: 105, name: "Upper Bucks", wfo: "PHI" },
ForecastZone::PA106 => crate::ZoneDetails {state: "PA", zone: "106", zone_numeric: 106, name: "Lower Bucks", wfo: "PHI" },
ForecastZone::PR001 => crate::ZoneDetails {state: "PR", zone: "001", zone_numeric: 001, name: "San Juan and Vicinity", wfo: "SJU" },
ForecastZone::PR002 => crate::ZoneDetails {state: "PR", zone: "002", zone_numeric: 002, name: "Northeast", wfo: "SJU" },
ForecastZone::PR003 => crate::ZoneDetails {state: "PR", zone: "003", zone_numeric: 003, name: "Southeast", wfo: "SJU" },
ForecastZone::PR004 => crate::ZoneDetails {state: "PR", zone: "004", zone_numeric: 004, name: "Eastern Interior", wfo: "SJU" },
ForecastZone::PR005 => crate::ZoneDetails {state: "PR", zone: "005", zone_numeric: 005, name: "North Central", wfo: "SJU" },
ForecastZone::PR006 => crate::ZoneDetails {state: "PR", zone: "006", zone_numeric: 006, name: "Central Interior", wfo: "SJU" },
ForecastZone::PR007 => crate::ZoneDetails {state: "PR", zone: "007", zone_numeric: 007, name: "Ponce and Vicinity", wfo: "SJU" },
ForecastZone::PR008 => crate::ZoneDetails {state: "PR", zone: "008", zone_numeric: 008, name: "Northwest", wfo: "SJU" },
ForecastZone::PR009 => crate::ZoneDetails {state: "PR", zone: "009", zone_numeric: 009, name: "Western Interior", wfo: "SJU" },
ForecastZone::PR010 => crate::ZoneDetails {state: "PR", zone: "010", zone_numeric: 010, name: "Mayaguez and Vicinity", wfo: "SJU" },
ForecastZone::PR011 => crate::ZoneDetails {state: "PR", zone: "011", zone_numeric: 011, name: "Southwest", wfo: "SJU" },
ForecastZone::PR012 => crate::ZoneDetails {state: "PR", zone: "012", zone_numeric: 012, name: "Culebra", wfo: "SJU" },
ForecastZone::PR013 => crate::ZoneDetails {state: "PR", zone: "013", zone_numeric: 013, name: "Vieques", wfo: "SJU" },
ForecastZone::PW001 => crate::ZoneDetails {state: "PW", zone: "001", zone_numeric: 001, name: "Kayangel", wfo: "PQW" },
ForecastZone::PW002 => crate::ZoneDetails {state: "PW", zone: "002", zone_numeric: 002, name: "Melekeok", wfo: "PQW" },
ForecastZone::PW003 => crate::ZoneDetails {state: "PW", zone: "003", zone_numeric: 003, name: "Airai", wfo: "PQW" },
ForecastZone::PW004 => crate::ZoneDetails {state: "PW", zone: "004", zone_numeric: 004, name: "Koror", wfo: "PQW" },
ForecastZone::PW005 => crate::ZoneDetails {state: "PW", zone: "005", zone_numeric: 005, name: "Peleliu", wfo: "PQW" },
ForecastZone::PW006 => crate::ZoneDetails {state: "PW", zone: "006", zone_numeric: 006, name: "Angaur", wfo: "PQW" },
ForecastZone::PW007 => crate::ZoneDetails {state: "PW", zone: "007", zone_numeric: 007, name: "Sonsorol", wfo: "PQW" },
ForecastZone::PW008 => crate::ZoneDetails {state: "PW", zone: "008", zone_numeric: 008, name: "Tobi", wfo: "PQW" },
ForecastZone::RI001 => crate::ZoneDetails {state: "RI", zone: "001", zone_numeric: 001, name: "Northwest Providence", wfo: "BOX" },
ForecastZone::RI002 => crate::ZoneDetails {state: "RI", zone: "002", zone_numeric: 002, name: "Southeast Providence", wfo: "BOX" },
ForecastZone::RI003 => crate::ZoneDetails {state: "RI", zone: "003", zone_numeric: 003, name: "Western Kent", wfo: "BOX" },
ForecastZone::RI004 => crate::ZoneDetails {state: "RI", zone: "004", zone_numeric: 004, name: "Eastern Kent", wfo: "BOX" },
ForecastZone::RI005 => crate::ZoneDetails {state: "RI", zone: "005", zone_numeric: 005, name: "Bristol", wfo: "BOX" },
ForecastZone::RI006 => crate::ZoneDetails {state: "RI", zone: "006", zone_numeric: 006, name: "Washington", wfo: "BOX" },
ForecastZone::RI007 => crate::ZoneDetails {state: "RI", zone: "007", zone_numeric: 007, name: "Newport", wfo: "BOX" },
ForecastZone::RI008 => crate::ZoneDetails {state: "RI", zone: "008", zone_numeric: 008, name: "Block Island", wfo: "BOX" },
ForecastZone::SC008 => crate::ZoneDetails {state: "SC", zone: "008", zone_numeric: 008, name: "Cherokee", wfo: "GSP" },
ForecastZone::SC009 => crate::ZoneDetails {state: "SC", zone: "009", zone_numeric: 009, name: "York", wfo: "GSP" },
ForecastZone::SC010 => crate::ZoneDetails {state: "SC", zone: "010", zone_numeric: 010, name: "Anderson", wfo: "GSP" },
ForecastZone::SC011 => crate::ZoneDetails {state: "SC", zone: "011", zone_numeric: 011, name: "Abbeville", wfo: "GSP" },
ForecastZone::SC012 => crate::ZoneDetails {state: "SC", zone: "012", zone_numeric: 012, name: "Laurens", wfo: "GSP" },
ForecastZone::SC013 => crate::ZoneDetails {state: "SC", zone: "013", zone_numeric: 013, name: "Union", wfo: "GSP" },
ForecastZone::SC014 => crate::ZoneDetails {state: "SC", zone: "014", zone_numeric: 014, name: "Chester", wfo: "GSP" },
ForecastZone::SC016 => crate::ZoneDetails {state: "SC", zone: "016", zone_numeric: 016, name: "Chesterfield", wfo: "CAE" },
ForecastZone::SC017 => crate::ZoneDetails {state: "SC", zone: "017", zone_numeric: 017, name: "Marlboro", wfo: "ILM" },
ForecastZone::SC018 => crate::ZoneDetails {state: "SC", zone: "018", zone_numeric: 018, name: "McCormick", wfo: "CAE" },
ForecastZone::SC019 => crate::ZoneDetails {state: "SC", zone: "019", zone_numeric: 019, name: "Greenwood", wfo: "GSP" },
ForecastZone::SC020 => crate::ZoneDetails {state: "SC", zone: "020", zone_numeric: 020, name: "Newberry", wfo: "CAE" },
ForecastZone::SC021 => crate::ZoneDetails {state: "SC", zone: "021", zone_numeric: 021, name: "Fairfield", wfo: "CAE" },
ForecastZone::SC022 => crate::ZoneDetails {state: "SC", zone: "022", zone_numeric: 022, name: "Kershaw", wfo: "CAE" },
ForecastZone::SC023 => crate::ZoneDetails {state: "SC", zone: "023", zone_numeric: 023, name: "Darlington", wfo: "ILM" },
ForecastZone::SC024 => crate::ZoneDetails {state: "SC", zone: "024", zone_numeric: 024, name: "Dillon", wfo: "ILM" },
ForecastZone::SC025 => crate::ZoneDetails {state: "SC", zone: "025", zone_numeric: 025, name: "Edgefield", wfo: "CAE" },
ForecastZone::SC026 => crate::ZoneDetails {state: "SC", zone: "026", zone_numeric: 026, name: "Saluda", wfo: "CAE" },
ForecastZone::SC027 => crate::ZoneDetails {state: "SC", zone: "027", zone_numeric: 027, name: "Lexington", wfo: "CAE" },
ForecastZone::SC028 => crate::ZoneDetails {state: "SC", zone: "028", zone_numeric: 028, name: "Richland", wfo: "CAE" },
ForecastZone::SC029 => crate::ZoneDetails {state: "SC", zone: "029", zone_numeric: 029, name: "Lee", wfo: "CAE" },
ForecastZone::SC030 => crate::ZoneDetails {state: "SC", zone: "030", zone_numeric: 030, name: "Aiken", wfo: "CAE" },
ForecastZone::SC031 => crate::ZoneDetails {state: "SC", zone: "031", zone_numeric: 031, name: "Sumter", wfo: "CAE" },
ForecastZone::SC032 => crate::ZoneDetails {state: "SC", zone: "032", zone_numeric: 032, name: "Florence", wfo: "ILM" },
ForecastZone::SC033 => crate::ZoneDetails {state: "SC", zone: "033", zone_numeric: 033, name: "Marion", wfo: "ILM" },
ForecastZone::SC035 => crate::ZoneDetails {state: "SC", zone: "035", zone_numeric: 035, name: "Barnwell", wfo: "CAE" },
ForecastZone::SC037 => crate::ZoneDetails {state: "SC", zone: "037", zone_numeric: 037, name: "Calhoun", wfo: "CAE" },
ForecastZone::SC038 => crate::ZoneDetails {state: "SC", zone: "038", zone_numeric: 038, name: "Clarendon", wfo: "CAE" },
ForecastZone::SC039 => crate::ZoneDetails {state: "SC", zone: "039", zone_numeric: 039, name: "Williamsburg", wfo: "ILM" },
ForecastZone::SC040 => crate::ZoneDetails {state: "SC", zone: "040", zone_numeric: 040, name: "Allendale", wfo: "CHS" },
ForecastZone::SC041 => crate::ZoneDetails {state: "SC", zone: "041", zone_numeric: 041, name: "Bamberg", wfo: "CAE" },
ForecastZone::SC042 => crate::ZoneDetails {state: "SC", zone: "042", zone_numeric: 042, name: "Hampton", wfo: "CHS" },
ForecastZone::SC043 => crate::ZoneDetails {state: "SC", zone: "043", zone_numeric: 043, name: "Inland Colleton", wfo: "CHS" },
ForecastZone::SC044 => crate::ZoneDetails {state: "SC", zone: "044", zone_numeric: 044, name: "Dorchester", wfo: "CHS" },
ForecastZone::SC045 => crate::ZoneDetails {state: "SC", zone: "045", zone_numeric: 045, name: "Inland Berkeley", wfo: "CHS" },
ForecastZone::SC047 => crate::ZoneDetails {state: "SC", zone: "047", zone_numeric: 047, name: "Inland Jasper", wfo: "CHS" },
ForecastZone::SC048 => crate::ZoneDetails {state: "SC", zone: "048", zone_numeric: 048, name: "Beaufort", wfo: "CHS" },
ForecastZone::SC049 => crate::ZoneDetails {state: "SC", zone: "049", zone_numeric: 049, name: "Coastal Colleton", wfo: "CHS" },
ForecastZone::SC050 => crate::ZoneDetails {state: "SC", zone: "050", zone_numeric: 050, name: "Charleston", wfo: "CHS" },
ForecastZone::SC051 => crate::ZoneDetails {state: "SC", zone: "051", zone_numeric: 051, name: "Coastal Jasper", wfo: "CHS" },
ForecastZone::SC052 => crate::ZoneDetails {state: "SC", zone: "052", zone_numeric: 052, name: "Tidal Berkeley", wfo: "CHS" },
ForecastZone::SC054 => crate::ZoneDetails {state: "SC", zone: "054", zone_numeric: 054, name: "Coastal Horry", wfo: "ILM" },
ForecastZone::SC055 => crate::ZoneDetails {state: "SC", zone: "055", zone_numeric: 055, name: "Inland Georgetown", wfo: "ILM" },
ForecastZone::SC056 => crate::ZoneDetails {state: "SC", zone: "056", zone_numeric: 056, name: "Coastal Georgetown", wfo: "ILM" },
ForecastZone::SC058 => crate::ZoneDetails {state: "SC", zone: "058", zone_numeric: 058, name: "Central Horry", wfo: "ILM" },
ForecastZone::SC059 => crate::ZoneDetails {state: "SC", zone: "059", zone_numeric: 059, name: "Northern Horry", wfo: "ILM" },
ForecastZone::SC101 => crate::ZoneDetails {state: "SC", zone: "101", zone_numeric: 101, name: "Oconee Mountains", wfo: "GSP" },
ForecastZone::SC102 => crate::ZoneDetails {state: "SC", zone: "102", zone_numeric: 102, name: "Pickens Mountains", wfo: "GSP" },
ForecastZone::SC103 => crate::ZoneDetails {state: "SC", zone: "103", zone_numeric: 103, name: "Greenville Mountains", wfo: "GSP" },
ForecastZone::SC104 => crate::ZoneDetails {state: "SC", zone: "104", zone_numeric: 104, name: "Greater Oconee", wfo: "GSP" },
ForecastZone::SC105 => crate::ZoneDetails {state: "SC", zone: "105", zone_numeric: 105, name: "Greater Pickens", wfo: "GSP" },
ForecastZone::SC106 => crate::ZoneDetails {state: "SC", zone: "106", zone_numeric: 106, name: "Central Greenville", wfo: "GSP" },
ForecastZone::SC107 => crate::ZoneDetails {state: "SC", zone: "107", zone_numeric: 107, name: "Southern Greenville", wfo: "GSP" },
ForecastZone::SC108 => crate::ZoneDetails {state: "SC", zone: "108", zone_numeric: 108, name: "Northern Spartanburg", wfo: "GSP" },
ForecastZone::SC109 => crate::ZoneDetails {state: "SC", zone: "109", zone_numeric: 109, name: "Southern Spartanburg", wfo: "GSP" },
ForecastZone::SC115 => crate::ZoneDetails {state: "SC", zone: "115", zone_numeric: 115, name: "Northern Lancaster", wfo: "CAE" },
ForecastZone::SC116 => crate::ZoneDetails {state: "SC", zone: "116", zone_numeric: 116, name: "Southern Lancaster", wfo: "CAE" },
ForecastZone::SC135 => crate::ZoneDetails {state: "SC", zone: "135", zone_numeric: 135, name: "Northwestern Orangeburg", wfo: "CAE" },
ForecastZone::SC136 => crate::ZoneDetails {state: "SC", zone: "136", zone_numeric: 136, name: "Central Orangeburg", wfo: "CAE" },
ForecastZone::SC137 => crate::ZoneDetails {state: "SC", zone: "137", zone_numeric: 137, name: "Southeastern Orangeburg", wfo: "CAE" },
ForecastZone::SD001 => crate::ZoneDetails {state: "SD", zone: "001", zone_numeric: 001, name: "Harding", wfo: "UNR" },
ForecastZone::SD002 => crate::ZoneDetails {state: "SD", zone: "002", zone_numeric: 002, name: "Perkins", wfo: "UNR" },
ForecastZone::SD003 => crate::ZoneDetails {state: "SD", zone: "003", zone_numeric: 003, name: "Corson", wfo: "ABR" },
ForecastZone::SD004 => crate::ZoneDetails {state: "SD", zone: "004", zone_numeric: 004, name: "Campbell", wfo: "ABR" },
ForecastZone::SD005 => crate::ZoneDetails {state: "SD", zone: "005", zone_numeric: 005, name: "McPherson", wfo: "ABR" },
ForecastZone::SD006 => crate::ZoneDetails {state: "SD", zone: "006", zone_numeric: 006, name: "Brown", wfo: "ABR" },
ForecastZone::SD007 => crate::ZoneDetails {state: "SD", zone: "007", zone_numeric: 007, name: "Marshall", wfo: "ABR" },
ForecastZone::SD008 => crate::ZoneDetails {state: "SD", zone: "008", zone_numeric: 008, name: "Roberts", wfo: "ABR" },
ForecastZone::SD009 => crate::ZoneDetails {state: "SD", zone: "009", zone_numeric: 009, name: "Walworth", wfo: "ABR" },
ForecastZone::SD010 => crate::ZoneDetails {state: "SD", zone: "010", zone_numeric: 010, name: "Edmunds", wfo: "ABR" },
ForecastZone::SD011 => crate::ZoneDetails {state: "SD", zone: "011", zone_numeric: 011, name: "Day", wfo: "ABR" },
ForecastZone::SD012 => crate::ZoneDetails {state: "SD", zone: "012", zone_numeric: 012, name: "Butte", wfo: "UNR" },
ForecastZone::SD013 => crate::ZoneDetails {state: "SD", zone: "013", zone_numeric: 013, name: "Northern Meade Co Plains", wfo: "UNR" },
ForecastZone::SD014 => crate::ZoneDetails {state: "SD", zone: "014", zone_numeric: 014, name: "Ziebach", wfo: "UNR" },
ForecastZone::SD015 => crate::ZoneDetails {state: "SD", zone: "015", zone_numeric: 015, name: "Dewey", wfo: "ABR" },
ForecastZone::SD016 => crate::ZoneDetails {state: "SD", zone: "016", zone_numeric: 016, name: "Potter", wfo: "ABR" },
ForecastZone::SD017 => crate::ZoneDetails {state: "SD", zone: "017", zone_numeric: 017, name: "Faulk", wfo: "ABR" },
ForecastZone::SD018 => crate::ZoneDetails {state: "SD", zone: "018", zone_numeric: 018, name: "Spink", wfo: "ABR" },
ForecastZone::SD019 => crate::ZoneDetails {state: "SD", zone: "019", zone_numeric: 019, name: "Clark", wfo: "ABR" },
ForecastZone::SD020 => crate::ZoneDetails {state: "SD", zone: "020", zone_numeric: 020, name: "Codington", wfo: "ABR" },
ForecastZone::SD021 => crate::ZoneDetails {state: "SD", zone: "021", zone_numeric: 021, name: "Grant", wfo: "ABR" },
ForecastZone::SD022 => crate::ZoneDetails {state: "SD", zone: "022", zone_numeric: 022, name: "Hamlin", wfo: "ABR" },
ForecastZone::SD023 => crate::ZoneDetails {state: "SD", zone: "023", zone_numeric: 023, name: "Deuel", wfo: "ABR" },
ForecastZone::SD024 => crate::ZoneDetails {state: "SD", zone: "024", zone_numeric: 024, name: "Northern Black Hills", wfo: "UNR" },
ForecastZone::SD025 => crate::ZoneDetails {state: "SD", zone: "025", zone_numeric: 025, name: "Northern Foot Hills", wfo: "UNR" },
ForecastZone::SD026 => crate::ZoneDetails {state: "SD", zone: "026", zone_numeric: 026, name: "Rapid City", wfo: "UNR" },
ForecastZone::SD027 => crate::ZoneDetails {state: "SD", zone: "027", zone_numeric: 027, name: "Southern Foot Hills", wfo: "UNR" },
ForecastZone::SD028 => crate::ZoneDetails {state: "SD", zone: "028", zone_numeric: 028, name: "Central Black Hills", wfo: "UNR" },
ForecastZone::SD029 => crate::ZoneDetails {state: "SD", zone: "029", zone_numeric: 029, name: "Southern Black Hills", wfo: "UNR" },
ForecastZone::SD030 => crate::ZoneDetails {state: "SD", zone: "030", zone_numeric: 030, name: "Custer Co Plains", wfo: "UNR" },
ForecastZone::SD031 => crate::ZoneDetails {state: "SD", zone: "031", zone_numeric: 031, name: "Pennington Co Plains", wfo: "UNR" },
ForecastZone::SD032 => crate::ZoneDetails {state: "SD", zone: "032", zone_numeric: 032, name: "Haakon", wfo: "UNR" },
ForecastZone::SD033 => crate::ZoneDetails {state: "SD", zone: "033", zone_numeric: 033, name: "Stanley", wfo: "ABR" },
ForecastZone::SD034 => crate::ZoneDetails {state: "SD", zone: "034", zone_numeric: 034, name: "Sully", wfo: "ABR" },
ForecastZone::SD035 => crate::ZoneDetails {state: "SD", zone: "035", zone_numeric: 035, name: "Hughes", wfo: "ABR" },
ForecastZone::SD036 => crate::ZoneDetails {state: "SD", zone: "036", zone_numeric: 036, name: "Hyde", wfo: "ABR" },
ForecastZone::SD037 => crate::ZoneDetails {state: "SD", zone: "037", zone_numeric: 037, name: "Hand", wfo: "ABR" },
ForecastZone::SD038 => crate::ZoneDetails {state: "SD", zone: "038", zone_numeric: 038, name: "Beadle", wfo: "FSD" },
ForecastZone::SD039 => crate::ZoneDetails {state: "SD", zone: "039", zone_numeric: 039, name: "Kingsbury", wfo: "FSD" },
ForecastZone::SD040 => crate::ZoneDetails {state: "SD", zone: "040", zone_numeric: 040, name: "Brookings", wfo: "FSD" },
ForecastZone::SD041 => crate::ZoneDetails {state: "SD", zone: "041", zone_numeric: 041, name: "Fall River", wfo: "UNR" },
ForecastZone::SD042 => crate::ZoneDetails {state: "SD", zone: "042", zone_numeric: 042, name: "Oglala Lakota", wfo: "UNR" },
ForecastZone::SD043 => crate::ZoneDetails {state: "SD", zone: "043", zone_numeric: 043, name: "Jackson", wfo: "UNR" },
ForecastZone::SD044 => crate::ZoneDetails {state: "SD", zone: "044", zone_numeric: 044, name: "Bennett", wfo: "UNR" },
ForecastZone::SD045 => crate::ZoneDetails {state: "SD", zone: "045", zone_numeric: 045, name: "Jones", wfo: "ABR" },
ForecastZone::SD046 => crate::ZoneDetails {state: "SD", zone: "046", zone_numeric: 046, name: "Mellette", wfo: "UNR" },
ForecastZone::SD047 => crate::ZoneDetails {state: "SD", zone: "047", zone_numeric: 047, name: "Todd", wfo: "UNR" },
ForecastZone::SD048 => crate::ZoneDetails {state: "SD", zone: "048", zone_numeric: 048, name: "Lyman", wfo: "ABR" },
ForecastZone::SD049 => crate::ZoneDetails {state: "SD", zone: "049", zone_numeric: 049, name: "Tripp", wfo: "UNR" },
ForecastZone::SD050 => crate::ZoneDetails {state: "SD", zone: "050", zone_numeric: 050, name: "Gregory", wfo: "FSD" },
ForecastZone::SD051 => crate::ZoneDetails {state: "SD", zone: "051", zone_numeric: 051, name: "Buffalo", wfo: "ABR" },
ForecastZone::SD052 => crate::ZoneDetails {state: "SD", zone: "052", zone_numeric: 052, name: "Jerauld", wfo: "FSD" },
ForecastZone::SD053 => crate::ZoneDetails {state: "SD", zone: "053", zone_numeric: 053, name: "Sanborn", wfo: "FSD" },
ForecastZone::SD054 => crate::ZoneDetails {state: "SD", zone: "054", zone_numeric: 054, name: "Miner", wfo: "FSD" },
ForecastZone::SD055 => crate::ZoneDetails {state: "SD", zone: "055", zone_numeric: 055, name: "Lake", wfo: "FSD" },
ForecastZone::SD056 => crate::ZoneDetails {state: "SD", zone: "056", zone_numeric: 056, name: "Moody", wfo: "FSD" },
ForecastZone::SD057 => crate::ZoneDetails {state: "SD", zone: "057", zone_numeric: 057, name: "Brule", wfo: "FSD" },
ForecastZone::SD058 => crate::ZoneDetails {state: "SD", zone: "058", zone_numeric: 058, name: "Aurora", wfo: "FSD" },
ForecastZone::SD059 => crate::ZoneDetails {state: "SD", zone: "059", zone_numeric: 059, name: "Davison", wfo: "FSD" },
ForecastZone::SD060 => crate::ZoneDetails {state: "SD", zone: "060", zone_numeric: 060, name: "Hanson", wfo: "FSD" },
ForecastZone::SD061 => crate::ZoneDetails {state: "SD", zone: "061", zone_numeric: 061, name: "McCook", wfo: "FSD" },
ForecastZone::SD062 => crate::ZoneDetails {state: "SD", zone: "062", zone_numeric: 062, name: "Minnehaha", wfo: "FSD" },
ForecastZone::SD063 => crate::ZoneDetails {state: "SD", zone: "063", zone_numeric: 063, name: "Charles Mix", wfo: "FSD" },
ForecastZone::SD064 => crate::ZoneDetails {state: "SD", zone: "064", zone_numeric: 064, name: "Douglas", wfo: "FSD" },
ForecastZone::SD065 => crate::ZoneDetails {state: "SD", zone: "065", zone_numeric: 065, name: "Hutchinson", wfo: "FSD" },
ForecastZone::SD066 => crate::ZoneDetails {state: "SD", zone: "066", zone_numeric: 066, name: "Turner", wfo: "FSD" },
ForecastZone::SD067 => crate::ZoneDetails {state: "SD", zone: "067", zone_numeric: 067, name: "Lincoln", wfo: "FSD" },
ForecastZone::SD068 => crate::ZoneDetails {state: "SD", zone: "068", zone_numeric: 068, name: "Bon Homme", wfo: "FSD" },
ForecastZone::SD069 => crate::ZoneDetails {state: "SD", zone: "069", zone_numeric: 069, name: "Yankton", wfo: "FSD" },
ForecastZone::SD070 => crate::ZoneDetails {state: "SD", zone: "070", zone_numeric: 070, name: "Clay", wfo: "FSD" },
ForecastZone::SD071 => crate::ZoneDetails {state: "SD", zone: "071", zone_numeric: 071, name: "Union", wfo: "FSD" },
ForecastZone::SD072 => crate::ZoneDetails {state: "SD", zone: "072", zone_numeric: 072, name: "Sturgis/Piedmont Foot Hills", wfo: "UNR" },
ForecastZone::SD073 => crate::ZoneDetails {state: "SD", zone: "073", zone_numeric: 073, name: "Southern Meade Co Plains", wfo: "UNR" },
ForecastZone::SD074 => crate::ZoneDetails {state: "SD", zone: "074", zone_numeric: 074, name: "Hermosa Foot Hills", wfo: "UNR" },
ForecastZone::TN001 => crate::ZoneDetails {state: "TN", zone: "001", zone_numeric: 001, name: "Lake", wfo: "MEG" },
ForecastZone::TN002 => crate::ZoneDetails {state: "TN", zone: "002", zone_numeric: 002, name: "Obion", wfo: "MEG" },
ForecastZone::TN003 => crate::ZoneDetails {state: "TN", zone: "003", zone_numeric: 003, name: "Weakley", wfo: "MEG" },
ForecastZone::TN004 => crate::ZoneDetails {state: "TN", zone: "004", zone_numeric: 004, name: "Henry", wfo: "MEG" },
ForecastZone::TN005 => crate::ZoneDetails {state: "TN", zone: "005", zone_numeric: 005, name: "Stewart", wfo: "OHX" },
ForecastZone::TN006 => crate::ZoneDetails {state: "TN", zone: "006", zone_numeric: 006, name: "Montgomery", wfo: "OHX" },
ForecastZone::TN007 => crate::ZoneDetails {state: "TN", zone: "007", zone_numeric: 007, name: "Robertson", wfo: "OHX" },
ForecastZone::TN008 => crate::ZoneDetails {state: "TN", zone: "008", zone_numeric: 008, name: "Sumner", wfo: "OHX" },
ForecastZone::TN009 => crate::ZoneDetails {state: "TN", zone: "009", zone_numeric: 009, name: "Macon", wfo: "OHX" },
ForecastZone::TN010 => crate::ZoneDetails {state: "TN", zone: "010", zone_numeric: 010, name: "Clay", wfo: "OHX" },
ForecastZone::TN011 => crate::ZoneDetails {state: "TN", zone: "011", zone_numeric: 011, name: "Pickett", wfo: "OHX" },
ForecastZone::TN012 => crate::ZoneDetails {state: "TN", zone: "012", zone_numeric: 012, name: "Scott", wfo: "MRX" },
ForecastZone::TN013 => crate::ZoneDetails {state: "TN", zone: "013", zone_numeric: 013, name: "Campbell", wfo: "MRX" },
ForecastZone::TN014 => crate::ZoneDetails {state: "TN", zone: "014", zone_numeric: 014, name: "Claiborne", wfo: "MRX" },
ForecastZone::TN015 => crate::ZoneDetails {state: "TN", zone: "015", zone_numeric: 015, name: "Hancock", wfo: "MRX" },
ForecastZone::TN016 => crate::ZoneDetails {state: "TN", zone: "016", zone_numeric: 016, name: "Hawkins", wfo: "MRX" },
ForecastZone::TN017 => crate::ZoneDetails {state: "TN", zone: "017", zone_numeric: 017, name: "Sullivan", wfo: "MRX" },
ForecastZone::TN018 => crate::ZoneDetails {state: "TN", zone: "018", zone_numeric: 018, name: "Johnson", wfo: "MRX" },
ForecastZone::TN019 => crate::ZoneDetails {state: "TN", zone: "019", zone_numeric: 019, name: "Dyer", wfo: "MEG" },
ForecastZone::TN020 => crate::ZoneDetails {state: "TN", zone: "020", zone_numeric: 020, name: "Gibson", wfo: "MEG" },
ForecastZone::TN021 => crate::ZoneDetails {state: "TN", zone: "021", zone_numeric: 021, name: "Carroll", wfo: "MEG" },
ForecastZone::TN022 => crate::ZoneDetails {state: "TN", zone: "022", zone_numeric: 022, name: "Benton", wfo: "MEG" },
ForecastZone::TN023 => crate::ZoneDetails {state: "TN", zone: "023", zone_numeric: 023, name: "Houston", wfo: "OHX" },
ForecastZone::TN024 => crate::ZoneDetails {state: "TN", zone: "024", zone_numeric: 024, name: "Humphreys", wfo: "OHX" },
ForecastZone::TN025 => crate::ZoneDetails {state: "TN", zone: "025", zone_numeric: 025, name: "Dickson", wfo: "OHX" },
ForecastZone::TN026 => crate::ZoneDetails {state: "TN", zone: "026", zone_numeric: 026, name: "Cheatham", wfo: "OHX" },
ForecastZone::TN027 => crate::ZoneDetails {state: "TN", zone: "027", zone_numeric: 027, name: "Davidson", wfo: "OHX" },
ForecastZone::TN028 => crate::ZoneDetails {state: "TN", zone: "028", zone_numeric: 028, name: "Wilson", wfo: "OHX" },
ForecastZone::TN029 => crate::ZoneDetails {state: "TN", zone: "029", zone_numeric: 029, name: "Trousdale", wfo: "OHX" },
ForecastZone::TN030 => crate::ZoneDetails {state: "TN", zone: "030", zone_numeric: 030, name: "Smith", wfo: "OHX" },
ForecastZone::TN031 => crate::ZoneDetails {state: "TN", zone: "031", zone_numeric: 031, name: "Jackson", wfo: "OHX" },
ForecastZone::TN032 => crate::ZoneDetails {state: "TN", zone: "032", zone_numeric: 032, name: "Putnam", wfo: "OHX" },
ForecastZone::TN033 => crate::ZoneDetails {state: "TN", zone: "033", zone_numeric: 033, name: "Overton", wfo: "OHX" },
ForecastZone::TN034 => crate::ZoneDetails {state: "TN", zone: "034", zone_numeric: 034, name: "Fentress", wfo: "OHX" },
ForecastZone::TN035 => crate::ZoneDetails {state: "TN", zone: "035", zone_numeric: 035, name: "Morgan", wfo: "MRX" },
ForecastZone::TN036 => crate::ZoneDetails {state: "TN", zone: "036", zone_numeric: 036, name: "Anderson", wfo: "MRX" },
ForecastZone::TN037 => crate::ZoneDetails {state: "TN", zone: "037", zone_numeric: 037, name: "Union", wfo: "MRX" },
ForecastZone::TN038 => crate::ZoneDetails {state: "TN", zone: "038", zone_numeric: 038, name: "Grainger", wfo: "MRX" },
ForecastZone::TN039 => crate::ZoneDetails {state: "TN", zone: "039", zone_numeric: 039, name: "Hamblen", wfo: "MRX" },
ForecastZone::TN040 => crate::ZoneDetails {state: "TN", zone: "040", zone_numeric: 040, name: "Northwest Cocke", wfo: "MRX" },
ForecastZone::TN041 => crate::ZoneDetails {state: "TN", zone: "041", zone_numeric: 041, name: "Cocke Smoky Mountains", wfo: "MRX" },
ForecastZone::TN042 => crate::ZoneDetails {state: "TN", zone: "042", zone_numeric: 042, name: "Northwest Greene", wfo: "MRX" },
ForecastZone::TN043 => crate::ZoneDetails {state: "TN", zone: "043", zone_numeric: 043, name: "Southeast Greene", wfo: "MRX" },
ForecastZone::TN044 => crate::ZoneDetails {state: "TN", zone: "044", zone_numeric: 044, name: "Washington", wfo: "MRX" },
ForecastZone::TN045 => crate::ZoneDetails {state: "TN", zone: "045", zone_numeric: 045, name: "Unicoi", wfo: "MRX" },
ForecastZone::TN046 => crate::ZoneDetails {state: "TN", zone: "046", zone_numeric: 046, name: "Northwest Carter", wfo: "MRX" },
ForecastZone::TN047 => crate::ZoneDetails {state: "TN", zone: "047", zone_numeric: 047, name: "Southeast Carter", wfo: "MRX" },
ForecastZone::TN048 => crate::ZoneDetails {state: "TN", zone: "048", zone_numeric: 048, name: "Lauderdale", wfo: "MEG" },
ForecastZone::TN049 => crate::ZoneDetails {state: "TN", zone: "049", zone_numeric: 049, name: "Tipton", wfo: "MEG" },
ForecastZone::TN050 => crate::ZoneDetails {state: "TN", zone: "050", zone_numeric: 050, name: "Haywood", wfo: "MEG" },
ForecastZone::TN051 => crate::ZoneDetails {state: "TN", zone: "051", zone_numeric: 051, name: "Crockett", wfo: "MEG" },
ForecastZone::TN052 => crate::ZoneDetails {state: "TN", zone: "052", zone_numeric: 052, name: "Madison", wfo: "MEG" },
ForecastZone::TN053 => crate::ZoneDetails {state: "TN", zone: "053", zone_numeric: 053, name: "Chester", wfo: "MEG" },
ForecastZone::TN054 => crate::ZoneDetails {state: "TN", zone: "054", zone_numeric: 054, name: "Henderson", wfo: "MEG" },
ForecastZone::TN055 => crate::ZoneDetails {state: "TN", zone: "055", zone_numeric: 055, name: "Decatur", wfo: "MEG" },
ForecastZone::TN056 => crate::ZoneDetails {state: "TN", zone: "056", zone_numeric: 056, name: "Perry", wfo: "OHX" },
ForecastZone::TN057 => crate::ZoneDetails {state: "TN", zone: "057", zone_numeric: 057, name: "Hickman", wfo: "OHX" },
ForecastZone::TN058 => crate::ZoneDetails {state: "TN", zone: "058", zone_numeric: 058, name: "Lewis", wfo: "OHX" },
ForecastZone::TN059 => crate::ZoneDetails {state: "TN", zone: "059", zone_numeric: 059, name: "Williamson", wfo: "OHX" },
ForecastZone::TN060 => crate::ZoneDetails {state: "TN", zone: "060", zone_numeric: 060, name: "Maury", wfo: "OHX" },
ForecastZone::TN061 => crate::ZoneDetails {state: "TN", zone: "061", zone_numeric: 061, name: "Marshall", wfo: "OHX" },
ForecastZone::TN062 => crate::ZoneDetails {state: "TN", zone: "062", zone_numeric: 062, name: "Rutherford", wfo: "OHX" },
ForecastZone::TN063 => crate::ZoneDetails {state: "TN", zone: "063", zone_numeric: 063, name: "Cannon", wfo: "OHX" },
ForecastZone::TN064 => crate::ZoneDetails {state: "TN", zone: "064", zone_numeric: 064, name: "De Kalb", wfo: "OHX" },
ForecastZone::TN065 => crate::ZoneDetails {state: "TN", zone: "065", zone_numeric: 065, name: "White", wfo: "OHX" },
ForecastZone::TN066 => crate::ZoneDetails {state: "TN", zone: "066", zone_numeric: 066, name: "Cumberland", wfo: "OHX" },
ForecastZone::TN067 => crate::ZoneDetails {state: "TN", zone: "067", zone_numeric: 067, name: "Roane", wfo: "MRX" },
ForecastZone::TN068 => crate::ZoneDetails {state: "TN", zone: "068", zone_numeric: 068, name: "Loudon", wfo: "MRX" },
ForecastZone::TN069 => crate::ZoneDetails {state: "TN", zone: "069", zone_numeric: 069, name: "Knox", wfo: "MRX" },
ForecastZone::TN070 => crate::ZoneDetails {state: "TN", zone: "070", zone_numeric: 070, name: "Jefferson", wfo: "MRX" },
ForecastZone::TN071 => crate::ZoneDetails {state: "TN", zone: "071", zone_numeric: 071, name: "NW Blount", wfo: "MRX" },
ForecastZone::TN072 => crate::ZoneDetails {state: "TN", zone: "072", zone_numeric: 072, name: "Blount Smoky Mountains", wfo: "MRX" },
ForecastZone::TN073 => crate::ZoneDetails {state: "TN", zone: "073", zone_numeric: 073, name: "North Sevier", wfo: "MRX" },
ForecastZone::TN074 => crate::ZoneDetails {state: "TN", zone: "074", zone_numeric: 074, name: "Sevier Smoky Mountains", wfo: "MRX" },
ForecastZone::TN075 => crate::ZoneDetails {state: "TN", zone: "075", zone_numeric: 075, name: "Bedford", wfo: "OHX" },
ForecastZone::TN076 => crate::ZoneDetails {state: "TN", zone: "076", zone_numeric: 076, name: "Moore", wfo: "HUN" },
ForecastZone::TN077 => crate::ZoneDetails {state: "TN", zone: "077", zone_numeric: 077, name: "Coffee", wfo: "OHX" },
ForecastZone::TN078 => crate::ZoneDetails {state: "TN", zone: "078", zone_numeric: 078, name: "Warren", wfo: "OHX" },
ForecastZone::TN079 => crate::ZoneDetails {state: "TN", zone: "079", zone_numeric: 079, name: "Grundy", wfo: "OHX" },
ForecastZone::TN080 => crate::ZoneDetails {state: "TN", zone: "080", zone_numeric: 080, name: "Van Buren", wfo: "OHX" },
ForecastZone::TN081 => crate::ZoneDetails {state: "TN", zone: "081", zone_numeric: 081, name: "Sequatchie", wfo: "MRX" },
ForecastZone::TN082 => crate::ZoneDetails {state: "TN", zone: "082", zone_numeric: 082, name: "Bledsoe", wfo: "MRX" },
ForecastZone::TN083 => crate::ZoneDetails {state: "TN", zone: "083", zone_numeric: 083, name: "Rhea", wfo: "MRX" },
ForecastZone::TN084 => crate::ZoneDetails {state: "TN", zone: "084", zone_numeric: 084, name: "Meigs", wfo: "MRX" },
ForecastZone::TN085 => crate::ZoneDetails {state: "TN", zone: "085", zone_numeric: 085, name: "McMinn", wfo: "MRX" },
ForecastZone::TN086 => crate::ZoneDetails {state: "TN", zone: "086", zone_numeric: 086, name: "Northwest Monroe", wfo: "MRX" },
ForecastZone::TN087 => crate::ZoneDetails {state: "TN", zone: "087", zone_numeric: 087, name: "Southeast Monroe", wfo: "MRX" },
ForecastZone::TN088 => crate::ZoneDetails {state: "TN", zone: "088", zone_numeric: 088, name: "Shelby", wfo: "MEG" },
ForecastZone::TN089 => crate::ZoneDetails {state: "TN", zone: "089", zone_numeric: 089, name: "Fayette", wfo: "MEG" },
ForecastZone::TN090 => crate::ZoneDetails {state: "TN", zone: "090", zone_numeric: 090, name: "Hardeman", wfo: "MEG" },
ForecastZone::TN091 => crate::ZoneDetails {state: "TN", zone: "091", zone_numeric: 091, name: "McNairy", wfo: "MEG" },
ForecastZone::TN092 => crate::ZoneDetails {state: "TN", zone: "092", zone_numeric: 092, name: "Hardin", wfo: "MEG" },
ForecastZone::TN093 => crate::ZoneDetails {state: "TN", zone: "093", zone_numeric: 093, name: "Wayne", wfo: "OHX" },
ForecastZone::TN094 => crate::ZoneDetails {state: "TN", zone: "094", zone_numeric: 094, name: "Lawrence", wfo: "OHX" },
ForecastZone::TN095 => crate::ZoneDetails {state: "TN", zone: "095", zone_numeric: 095, name: "Giles", wfo: "OHX" },
ForecastZone::TN096 => crate::ZoneDetails {state: "TN", zone: "096", zone_numeric: 096, name: "Lincoln", wfo: "HUN" },
ForecastZone::TN097 => crate::ZoneDetails {state: "TN", zone: "097", zone_numeric: 097, name: "Franklin", wfo: "HUN" },
ForecastZone::TN098 => crate::ZoneDetails {state: "TN", zone: "098", zone_numeric: 098, name: "Marion", wfo: "MRX" },
ForecastZone::TN099 => crate::ZoneDetails {state: "TN", zone: "099", zone_numeric: 099, name: "Hamilton", wfo: "MRX" },
ForecastZone::TN100 => crate::ZoneDetails {state: "TN", zone: "100", zone_numeric: 100, name: "Bradley", wfo: "MRX" },
ForecastZone::TN101 => crate::ZoneDetails {state: "TN", zone: "101", zone_numeric: 101, name: "West Polk", wfo: "MRX" },
ForecastZone::TN102 => crate::ZoneDetails {state: "TN", zone: "102", zone_numeric: 102, name: "East Polk", wfo: "MRX" },
ForecastZone::TX001 => crate::ZoneDetails {state: "TX", zone: "001", zone_numeric: 001, name: "Dallam", wfo: "AMA" },
ForecastZone::TX002 => crate::ZoneDetails {state: "TX", zone: "002", zone_numeric: 002, name: "Sherman", wfo: "AMA" },
ForecastZone::TX003 => crate::ZoneDetails {state: "TX", zone: "003", zone_numeric: 003, name: "Hansford", wfo: "AMA" },
ForecastZone::TX004 => crate::ZoneDetails {state: "TX", zone: "004", zone_numeric: 004, name: "Ochiltree", wfo: "AMA" },
ForecastZone::TX005 => crate::ZoneDetails {state: "TX", zone: "005", zone_numeric: 005, name: "Lipscomb", wfo: "AMA" },
ForecastZone::TX006 => crate::ZoneDetails {state: "TX", zone: "006", zone_numeric: 006, name: "Hartley", wfo: "AMA" },
ForecastZone::TX007 => crate::ZoneDetails {state: "TX", zone: "007", zone_numeric: 007, name: "Moore", wfo: "AMA" },
ForecastZone::TX008 => crate::ZoneDetails {state: "TX", zone: "008", zone_numeric: 008, name: "Hutchinson", wfo: "AMA" },
ForecastZone::TX009 => crate::ZoneDetails {state: "TX", zone: "009", zone_numeric: 009, name: "Roberts", wfo: "AMA" },
ForecastZone::TX010 => crate::ZoneDetails {state: "TX", zone: "010", zone_numeric: 010, name: "Hemphill", wfo: "AMA" },
ForecastZone::TX011 => crate::ZoneDetails {state: "TX", zone: "011", zone_numeric: 011, name: "Oldham", wfo: "AMA" },
ForecastZone::TX012 => crate::ZoneDetails {state: "TX", zone: "012", zone_numeric: 012, name: "Potter", wfo: "AMA" },
ForecastZone::TX013 => crate::ZoneDetails {state: "TX", zone: "013", zone_numeric: 013, name: "Carson", wfo: "AMA" },
ForecastZone::TX014 => crate::ZoneDetails {state: "TX", zone: "014", zone_numeric: 014, name: "Gray", wfo: "AMA" },
ForecastZone::TX015 => crate::ZoneDetails {state: "TX", zone: "015", zone_numeric: 015, name: "Wheeler", wfo: "AMA" },
ForecastZone::TX016 => crate::ZoneDetails {state: "TX", zone: "016", zone_numeric: 016, name: "Deaf Smith", wfo: "AMA" },
ForecastZone::TX017 => crate::ZoneDetails {state: "TX", zone: "017", zone_numeric: 017, name: "Randall", wfo: "AMA" },
ForecastZone::TX018 => crate::ZoneDetails {state: "TX", zone: "018", zone_numeric: 018, name: "Armstrong", wfo: "AMA" },
ForecastZone::TX019 => crate::ZoneDetails {state: "TX", zone: "019", zone_numeric: 019, name: "Donley", wfo: "AMA" },
ForecastZone::TX020 => crate::ZoneDetails {state: "TX", zone: "020", zone_numeric: 020, name: "Collingsworth", wfo: "AMA" },
ForecastZone::TX021 => crate::ZoneDetails {state: "TX", zone: "021", zone_numeric: 021, name: "Parmer", wfo: "LUB" },
ForecastZone::TX022 => crate::ZoneDetails {state: "TX", zone: "022", zone_numeric: 022, name: "Castro", wfo: "LUB" },
ForecastZone::TX023 => crate::ZoneDetails {state: "TX", zone: "023", zone_numeric: 023, name: "Swisher", wfo: "LUB" },
ForecastZone::TX024 => crate::ZoneDetails {state: "TX", zone: "024", zone_numeric: 024, name: "Briscoe", wfo: "LUB" },
ForecastZone::TX025 => crate::ZoneDetails {state: "TX", zone: "025", zone_numeric: 025, name: "Hall", wfo: "LUB" },
ForecastZone::TX026 => crate::ZoneDetails {state: "TX", zone: "026", zone_numeric: 026, name: "Childress", wfo: "LUB" },
ForecastZone::TX027 => crate::ZoneDetails {state: "TX", zone: "027", zone_numeric: 027, name: "Bailey", wfo: "LUB" },
ForecastZone::TX028 => crate::ZoneDetails {state: "TX", zone: "028", zone_numeric: 028, name: "Lamb", wfo: "LUB" },
ForecastZone::TX029 => crate::ZoneDetails {state: "TX", zone: "029", zone_numeric: 029, name: "Hale", wfo: "LUB" },
ForecastZone::TX030 => crate::ZoneDetails {state: "TX", zone: "030", zone_numeric: 030, name: "Floyd", wfo: "LUB" },
ForecastZone::TX031 => crate::ZoneDetails {state: "TX", zone: "031", zone_numeric: 031, name: "Motley", wfo: "LUB" },
ForecastZone::TX032 => crate::ZoneDetails {state: "TX", zone: "032", zone_numeric: 032, name: "Cottle", wfo: "LUB" },
ForecastZone::TX033 => crate::ZoneDetails {state: "TX", zone: "033", zone_numeric: 033, name: "Cochran", wfo: "LUB" },
ForecastZone::TX034 => crate::ZoneDetails {state: "TX", zone: "034", zone_numeric: 034, name: "Hockley", wfo: "LUB" },
ForecastZone::TX035 => crate::ZoneDetails {state: "TX", zone: "035", zone_numeric: 035, name: "Lubbock", wfo: "LUB" },
ForecastZone::TX036 => crate::ZoneDetails {state: "TX", zone: "036", zone_numeric: 036, name: "Crosby", wfo: "LUB" },
ForecastZone::TX037 => crate::ZoneDetails {state: "TX", zone: "037", zone_numeric: 037, name: "Dickens", wfo: "LUB" },
ForecastZone::TX038 => crate::ZoneDetails {state: "TX", zone: "038", zone_numeric: 038, name: "King", wfo: "LUB" },
ForecastZone::TX039 => crate::ZoneDetails {state: "TX", zone: "039", zone_numeric: 039, name: "Yoakum", wfo: "LUB" },
ForecastZone::TX040 => crate::ZoneDetails {state: "TX", zone: "040", zone_numeric: 040, name: "Terry", wfo: "LUB" },
ForecastZone::TX041 => crate::ZoneDetails {state: "TX", zone: "041", zone_numeric: 041, name: "Lynn", wfo: "LUB" },
ForecastZone::TX042 => crate::ZoneDetails {state: "TX", zone: "042", zone_numeric: 042, name: "Garza", wfo: "LUB" },
ForecastZone::TX043 => crate::ZoneDetails {state: "TX", zone: "043", zone_numeric: 043, name: "Kent", wfo: "LUB" },
ForecastZone::TX044 => crate::ZoneDetails {state: "TX", zone: "044", zone_numeric: 044, name: "Stonewall", wfo: "LUB" },
ForecastZone::TX045 => crate::ZoneDetails {state: "TX", zone: "045", zone_numeric: 045, name: "Gaines", wfo: "MAF" },
ForecastZone::TX046 => crate::ZoneDetails {state: "TX", zone: "046", zone_numeric: 046, name: "Dawson", wfo: "MAF" },
ForecastZone::TX047 => crate::ZoneDetails {state: "TX", zone: "047", zone_numeric: 047, name: "Borden", wfo: "MAF" },
ForecastZone::TX048 => crate::ZoneDetails {state: "TX", zone: "048", zone_numeric: 048, name: "Scurry", wfo: "MAF" },
ForecastZone::TX049 => crate::ZoneDetails {state: "TX", zone: "049", zone_numeric: 049, name: "Fisher", wfo: "SJT" },
ForecastZone::TX050 => crate::ZoneDetails {state: "TX", zone: "050", zone_numeric: 050, name: "Andrews", wfo: "MAF" },
ForecastZone::TX051 => crate::ZoneDetails {state: "TX", zone: "051", zone_numeric: 051, name: "Martin", wfo: "MAF" },
ForecastZone::TX052 => crate::ZoneDetails {state: "TX", zone: "052", zone_numeric: 052, name: "Howard", wfo: "MAF" },
ForecastZone::TX053 => crate::ZoneDetails {state: "TX", zone: "053", zone_numeric: 053, name: "Mitchell", wfo: "MAF" },
ForecastZone::TX054 => crate::ZoneDetails {state: "TX", zone: "054", zone_numeric: 054, name: "Nolan", wfo: "SJT" },
ForecastZone::TX059 => crate::ZoneDetails {state: "TX", zone: "059", zone_numeric: 059, name: "Loving", wfo: "MAF" },
ForecastZone::TX060 => crate::ZoneDetails {state: "TX", zone: "060", zone_numeric: 060, name: "Winkler", wfo: "MAF" },
ForecastZone::TX061 => crate::ZoneDetails {state: "TX", zone: "061", zone_numeric: 061, name: "Ector", wfo: "MAF" },
ForecastZone::TX062 => crate::ZoneDetails {state: "TX", zone: "062", zone_numeric: 062, name: "Midland", wfo: "MAF" },
ForecastZone::TX063 => crate::ZoneDetails {state: "TX", zone: "063", zone_numeric: 063, name: "Glasscock", wfo: "MAF" },
ForecastZone::TX064 => crate::ZoneDetails {state: "TX", zone: "064", zone_numeric: 064, name: "Sterling", wfo: "SJT" },
ForecastZone::TX065 => crate::ZoneDetails {state: "TX", zone: "065", zone_numeric: 065, name: "Coke", wfo: "SJT" },
ForecastZone::TX066 => crate::ZoneDetails {state: "TX", zone: "066", zone_numeric: 066, name: "Runnels", wfo: "SJT" },
ForecastZone::TX067 => crate::ZoneDetails {state: "TX", zone: "067", zone_numeric: 067, name: "Ward", wfo: "MAF" },
ForecastZone::TX068 => crate::ZoneDetails {state: "TX", zone: "068", zone_numeric: 068, name: "Crane", wfo: "MAF" },
ForecastZone::TX069 => crate::ZoneDetails {state: "TX", zone: "069", zone_numeric: 069, name: "Upton", wfo: "MAF" },
ForecastZone::TX070 => crate::ZoneDetails {state: "TX", zone: "070", zone_numeric: 070, name: "Reagan", wfo: "MAF" },
ForecastZone::TX071 => crate::ZoneDetails {state: "TX", zone: "071", zone_numeric: 071, name: "Irion", wfo: "SJT" },
ForecastZone::TX072 => crate::ZoneDetails {state: "TX", zone: "072", zone_numeric: 072, name: "Tom Green", wfo: "SJT" },
ForecastZone::TX073 => crate::ZoneDetails {state: "TX", zone: "073", zone_numeric: 073, name: "Concho", wfo: "SJT" },
ForecastZone::TX075 => crate::ZoneDetails {state: "TX", zone: "075", zone_numeric: 075, name: "Pecos", wfo: "MAF" },
ForecastZone::TX076 => crate::ZoneDetails {state: "TX", zone: "076", zone_numeric: 076, name: "Crockett", wfo: "SJT" },
ForecastZone::TX077 => crate::ZoneDetails {state: "TX", zone: "077", zone_numeric: 077, name: "Schleicher", wfo: "SJT" },
ForecastZone::TX078 => crate::ZoneDetails {state: "TX", zone: "078", zone_numeric: 078, name: "Sutton", wfo: "SJT" },
ForecastZone::TX082 => crate::ZoneDetails {state: "TX", zone: "082", zone_numeric: 082, name: "Terrell", wfo: "MAF" },
ForecastZone::TX083 => crate::ZoneDetails {state: "TX", zone: "083", zone_numeric: 083, name: "Hardeman", wfo: "OUN" },
ForecastZone::TX084 => crate::ZoneDetails {state: "TX", zone: "084", zone_numeric: 084, name: "Foard", wfo: "OUN" },
ForecastZone::TX085 => crate::ZoneDetails {state: "TX", zone: "085", zone_numeric: 085, name: "Wilbarger", wfo: "OUN" },
ForecastZone::TX086 => crate::ZoneDetails {state: "TX", zone: "086", zone_numeric: 086, name: "Wichita", wfo: "OUN" },
ForecastZone::TX087 => crate::ZoneDetails {state: "TX", zone: "087", zone_numeric: 087, name: "Knox", wfo: "OUN" },
ForecastZone::TX088 => crate::ZoneDetails {state: "TX", zone: "088", zone_numeric: 088, name: "Baylor", wfo: "OUN" },
ForecastZone::TX089 => crate::ZoneDetails {state: "TX", zone: "089", zone_numeric: 089, name: "Archer", wfo: "OUN" },
ForecastZone::TX090 => crate::ZoneDetails {state: "TX", zone: "090", zone_numeric: 090, name: "Clay", wfo: "OUN" },
ForecastZone::TX091 => crate::ZoneDetails {state: "TX", zone: "091", zone_numeric: 091, name: "Montague", wfo: "FWD" },
ForecastZone::TX092 => crate::ZoneDetails {state: "TX", zone: "092", zone_numeric: 092, name: "Cooke", wfo: "FWD" },
ForecastZone::TX093 => crate::ZoneDetails {state: "TX", zone: "093", zone_numeric: 093, name: "Grayson", wfo: "FWD" },
ForecastZone::TX094 => crate::ZoneDetails {state: "TX", zone: "094", zone_numeric: 094, name: "Fannin", wfo: "FWD" },
ForecastZone::TX095 => crate::ZoneDetails {state: "TX", zone: "095", zone_numeric: 095, name: "Lamar", wfo: "FWD" },
ForecastZone::TX096 => crate::ZoneDetails {state: "TX", zone: "096", zone_numeric: 096, name: "Red River", wfo: "SHV" },
ForecastZone::TX097 => crate::ZoneDetails {state: "TX", zone: "097", zone_numeric: 097, name: "Bowie", wfo: "SHV" },
ForecastZone::TX098 => crate::ZoneDetails {state: "TX", zone: "098", zone_numeric: 098, name: "Haskell", wfo: "SJT" },
ForecastZone::TX099 => crate::ZoneDetails {state: "TX", zone: "099", zone_numeric: 099, name: "Throckmorton", wfo: "SJT" },
ForecastZone::TX100 => crate::ZoneDetails {state: "TX", zone: "100", zone_numeric: 100, name: "Young", wfo: "FWD" },
ForecastZone::TX101 => crate::ZoneDetails {state: "TX", zone: "101", zone_numeric: 101, name: "Jack", wfo: "FWD" },
ForecastZone::TX102 => crate::ZoneDetails {state: "TX", zone: "102", zone_numeric: 102, name: "Wise", wfo: "FWD" },
ForecastZone::TX103 => crate::ZoneDetails {state: "TX", zone: "103", zone_numeric: 103, name: "Denton", wfo: "FWD" },
ForecastZone::TX104 => crate::ZoneDetails {state: "TX", zone: "104", zone_numeric: 104, name: "Collin", wfo: "FWD" },
ForecastZone::TX105 => crate::ZoneDetails {state: "TX", zone: "105", zone_numeric: 105, name: "Hunt", wfo: "FWD" },
ForecastZone::TX106 => crate::ZoneDetails {state: "TX", zone: "106", zone_numeric: 106, name: "Delta", wfo: "FWD" },
ForecastZone::TX107 => crate::ZoneDetails {state: "TX", zone: "107", zone_numeric: 107, name: "Hopkins", wfo: "FWD" },
ForecastZone::TX108 => crate::ZoneDetails {state: "TX", zone: "108", zone_numeric: 108, name: "Franklin", wfo: "SHV" },
ForecastZone::TX109 => crate::ZoneDetails {state: "TX", zone: "109", zone_numeric: 109, name: "Titus", wfo: "SHV" },
ForecastZone::TX110 => crate::ZoneDetails {state: "TX", zone: "110", zone_numeric: 110, name: "Camp", wfo: "SHV" },
ForecastZone::TX111 => crate::ZoneDetails {state: "TX", zone: "111", zone_numeric: 111, name: "Morris", wfo: "SHV" },
ForecastZone::TX112 => crate::ZoneDetails {state: "TX", zone: "112", zone_numeric: 112, name: "Cass", wfo: "SHV" },
ForecastZone::TX113 => crate::ZoneDetails {state: "TX", zone: "113", zone_numeric: 113, name: "Jones", wfo: "SJT" },
ForecastZone::TX114 => crate::ZoneDetails {state: "TX", zone: "114", zone_numeric: 114, name: "Shackelford", wfo: "SJT" },
ForecastZone::TX115 => crate::ZoneDetails {state: "TX", zone: "115", zone_numeric: 115, name: "Stephens", wfo: "FWD" },
ForecastZone::TX116 => crate::ZoneDetails {state: "TX", zone: "116", zone_numeric: 116, name: "Palo Pinto", wfo: "FWD" },
ForecastZone::TX117 => crate::ZoneDetails {state: "TX", zone: "117", zone_numeric: 117, name: "Parker", wfo: "FWD" },
ForecastZone::TX118 => crate::ZoneDetails {state: "TX", zone: "118", zone_numeric: 118, name: "Tarrant", wfo: "FWD" },
ForecastZone::TX119 => crate::ZoneDetails {state: "TX", zone: "119", zone_numeric: 119, name: "Dallas", wfo: "FWD" },
ForecastZone::TX120 => crate::ZoneDetails {state: "TX", zone: "120", zone_numeric: 120, name: "Rockwall", wfo: "FWD" },
ForecastZone::TX121 => crate::ZoneDetails {state: "TX", zone: "121", zone_numeric: 121, name: "Kaufman", wfo: "FWD" },
ForecastZone::TX122 => crate::ZoneDetails {state: "TX", zone: "122", zone_numeric: 122, name: "Van Zandt", wfo: "FWD" },
ForecastZone::TX123 => crate::ZoneDetails {state: "TX", zone: "123", zone_numeric: 123, name: "Rains", wfo: "FWD" },
ForecastZone::TX124 => crate::ZoneDetails {state: "TX", zone: "124", zone_numeric: 124, name: "Wood", wfo: "SHV" },
ForecastZone::TX125 => crate::ZoneDetails {state: "TX", zone: "125", zone_numeric: 125, name: "Upshur", wfo: "SHV" },
ForecastZone::TX126 => crate::ZoneDetails {state: "TX", zone: "126", zone_numeric: 126, name: "Marion", wfo: "SHV" },
ForecastZone::TX127 => crate::ZoneDetails {state: "TX", zone: "127", zone_numeric: 127, name: "Taylor", wfo: "SJT" },
ForecastZone::TX128 => crate::ZoneDetails {state: "TX", zone: "128", zone_numeric: 128, name: "Callahan", wfo: "SJT" },
ForecastZone::TX129 => crate::ZoneDetails {state: "TX", zone: "129", zone_numeric: 129, name: "Eastland", wfo: "FWD" },
ForecastZone::TX130 => crate::ZoneDetails {state: "TX", zone: "130", zone_numeric: 130, name: "Erath", wfo: "FWD" },
ForecastZone::TX131 => crate::ZoneDetails {state: "TX", zone: "131", zone_numeric: 131, name: "Hood", wfo: "FWD" },
ForecastZone::TX132 => crate::ZoneDetails {state: "TX", zone: "132", zone_numeric: 132, name: "Somervell", wfo: "FWD" },
ForecastZone::TX133 => crate::ZoneDetails {state: "TX", zone: "133", zone_numeric: 133, name: "Johnson", wfo: "FWD" },
ForecastZone::TX134 => crate::ZoneDetails {state: "TX", zone: "134", zone_numeric: 134, name: "Ellis", wfo: "FWD" },
ForecastZone::TX135 => crate::ZoneDetails {state: "TX", zone: "135", zone_numeric: 135, name: "Henderson", wfo: "FWD" },
ForecastZone::TX136 => crate::ZoneDetails {state: "TX", zone: "136", zone_numeric: 136, name: "Smith", wfo: "SHV" },
ForecastZone::TX137 => crate::ZoneDetails {state: "TX", zone: "137", zone_numeric: 137, name: "Gregg", wfo: "SHV" },
ForecastZone::TX138 => crate::ZoneDetails {state: "TX", zone: "138", zone_numeric: 138, name: "Harrison", wfo: "SHV" },
ForecastZone::TX139 => crate::ZoneDetails {state: "TX", zone: "139", zone_numeric: 139, name: "Coleman", wfo: "SJT" },
ForecastZone::TX140 => crate::ZoneDetails {state: "TX", zone: "140", zone_numeric: 140, name: "Brown", wfo: "SJT" },
ForecastZone::TX141 => crate::ZoneDetails {state: "TX", zone: "141", zone_numeric: 141, name: "Comanche", wfo: "FWD" },
ForecastZone::TX142 => crate::ZoneDetails {state: "TX", zone: "142", zone_numeric: 142, name: "Mills", wfo: "FWD" },
ForecastZone::TX143 => crate::ZoneDetails {state: "TX", zone: "143", zone_numeric: 143, name: "Hamilton", wfo: "FWD" },
ForecastZone::TX144 => crate::ZoneDetails {state: "TX", zone: "144", zone_numeric: 144, name: "Bosque", wfo: "FWD" },
ForecastZone::TX145 => crate::ZoneDetails {state: "TX", zone: "145", zone_numeric: 145, name: "Hill", wfo: "FWD" },
ForecastZone::TX146 => crate::ZoneDetails {state: "TX", zone: "146", zone_numeric: 146, name: "Navarro", wfo: "FWD" },
ForecastZone::TX147 => crate::ZoneDetails {state: "TX", zone: "147", zone_numeric: 147, name: "Freestone", wfo: "FWD" },
ForecastZone::TX148 => crate::ZoneDetails {state: "TX", zone: "148", zone_numeric: 148, name: "Anderson", wfo: "FWD" },
ForecastZone::TX149 => crate::ZoneDetails {state: "TX", zone: "149", zone_numeric: 149, name: "Cherokee", wfo: "SHV" },
ForecastZone::TX150 => crate::ZoneDetails {state: "TX", zone: "150", zone_numeric: 150, name: "Rusk", wfo: "SHV" },
ForecastZone::TX151 => crate::ZoneDetails {state: "TX", zone: "151", zone_numeric: 151, name: "Panola", wfo: "SHV" },
ForecastZone::TX152 => crate::ZoneDetails {state: "TX", zone: "152", zone_numeric: 152, name: "Nacogdoches", wfo: "SHV" },
ForecastZone::TX153 => crate::ZoneDetails {state: "TX", zone: "153", zone_numeric: 153, name: "Shelby", wfo: "SHV" },
ForecastZone::TX154 => crate::ZoneDetails {state: "TX", zone: "154", zone_numeric: 154, name: "McCulloch", wfo: "SJT" },
ForecastZone::TX155 => crate::ZoneDetails {state: "TX", zone: "155", zone_numeric: 155, name: "San Saba", wfo: "SJT" },
ForecastZone::TX156 => crate::ZoneDetails {state: "TX", zone: "156", zone_numeric: 156, name: "Lampasas", wfo: "FWD" },
ForecastZone::TX157 => crate::ZoneDetails {state: "TX", zone: "157", zone_numeric: 157, name: "Coryell", wfo: "FWD" },
ForecastZone::TX158 => crate::ZoneDetails {state: "TX", zone: "158", zone_numeric: 158, name: "Bell", wfo: "FWD" },
ForecastZone::TX159 => crate::ZoneDetails {state: "TX", zone: "159", zone_numeric: 159, name: "McLennan", wfo: "FWD" },
ForecastZone::TX160 => crate::ZoneDetails {state: "TX", zone: "160", zone_numeric: 160, name: "Falls", wfo: "FWD" },
ForecastZone::TX161 => crate::ZoneDetails {state: "TX", zone: "161", zone_numeric: 161, name: "Limestone", wfo: "FWD" },
ForecastZone::TX162 => crate::ZoneDetails {state: "TX", zone: "162", zone_numeric: 162, name: "Leon", wfo: "FWD" },
ForecastZone::TX163 => crate::ZoneDetails {state: "TX", zone: "163", zone_numeric: 163, name: "Houston", wfo: "HGX" },
ForecastZone::TX164 => crate::ZoneDetails {state: "TX", zone: "164", zone_numeric: 164, name: "Trinity", wfo: "HGX" },
ForecastZone::TX165 => crate::ZoneDetails {state: "TX", zone: "165", zone_numeric: 165, name: "Angelina", wfo: "SHV" },
ForecastZone::TX166 => crate::ZoneDetails {state: "TX", zone: "166", zone_numeric: 166, name: "San Augustine", wfo: "SHV" },
ForecastZone::TX167 => crate::ZoneDetails {state: "TX", zone: "167", zone_numeric: 167, name: "Sabine", wfo: "SHV" },
ForecastZone::TX168 => crate::ZoneDetails {state: "TX", zone: "168", zone_numeric: 168, name: "Menard", wfo: "SJT" },
ForecastZone::TX169 => crate::ZoneDetails {state: "TX", zone: "169", zone_numeric: 169, name: "Kimble", wfo: "SJT" },
ForecastZone::TX170 => crate::ZoneDetails {state: "TX", zone: "170", zone_numeric: 170, name: "Mason", wfo: "SJT" },
ForecastZone::TX171 => crate::ZoneDetails {state: "TX", zone: "171", zone_numeric: 171, name: "Llano", wfo: "EWX" },
ForecastZone::TX172 => crate::ZoneDetails {state: "TX", zone: "172", zone_numeric: 172, name: "Burnet", wfo: "EWX" },
ForecastZone::TX173 => crate::ZoneDetails {state: "TX", zone: "173", zone_numeric: 173, name: "Williamson", wfo: "EWX" },
ForecastZone::TX174 => crate::ZoneDetails {state: "TX", zone: "174", zone_numeric: 174, name: "Milam", wfo: "FWD" },
ForecastZone::TX175 => crate::ZoneDetails {state: "TX", zone: "175", zone_numeric: 175, name: "Robertson", wfo: "FWD" },
ForecastZone::TX176 => crate::ZoneDetails {state: "TX", zone: "176", zone_numeric: 176, name: "Madison", wfo: "HGX" },
ForecastZone::TX177 => crate::ZoneDetails {state: "TX", zone: "177", zone_numeric: 177, name: "Walker", wfo: "HGX" },
ForecastZone::TX178 => crate::ZoneDetails {state: "TX", zone: "178", zone_numeric: 178, name: "San Jacinto", wfo: "HGX" },
ForecastZone::TX179 => crate::ZoneDetails {state: "TX", zone: "179", zone_numeric: 179, name: "Polk", wfo: "HGX" },
ForecastZone::TX180 => crate::ZoneDetails {state: "TX", zone: "180", zone_numeric: 180, name: "Tyler", wfo: "LCH" },
ForecastZone::TX183 => crate::ZoneDetails {state: "TX", zone: "183", zone_numeric: 183, name: "Val Verde", wfo: "EWX" },
ForecastZone::TX184 => crate::ZoneDetails {state: "TX", zone: "184", zone_numeric: 184, name: "Edwards", wfo: "EWX" },
ForecastZone::TX185 => crate::ZoneDetails {state: "TX", zone: "185", zone_numeric: 185, name: "Real", wfo: "EWX" },
ForecastZone::TX186 => crate::ZoneDetails {state: "TX", zone: "186", zone_numeric: 186, name: "Kerr", wfo: "EWX" },
ForecastZone::TX187 => crate::ZoneDetails {state: "TX", zone: "187", zone_numeric: 187, name: "Bandera", wfo: "EWX" },
ForecastZone::TX188 => crate::ZoneDetails {state: "TX", zone: "188", zone_numeric: 188, name: "Gillespie", wfo: "EWX" },
ForecastZone::TX189 => crate::ZoneDetails {state: "TX", zone: "189", zone_numeric: 189, name: "Kendall", wfo: "EWX" },
ForecastZone::TX190 => crate::ZoneDetails {state: "TX", zone: "190", zone_numeric: 190, name: "Blanco", wfo: "EWX" },
ForecastZone::TX191 => crate::ZoneDetails {state: "TX", zone: "191", zone_numeric: 191, name: "Hays", wfo: "EWX" },
ForecastZone::TX192 => crate::ZoneDetails {state: "TX", zone: "192", zone_numeric: 192, name: "Travis", wfo: "EWX" },
ForecastZone::TX193 => crate::ZoneDetails {state: "TX", zone: "193", zone_numeric: 193, name: "Bastrop", wfo: "EWX" },
ForecastZone::TX194 => crate::ZoneDetails {state: "TX", zone: "194", zone_numeric: 194, name: "Lee", wfo: "EWX" },
ForecastZone::TX195 => crate::ZoneDetails {state: "TX", zone: "195", zone_numeric: 195, name: "Burleson", wfo: "HGX" },
ForecastZone::TX196 => crate::ZoneDetails {state: "TX", zone: "196", zone_numeric: 196, name: "Brazos", wfo: "HGX" },
ForecastZone::TX197 => crate::ZoneDetails {state: "TX", zone: "197", zone_numeric: 197, name: "Washington", wfo: "HGX" },
ForecastZone::TX198 => crate::ZoneDetails {state: "TX", zone: "198", zone_numeric: 198, name: "Grimes", wfo: "HGX" },
ForecastZone::TX199 => crate::ZoneDetails {state: "TX", zone: "199", zone_numeric: 199, name: "Montgomery", wfo: "HGX" },
ForecastZone::TX200 => crate::ZoneDetails {state: "TX", zone: "200", zone_numeric: 200, name: "Northern Liberty", wfo: "HGX" },
ForecastZone::TX201 => crate::ZoneDetails {state: "TX", zone: "201", zone_numeric: 201, name: "Hardin", wfo: "LCH" },
ForecastZone::TX202 => crate::ZoneDetails {state: "TX", zone: "202", zone_numeric: 202, name: "Kinney", wfo: "EWX" },
ForecastZone::TX203 => crate::ZoneDetails {state: "TX", zone: "203", zone_numeric: 203, name: "Uvalde", wfo: "EWX" },
ForecastZone::TX204 => crate::ZoneDetails {state: "TX", zone: "204", zone_numeric: 204, name: "Medina", wfo: "EWX" },
ForecastZone::TX205 => crate::ZoneDetails {state: "TX", zone: "205", zone_numeric: 205, name: "Bexar", wfo: "EWX" },
ForecastZone::TX206 => crate::ZoneDetails {state: "TX", zone: "206", zone_numeric: 206, name: "Comal", wfo: "EWX" },
ForecastZone::TX207 => crate::ZoneDetails {state: "TX", zone: "207", zone_numeric: 207, name: "Guadalupe", wfo: "EWX" },
ForecastZone::TX208 => crate::ZoneDetails {state: "TX", zone: "208", zone_numeric: 208, name: "Caldwell", wfo: "EWX" },
ForecastZone::TX209 => crate::ZoneDetails {state: "TX", zone: "209", zone_numeric: 209, name: "Fayette", wfo: "EWX" },
ForecastZone::TX210 => crate::ZoneDetails {state: "TX", zone: "210", zone_numeric: 210, name: "Colorado", wfo: "HGX" },
ForecastZone::TX211 => crate::ZoneDetails {state: "TX", zone: "211", zone_numeric: 211, name: "Austin", wfo: "HGX" },
ForecastZone::TX212 => crate::ZoneDetails {state: "TX", zone: "212", zone_numeric: 212, name: "Waller", wfo: "HGX" },
ForecastZone::TX213 => crate::ZoneDetails {state: "TX", zone: "213", zone_numeric: 213, name: "Inland Harris", wfo: "HGX" },
ForecastZone::TX214 => crate::ZoneDetails {state: "TX", zone: "214", zone_numeric: 214, name: "Chambers", wfo: "HGX" },
ForecastZone::TX215 => crate::ZoneDetails {state: "TX", zone: "215", zone_numeric: 215, name: "Jefferson", wfo: "LCH" },
ForecastZone::TX216 => crate::ZoneDetails {state: "TX", zone: "216", zone_numeric: 216, name: "Orange", wfo: "LCH" },
ForecastZone::TX217 => crate::ZoneDetails {state: "TX", zone: "217", zone_numeric: 217, name: "Maverick", wfo: "EWX" },
ForecastZone::TX218 => crate::ZoneDetails {state: "TX", zone: "218", zone_numeric: 218, name: "Zavala", wfo: "EWX" },
ForecastZone::TX219 => crate::ZoneDetails {state: "TX", zone: "219", zone_numeric: 219, name: "Frio", wfo: "EWX" },
ForecastZone::TX220 => crate::ZoneDetails {state: "TX", zone: "220", zone_numeric: 220, name: "Atascosa", wfo: "EWX" },
ForecastZone::TX221 => crate::ZoneDetails {state: "TX", zone: "221", zone_numeric: 221, name: "Wilson", wfo: "EWX" },
ForecastZone::TX222 => crate::ZoneDetails {state: "TX", zone: "222", zone_numeric: 222, name: "Karnes", wfo: "EWX" },
ForecastZone::TX223 => crate::ZoneDetails {state: "TX", zone: "223", zone_numeric: 223, name: "Gonzales", wfo: "EWX" },
ForecastZone::TX224 => crate::ZoneDetails {state: "TX", zone: "224", zone_numeric: 224, name: "De Witt", wfo: "EWX" },
ForecastZone::TX225 => crate::ZoneDetails {state: "TX", zone: "225", zone_numeric: 225, name: "Lavaca", wfo: "EWX" },
ForecastZone::TX226 => crate::ZoneDetails {state: "TX", zone: "226", zone_numeric: 226, name: "Wharton", wfo: "HGX" },
ForecastZone::TX227 => crate::ZoneDetails {state: "TX", zone: "227", zone_numeric: 227, name: "Fort Bend", wfo: "HGX" },
ForecastZone::TX228 => crate::ZoneDetails {state: "TX", zone: "228", zone_numeric: 228, name: "Dimmit", wfo: "EWX" },
ForecastZone::TX229 => crate::ZoneDetails {state: "TX", zone: "229", zone_numeric: 229, name: "La Salle", wfo: "CRP" },
ForecastZone::TX230 => crate::ZoneDetails {state: "TX", zone: "230", zone_numeric: 230, name: "McMullen", wfo: "CRP" },
ForecastZone::TX231 => crate::ZoneDetails {state: "TX", zone: "231", zone_numeric: 231, name: "Live Oak", wfo: "CRP" },
ForecastZone::TX232 => crate::ZoneDetails {state: "TX", zone: "232", zone_numeric: 232, name: "Bee", wfo: "CRP" },
ForecastZone::TX233 => crate::ZoneDetails {state: "TX", zone: "233", zone_numeric: 233, name: "Goliad", wfo: "CRP" },
ForecastZone::TX234 => crate::ZoneDetails {state: "TX", zone: "234", zone_numeric: 234, name: "Victoria", wfo: "CRP" },
ForecastZone::TX235 => crate::ZoneDetails {state: "TX", zone: "235", zone_numeric: 235, name: "Inland Jackson", wfo: "HGX" },
ForecastZone::TX236 => crate::ZoneDetails {state: "TX", zone: "236", zone_numeric: 236, name: "Inland Matagorda", wfo: "HGX" },
ForecastZone::TX237 => crate::ZoneDetails {state: "TX", zone: "237", zone_numeric: 237, name: "Inland Brazoria", wfo: "HGX" },
ForecastZone::TX238 => crate::ZoneDetails {state: "TX", zone: "238", zone_numeric: 238, name: "Inland Galveston", wfo: "HGX" },
ForecastZone::TX239 => crate::ZoneDetails {state: "TX", zone: "239", zone_numeric: 239, name: "Webb", wfo: "CRP" },
ForecastZone::TX240 => crate::ZoneDetails {state: "TX", zone: "240", zone_numeric: 240, name: "Duval", wfo: "CRP" },
ForecastZone::TX241 => crate::ZoneDetails {state: "TX", zone: "241", zone_numeric: 241, name: "Jim Wells", wfo: "CRP" },
ForecastZone::TX242 => crate::ZoneDetails {state: "TX", zone: "242", zone_numeric: 242, name: "Inland Kleberg", wfo: "CRP" },
ForecastZone::TX243 => crate::ZoneDetails {state: "TX", zone: "243", zone_numeric: 243, name: "Inland Nueces", wfo: "CRP" },
ForecastZone::TX244 => crate::ZoneDetails {state: "TX", zone: "244", zone_numeric: 244, name: "Inland San Patricio", wfo: "CRP" },
ForecastZone::TX245 => crate::ZoneDetails {state: "TX", zone: "245", zone_numeric: 245, name: "Coastal Aransas", wfo: "CRP" },
ForecastZone::TX246 => crate::ZoneDetails {state: "TX", zone: "246", zone_numeric: 246, name: "Inland Refugio", wfo: "CRP" },
ForecastZone::TX247 => crate::ZoneDetails {state: "TX", zone: "247", zone_numeric: 247, name: "Inland Calhoun", wfo: "CRP" },
ForecastZone::TX248 => crate::ZoneDetails {state: "TX", zone: "248", zone_numeric: 248, name: "Zapata", wfo: "BRO" },
ForecastZone::TX249 => crate::ZoneDetails {state: "TX", zone: "249", zone_numeric: 249, name: "Jim Hogg", wfo: "BRO" },
ForecastZone::TX250 => crate::ZoneDetails {state: "TX", zone: "250", zone_numeric: 250, name: "Brooks", wfo: "BRO" },
ForecastZone::TX251 => crate::ZoneDetails {state: "TX", zone: "251", zone_numeric: 251, name: "Inland Kenedy", wfo: "BRO" },
ForecastZone::TX252 => crate::ZoneDetails {state: "TX", zone: "252", zone_numeric: 252, name: "Starr", wfo: "BRO" },
ForecastZone::TX253 => crate::ZoneDetails {state: "TX", zone: "253", zone_numeric: 253, name: "Southern Hidalgo", wfo: "BRO" },
ForecastZone::TX254 => crate::ZoneDetails {state: "TX", zone: "254", zone_numeric: 254, name: "Inland Willacy", wfo: "BRO" },
ForecastZone::TX255 => crate::ZoneDetails {state: "TX", zone: "255", zone_numeric: 255, name: "Inland Cameron", wfo: "BRO" },
ForecastZone::TX259 => crate::ZoneDetails {state: "TX", zone: "259", zone_numeric: 259, name: "Northern Jasper", wfo: "LCH" },
ForecastZone::TX260 => crate::ZoneDetails {state: "TX", zone: "260", zone_numeric: 260, name: "Northern Newton", wfo: "LCH" },
ForecastZone::TX261 => crate::ZoneDetails {state: "TX", zone: "261", zone_numeric: 261, name: "Southern Jasper", wfo: "LCH" },
ForecastZone::TX262 => crate::ZoneDetails {state: "TX", zone: "262", zone_numeric: 262, name: "Southern Newton", wfo: "LCH" },
ForecastZone::TX270 => crate::ZoneDetails {state: "TX", zone: "270", zone_numeric: 270, name: "Guadalupe Mountains Above 7000 Feet", wfo: "MAF" },
ForecastZone::TX271 => crate::ZoneDetails {state: "TX", zone: "271", zone_numeric: 271, name: "Guadalupe and Delaware Mountains", wfo: "MAF" },
ForecastZone::TX272 => crate::ZoneDetails {state: "TX", zone: "272", zone_numeric: 272, name: "Van Horn and Highway 54 Corridor", wfo: "MAF" },
ForecastZone::TX273 => crate::ZoneDetails {state: "TX", zone: "273", zone_numeric: 273, name: "Eastern Culberson County", wfo: "MAF" },
ForecastZone::TX274 => crate::ZoneDetails {state: "TX", zone: "274", zone_numeric: 274, name: "Reeves County Plains", wfo: "MAF" },
ForecastZone::TX275 => crate::ZoneDetails {state: "TX", zone: "275", zone_numeric: 275, name: "Chinati Mountains", wfo: "MAF" },
ForecastZone::TX276 => crate::ZoneDetails {state: "TX", zone: "276", zone_numeric: 276, name: "Marfa Plateau", wfo: "MAF" },
ForecastZone::TX277 => crate::ZoneDetails {state: "TX", zone: "277", zone_numeric: 277, name: "Davis Mountains", wfo: "MAF" },
ForecastZone::TX278 => crate::ZoneDetails {state: "TX", zone: "278", zone_numeric: 278, name: "Davis Mountains Foothills", wfo: "MAF" },
ForecastZone::TX279 => crate::ZoneDetails {state: "TX", zone: "279", zone_numeric: 279, name: "Central Brewster County", wfo: "MAF" },
ForecastZone::TX280 => crate::ZoneDetails {state: "TX", zone: "280", zone_numeric: 280, name: "Chisos Basin", wfo: "MAF" },
ForecastZone::TX281 => crate::ZoneDetails {state: "TX", zone: "281", zone_numeric: 281, name: "Presidio Valley", wfo: "MAF" },
ForecastZone::TX282 => crate::ZoneDetails {state: "TX", zone: "282", zone_numeric: 282, name: "Lower Brewster County", wfo: "MAF" },
ForecastZone::TX300 => crate::ZoneDetails {state: "TX", zone: "300", zone_numeric: 300, name: "Southern Liberty", wfo: "HGX" },
ForecastZone::TX313 => crate::ZoneDetails {state: "TX", zone: "313", zone_numeric: 313, name: "Coastal Harris", wfo: "HGX" },
ForecastZone::TX317 => crate::ZoneDetails {state: "TX", zone: "317", zone_numeric: 317, name: "Palo Duro Canyon", wfo: "AMA" },
ForecastZone::TX335 => crate::ZoneDetails {state: "TX", zone: "335", zone_numeric: 335, name: "Coastal Jackson", wfo: "HGX" },
ForecastZone::TX336 => crate::ZoneDetails {state: "TX", zone: "336", zone_numeric: 336, name: "Coastal Matagorda", wfo: "HGX" },
ForecastZone::TX337 => crate::ZoneDetails {state: "TX", zone: "337", zone_numeric: 337, name: "Coastal Brazoria", wfo: "HGX" },
ForecastZone::TX338 => crate::ZoneDetails {state: "TX", zone: "338", zone_numeric: 338, name: "Coastal Galveston", wfo: "HGX" },
ForecastZone::TX342 => crate::ZoneDetails {state: "TX", zone: "342", zone_numeric: 342, name: "Coastal Kleberg", wfo: "CRP" },
ForecastZone::TX343 => crate::ZoneDetails {state: "TX", zone: "343", zone_numeric: 343, name: "Coastal Nueces", wfo: "CRP" },
ForecastZone::TX344 => crate::ZoneDetails {state: "TX", zone: "344", zone_numeric: 344, name: "Coastal San Patricio", wfo: "CRP" },
ForecastZone::TX345 => crate::ZoneDetails {state: "TX", zone: "345", zone_numeric: 345, name: "Aransas Islands", wfo: "CRP" },
ForecastZone::TX346 => crate::ZoneDetails {state: "TX", zone: "346", zone_numeric: 346, name: "Coastal Refugio", wfo: "CRP" },
ForecastZone::TX347 => crate::ZoneDetails {state: "TX", zone: "347", zone_numeric: 347, name: "Coastal Calhoun", wfo: "CRP" },
ForecastZone::TX351 => crate::ZoneDetails {state: "TX", zone: "351", zone_numeric: 351, name: "Coastal Kenedy", wfo: "BRO" },
ForecastZone::TX353 => crate::ZoneDetails {state: "TX", zone: "353", zone_numeric: 353, name: "Northern Hidalgo", wfo: "BRO" },
ForecastZone::TX354 => crate::ZoneDetails {state: "TX", zone: "354", zone_numeric: 354, name: "Coastal Willacy", wfo: "BRO" },
ForecastZone::TX355 => crate::ZoneDetails {state: "TX", zone: "355", zone_numeric: 355, name: "Coastal Cameron", wfo: "BRO" },
ForecastZone::TX418 => crate::ZoneDetails {state: "TX", zone: "418", zone_numeric: 418, name: "Western El Paso County", wfo: "EPZ" },
ForecastZone::TX419 => crate::ZoneDetails {state: "TX", zone: "419", zone_numeric: 419, name: "Eastern/Central El Paso County", wfo: "EPZ" },
ForecastZone::TX420 => crate::ZoneDetails {state: "TX", zone: "420", zone_numeric: 420, name: "Northern Hudspeth Highlands/Hueco Mountains", wfo: "EPZ" },
ForecastZone::TX421 => crate::ZoneDetails {state: "TX", zone: "421", zone_numeric: 421, name: "Salt Basin", wfo: "EPZ" },
ForecastZone::TX422 => crate::ZoneDetails {state: "TX", zone: "422", zone_numeric: 422, name: "Southern Hudspeth Highlands", wfo: "EPZ" },
ForecastZone::TX423 => crate::ZoneDetails {state: "TX", zone: "423", zone_numeric: 423, name: "Rio Grande Valley of Eastern El Paso/Western Hudspeth Counties", wfo: "EPZ" },
ForecastZone::TX424 => crate::ZoneDetails {state: "TX", zone: "424", zone_numeric: 424, name: "Rio Grande Valley of Eastern Hudspeth County", wfo: "EPZ" },
ForecastZone::TX436 => crate::ZoneDetails {state: "TX", zone: "436", zone_numeric: 436, name: "Matagorda Islands", wfo: "HGX" },
ForecastZone::TX437 => crate::ZoneDetails {state: "TX", zone: "437", zone_numeric: 437, name: "Brazoria Islands", wfo: "HGX" },
ForecastZone::TX438 => crate::ZoneDetails {state: "TX", zone: "438", zone_numeric: 438, name: "Galveston Island", wfo: "HGX" },
ForecastZone::TX439 => crate::ZoneDetails {state: "TX", zone: "439", zone_numeric: 439, name: "Bolivar Peninsula", wfo: "HGX" },
ForecastZone::TX442 => crate::ZoneDetails {state: "TX", zone: "442", zone_numeric: 442, name: "Kleberg Islands", wfo: "CRP" },
ForecastZone::TX443 => crate::ZoneDetails {state: "TX", zone: "443", zone_numeric: 443, name: "Nueces Islands", wfo: "CRP" },
ForecastZone::TX447 => crate::ZoneDetails {state: "TX", zone: "447", zone_numeric: 447, name: "Calhoun Islands", wfo: "CRP" },
ForecastZone::TX451 => crate::ZoneDetails {state: "TX", zone: "451", zone_numeric: 451, name: "Kenedy Island", wfo: "BRO" },
ForecastZone::TX454 => crate::ZoneDetails {state: "TX", zone: "454", zone_numeric: 454, name: "Willacy Island", wfo: "BRO" },
ForecastZone::TX455 => crate::ZoneDetails {state: "TX", zone: "455", zone_numeric: 455, name: "Cameron Island", wfo: "BRO" },
ForecastZone::UT022 => crate::ZoneDetails {state: "UT", zone: "022", zone_numeric: 022, name: "Southeast Utah", wfo: "GJT" },
ForecastZone::UT023 => crate::ZoneDetails {state: "UT", zone: "023", zone_numeric: 023, name: "Eastern Uinta Mountains", wfo: "GJT" },
ForecastZone::UT024 => crate::ZoneDetails {state: "UT", zone: "024", zone_numeric: 024, name: "Eastern Uinta Basin", wfo: "GJT" },
ForecastZone::UT025 => crate::ZoneDetails {state: "UT", zone: "025", zone_numeric: 025, name: "Tavaputs Plateau", wfo: "GJT" },
ForecastZone::UT027 => crate::ZoneDetails {state: "UT", zone: "027", zone_numeric: 027, name: "Arches/Grand Flat", wfo: "GJT" },
ForecastZone::UT028 => crate::ZoneDetails {state: "UT", zone: "028", zone_numeric: 028, name: "La Sal and Abajo Mountains", wfo: "GJT" },
ForecastZone::UT029 => crate::ZoneDetails {state: "UT", zone: "029", zone_numeric: 029, name: "Canyonlands/Natural Bridges", wfo: "GJT" },
ForecastZone::UT101 => crate::ZoneDetails {state: "UT", zone: "101", zone_numeric: 101, name: "Great Salt Lake Desert and Mountains", wfo: "SLC" },
ForecastZone::UT102 => crate::ZoneDetails {state: "UT", zone: "102", zone_numeric: 102, name: "Tooele and Rush Valleys", wfo: "SLC" },
ForecastZone::UT103 => crate::ZoneDetails {state: "UT", zone: "103", zone_numeric: 103, name: "Eastern Box Elder County", wfo: "SLC" },
ForecastZone::UT104 => crate::ZoneDetails {state: "UT", zone: "104", zone_numeric: 104, name: "Northern Wasatch Front", wfo: "SLC" },
ForecastZone::UT105 => crate::ZoneDetails {state: "UT", zone: "105", zone_numeric: 105, name: "Salt Lake Valley", wfo: "SLC" },
ForecastZone::UT106 => crate::ZoneDetails {state: "UT", zone: "106", zone_numeric: 106, name: "Utah Valley", wfo: "SLC" },
ForecastZone::UT107 => crate::ZoneDetails {state: "UT", zone: "107", zone_numeric: 107, name: "Cache Valley/Utah Portion", wfo: "SLC" },
ForecastZone::UT108 => crate::ZoneDetails {state: "UT", zone: "108", zone_numeric: 108, name: "Wasatch Back", wfo: "SLC" },
ForecastZone::UT109 => crate::ZoneDetails {state: "UT", zone: "109", zone_numeric: 109, name: "Bear Lake and Bear River Valley", wfo: "SLC" },
ForecastZone::UT110 => crate::ZoneDetails {state: "UT", zone: "110", zone_numeric: 110, name: "Wasatch Mountains I-80 North", wfo: "SLC" },
ForecastZone::UT111 => crate::ZoneDetails {state: "UT", zone: "111", zone_numeric: 111, name: "Wasatch Mountains South of I-80", wfo: "SLC" },
ForecastZone::UT112 => crate::ZoneDetails {state: "UT", zone: "112", zone_numeric: 112, name: "Western Uinta Mountains", wfo: "SLC" },
ForecastZone::UT113 => crate::ZoneDetails {state: "UT", zone: "113", zone_numeric: 113, name: "Wasatch Plateau/Book Cliffs", wfo: "SLC" },
ForecastZone::UT114 => crate::ZoneDetails {state: "UT", zone: "114", zone_numeric: 114, name: "Western Uinta Basin", wfo: "SLC" },
ForecastZone::UT115 => crate::ZoneDetails {state: "UT", zone: "115", zone_numeric: 115, name: "Western Millard and Juab Counties", wfo: "SLC" },
ForecastZone::UT116 => crate::ZoneDetails {state: "UT", zone: "116", zone_numeric: 116, name: "Eastern Juab/Millard Counties", wfo: "SLC" },
ForecastZone::UT117 => crate::ZoneDetails {state: "UT", zone: "117", zone_numeric: 117, name: "Central Mountains", wfo: "SLC" },
ForecastZone::UT118 => crate::ZoneDetails {state: "UT", zone: "118", zone_numeric: 118, name: "Sanpete Valley", wfo: "SLC" },
ForecastZone::UT119 => crate::ZoneDetails {state: "UT", zone: "119", zone_numeric: 119, name: "Sevier Valley", wfo: "SLC" },
ForecastZone::UT120 => crate::ZoneDetails {state: "UT", zone: "120", zone_numeric: 120, name: "Castle Country", wfo: "SLC" },
ForecastZone::UT121 => crate::ZoneDetails {state: "UT", zone: "121", zone_numeric: 121, name: "San Rafael Swell", wfo: "SLC" },
ForecastZone::UT122 => crate::ZoneDetails {state: "UT", zone: "122", zone_numeric: 122, name: "Southwest Utah", wfo: "SLC" },
ForecastZone::UT123 => crate::ZoneDetails {state: "UT", zone: "123", zone_numeric: 123, name: "Lower Washington County", wfo: "SLC" },
ForecastZone::UT124 => crate::ZoneDetails {state: "UT", zone: "124", zone_numeric: 124, name: "Zion National Park", wfo: "SLC" },
ForecastZone::UT125 => crate::ZoneDetails {state: "UT", zone: "125", zone_numeric: 125, name: "Southern Mountains", wfo: "SLC" },
ForecastZone::UT126 => crate::ZoneDetails {state: "UT", zone: "126", zone_numeric: 126, name: "Upper Sevier River Valleys", wfo: "SLC" },
ForecastZone::UT127 => crate::ZoneDetails {state: "UT", zone: "127", zone_numeric: 127, name: "Bryce Canyon Country", wfo: "SLC" },
ForecastZone::UT128 => crate::ZoneDetails {state: "UT", zone: "128", zone_numeric: 128, name: "South Central Utah", wfo: "SLC" },
ForecastZone::UT129 => crate::ZoneDetails {state: "UT", zone: "129", zone_numeric: 129, name: "Capitol Reef National Park and Vicinity", wfo: "SLC" },
ForecastZone::UT130 => crate::ZoneDetails {state: "UT", zone: "130", zone_numeric: 130, name: "Western Canyonlands", wfo: "SLC" },
ForecastZone::UT131 => crate::ZoneDetails {state: "UT", zone: "131", zone_numeric: 131, name: "Glen Canyon Recreation Area/Lake Powell", wfo: "SLC" },
ForecastZone::VA001 => crate::ZoneDetails {state: "VA", zone: "001", zone_numeric: 001, name: "Lee", wfo: "MRX" },
ForecastZone::VA002 => crate::ZoneDetails {state: "VA", zone: "002", zone_numeric: 002, name: "Wise", wfo: "MRX" },
ForecastZone::VA003 => crate::ZoneDetails {state: "VA", zone: "003", zone_numeric: 003, name: "Dickenson", wfo: "RLX" },
ForecastZone::VA004 => crate::ZoneDetails {state: "VA", zone: "004", zone_numeric: 004, name: "Buchanan", wfo: "RLX" },
ForecastZone::VA005 => crate::ZoneDetails {state: "VA", zone: "005", zone_numeric: 005, name: "Scott", wfo: "MRX" },
ForecastZone::VA006 => crate::ZoneDetails {state: "VA", zone: "006", zone_numeric: 006, name: "Russell", wfo: "MRX" },
ForecastZone::VA007 => crate::ZoneDetails {state: "VA", zone: "007", zone_numeric: 007, name: "Tazewell", wfo: "RNK" },
ForecastZone::VA008 => crate::ZoneDetails {state: "VA", zone: "008", zone_numeric: 008, name: "Washington", wfo: "MRX" },
ForecastZone::VA009 => crate::ZoneDetails {state: "VA", zone: "009", zone_numeric: 009, name: "Smyth", wfo: "RNK" },
ForecastZone::VA010 => crate::ZoneDetails {state: "VA", zone: "010", zone_numeric: 010, name: "Bland", wfo: "RNK" },
ForecastZone::VA011 => crate::ZoneDetails {state: "VA", zone: "011", zone_numeric: 011, name: "Giles", wfo: "RNK" },
ForecastZone::VA012 => crate::ZoneDetails {state: "VA", zone: "012", zone_numeric: 012, name: "Wythe", wfo: "RNK" },
ForecastZone::VA013 => crate::ZoneDetails {state: "VA", zone: "013", zone_numeric: 013, name: "Pulaski", wfo: "RNK" },
ForecastZone::VA014 => crate::ZoneDetails {state: "VA", zone: "014", zone_numeric: 014, name: "Montgomery", wfo: "RNK" },
ForecastZone::VA015 => crate::ZoneDetails {state: "VA", zone: "015", zone_numeric: 015, name: "Grayson", wfo: "RNK" },
ForecastZone::VA016 => crate::ZoneDetails {state: "VA", zone: "016", zone_numeric: 016, name: "Carroll", wfo: "RNK" },
ForecastZone::VA017 => crate::ZoneDetails {state: "VA", zone: "017", zone_numeric: 017, name: "Floyd", wfo: "RNK" },
ForecastZone::VA018 => crate::ZoneDetails {state: "VA", zone: "018", zone_numeric: 018, name: "Craig", wfo: "RNK" },
ForecastZone::VA019 => crate::ZoneDetails {state: "VA", zone: "019", zone_numeric: 019, name: "Alleghany", wfo: "RNK" },
ForecastZone::VA020 => crate::ZoneDetails {state: "VA", zone: "020", zone_numeric: 020, name: "Bath", wfo: "RNK" },
ForecastZone::VA022 => crate::ZoneDetails {state: "VA", zone: "022", zone_numeric: 022, name: "Roanoke", wfo: "RNK" },
ForecastZone::VA023 => crate::ZoneDetails {state: "VA", zone: "023", zone_numeric: 023, name: "Botetourt", wfo: "RNK" },
ForecastZone::VA024 => crate::ZoneDetails {state: "VA", zone: "024", zone_numeric: 024, name: "Rockbridge", wfo: "RNK" },
ForecastZone::VA025 => crate::ZoneDetails {state: "VA", zone: "025", zone_numeric: 025, name: "Augusta", wfo: "LWX" },
ForecastZone::VA026 => crate::ZoneDetails {state: "VA", zone: "026", zone_numeric: 026, name: "Rockingham", wfo: "LWX" },
ForecastZone::VA027 => crate::ZoneDetails {state: "VA", zone: "027", zone_numeric: 027, name: "Shenandoah", wfo: "LWX" },
ForecastZone::VA028 => crate::ZoneDetails {state: "VA", zone: "028", zone_numeric: 028, name: "Frederick", wfo: "LWX" },
ForecastZone::VA029 => crate::ZoneDetails {state: "VA", zone: "029", zone_numeric: 029, name: "Page", wfo: "LWX" },
ForecastZone::VA030 => crate::ZoneDetails {state: "VA", zone: "030", zone_numeric: 030, name: "Warren", wfo: "LWX" },
ForecastZone::VA031 => crate::ZoneDetails {state: "VA", zone: "031", zone_numeric: 031, name: "Clarke", wfo: "LWX" },
ForecastZone::VA032 => crate::ZoneDetails {state: "VA", zone: "032", zone_numeric: 032, name: "Patrick", wfo: "RNK" },
ForecastZone::VA033 => crate::ZoneDetails {state: "VA", zone: "033", zone_numeric: 033, name: "Franklin", wfo: "RNK" },
ForecastZone::VA034 => crate::ZoneDetails {state: "VA", zone: "034", zone_numeric: 034, name: "Bedford", wfo: "RNK" },
ForecastZone::VA035 => crate::ZoneDetails {state: "VA", zone: "035", zone_numeric: 035, name: "Amherst", wfo: "RNK" },
ForecastZone::VA036 => crate::ZoneDetails {state: "VA", zone: "036", zone_numeric: 036, name: "Nelson", wfo: "LWX" },
ForecastZone::VA037 => crate::ZoneDetails {state: "VA", zone: "037", zone_numeric: 037, name: "Albemarle", wfo: "LWX" },
ForecastZone::VA038 => crate::ZoneDetails {state: "VA", zone: "038", zone_numeric: 038, name: "Greene", wfo: "LWX" },
ForecastZone::VA039 => crate::ZoneDetails {state: "VA", zone: "039", zone_numeric: 039, name: "Madison", wfo: "LWX" },
ForecastZone::VA040 => crate::ZoneDetails {state: "VA", zone: "040", zone_numeric: 040, name: "Rappahannock", wfo: "LWX" },
ForecastZone::VA043 => crate::ZoneDetails {state: "VA", zone: "043", zone_numeric: 043, name: "Henry", wfo: "RNK" },
ForecastZone::VA044 => crate::ZoneDetails {state: "VA", zone: "044", zone_numeric: 044, name: "Pittsylvania", wfo: "RNK" },
ForecastZone::VA045 => crate::ZoneDetails {state: "VA", zone: "045", zone_numeric: 045, name: "Campbell", wfo: "RNK" },
ForecastZone::VA046 => crate::ZoneDetails {state: "VA", zone: "046", zone_numeric: 046, name: "Appomattox", wfo: "RNK" },
ForecastZone::VA047 => crate::ZoneDetails {state: "VA", zone: "047", zone_numeric: 047, name: "Buckingham", wfo: "RNK" },
ForecastZone::VA048 => crate::ZoneDetails {state: "VA", zone: "048", zone_numeric: 048, name: "Fluvanna", wfo: "AKQ" },
ForecastZone::VA050 => crate::ZoneDetails {state: "VA", zone: "050", zone_numeric: 050, name: "Orange", wfo: "LWX" },
ForecastZone::VA051 => crate::ZoneDetails {state: "VA", zone: "051", zone_numeric: 051, name: "Culpeper", wfo: "LWX" },
ForecastZone::VA052 => crate::ZoneDetails {state: "VA", zone: "052", zone_numeric: 052, name: "Prince William/Manassas/Manassas Park", wfo: "LWX" },
ForecastZone::VA053 => crate::ZoneDetails {state: "VA", zone: "053", zone_numeric: 053, name: "Fairfax", wfo: "LWX" },
ForecastZone::VA054 => crate::ZoneDetails {state: "VA", zone: "054", zone_numeric: 054, name: "Arlington/Falls Church/Alexandria", wfo: "LWX" },
ForecastZone::VA055 => crate::ZoneDetails {state: "VA", zone: "055", zone_numeric: 055, name: "Stafford", wfo: "LWX" },
ForecastZone::VA056 => crate::ZoneDetails {state: "VA", zone: "056", zone_numeric: 056, name: "Spotsylvania", wfo: "LWX" },
ForecastZone::VA057 => crate::ZoneDetails {state: "VA", zone: "057", zone_numeric: 057, name: "King George", wfo: "LWX" },
ForecastZone::VA058 => crate::ZoneDetails {state: "VA", zone: "058", zone_numeric: 058, name: "Halifax", wfo: "RNK" },
ForecastZone::VA059 => crate::ZoneDetails {state: "VA", zone: "059", zone_numeric: 059, name: "Charlotte", wfo: "RNK" },
ForecastZone::VA060 => crate::ZoneDetails {state: "VA", zone: "060", zone_numeric: 060, name: "Prince Edward", wfo: "AKQ" },
ForecastZone::VA061 => crate::ZoneDetails {state: "VA", zone: "061", zone_numeric: 061, name: "Cumberland", wfo: "AKQ" },
ForecastZone::VA062 => crate::ZoneDetails {state: "VA", zone: "062", zone_numeric: 062, name: "Goochland", wfo: "AKQ" },
ForecastZone::VA064 => crate::ZoneDetails {state: "VA", zone: "064", zone_numeric: 064, name: "Caroline", wfo: "AKQ" },
ForecastZone::VA065 => crate::ZoneDetails {state: "VA", zone: "065", zone_numeric: 065, name: "Mecklenburg", wfo: "AKQ" },
ForecastZone::VA066 => crate::ZoneDetails {state: "VA", zone: "066", zone_numeric: 066, name: "Lunenburg", wfo: "AKQ" },
ForecastZone::VA067 => crate::ZoneDetails {state: "VA", zone: "067", zone_numeric: 067, name: "Nottoway", wfo: "AKQ" },
ForecastZone::VA068 => crate::ZoneDetails {state: "VA", zone: "068", zone_numeric: 068, name: "Amelia", wfo: "AKQ" },
ForecastZone::VA069 => crate::ZoneDetails {state: "VA", zone: "069", zone_numeric: 069, name: "Powhatan", wfo: "AKQ" },
ForecastZone::VA075 => crate::ZoneDetails {state: "VA", zone: "075", zone_numeric: 075, name: "Westmoreland", wfo: "AKQ" },
ForecastZone::VA076 => crate::ZoneDetails {state: "VA", zone: "076", zone_numeric: 076, name: "Richmond", wfo: "AKQ" },
ForecastZone::VA077 => crate::ZoneDetails {state: "VA", zone: "077", zone_numeric: 077, name: "Northumberland", wfo: "AKQ" },
ForecastZone::VA078 => crate::ZoneDetails {state: "VA", zone: "078", zone_numeric: 078, name: "Lancaster", wfo: "AKQ" },
ForecastZone::VA079 => crate::ZoneDetails {state: "VA", zone: "079", zone_numeric: 079, name: "Brunswick", wfo: "AKQ" },
ForecastZone::VA080 => crate::ZoneDetails {state: "VA", zone: "080", zone_numeric: 080, name: "Dinwiddie", wfo: "AKQ" },
ForecastZone::VA081 => crate::ZoneDetails {state: "VA", zone: "081", zone_numeric: 081, name: "Prince George", wfo: "AKQ" },
ForecastZone::VA082 => crate::ZoneDetails {state: "VA", zone: "082", zone_numeric: 082, name: "Charles City", wfo: "AKQ" },
ForecastZone::VA083 => crate::ZoneDetails {state: "VA", zone: "083", zone_numeric: 083, name: "New Kent", wfo: "AKQ" },
ForecastZone::VA084 => crate::ZoneDetails {state: "VA", zone: "084", zone_numeric: 084, name: "Gloucester", wfo: "AKQ" },
ForecastZone::VA085 => crate::ZoneDetails {state: "VA", zone: "085", zone_numeric: 085, name: "Middlesex", wfo: "AKQ" },
ForecastZone::VA086 => crate::ZoneDetails {state: "VA", zone: "086", zone_numeric: 086, name: "Mathews", wfo: "AKQ" },
ForecastZone::VA087 => crate::ZoneDetails {state: "VA", zone: "087", zone_numeric: 087, name: "Greensville", wfo: "AKQ" },
ForecastZone::VA088 => crate::ZoneDetails {state: "VA", zone: "088", zone_numeric: 088, name: "Sussex", wfo: "AKQ" },
ForecastZone::VA089 => crate::ZoneDetails {state: "VA", zone: "089", zone_numeric: 089, name: "Surry", wfo: "AKQ" },
ForecastZone::VA090 => crate::ZoneDetails {state: "VA", zone: "090", zone_numeric: 090, name: "James City", wfo: "AKQ" },
ForecastZone::VA092 => crate::ZoneDetails {state: "VA", zone: "092", zone_numeric: 092, name: "Southampton", wfo: "AKQ" },
ForecastZone::VA093 => crate::ZoneDetails {state: "VA", zone: "093", zone_numeric: 093, name: "Isle of Wight", wfo: "AKQ" },
ForecastZone::VA095 => crate::ZoneDetails {state: "VA", zone: "095", zone_numeric: 095, name: "Norfolk/Portsmouth", wfo: "AKQ" },
ForecastZone::VA096 => crate::ZoneDetails {state: "VA", zone: "096", zone_numeric: 096, name: "Suffolk", wfo: "AKQ" },
ForecastZone::VA097 => crate::ZoneDetails {state: "VA", zone: "097", zone_numeric: 097, name: "Chesapeake", wfo: "AKQ" },
ForecastZone::VA098 => crate::ZoneDetails {state: "VA", zone: "098", zone_numeric: 098, name: "Virginia Beach", wfo: "AKQ" },
ForecastZone::VA099 => crate::ZoneDetails {state: "VA", zone: "099", zone_numeric: 099, name: "Accomack", wfo: "AKQ" },
ForecastZone::VA100 => crate::ZoneDetails {state: "VA", zone: "100", zone_numeric: 100, name: "Northampton", wfo: "AKQ" },
ForecastZone::VA501 => crate::ZoneDetails {state: "VA", zone: "501", zone_numeric: 501, name: "Northern Fauquier", wfo: "LWX" },
ForecastZone::VA502 => crate::ZoneDetails {state: "VA", zone: "502", zone_numeric: 502, name: "Southern Fauquier", wfo: "LWX" },
ForecastZone::VA503 => crate::ZoneDetails {state: "VA", zone: "503", zone_numeric: 503, name: "Western Highland", wfo: "LWX" },
ForecastZone::VA504 => crate::ZoneDetails {state: "VA", zone: "504", zone_numeric: 504, name: "Eastern Highland", wfo: "LWX" },
ForecastZone::VA505 => crate::ZoneDetails {state: "VA", zone: "505", zone_numeric: 505, name: "Western Loudoun", wfo: "LWX" },
ForecastZone::VA506 => crate::ZoneDetails {state: "VA", zone: "506", zone_numeric: 506, name: "Eastern Loudoun", wfo: "LWX" },
ForecastZone::VA507 => crate::ZoneDetails {state: "VA", zone: "507", zone_numeric: 507, name: "Northern Virginia Blue Ridge", wfo: "LWX" },
ForecastZone::VA508 => crate::ZoneDetails {state: "VA", zone: "508", zone_numeric: 508, name: "Central Virginia Blue Ridge", wfo: "LWX" },
ForecastZone::VA509 => crate::ZoneDetails {state: "VA", zone: "509", zone_numeric: 509, name: "Western Louisa", wfo: "AKQ" },
ForecastZone::VA510 => crate::ZoneDetails {state: "VA", zone: "510", zone_numeric: 510, name: "Eastern Louisa", wfo: "AKQ" },
ForecastZone::VA511 => crate::ZoneDetails {state: "VA", zone: "511", zone_numeric: 511, name: "Western Hanover", wfo: "AKQ" },
ForecastZone::VA512 => crate::ZoneDetails {state: "VA", zone: "512", zone_numeric: 512, name: "Eastern Hanover", wfo: "AKQ" },
ForecastZone::VA513 => crate::ZoneDetails {state: "VA", zone: "513", zone_numeric: 513, name: "Western Chesterfield", wfo: "AKQ" },
ForecastZone::VA514 => crate::ZoneDetails {state: "VA", zone: "514", zone_numeric: 514, name: "Eastern Chesterfield (Including Col. Heights)", wfo: "AKQ" },
ForecastZone::VA515 => crate::ZoneDetails {state: "VA", zone: "515", zone_numeric: 515, name: "Western Henrico (Including the City of Richmond)", wfo: "AKQ" },
ForecastZone::VA516 => crate::ZoneDetails {state: "VA", zone: "516", zone_numeric: 516, name: "Eastern Henrico", wfo: "AKQ" },
ForecastZone::VA517 => crate::ZoneDetails {state: "VA", zone: "517", zone_numeric: 517, name: "Western King William", wfo: "AKQ" },
ForecastZone::VA518 => crate::ZoneDetails {state: "VA", zone: "518", zone_numeric: 518, name: "Eastern King William", wfo: "AKQ" },
ForecastZone::VA519 => crate::ZoneDetails {state: "VA", zone: "519", zone_numeric: 519, name: "Western King and Queen", wfo: "AKQ" },
ForecastZone::VA520 => crate::ZoneDetails {state: "VA", zone: "520", zone_numeric: 520, name: "Eastern King and Queen", wfo: "AKQ" },
ForecastZone::VA521 => crate::ZoneDetails {state: "VA", zone: "521", zone_numeric: 521, name: "Western Essex", wfo: "AKQ" },
ForecastZone::VA522 => crate::ZoneDetails {state: "VA", zone: "522", zone_numeric: 522, name: "Eastern Essex", wfo: "AKQ" },
ForecastZone::VA523 => crate::ZoneDetails {state: "VA", zone: "523", zone_numeric: 523, name: "York", wfo: "AKQ" },
ForecastZone::VA524 => crate::ZoneDetails {state: "VA", zone: "524", zone_numeric: 524, name: "Newport News", wfo: "AKQ" },
ForecastZone::VA525 => crate::ZoneDetails {state: "VA", zone: "525", zone_numeric: 525, name: "Hampton/Poquoson", wfo: "AKQ" },
ForecastZone::VI001 => crate::ZoneDetails {state: "VI", zone: "001", zone_numeric: 001, name: "St.Thomas...St. John.. and Adjacent Islands", wfo: "SJU" },
ForecastZone::VI002 => crate::ZoneDetails {state: "VI", zone: "002", zone_numeric: 002, name: "St Croix", wfo: "SJU" },
ForecastZone::VT001 => crate::ZoneDetails {state: "VT", zone: "001", zone_numeric: 001, name: "Grand Isle", wfo: "BTV" },
ForecastZone::VT002 => crate::ZoneDetails {state: "VT", zone: "002", zone_numeric: 002, name: "Western Franklin", wfo: "BTV" },
ForecastZone::VT003 => crate::ZoneDetails {state: "VT", zone: "003", zone_numeric: 003, name: "Orleans", wfo: "BTV" },
ForecastZone::VT004 => crate::ZoneDetails {state: "VT", zone: "004", zone_numeric: 004, name: "Essex", wfo: "BTV" },
ForecastZone::VT005 => crate::ZoneDetails {state: "VT", zone: "005", zone_numeric: 005, name: "Western Chittenden", wfo: "BTV" },
ForecastZone::VT006 => crate::ZoneDetails {state: "VT", zone: "006", zone_numeric: 006, name: "Lamoille", wfo: "BTV" },
ForecastZone::VT007 => crate::ZoneDetails {state: "VT", zone: "007", zone_numeric: 007, name: "Caledonia", wfo: "BTV" },
ForecastZone::VT008 => crate::ZoneDetails {state: "VT", zone: "008", zone_numeric: 008, name: "Washington", wfo: "BTV" },
ForecastZone::VT009 => crate::ZoneDetails {state: "VT", zone: "009", zone_numeric: 009, name: "Western Addison", wfo: "BTV" },
ForecastZone::VT010 => crate::ZoneDetails {state: "VT", zone: "010", zone_numeric: 010, name: "Orange", wfo: "BTV" },
ForecastZone::VT011 => crate::ZoneDetails {state: "VT", zone: "011", zone_numeric: 011, name: "Western Rutland", wfo: "BTV" },
ForecastZone::VT013 => crate::ZoneDetails {state: "VT", zone: "013", zone_numeric: 013, name: "Bennington", wfo: "ALY" },
ForecastZone::VT014 => crate::ZoneDetails {state: "VT", zone: "014", zone_numeric: 014, name: "Western Windham", wfo: "ALY" },
ForecastZone::VT015 => crate::ZoneDetails {state: "VT", zone: "015", zone_numeric: 015, name: "Eastern Windham", wfo: "ALY" },
ForecastZone::VT016 => crate::ZoneDetails {state: "VT", zone: "016", zone_numeric: 016, name: "Eastern Franklin", wfo: "BTV" },
ForecastZone::VT017 => crate::ZoneDetails {state: "VT", zone: "017", zone_numeric: 017, name: "Eastern Chittenden", wfo: "BTV" },
ForecastZone::VT018 => crate::ZoneDetails {state: "VT", zone: "018", zone_numeric: 018, name: "Eastern Addison", wfo: "BTV" },
ForecastZone::VT019 => crate::ZoneDetails {state: "VT", zone: "019", zone_numeric: 019, name: "Eastern Rutland", wfo: "BTV" },
ForecastZone::VT020 => crate::ZoneDetails {state: "VT", zone: "020", zone_numeric: 020, name: "Western Windsor", wfo: "BTV" },
ForecastZone::VT021 => crate::ZoneDetails {state: "VT", zone: "021", zone_numeric: 021, name: "Eastern Windsor", wfo: "BTV" },
ForecastZone::WA001 => crate::ZoneDetails {state: "WA", zone: "001", zone_numeric: 001, name: "San Juan County", wfo: "SEW" },
ForecastZone::WA019 => crate::ZoneDetails {state: "WA", zone: "019", zone_numeric: 019, name: "South Washington Cascades", wfo: "PQR" },
ForecastZone::WA020 => crate::ZoneDetails {state: "WA", zone: "020", zone_numeric: 020, name: "Willapa Hills", wfo: "PQR" },
ForecastZone::WA021 => crate::ZoneDetails {state: "WA", zone: "021", zone_numeric: 021, name: "South Washington Coast", wfo: "PQR" },
ForecastZone::WA022 => crate::ZoneDetails {state: "WA", zone: "022", zone_numeric: 022, name: "Lower Columbia and I - 5 Corridor in Cowlitz County", wfo: "PQR" },
ForecastZone::WA024 => crate::ZoneDetails {state: "WA", zone: "024", zone_numeric: 024, name: "Eastern Columbia River Gorge of Washington", wfo: "PDT" },
ForecastZone::WA026 => crate::ZoneDetails {state: "WA", zone: "026", zone_numeric: 026, name: "Kittitas Valley", wfo: "PDT" },
ForecastZone::WA027 => crate::ZoneDetails {state: "WA", zone: "027", zone_numeric: 027, name: "Yakima Valley", wfo: "PDT" },
ForecastZone::WA028 => crate::ZoneDetails {state: "WA", zone: "028", zone_numeric: 028, name: "Lower Columbia Basin of Washington", wfo: "PDT" },
ForecastZone::WA029 => crate::ZoneDetails {state: "WA", zone: "029", zone_numeric: 029, name: "Foothills of the Blue Mountains of Washington", wfo: "PDT" },
ForecastZone::WA030 => crate::ZoneDetails {state: "WA", zone: "030", zone_numeric: 030, name: "Northwest Blue Mountains", wfo: "PDT" },
ForecastZone::WA031 => crate::ZoneDetails {state: "WA", zone: "031", zone_numeric: 031, name: "Northeast Blue Mountains", wfo: "OTX" },
ForecastZone::WA032 => crate::ZoneDetails {state: "WA", zone: "032", zone_numeric: 032, name: "Lower Garfield and Asotin Counties", wfo: "OTX" },
ForecastZone::WA033 => crate::ZoneDetails {state: "WA", zone: "033", zone_numeric: 033, name: "Washington Palouse", wfo: "OTX" },
ForecastZone::WA034 => crate::ZoneDetails {state: "WA", zone: "034", zone_numeric: 034, name: "Moses Lake Area", wfo: "OTX" },
ForecastZone::WA035 => crate::ZoneDetails {state: "WA", zone: "035", zone_numeric: 035, name: "Upper Columbia Basin", wfo: "OTX" },
ForecastZone::WA036 => crate::ZoneDetails {state: "WA", zone: "036", zone_numeric: 036, name: "Spokane Area", wfo: "OTX" },
ForecastZone::WA037 => crate::ZoneDetails {state: "WA", zone: "037", zone_numeric: 037, name: "Northeast Mountains", wfo: "OTX" },
ForecastZone::WA038 => crate::ZoneDetails {state: "WA", zone: "038", zone_numeric: 038, name: "Okanogan Highlands", wfo: "OTX" },
ForecastZone::WA039 => crate::ZoneDetails {state: "WA", zone: "039", zone_numeric: 039, name: "Greater Vancouver Area", wfo: "PQR" },
ForecastZone::WA040 => crate::ZoneDetails {state: "WA", zone: "040", zone_numeric: 040, name: "South Washington Cascade Foothills", wfo: "PQR" },
ForecastZone::WA041 => crate::ZoneDetails {state: "WA", zone: "041", zone_numeric: 041, name: "Wenatchee Area", wfo: "OTX" },
ForecastZone::WA043 => crate::ZoneDetails {state: "WA", zone: "043", zone_numeric: 043, name: "Okanogan Valley", wfo: "OTX" },
ForecastZone::WA044 => crate::ZoneDetails {state: "WA", zone: "044", zone_numeric: 044, name: "Waterville Plateau", wfo: "OTX" },
ForecastZone::WA045 => crate::ZoneDetails {state: "WA", zone: "045", zone_numeric: 045, name: "Western Columbia River Gorge", wfo: "PQR" },
ForecastZone::WA046 => crate::ZoneDetails {state: "WA", zone: "046", zone_numeric: 046, name: "Central Columbia River Gorge", wfo: "PQR" },
ForecastZone::WA047 => crate::ZoneDetails {state: "WA", zone: "047", zone_numeric: 047, name: "Central Chelan County", wfo: "OTX" },
ForecastZone::WA048 => crate::ZoneDetails {state: "WA", zone: "048", zone_numeric: 048, name: "Western Chelan County", wfo: "OTX" },
ForecastZone::WA049 => crate::ZoneDetails {state: "WA", zone: "049", zone_numeric: 049, name: "Western Okanogan County", wfo: "OTX" },
ForecastZone::WA503 => crate::ZoneDetails {state: "WA", zone: "503", zone_numeric: 503, name: "Western Whatcom County", wfo: "SEW" },
ForecastZone::WA504 => crate::ZoneDetails {state: "WA", zone: "504", zone_numeric: 504, name: "Southwest Interior", wfo: "SEW" },
ForecastZone::WA506 => crate::ZoneDetails {state: "WA", zone: "506", zone_numeric: 506, name: "Western Skagit County", wfo: "SEW" },
ForecastZone::WA507 => crate::ZoneDetails {state: "WA", zone: "507", zone_numeric: 507, name: "Everett and Vicinity", wfo: "SEW" },
ForecastZone::WA509 => crate::ZoneDetails {state: "WA", zone: "509", zone_numeric: 509, name: "Tacoma Area", wfo: "SEW" },
ForecastZone::WA510 => crate::ZoneDetails {state: "WA", zone: "510", zone_numeric: 510, name: "Admiralty Inlet Area", wfo: "SEW" },
ForecastZone::WA511 => crate::ZoneDetails {state: "WA", zone: "511", zone_numeric: 511, name: "Hood Canal Area", wfo: "SEW" },
ForecastZone::WA512 => crate::ZoneDetails {state: "WA", zone: "512", zone_numeric: 512, name: "Lower Chehalis Valley Area", wfo: "SEW" },
ForecastZone::WA513 => crate::ZoneDetails {state: "WA", zone: "513", zone_numeric: 513, name: "Olympics", wfo: "SEW" },
ForecastZone::WA514 => crate::ZoneDetails {state: "WA", zone: "514", zone_numeric: 514, name: "Eastern Strait of Juan de Fuca", wfo: "SEW" },
ForecastZone::WA515 => crate::ZoneDetails {state: "WA", zone: "515", zone_numeric: 515, name: "Western Strait of Juan De Fuca", wfo: "SEW" },
ForecastZone::WA516 => crate::ZoneDetails {state: "WA", zone: "516", zone_numeric: 516, name: "North Coast", wfo: "SEW" },
ForecastZone::WA517 => crate::ZoneDetails {state: "WA", zone: "517", zone_numeric: 517, name: "Central Coast", wfo: "SEW" },
ForecastZone::WA520 => crate::ZoneDetails {state: "WA", zone: "520", zone_numeric: 520, name: "East Slopes of the Washington Cascades", wfo: "PDT" },
ForecastZone::WA521 => crate::ZoneDetails {state: "WA", zone: "521", zone_numeric: 521, name: "Simcoe Highlands", wfo: "PDT" },
ForecastZone::WA555 => crate::ZoneDetails {state: "WA", zone: "555", zone_numeric: 555, name: "East Puget Sound Lowlands", wfo: "SEW" },
ForecastZone::WA556 => crate::ZoneDetails {state: "WA", zone: "556", zone_numeric: 556, name: "Bellevue and Vicinity", wfo: "SEW" },
ForecastZone::WA558 => crate::ZoneDetails {state: "WA", zone: "558", zone_numeric: 558, name: "Seattle and Vicinity", wfo: "SEW" },
ForecastZone::WA559 => crate::ZoneDetails {state: "WA", zone: "559", zone_numeric: 559, name: "Bremerton and Vicinity", wfo: "SEW" },
ForecastZone::WA567 => crate::ZoneDetails {state: "WA", zone: "567", zone_numeric: 567, name: "West Slopes North Cascades and Passes", wfo: "SEW" },
ForecastZone::WA568 => crate::ZoneDetails {state: "WA", zone: "568", zone_numeric: 568, name: "West Slopes North Central Cascades and Passes", wfo: "SEW" },
ForecastZone::WA569 => crate::ZoneDetails {state: "WA", zone: "569", zone_numeric: 569, name: "West Slopes South Central Cascades and Passes", wfo: "SEW" },
ForecastZone::WI001 => crate::ZoneDetails {state: "WI", zone: "001", zone_numeric: 001, name: "Douglas", wfo: "DLH" },
ForecastZone::WI002 => crate::ZoneDetails {state: "WI", zone: "002", zone_numeric: 002, name: "Bayfield", wfo: "DLH" },
ForecastZone::WI003 => crate::ZoneDetails {state: "WI", zone: "003", zone_numeric: 003, name: "Ashland", wfo: "DLH" },
ForecastZone::WI004 => crate::ZoneDetails {state: "WI", zone: "004", zone_numeric: 004, name: "Iron", wfo: "DLH" },
ForecastZone::WI005 => crate::ZoneDetails {state: "WI", zone: "005", zone_numeric: 005, name: "Vilas", wfo: "GRB" },
ForecastZone::WI006 => crate::ZoneDetails {state: "WI", zone: "006", zone_numeric: 006, name: "Burnett", wfo: "DLH" },
ForecastZone::WI007 => crate::ZoneDetails {state: "WI", zone: "007", zone_numeric: 007, name: "Washburn", wfo: "DLH" },
ForecastZone::WI008 => crate::ZoneDetails {state: "WI", zone: "008", zone_numeric: 008, name: "Sawyer", wfo: "DLH" },
ForecastZone::WI009 => crate::ZoneDetails {state: "WI", zone: "009", zone_numeric: 009, name: "Price", wfo: "DLH" },
ForecastZone::WI010 => crate::ZoneDetails {state: "WI", zone: "010", zone_numeric: 010, name: "Oneida", wfo: "GRB" },
ForecastZone::WI011 => crate::ZoneDetails {state: "WI", zone: "011", zone_numeric: 011, name: "Forest", wfo: "GRB" },
ForecastZone::WI012 => crate::ZoneDetails {state: "WI", zone: "012", zone_numeric: 012, name: "Florence", wfo: "GRB" },
ForecastZone::WI013 => crate::ZoneDetails {state: "WI", zone: "013", zone_numeric: 013, name: "Northern Marinette County", wfo: "GRB" },
ForecastZone::WI014 => crate::ZoneDetails {state: "WI", zone: "014", zone_numeric: 014, name: "Polk", wfo: "MPX" },
ForecastZone::WI015 => crate::ZoneDetails {state: "WI", zone: "015", zone_numeric: 015, name: "Barron", wfo: "MPX" },
ForecastZone::WI016 => crate::ZoneDetails {state: "WI", zone: "016", zone_numeric: 016, name: "Rusk", wfo: "MPX" },
ForecastZone::WI017 => crate::ZoneDetails {state: "WI", zone: "017", zone_numeric: 017, name: "Taylor", wfo: "ARX" },
ForecastZone::WI018 => crate::ZoneDetails {state: "WI", zone: "018", zone_numeric: 018, name: "Lincoln", wfo: "GRB" },
ForecastZone::WI019 => crate::ZoneDetails {state: "WI", zone: "019", zone_numeric: 019, name: "Langlade", wfo: "GRB" },
ForecastZone::WI020 => crate::ZoneDetails {state: "WI", zone: "020", zone_numeric: 020, name: "Menominee", wfo: "GRB" },
ForecastZone::WI021 => crate::ZoneDetails {state: "WI", zone: "021", zone_numeric: 021, name: "Northern Oconto County", wfo: "GRB" },
ForecastZone::WI022 => crate::ZoneDetails {state: "WI", zone: "022", zone_numeric: 022, name: "Door", wfo: "GRB" },
ForecastZone::WI023 => crate::ZoneDetails {state: "WI", zone: "023", zone_numeric: 023, name: "St. Croix", wfo: "MPX" },
ForecastZone::WI024 => crate::ZoneDetails {state: "WI", zone: "024", zone_numeric: 024, name: "Pierce", wfo: "MPX" },
ForecastZone::WI025 => crate::ZoneDetails {state: "WI", zone: "025", zone_numeric: 025, name: "Dunn", wfo: "MPX" },
ForecastZone::WI026 => crate::ZoneDetails {state: "WI", zone: "026", zone_numeric: 026, name: "Pepin", wfo: "MPX" },
ForecastZone::WI027 => crate::ZoneDetails {state: "WI", zone: "027", zone_numeric: 027, name: "Chippewa", wfo: "MPX" },
ForecastZone::WI028 => crate::ZoneDetails {state: "WI", zone: "028", zone_numeric: 028, name: "Eau Claire", wfo: "MPX" },
ForecastZone::WI029 => crate::ZoneDetails {state: "WI", zone: "029", zone_numeric: 029, name: "Clark", wfo: "ARX" },
ForecastZone::WI030 => crate::ZoneDetails {state: "WI", zone: "030", zone_numeric: 030, name: "Marathon", wfo: "GRB" },
ForecastZone::WI031 => crate::ZoneDetails {state: "WI", zone: "031", zone_numeric: 031, name: "Shawano", wfo: "GRB" },
ForecastZone::WI032 => crate::ZoneDetails {state: "WI", zone: "032", zone_numeric: 032, name: "Buffalo", wfo: "ARX" },
ForecastZone::WI033 => crate::ZoneDetails {state: "WI", zone: "033", zone_numeric: 033, name: "Trempealeau", wfo: "ARX" },
ForecastZone::WI034 => crate::ZoneDetails {state: "WI", zone: "034", zone_numeric: 034, name: "Jackson", wfo: "ARX" },
ForecastZone::WI035 => crate::ZoneDetails {state: "WI", zone: "035", zone_numeric: 035, name: "Wood", wfo: "GRB" },
ForecastZone::WI036 => crate::ZoneDetails {state: "WI", zone: "036", zone_numeric: 036, name: "Portage", wfo: "GRB" },
ForecastZone::WI037 => crate::ZoneDetails {state: "WI", zone: "037", zone_numeric: 037, name: "Waupaca", wfo: "GRB" },
ForecastZone::WI038 => crate::ZoneDetails {state: "WI", zone: "038", zone_numeric: 038, name: "Outagamie", wfo: "GRB" },
ForecastZone::WI039 => crate::ZoneDetails {state: "WI", zone: "039", zone_numeric: 039, name: "Brown", wfo: "GRB" },
ForecastZone::WI040 => crate::ZoneDetails {state: "WI", zone: "040", zone_numeric: 040, name: "Kewaunee", wfo: "GRB" },
ForecastZone::WI041 => crate::ZoneDetails {state: "WI", zone: "041", zone_numeric: 041, name: "La Crosse", wfo: "ARX" },
ForecastZone::WI042 => crate::ZoneDetails {state: "WI", zone: "042", zone_numeric: 042, name: "Monroe", wfo: "ARX" },
ForecastZone::WI043 => crate::ZoneDetails {state: "WI", zone: "043", zone_numeric: 043, name: "Juneau", wfo: "ARX" },
ForecastZone::WI044 => crate::ZoneDetails {state: "WI", zone: "044", zone_numeric: 044, name: "Adams", wfo: "ARX" },
ForecastZone::WI045 => crate::ZoneDetails {state: "WI", zone: "045", zone_numeric: 045, name: "Waushara", wfo: "GRB" },
ForecastZone::WI046 => crate::ZoneDetails {state: "WI", zone: "046", zone_numeric: 046, name: "Marquette", wfo: "MKX" },
ForecastZone::WI047 => crate::ZoneDetails {state: "WI", zone: "047", zone_numeric: 047, name: "Green Lake", wfo: "MKX" },
ForecastZone::WI048 => crate::ZoneDetails {state: "WI", zone: "048", zone_numeric: 048, name: "Winnebago", wfo: "GRB" },
ForecastZone::WI049 => crate::ZoneDetails {state: "WI", zone: "049", zone_numeric: 049, name: "Calumet", wfo: "GRB" },
ForecastZone::WI050 => crate::ZoneDetails {state: "WI", zone: "050", zone_numeric: 050, name: "Manitowoc", wfo: "GRB" },
ForecastZone::WI051 => crate::ZoneDetails {state: "WI", zone: "051", zone_numeric: 051, name: "Fond Du Lac", wfo: "MKX" },
ForecastZone::WI052 => crate::ZoneDetails {state: "WI", zone: "052", zone_numeric: 052, name: "Sheboygan", wfo: "MKX" },
ForecastZone::WI053 => crate::ZoneDetails {state: "WI", zone: "053", zone_numeric: 053, name: "Vernon", wfo: "ARX" },
ForecastZone::WI054 => crate::ZoneDetails {state: "WI", zone: "054", zone_numeric: 054, name: "Crawford", wfo: "ARX" },
ForecastZone::WI055 => crate::ZoneDetails {state: "WI", zone: "055", zone_numeric: 055, name: "Richland", wfo: "ARX" },
ForecastZone::WI056 => crate::ZoneDetails {state: "WI", zone: "056", zone_numeric: 056, name: "Sauk", wfo: "MKX" },
ForecastZone::WI057 => crate::ZoneDetails {state: "WI", zone: "057", zone_numeric: 057, name: "Columbia", wfo: "MKX" },
ForecastZone::WI058 => crate::ZoneDetails {state: "WI", zone: "058", zone_numeric: 058, name: "Dodge", wfo: "MKX" },
ForecastZone::WI059 => crate::ZoneDetails {state: "WI", zone: "059", zone_numeric: 059, name: "Washington", wfo: "MKX" },
ForecastZone::WI060 => crate::ZoneDetails {state: "WI", zone: "060", zone_numeric: 060, name: "Ozaukee", wfo: "MKX" },
ForecastZone::WI061 => crate::ZoneDetails {state: "WI", zone: "061", zone_numeric: 061, name: "Grant", wfo: "ARX" },
ForecastZone::WI062 => crate::ZoneDetails {state: "WI", zone: "062", zone_numeric: 062, name: "Iowa", wfo: "MKX" },
ForecastZone::WI063 => crate::ZoneDetails {state: "WI", zone: "063", zone_numeric: 063, name: "Dane", wfo: "MKX" },
ForecastZone::WI064 => crate::ZoneDetails {state: "WI", zone: "064", zone_numeric: 064, name: "Jefferson", wfo: "MKX" },
ForecastZone::WI065 => crate::ZoneDetails {state: "WI", zone: "065", zone_numeric: 065, name: "Waukesha", wfo: "MKX" },
ForecastZone::WI066 => crate::ZoneDetails {state: "WI", zone: "066", zone_numeric: 066, name: "Milwaukee", wfo: "MKX" },
ForecastZone::WI067 => crate::ZoneDetails {state: "WI", zone: "067", zone_numeric: 067, name: "Lafayette", wfo: "MKX" },
ForecastZone::WI068 => crate::ZoneDetails {state: "WI", zone: "068", zone_numeric: 068, name: "Green", wfo: "MKX" },
ForecastZone::WI069 => crate::ZoneDetails {state: "WI", zone: "069", zone_numeric: 069, name: "Rock", wfo: "MKX" },
ForecastZone::WI070 => crate::ZoneDetails {state: "WI", zone: "070", zone_numeric: 070, name: "Walworth", wfo: "MKX" },
ForecastZone::WI071 => crate::ZoneDetails {state: "WI", zone: "071", zone_numeric: 071, name: "Racine", wfo: "MKX" },
ForecastZone::WI072 => crate::ZoneDetails {state: "WI", zone: "072", zone_numeric: 072, name: "Kenosha", wfo: "MKX" },
ForecastZone::WI073 => crate::ZoneDetails {state: "WI", zone: "073", zone_numeric: 073, name: "Southern Marinette County", wfo: "GRB" },
ForecastZone::WI074 => crate::ZoneDetails {state: "WI", zone: "074", zone_numeric: 074, name: "Southern Oconto County", wfo: "GRB" },
ForecastZone::WV001 => crate::ZoneDetails {state: "WV", zone: "001", zone_numeric: 001, name: "Hancock", wfo: "PBZ" },
ForecastZone::WV002 => crate::ZoneDetails {state: "WV", zone: "002", zone_numeric: 002, name: "Brooke", wfo: "PBZ" },
ForecastZone::WV003 => crate::ZoneDetails {state: "WV", zone: "003", zone_numeric: 003, name: "Ohio", wfo: "PBZ" },
ForecastZone::WV004 => crate::ZoneDetails {state: "WV", zone: "004", zone_numeric: 004, name: "Marshall", wfo: "PBZ" },
ForecastZone::WV005 => crate::ZoneDetails {state: "WV", zone: "005", zone_numeric: 005, name: "Wayne", wfo: "RLX" },
ForecastZone::WV006 => crate::ZoneDetails {state: "WV", zone: "006", zone_numeric: 006, name: "Cabell", wfo: "RLX" },
ForecastZone::WV007 => crate::ZoneDetails {state: "WV", zone: "007", zone_numeric: 007, name: "Mason", wfo: "RLX" },
ForecastZone::WV008 => crate::ZoneDetails {state: "WV", zone: "008", zone_numeric: 008, name: "Jackson", wfo: "RLX" },
ForecastZone::WV009 => crate::ZoneDetails {state: "WV", zone: "009", zone_numeric: 009, name: "Wood", wfo: "RLX" },
ForecastZone::WV010 => crate::ZoneDetails {state: "WV", zone: "010", zone_numeric: 010, name: "Pleasants", wfo: "RLX" },
ForecastZone::WV011 => crate::ZoneDetails {state: "WV", zone: "011", zone_numeric: 011, name: "Tyler", wfo: "RLX" },
ForecastZone::WV012 => crate::ZoneDetails {state: "WV", zone: "012", zone_numeric: 012, name: "Wetzel", wfo: "PBZ" },
ForecastZone::WV013 => crate::ZoneDetails {state: "WV", zone: "013", zone_numeric: 013, name: "Lincoln", wfo: "RLX" },
ForecastZone::WV014 => crate::ZoneDetails {state: "WV", zone: "014", zone_numeric: 014, name: "Putnam", wfo: "RLX" },
ForecastZone::WV015 => crate::ZoneDetails {state: "WV", zone: "015", zone_numeric: 015, name: "Kanawha", wfo: "RLX" },
ForecastZone::WV016 => crate::ZoneDetails {state: "WV", zone: "016", zone_numeric: 016, name: "Roane", wfo: "RLX" },
ForecastZone::WV017 => crate::ZoneDetails {state: "WV", zone: "017", zone_numeric: 017, name: "Wirt", wfo: "RLX" },
ForecastZone::WV018 => crate::ZoneDetails {state: "WV", zone: "018", zone_numeric: 018, name: "Calhoun", wfo: "RLX" },
ForecastZone::WV019 => crate::ZoneDetails {state: "WV", zone: "019", zone_numeric: 019, name: "Ritchie", wfo: "RLX" },
ForecastZone::WV020 => crate::ZoneDetails {state: "WV", zone: "020", zone_numeric: 020, name: "Doddridge", wfo: "RLX" },
ForecastZone::WV021 => crate::ZoneDetails {state: "WV", zone: "021", zone_numeric: 021, name: "Marion", wfo: "PBZ" },
ForecastZone::WV024 => crate::ZoneDetails {state: "WV", zone: "024", zone_numeric: 024, name: "Mingo", wfo: "RLX" },
ForecastZone::WV025 => crate::ZoneDetails {state: "WV", zone: "025", zone_numeric: 025, name: "Logan", wfo: "RLX" },
ForecastZone::WV026 => crate::ZoneDetails {state: "WV", zone: "026", zone_numeric: 026, name: "Boone", wfo: "RLX" },
ForecastZone::WV027 => crate::ZoneDetails {state: "WV", zone: "027", zone_numeric: 027, name: "Clay", wfo: "RLX" },
ForecastZone::WV028 => crate::ZoneDetails {state: "WV", zone: "028", zone_numeric: 028, name: "Braxton", wfo: "RLX" },
ForecastZone::WV029 => crate::ZoneDetails {state: "WV", zone: "029", zone_numeric: 029, name: "Gilmer", wfo: "RLX" },
ForecastZone::WV030 => crate::ZoneDetails {state: "WV", zone: "030", zone_numeric: 030, name: "Lewis", wfo: "RLX" },
ForecastZone::WV031 => crate::ZoneDetails {state: "WV", zone: "031", zone_numeric: 031, name: "Harrison", wfo: "RLX" },
ForecastZone::WV032 => crate::ZoneDetails {state: "WV", zone: "032", zone_numeric: 032, name: "Taylor", wfo: "RLX" },
ForecastZone::WV033 => crate::ZoneDetails {state: "WV", zone: "033", zone_numeric: 033, name: "McDowell", wfo: "RLX" },
ForecastZone::WV034 => crate::ZoneDetails {state: "WV", zone: "034", zone_numeric: 034, name: "Wyoming", wfo: "RLX" },
ForecastZone::WV039 => crate::ZoneDetails {state: "WV", zone: "039", zone_numeric: 039, name: "Upshur", wfo: "RLX" },
ForecastZone::WV040 => crate::ZoneDetails {state: "WV", zone: "040", zone_numeric: 040, name: "Barbour", wfo: "RLX" },
ForecastZone::WV042 => crate::ZoneDetails {state: "WV", zone: "042", zone_numeric: 042, name: "Mercer", wfo: "RNK" },
ForecastZone::WV043 => crate::ZoneDetails {state: "WV", zone: "043", zone_numeric: 043, name: "Summers", wfo: "RNK" },
ForecastZone::WV044 => crate::ZoneDetails {state: "WV", zone: "044", zone_numeric: 044, name: "Monroe", wfo: "RNK" },
ForecastZone::WV050 => crate::ZoneDetails {state: "WV", zone: "050", zone_numeric: 050, name: "Hampshire", wfo: "LWX" },
ForecastZone::WV051 => crate::ZoneDetails {state: "WV", zone: "051", zone_numeric: 051, name: "Morgan", wfo: "LWX" },
ForecastZone::WV052 => crate::ZoneDetails {state: "WV", zone: "052", zone_numeric: 052, name: "Berkeley", wfo: "LWX" },
ForecastZone::WV053 => crate::ZoneDetails {state: "WV", zone: "053", zone_numeric: 053, name: "Jefferson", wfo: "LWX" },
ForecastZone::WV055 => crate::ZoneDetails {state: "WV", zone: "055", zone_numeric: 055, name: "Hardy", wfo: "LWX" },
ForecastZone::WV501 => crate::ZoneDetails {state: "WV", zone: "501", zone_numeric: 501, name: "Western Grant", wfo: "LWX" },
ForecastZone::WV502 => crate::ZoneDetails {state: "WV", zone: "502", zone_numeric: 502, name: "Eastern Grant", wfo: "LWX" },
ForecastZone::WV503 => crate::ZoneDetails {state: "WV", zone: "503", zone_numeric: 503, name: "Western Mineral", wfo: "LWX" },
ForecastZone::WV504 => crate::ZoneDetails {state: "WV", zone: "504", zone_numeric: 504, name: "Eastern Mineral", wfo: "LWX" },
ForecastZone::WV505 => crate::ZoneDetails {state: "WV", zone: "505", zone_numeric: 505, name: "Western Pendleton", wfo: "LWX" },
ForecastZone::WV506 => crate::ZoneDetails {state: "WV", zone: "506", zone_numeric: 506, name: "Eastern Pendleton", wfo: "LWX" },
ForecastZone::WV507 => crate::ZoneDetails {state: "WV", zone: "507", zone_numeric: 507, name: "Eastern Greenbrier", wfo: "RNK" },
ForecastZone::WV508 => crate::ZoneDetails {state: "WV", zone: "508", zone_numeric: 508, name: "Western Greenbrier", wfo: "RNK" },
ForecastZone::WV509 => crate::ZoneDetails {state: "WV", zone: "509", zone_numeric: 509, name: "Monongalia", wfo: "PBZ" },
ForecastZone::WV510 => crate::ZoneDetails {state: "WV", zone: "510", zone_numeric: 510, name: "Ridges of Eastern Monongalia and Northwestern Preston", wfo: "PBZ" },
ForecastZone::WV511 => crate::ZoneDetails {state: "WV", zone: "511", zone_numeric: 511, name: "Preston", wfo: "PBZ" },
ForecastZone::WV512 => crate::ZoneDetails {state: "WV", zone: "512", zone_numeric: 512, name: "Eastern Preston", wfo: "PBZ" },
ForecastZone::WV513 => crate::ZoneDetails {state: "WV", zone: "513", zone_numeric: 513, name: "Western Tucker", wfo: "PBZ" },
ForecastZone::WV514 => crate::ZoneDetails {state: "WV", zone: "514", zone_numeric: 514, name: "Eastern Tucker", wfo: "PBZ" },
ForecastZone::WV515 => crate::ZoneDetails {state: "WV", zone: "515", zone_numeric: 515, name: "Northwest Raleigh", wfo: "RLX" },
ForecastZone::WV516 => crate::ZoneDetails {state: "WV", zone: "516", zone_numeric: 516, name: "Southeast Raleigh", wfo: "RLX" },
ForecastZone::WV517 => crate::ZoneDetails {state: "WV", zone: "517", zone_numeric: 517, name: "Northwest Fayette", wfo: "RLX" },
ForecastZone::WV518 => crate::ZoneDetails {state: "WV", zone: "518", zone_numeric: 518, name: "Southeast Fayette", wfo: "RLX" },
ForecastZone::WV519 => crate::ZoneDetails {state: "WV", zone: "519", zone_numeric: 519, name: "Northwest Nicholas", wfo: "RLX" },
ForecastZone::WV520 => crate::ZoneDetails {state: "WV", zone: "520", zone_numeric: 520, name: "Southeast Nicholas", wfo: "RLX" },
ForecastZone::WV521 => crate::ZoneDetails {state: "WV", zone: "521", zone_numeric: 521, name: "Northwest Webster", wfo: "RLX" },
ForecastZone::WV522 => crate::ZoneDetails {state: "WV", zone: "522", zone_numeric: 522, name: "Southeast Webster", wfo: "RLX" },
ForecastZone::WV523 => crate::ZoneDetails {state: "WV", zone: "523", zone_numeric: 523, name: "Northwest Pocahontas", wfo: "RLX" },
ForecastZone::WV524 => crate::ZoneDetails {state: "WV", zone: "524", zone_numeric: 524, name: "Southeast Pocahontas", wfo: "RLX" },
ForecastZone::WV525 => crate::ZoneDetails {state: "WV", zone: "525", zone_numeric: 525, name: "Northwest Randolph", wfo: "RLX" },
ForecastZone::WV526 => crate::ZoneDetails {state: "WV", zone: "526", zone_numeric: 526, name: "Southeast Randolph", wfo: "RLX" },
ForecastZone::WY001 => crate::ZoneDetails {state: "WY", zone: "001", zone_numeric: 001, name: "Yellowstone National Park", wfo: "RIW" },
ForecastZone::WY002 => crate::ZoneDetails {state: "WY", zone: "002", zone_numeric: 002, name: "Absaroka Mountains", wfo: "RIW" },
ForecastZone::WY003 => crate::ZoneDetails {state: "WY", zone: "003", zone_numeric: 003, name: "Cody Foothills", wfo: "RIW" },
ForecastZone::WY004 => crate::ZoneDetails {state: "WY", zone: "004", zone_numeric: 004, name: "North Big Horn Basin", wfo: "RIW" },
ForecastZone::WY005 => crate::ZoneDetails {state: "WY", zone: "005", zone_numeric: 005, name: "Southwest Big Horn Basin", wfo: "RIW" },
ForecastZone::WY006 => crate::ZoneDetails {state: "WY", zone: "006", zone_numeric: 006, name: "Southeast Big Horn Basin", wfo: "RIW" },
ForecastZone::WY007 => crate::ZoneDetails {state: "WY", zone: "007", zone_numeric: 007, name: "Owl Creek and Bridger Mountains", wfo: "RIW" },
ForecastZone::WY008 => crate::ZoneDetails {state: "WY", zone: "008", zone_numeric: 008, name: "Bighorn Mountains West", wfo: "RIW" },
ForecastZone::WY009 => crate::ZoneDetails {state: "WY", zone: "009", zone_numeric: 009, name: "Bighorn Mountains Southeast", wfo: "RIW" },
ForecastZone::WY010 => crate::ZoneDetails {state: "WY", zone: "010", zone_numeric: 010, name: "Northeast Johnson County", wfo: "RIW" },
ForecastZone::WY011 => crate::ZoneDetails {state: "WY", zone: "011", zone_numeric: 011, name: "Southeast Johnson County", wfo: "RIW" },
ForecastZone::WY012 => crate::ZoneDetails {state: "WY", zone: "012", zone_numeric: 012, name: "Teton and Gros Ventre Mountains", wfo: "RIW" },
ForecastZone::WY013 => crate::ZoneDetails {state: "WY", zone: "013", zone_numeric: 013, name: "Jackson Hole", wfo: "RIW" },
ForecastZone::WY014 => crate::ZoneDetails {state: "WY", zone: "014", zone_numeric: 014, name: "Wind River Mountains West", wfo: "RIW" },
ForecastZone::WY015 => crate::ZoneDetails {state: "WY", zone: "015", zone_numeric: 015, name: "Wind River Mountains East", wfo: "RIW" },
ForecastZone::WY016 => crate::ZoneDetails {state: "WY", zone: "016", zone_numeric: 016, name: "Upper Wind River Basin", wfo: "RIW" },
ForecastZone::WY017 => crate::ZoneDetails {state: "WY", zone: "017", zone_numeric: 017, name: "Wind River Basin", wfo: "RIW" },
ForecastZone::WY018 => crate::ZoneDetails {state: "WY", zone: "018", zone_numeric: 018, name: "Lander Foothills", wfo: "RIW" },
ForecastZone::WY019 => crate::ZoneDetails {state: "WY", zone: "019", zone_numeric: 019, name: "Green Mountains and Rattlesnake Range", wfo: "RIW" },
ForecastZone::WY020 => crate::ZoneDetails {state: "WY", zone: "020", zone_numeric: 020, name: "Natrona County Lower Elevations", wfo: "RIW" },
ForecastZone::WY021 => crate::ZoneDetails {state: "WY", zone: "021", zone_numeric: 021, name: "Southwest Wyoming", wfo: "SLC" },
ForecastZone::WY022 => crate::ZoneDetails {state: "WY", zone: "022", zone_numeric: 022, name: "Casper Mountain", wfo: "RIW" },
ForecastZone::WY023 => crate::ZoneDetails {state: "WY", zone: "023", zone_numeric: 023, name: "Star Valley", wfo: "RIW" },
ForecastZone::WY024 => crate::ZoneDetails {state: "WY", zone: "024", zone_numeric: 024, name: "Salt River and Wyoming Ranges", wfo: "RIW" },
ForecastZone::WY025 => crate::ZoneDetails {state: "WY", zone: "025", zone_numeric: 025, name: "Upper Green River Basin Foothills", wfo: "RIW" },
ForecastZone::WY026 => crate::ZoneDetails {state: "WY", zone: "026", zone_numeric: 026, name: "Upper Green River Basin", wfo: "RIW" },
ForecastZone::WY027 => crate::ZoneDetails {state: "WY", zone: "027", zone_numeric: 027, name: "South Lincoln County", wfo: "RIW" },
ForecastZone::WY028 => crate::ZoneDetails {state: "WY", zone: "028", zone_numeric: 028, name: "Rock Springs and Green River", wfo: "RIW" },
ForecastZone::WY029 => crate::ZoneDetails {state: "WY", zone: "029", zone_numeric: 029, name: "Flaming Gorge", wfo: "RIW" },
ForecastZone::WY030 => crate::ZoneDetails {state: "WY", zone: "030", zone_numeric: 030, name: "East Sweetwater County", wfo: "RIW" },
ForecastZone::WY054 => crate::ZoneDetails {state: "WY", zone: "054", zone_numeric: 054, name: "Northern Campbell", wfo: "UNR" },
ForecastZone::WY055 => crate::ZoneDetails {state: "WY", zone: "055", zone_numeric: 055, name: "Southern Campbell", wfo: "UNR" },
ForecastZone::WY056 => crate::ZoneDetails {state: "WY", zone: "056", zone_numeric: 056, name: "Western Crook", wfo: "UNR" },
ForecastZone::WY057 => crate::ZoneDetails {state: "WY", zone: "057", zone_numeric: 057, name: "Wyoming Black Hills", wfo: "UNR" },
ForecastZone::WY058 => crate::ZoneDetails {state: "WY", zone: "058", zone_numeric: 058, name: "Weston", wfo: "UNR" },
ForecastZone::WY071 => crate::ZoneDetails {state: "WY", zone: "071", zone_numeric: 071, name: "Northeastern Crook", wfo: "UNR" },
ForecastZone::WY101 => crate::ZoneDetails {state: "WY", zone: "101", zone_numeric: 101, name: "Converse County Lower Elevations", wfo: "CYS" },
ForecastZone::WY102 => crate::ZoneDetails {state: "WY", zone: "102", zone_numeric: 102, name: "Niobrara County", wfo: "CYS" },
ForecastZone::WY103 => crate::ZoneDetails {state: "WY", zone: "103", zone_numeric: 103, name: "North Laramie Range", wfo: "CYS" },
ForecastZone::WY104 => crate::ZoneDetails {state: "WY", zone: "104", zone_numeric: 104, name: "Ferris/Seminoe/Shirley Mountains", wfo: "CYS" },
ForecastZone::WY105 => crate::ZoneDetails {state: "WY", zone: "105", zone_numeric: 105, name: "Shirley Basin", wfo: "CYS" },
ForecastZone::WY106 => crate::ZoneDetails {state: "WY", zone: "106", zone_numeric: 106, name: "Central Laramie Range and Southwest Platte County", wfo: "CYS" },
ForecastZone::WY107 => crate::ZoneDetails {state: "WY", zone: "107", zone_numeric: 107, name: "East Platte County", wfo: "CYS" },
ForecastZone::WY108 => crate::ZoneDetails {state: "WY", zone: "108", zone_numeric: 108, name: "Goshen County", wfo: "CYS" },
ForecastZone::WY109 => crate::ZoneDetails {state: "WY", zone: "109", zone_numeric: 109, name: "Central Carbon County", wfo: "CYS" },
ForecastZone::WY110 => crate::ZoneDetails {state: "WY", zone: "110", zone_numeric: 110, name: "North Snowy Range Foothills", wfo: "CYS" },
ForecastZone::WY111 => crate::ZoneDetails {state: "WY", zone: "111", zone_numeric: 111, name: "Southwest Carbon County", wfo: "CYS" },
ForecastZone::WY112 => crate::ZoneDetails {state: "WY", zone: "112", zone_numeric: 112, name: "Sierra Madre Range", wfo: "CYS" },
ForecastZone::WY113 => crate::ZoneDetails {state: "WY", zone: "113", zone_numeric: 113, name: "Upper North Platte River Basin", wfo: "CYS" },
ForecastZone::WY114 => crate::ZoneDetails {state: "WY", zone: "114", zone_numeric: 114, name: "Snowy Range", wfo: "CYS" },
ForecastZone::WY115 => crate::ZoneDetails {state: "WY", zone: "115", zone_numeric: 115, name: "Laramie Valley", wfo: "CYS" },
ForecastZone::WY116 => crate::ZoneDetails {state: "WY", zone: "116", zone_numeric: 116, name: "South Laramie Range", wfo: "CYS" },
ForecastZone::WY117 => crate::ZoneDetails {state: "WY", zone: "117", zone_numeric: 117, name: "South Laramie Range Foothills", wfo: "CYS" },
ForecastZone::WY118 => crate::ZoneDetails {state: "WY", zone: "118", zone_numeric: 118, name: "Central Laramie County", wfo: "CYS" },
ForecastZone::WY119 => crate::ZoneDetails {state: "WY", zone: "119", zone_numeric: 119, name: "East Laramie County", wfo: "CYS" },
ForecastZone::WY198 => crate::ZoneDetails {state: "WY", zone: "198", zone_numeric: 198, name: "Northeast Bighorn Mountains", wfo: "BYZ" },
ForecastZone::WY199 => crate::ZoneDetails {state: "WY", zone: "199", zone_numeric: 199, name: "Sheridan Foothills", wfo: "BYZ" },
}
    }
}