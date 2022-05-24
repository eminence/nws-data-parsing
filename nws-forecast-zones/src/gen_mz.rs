#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CoastalMarineZone {
    /// Alligator River, AM
    #[doc(hidden)]
    AMZ131,
    /// Pamlico Sound, AM
    #[doc(hidden)]
    AMZ135,
    /// Pamlico and Pungo Rivers, AM
    #[doc(hidden)]
    AMZ136,
    /// Neuse and Bay Rivers, AM
    #[doc(hidden)]
    AMZ137,
    /// S of Currituck Beach Light NC to Oregon Inlet NC out to 20 nm, AM
    #[doc(hidden)]
    AMZ150,
    /// S of Oregon Inlet NC to Cape Hatteras NC out to 20 nm, AM
    #[doc(hidden)]
    AMZ152,
    /// S of Cape Hatteras NC to Ocracoke Inlet NC out to 20 nm, AM
    #[doc(hidden)]
    AMZ154,
    /// S of Ocracoke Inlet NC to Cape Lookout NC out to 20 nm, AM
    #[doc(hidden)]
    AMZ156,
    /// S of Cape Lookout NC to Surf City NC out to 20 nm, AM
    #[doc(hidden)]
    AMZ158,
    /// Waters from Currituck Beach Light to Oregon Inlet NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ170,
    /// Waters from Oregon Inlet to Cape Hatteras NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ172,
    /// Waters from Cape Hatteras to Ocracoke Inlet NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ174,
    /// Waters fromOcracoke Inlet to Cape Lookout NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ176,
    /// Waters from Cape Lookout  to Surf City NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ178,
    /// Albemarle Sound, AM
    #[doc(hidden)]
    AMZ230,
    /// Croatan and Roanoke Sounds, AM
    #[doc(hidden)]
    AMZ231,
    /// Coastal waters from Surf City to Cape Fear NC out 20 nm, AM
    #[doc(hidden)]
    AMZ250,
    /// Coastal waters from Cape Fear NC to Little River Inlet SC out 20 nm, AM
    #[doc(hidden)]
    AMZ252,
    /// Coastal waters from Little River Inlet to Murrells Inlet SC out 20 nm, AM
    #[doc(hidden)]
    AMZ254,
    /// Coastal waters from Murrells Inlet to South Santee River SC out 20 nm, AM
    #[doc(hidden)]
    AMZ256,
    /// Waters from Surf City to Cape Fear NC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ270,
    /// Waters from Cape Fear NC to Little River Inlet SC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ272,
    /// Waters from Little River Inlet to Murrells Inlet SC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ274,
    /// Waters from Murrells Inlet NC to South Santee River SC from 20 to 40 nm, AM
    #[doc(hidden)]
    AMZ276,
    /// Charleston Harbor, AM
    #[doc(hidden)]
    AMZ330,
    /// Coastal waters from South Santee River to Edisto Beach SC out 20 nm, AM
    #[doc(hidden)]
    AMZ350,
    /// Coastal waters from Edisto Beach SC to Savannah GA out 20 nm, AM
    #[doc(hidden)]
    AMZ352,
    /// Coastal waters from Savannah GA to Altamaha Sound GA out 20 nm ...including Grays Reef National Marine Sanctuary, AM
    #[doc(hidden)]
    AMZ354,
    /// Waters from South Santee River SC to Edisto Beach SC extending from 20 nm to 40 nm, AM
    #[doc(hidden)]
    AMZ370,
    /// Waters from Edisto Beach SC to Savannah GA extending from 20 nm to 40 nm, AM
    #[doc(hidden)]
    AMZ372,
    /// Waters from Savannah GA to Altamaha Sound GA extending from 20 nm to 60 nm, AM
    #[doc(hidden)]
    AMZ374,
    /// Coastal waters from Altamaha Sound to Fernandina Beach FL out 20 NM, AM
    #[doc(hidden)]
    AMZ450,
    /// Coastal waters from Fernandina Beach to St. Augustine FL out 20 NM, AM
    #[doc(hidden)]
    AMZ452,
    /// Coastal waters from St. Augustine to Flagler Beach FL out 20 NM, AM
    #[doc(hidden)]
    AMZ454,
    /// Waters from Altamaha Sound GA to Fernandina Beach FL from 20 to 60 NM, AM
    #[doc(hidden)]
    AMZ470,
    /// Waters from Fernandina Beach to St. Augustine FL from 20 to 60 NM, AM
    #[doc(hidden)]
    AMZ472,
    /// Waters from St. Augustine to Flagler Beach FL from 20 to 60 NM, AM
    #[doc(hidden)]
    AMZ474,
    /// Flagler Beach to Volusia-Brevard County Line 0-20 nm, AM
    #[doc(hidden)]
    AMZ550,
    /// Volusia-Brevard County Line to Sebastian Inlet 0-20 nm, AM
    #[doc(hidden)]
    AMZ552,
    /// Sebastian Inlet to Jupiter Inlet 0-20 nm, AM
    #[doc(hidden)]
    AMZ555,
    /// Flagler Beach to Volusia-Brevard County Line 20-60 nm, AM
    #[doc(hidden)]
    AMZ570,
    /// Volusia-Brevard County Line to Sebastian Inlet 20-60 nm, AM
    #[doc(hidden)]
    AMZ572,
    /// Sebastian Inlet to Jupiter Inlet 20-60 nm, AM
    #[doc(hidden)]
    AMZ575,
    /// Lake Okeechobee, AM
    #[doc(hidden)]
    AMZ610,
    /// Biscayne Bay, AM
    #[doc(hidden)]
    AMZ630,
    /// Coastal waters from Jupiter Inlet to Deerfield Beach FL out 20 NM, AM
    #[doc(hidden)]
    AMZ650,
    /// Coastal waters from Deerfield Beach to Ocean Reef FL out 20 NM, AM
    #[doc(hidden)]
    AMZ651,
    /// Waters from Jupiter Inlet to Deerfield Beach FL from 20 to 60 NM, AM
    #[doc(hidden)]
    AMZ670,
    /// Waters from Deerfield Beach to Ocean Reef FL from 20 to 60 NM excluding the territorial waters of Bahamas, AM
    #[doc(hidden)]
    AMZ671,
    /// Atlantic Waters of Puerto Rico AND USVI from 10 NM to 19.5N, AM
    #[doc(hidden)]
    AMZ710,
    /// Coastal Waters of Northern Puerto Rico out 10 NM, AM
    #[doc(hidden)]
    AMZ712,
    /// Coastal Waters of Northern USVI and Culebra out 10 NM, AM
    #[doc(hidden)]
    AMZ715,
    /// Anegada Passage Southward to 17N, AM
    #[doc(hidden)]
    AMZ722,
    /// Coastal Waters of Southern USVI, Vieques, and Eastern Puerto Rico out 10 NM, AM
    #[doc(hidden)]
    AMZ725,
    /// Caribbean Waters of Puerto Rico from 10 NM to 17N, AM
    #[doc(hidden)]
    AMZ732,
    /// Coastal Waters of Southern Puerto Rico out 10 NM, AM
    #[doc(hidden)]
    AMZ735,
    /// Mona Passage Southward to 17N, AM
    #[doc(hidden)]
    AMZ741,
    /// Coastal Waters OF Northwestern Puerto Rico out 10 NM, AM
    #[doc(hidden)]
    AMZ742,
    /// Coastal Waters OF Southwestern Puerto Rico out 10 NM, AM
    #[doc(hidden)]
    AMZ745,
    /// Coastal Waters from Eastport, ME to Schoodic Point, ME out 25 NM, AN
    #[doc(hidden)]
    ANZ050,
    /// Coastal Waters from Schoodic Point, ME to Stonington, ME out 25 NM, AN
    #[doc(hidden)]
    ANZ051,
    /// Intra Coastal Waters from Schoodic Point, ME to Stonington, ME, AN
    #[doc(hidden)]
    ANZ052,
    /// Waters from Eastport ME to Schoodic Point, ME from 25 to 40 nm, AN
    #[doc(hidden)]
    ANZ070,
    /// Waters from Schoodic Point, ME to Stonington ME from 25 to 40 nm, AN
    #[doc(hidden)]
    ANZ071,
    /// Coastal Waters from Stonington, ME to Port Clyde, ME out 25 NM, AN
    #[doc(hidden)]
    ANZ150,
    /// Penobscot Bay, AN
    #[doc(hidden)]
    ANZ151,
    /// Coastal Waters from Port Clyde, ME to Cape Elizabeth, ME out 25 NM, AN
    #[doc(hidden)]
    ANZ152,
    /// Casco Bay, AN
    #[doc(hidden)]
    ANZ153,
    /// Coastal Waters from Cape Elizabeth, ME to Merrimack River, MA out 25 NM, AN
    #[doc(hidden)]
    ANZ154,
    /// Waters from Stonington ME to Port Clyde ME from 25 to 40 nm, AN
    #[doc(hidden)]
    ANZ170,
    /// Waters from Port Clyde ME to Cape Elizabeth ME from 25 to 40 nm, AN
    #[doc(hidden)]
    ANZ172,
    /// Waters from Cape Elizabeth ME to Merrimack River MA from 25 to 40 nm, AN
    #[doc(hidden)]
    ANZ174,
    /// Boston Harbor, AN
    #[doc(hidden)]
    ANZ230,
    /// Cape Cod Bay, AN
    #[doc(hidden)]
    ANZ231,
    /// Nantucket Sound, AN
    #[doc(hidden)]
    ANZ232,
    /// Vineyard Sound, AN
    #[doc(hidden)]
    ANZ233,
    /// Buzzards Bay, AN
    #[doc(hidden)]
    ANZ234,
    /// Rhode Island Sound, AN
    #[doc(hidden)]
    ANZ235,
    /// Narragansett Bay, AN
    #[doc(hidden)]
    ANZ236,
    /// Block Island Sound, AN
    #[doc(hidden)]
    ANZ237,
    /// Coastal waters east of Ipswich Bay and the Stellwagen Bank National Marine Sanctuary, AN
    #[doc(hidden)]
    ANZ250,
    /// Massachusetts Bay and Ipswich Bay, AN
    #[doc(hidden)]
    ANZ251,
    /// Coastal waters from Provincetown MA to Chatham MA to Nantucket MA out 20 nm, AN
    #[doc(hidden)]
    ANZ254,
    /// Coastal Waters extending out to 25 nm South of Marthas Vineyard and Nantucket, AN
    #[doc(hidden)]
    ANZ255,
    /// Coastal Waters from Montauk NY to Marthas Vineyard extending out to 20 nm South of Block Island, AN
    #[doc(hidden)]
    ANZ256,
    /// Ocean Waters from the Merrimack River to Plymouth from 40 to 60 NM offshore, AN
    #[doc(hidden)]
    ANZ270,
    /// Ocean Waters from Provincetown to Nantucket from 20 to 35 NM offshore, AN
    #[doc(hidden)]
    ANZ271,
    /// Ocean Waters from Marthas Vineyard to Nantucket from 25 to 45 NM offshore, AN
    #[doc(hidden)]
    ANZ272,
    /// Ocean Waters from Montauk NY to Marthas Vineyard from 25 to 40 NM offshore, AN
    #[doc(hidden)]
    ANZ273,
    /// Long Island Sound East of New Haven CT/Port Jefferson NY to the Mouth of the Connecticut River, AN
    #[doc(hidden)]
    ANZ331,
    /// Long Island Sound East of the Mouth of the Connecticut River, AN
    #[doc(hidden)]
    ANZ332,
    /// Long Island Sound West of New Haven CT/Port Jefferson NY, AN
    #[doc(hidden)]
    ANZ335,
    /// New York Harbor, AN
    #[doc(hidden)]
    ANZ338,
    /// Peconic and Gardiners Bays, AN
    #[doc(hidden)]
    ANZ340,
    /// South Shore Bays from Jones Inlet through Shinnecock Bay, AN
    #[doc(hidden)]
    ANZ345,
    /// Moriches Inlet NY to Montauk Point NY out 20 nm, AN
    #[doc(hidden)]
    ANZ350,
    /// Fire Island Inlet NY to Moriches Inlet NY out 20 nm, AN
    #[doc(hidden)]
    ANZ353,
    /// Sandy Hook NJ to Fire Island Inlet NY out 20 nm, AN
    #[doc(hidden)]
    ANZ355,
    /// Waters from Moriches Inlet NY to Montauk Point NY from 20 to 40 NM, AN
    #[doc(hidden)]
    ANZ370,
    /// Waters from Fire Island Inlet NY to Moriches Inlet NY from 20 to 40 NM, AN
    #[doc(hidden)]
    ANZ373,
    /// Waters from Sandy Hook NJ to Fire Island Inlet out 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ375,
    /// Delaware Bay waters north of East Point NJ to Slaughter Beach DE, AN
    #[doc(hidden)]
    ANZ430,
    /// Delaware Bay waters south of East Point NJ to Slaughter Beach DE, AN
    #[doc(hidden)]
    ANZ431,
    /// Coastal waters from Sandy Hook to Manasquan Inlet NJ out 20 nm, AN
    #[doc(hidden)]
    ANZ450,
    /// Coastal waters from Manasquan Inlet to Little Egg Inlet NJ out 20 nm, AN
    #[doc(hidden)]
    ANZ451,
    /// Coastal waters from Little Egg Inlet to Great Egg Inlet NJ out 20 nm, AN
    #[doc(hidden)]
    ANZ452,
    /// Coastal waters from Great Egg Inlet to Cape May NJ out 20 nm, AN
    #[doc(hidden)]
    ANZ453,
    /// Coastal waters from Cape May NJ to Cape Henlopen DE out 20 nm, AN
    #[doc(hidden)]
    ANZ454,
    /// Coastal waters from Cape Henlopen to Fenwick Island DE out 20 nm, AN
    #[doc(hidden)]
    ANZ455,
    /// Waters from Sandy Hook NJ to Manasquan Inlet NJ out 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ470,
    /// Waters from Manasquan Inlet NJ to Little Egg Inlet NJ out 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ471,
    /// Waters from Little Egg Inlet NJ to Great Egg Inlet NJ out 20 to 40 nm from 20 to 40 NM, AN
    #[doc(hidden)]
    ANZ472,
    /// Waters from Great Egg Inlet NJ to Cape May NJ out 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ473,
    /// Waters from Cape May NJ to Fenwick Island DE out 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ475,
    /// Chesapeake Bay north of Pooles Island MD, AN
    #[doc(hidden)]
    ANZ530,
    /// Chesapeake Bay from Pooles Island to Sandy Point MD, AN
    #[doc(hidden)]
    ANZ531,
    /// Chesapeake Bay from Sandy Point to North Beach MD, AN
    #[doc(hidden)]
    ANZ532,
    /// Chesapeake Bay from North Beach to Drum Point MD, AN
    #[doc(hidden)]
    ANZ533,
    /// Chesapeake Bay from Drum Point MD to Smith Point VA, AN
    #[doc(hidden)]
    ANZ534,
    /// Tidal Potomac from Key Bridge to Indian Head MD, AN
    #[doc(hidden)]
    ANZ535,
    /// Tidal Potomac from Indian Head to Cobb Island MD, AN
    #[doc(hidden)]
    ANZ536,
    /// Tidal Potomac from Cobb Island MD to Smith Point VA, AN
    #[doc(hidden)]
    ANZ537,
    /// Patapsco River including Baltimore Harbor, AN
    #[doc(hidden)]
    ANZ538,
    /// Chester River to Queenstown MD, AN
    #[doc(hidden)]
    ANZ539,
    /// Eastern Bay, AN
    #[doc(hidden)]
    ANZ540,
    /// Choptank River to Cambridge MD and the Little Choptank River, AN
    #[doc(hidden)]
    ANZ541,
    /// Patuxent River to Broomes Island MD, AN
    #[doc(hidden)]
    ANZ542,
    /// Tangier Sound and the inland waters surrounding Bloodsworth Island, AN
    #[doc(hidden)]
    ANZ543,
    /// Chesapeake Bay from Smith Point to Windmill Point VA, AN
    #[doc(hidden)]
    ANZ630,
    /// Chesapeake Bay from Windmill Point to New Point Comfort VA, AN
    #[doc(hidden)]
    ANZ631,
    /// Chesapeake Bay from New Point Comfort to Little Creek VA, AN
    #[doc(hidden)]
    ANZ632,
    /// Currituck Sound, AN
    #[doc(hidden)]
    ANZ633,
    /// Chesapeake Bay from Little Creek VA to Cape Henry VA including the Chesapeake Bay Bridge Tunnel, AN
    #[doc(hidden)]
    ANZ634,
    /// Rappahannock River from Urbanna to Windmill Point, AN
    #[doc(hidden)]
    ANZ635,
    /// York River, AN
    #[doc(hidden)]
    ANZ636,
    /// James River from Jamestown to the James River Bridge, AN
    #[doc(hidden)]
    ANZ637,
    /// James River from James River Bridge to Hampton Roads Bridge-Tunnel, AN
    #[doc(hidden)]
    ANZ638,
    /// Coastal waters from Fenwick Island DE to Chincoteague VA out 20 nm, AN
    #[doc(hidden)]
    ANZ650,
    /// Coastal waters from Chincoteague to Parramore Island VA out 20 nm, AN
    #[doc(hidden)]
    ANZ652,
    /// Coastal waters from Parramore Island to Cape Charles Light VA out 20 nm, AN
    #[doc(hidden)]
    ANZ654,
    /// Coastal Waters from Cape Charles Light to Virginia-North Carolina border out to 20 nm, AN
    #[doc(hidden)]
    ANZ656,
    /// Coastal waters from NC VA border to Currituck Beach Light NC out 20 nm, AN
    #[doc(hidden)]
    ANZ658,
    /// Waters from Fenwick Island DE to Chintoteague VA from 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ670,
    /// Waters from Chincoteague VA to Parramore Island VA from 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ672,
    /// Waters from Parramore Island VA to Cape Charles Light VA from 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ674,
    /// Waters from Cape Charles Light to Virginia - North Carolina Border from 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ676,
    /// Waters from NC VA border to Currituck Beach Light NC from 20 to 40 nm, AN
    #[doc(hidden)]
    ANZ678,
    /// Florida Bay including Barnes Sound, Blackwater Sound, and Buttonwood Sound, GM
    #[doc(hidden)]
    GMZ031,
    /// Bayside and Gulf side from Craig Key to West End of Seven Mile Bridge, GM
    #[doc(hidden)]
    GMZ032,
    /// Gulf waters from East Cape Sable to Chokoloskee 20 to 60 NM out and beyond 5 fathoms, GM
    #[doc(hidden)]
    GMZ033,
    /// Gulf of Mexico including Dry Tortugas and Rebecca Shoal Channel, GM
    #[doc(hidden)]
    GMZ034,
    /// Gulf of Mexico from West End of Seven Mile Bridge to Halfmoon Shoal out to 5 Fathoms, GM
    #[doc(hidden)]
    GMZ035,
    /// Hawk Channel from Ocean Reef to Craig Key out to the reef, GM
    #[doc(hidden)]
    GMZ042,
    /// Hawk Channel from Craig Key to west end of Seven Mile Bridge out to the reef, GM
    #[doc(hidden)]
    GMZ043,
    /// Hawk Channel from west end of Seven Mile Bridge to Halfmoon Shoal out to the reef, GM
    #[doc(hidden)]
    GMZ044,
    /// Straits of Florida from Ocean Reef to Craig Key out 20 NM, GM
    #[doc(hidden)]
    GMZ052,
    /// Straits of Florida from Craig Key to west end of Seven Mile Bridge out 20 NM, GM
    #[doc(hidden)]
    GMZ053,
    /// Straits of Florida from west end of Seven Mile Bridge to south of Halfmoon Shoal out 20 NM, GM
    #[doc(hidden)]
    GMZ054,
    /// Straits of Florida from Halfmoon Shoal to 20 NM west of Dry Tortugas out 20 NM, GM
    #[doc(hidden)]
    GMZ055,
    /// Straits of Florida from Ocean Reef to Craig Key 20 to 60 NM out, GM
    #[doc(hidden)]
    GMZ072,
    /// Straits of Florida from Craig Key to west end of Seven Mile Bridge 20 to 60 NM out, GM
    #[doc(hidden)]
    GMZ073,
    /// Straits of Florida from west end of Seven Mile Bridge to south of Halfmoon Shoal 20 to 60 NM out, GM
    #[doc(hidden)]
    GMZ074,
    /// Straits of Florida from Halfmoon Shoal to 20 NM west of Dry Tortugas 20 to 60 NM out, GM
    #[doc(hidden)]
    GMZ075,
    /// Laguna Madre From the Port Of Brownsville to the Arroyo Colorado, GM
    #[doc(hidden)]
    GMZ130,
    /// Laguna Madre From The Arroyo Colorado To 5 NM North Of Port Mansfield TX, GM
    #[doc(hidden)]
    GMZ132,
    /// Laguna Madre From 5 nm North Of Port Mansfield To Baffin Bay TX, GM
    #[doc(hidden)]
    GMZ135,
    /// Coastal waters from Port Mansfield TX to the Rio Grande River out 20 NM, GM
    #[doc(hidden)]
    GMZ150,
    /// Coastal waters from Baffin Bay to Port Mansfield TX out 20 NM, GM
    #[doc(hidden)]
    GMZ155,
    /// Waters from Port Mansfield TX to the Rio Grande River from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ170,
    /// Waters from Baffin Bay to Port Mansfield TX from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ175,
    /// Baffin Bay and Upper Laguna Madre, GM
    #[doc(hidden)]
    GMZ231,
    /// Corpus Christi and Nueces Bays, GM
    #[doc(hidden)]
    GMZ232,
    /// Copano, Aransas, and Redfish Bays, GM
    #[doc(hidden)]
    GMZ236,
    /// San Antonio, Mesquite, and Espiritu Santo Bays, GM
    #[doc(hidden)]
    GMZ237,
    /// Coastal waters from Baffin Bay to Port Aransas out 20 NM, GM
    #[doc(hidden)]
    GMZ250,
    /// Coastal waters from Port Aransas to Matagorda Ship Channel out 20 NM, GM
    #[doc(hidden)]
    GMZ255,
    /// Waters from Baffin Bay to Port Aransas from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ270,
    /// Waters from Port Aransas to Matagorda Ship Channel from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ275,
    /// Matagorda Bay, GM
    #[doc(hidden)]
    GMZ330,
    /// Galveston Bay, GM
    #[doc(hidden)]
    GMZ335,
    /// Coastal waters from Freeport to Matagorda Ship Channel TX out 20 NM, GM
    #[doc(hidden)]
    GMZ350,
    /// Coastal waters from High Island to Freeport TX out 20 NM, GM
    #[doc(hidden)]
    GMZ355,
    /// Waters from Freeport to Matagorda Ship Channel TX from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ370,
    /// Waters from High Island to Freeport TX from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ375,
    /// Sabine Lake, GM
    #[doc(hidden)]
    GMZ430,
    /// Calcasieu Lake, GM
    #[doc(hidden)]
    GMZ432,
    /// Vermilion Bay, GM
    #[doc(hidden)]
    GMZ435,
    /// Coastal waters from Cameron LA to High Island TX out 20 NM, GM
    #[doc(hidden)]
    GMZ450,
    /// Coastal waters from Intracoastal City to Cameron LA out 20 NM, GM
    #[doc(hidden)]
    GMZ452,
    /// Coastal waters from Lower Atchafalaya River to Intracoastal City LA out 20 NM, GM
    #[doc(hidden)]
    GMZ455,
    /// Waters from Cameron LA to High Island TX from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ470,
    /// Waters from Intracoastal City to Cameron LA from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ472,
    /// Waters from Lower Atchafalaya River to Intracoastal City LA from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ475,
    /// Lake Pontchartrain and Lake Maurepas, GM
    #[doc(hidden)]
    GMZ530,
    /// Mississippi Sound, GM
    #[doc(hidden)]
    GMZ532,
    /// Lake Borgne, GM
    #[doc(hidden)]
    GMZ534,
    /// Chandeleur Sound, GM
    #[doc(hidden)]
    GMZ536,
    /// Breton Sound, GM
    #[doc(hidden)]
    GMZ538,
    /// Coastal Waters from Port Fourchon LA to Lower Atchafalaya River LA out 20 nm, GM
    #[doc(hidden)]
    GMZ550,
    /// Coastal waters from the Southwest Pass of the Mississippi River to Port Fourchon Louisiana out 20 NM, GM
    #[doc(hidden)]
    GMZ552,
    /// Coastal Waters from Boothville LA to Southwest Pass of the Mississippi River out 20 nm, GM
    #[doc(hidden)]
    GMZ555,
    /// Coastal waters from Pascagoula Mississippi to Stake Island out 20 NM, GM
    #[doc(hidden)]
    GMZ557,
    /// Coastal waters from Port Fourchon Louisiana to Lower Atchafalaya River LA from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ570,
    /// Coastal waters from Southwest Pass of the Mississippi River to Port Fourchon Louisiana from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ572,
    /// Coastal Waters from Stake Island LA to Southwest Pass of the Mississippi River from 20 to 60 nm, GM
    #[doc(hidden)]
    GMZ575,
    /// Coastal waters from Pascagoula Mississippi to Stake Island Louisiana out 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ577,
    /// North Mobile Bay, GM
    #[doc(hidden)]
    GMZ630,
    /// South Mobile Bay, GM
    #[doc(hidden)]
    GMZ631,
    /// Mississippi Sound, GM
    #[doc(hidden)]
    GMZ632,
    /// Perdido Bay Area, GM
    #[doc(hidden)]
    GMZ633,
    /// Pensacola Bay Area including Santa Rosa Sound, GM
    #[doc(hidden)]
    GMZ634,
    /// Western Choctawhatchee Bay, GM
    #[doc(hidden)]
    GMZ635,
    /// Eastern Choctawhatchee Bay, GM
    #[doc(hidden)]
    GMZ636,
    /// Coastal waters from Pensacola FL to Pascagoula MS out 20 NM, GM
    #[doc(hidden)]
    GMZ650,
    /// Coastal waters from Okaloosa-Walton County Line to Pensacola FL out 20 NM, GM
    #[doc(hidden)]
    GMZ655,
    /// Coastal waters from Chokoloskee to Bonita Beach FL out 20 NM, GM
    #[doc(hidden)]
    GMZ656,
    /// Coastal waters from East Cape Sable to Chokoloskee FL out 20 NM, GM
    #[doc(hidden)]
    GMZ657,
    /// Waters from Pensacola FL to Pascagoula MS from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ670,
    /// Waters from Okaloosa-Walton County Line to Pensacola FL from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ675,
    /// Waters from Chokoloskee to Bonita Beach FL from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ676,
    /// Apalachee Bay or Coastal Waters From Keaton Beach to Ochlockonee River Fl out to 20 Nm, GM
    #[doc(hidden)]
    GMZ730,
    /// Coastal waters from Okaloosa-Walton County Line to Mexico Beach out 20 NM, GM
    #[doc(hidden)]
    GMZ750,
    /// Coastal Waters from Mexico Beach to Apalachicola out 20 NM, GM
    #[doc(hidden)]
    GMZ752,
    /// Coastal Waters From  Ochlockonee River to Apalachicola FL out to 20 Nm, GM
    #[doc(hidden)]
    GMZ755,
    /// Coastal waters from  Suwannee River to Keaton Beach out 20 NM, GM
    #[doc(hidden)]
    GMZ765,
    /// Waters from Okaloosa-Walton County Line to Mexico Beach from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ770,
    /// Waters from Mexico Beach to Apalachicola FL from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ772,
    /// Waters from Suwannee River to Apalachicola FL from 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ775,
    /// Tampa Bay waters, GM
    #[doc(hidden)]
    GMZ830,
    /// Charlotte Harbor and Pine Island Sound, GM
    #[doc(hidden)]
    GMZ836,
    /// Coastal waters from Tarpon Springs to Suwannee River FL out 20 NM, GM
    #[doc(hidden)]
    GMZ850,
    /// Coastal waters from Englewood to Tarpon Springs FL out 20 NM, GM
    #[doc(hidden)]
    GMZ853,
    /// Coastal waters from Bonita Beach to Englewood FL out 20 NM, GM
    #[doc(hidden)]
    GMZ856,
    /// Waters from Tarpon Springs to Suwannee River FL out 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ870,
    /// Waters from Englewood to Tarpon Springs FL out 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ873,
    /// Waters from Bonita Beach to Englewood FL out 20 to 60 NM, GM
    #[doc(hidden)]
    GMZ876,
    /// St. Clair River, LC
    #[doc(hidden)]
    LCZ422,
    /// Detroit River, LC
    #[doc(hidden)]
    LCZ423,
    /// Lake St. Clair Open Lake (U.S. Portion), LC
    #[doc(hidden)]
    LCZ460,
    /// Upper Niagara River and Buffalo Harbor, LE
    #[doc(hidden)]
    LEZ020,
    /// Ripley to Dunkirk NY, LE
    #[doc(hidden)]
    LEZ040,
    /// Dunkirk to Buffalo NY, LE
    #[doc(hidden)]
    LEZ041,
    /// Ripley to Buffalo NY extending from 5NM off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ061,
    /// Maumee Bay to Reno Beach OH, LE
    #[doc(hidden)]
    LEZ142,
    /// Reno Beach to The Islands OH, LE
    #[doc(hidden)]
    LEZ143,
    /// The Islands to Vermilion OH, LE
    #[doc(hidden)]
    LEZ144,
    /// Vermilion to Avon Point OH, LE
    #[doc(hidden)]
    LEZ145,
    /// Avon Point to Willowick OH, LE
    #[doc(hidden)]
    LEZ146,
    /// Willowick to Geneva-on-the Lake OH, LE
    #[doc(hidden)]
    LEZ147,
    /// Geneva-on-the-Lake to Conneaut OH, LE
    #[doc(hidden)]
    LEZ148,
    /// Conneaut OH to Ripley NY, LE
    #[doc(hidden)]
    LEZ149,
    /// Detroit River Lt. to Maumee Bay OH to Reno Beach OH beyond 5NM offshoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ162,
    /// Reno Beach to The Islands OH beyond 5NM off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ163,
    /// The Islands to Vermilion OH beyond 5 nm off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ164,
    /// Vermilion to Avon Point OH beyond 5 nm off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ165,
    /// Avon Point to Willowick OH beyond 5 nm off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ166,
    /// Willowick to Geneva-on-the-Lake OH beyond 5NM off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ167,
    /// Geneva-on-the-Lake to Conneaut OH beyond 5 nm off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ168,
    /// Conneaut OH to Ripley NY beyond 5 nm off shoreline to US-Canadian border, LE
    #[doc(hidden)]
    LEZ169,
    /// Michigan Waters of Lake Erie from Detroit River to North Cape MI, LE
    #[doc(hidden)]
    LEZ444,
    /// Straits of Mackinac within 5 nm of Mackinac Bridge including Mackinac Island, LH
    #[doc(hidden)]
    LHZ345,
    /// St Ignace to False Detour Channel, LH
    #[doc(hidden)]
    LHZ346,
    /// 5NM East of Mackinac Bridge to Presque Isle Light MI including Bois Blanc Island, LH
    #[doc(hidden)]
    LHZ347,
    /// Presque Isle Light to Sturgeon Pt MI Including Thunder Bay National Marine Sanctuary, LH
    #[doc(hidden)]
    LHZ348,
    /// Sturgeon Pt to Alabaster MI, LH
    #[doc(hidden)]
    LHZ349,
    /// Lake Huron from 5NM east of Mackinac Bridge to Presque Isle Lt  to the US/Canadian border beyond 5 NM from shore, LH
    #[doc(hidden)]
    LHZ361,
    /// Lake Huron from Presque Isle Lt. to Sturgeon Point  MI 5NM off shore to US/Canadian border, LH
    #[doc(hidden)]
    LHZ362,
    /// Lake Huron from Sturgeon Point to Alabaster MI 5NM off shore to US/Canadian border, LH
    #[doc(hidden)]
    LHZ363,
    /// Outer Saginaw Bay SW of Alabaster to Port Austin MI to Inner Saginaw Bay, LH
    #[doc(hidden)]
    LHZ421,
    /// Inner Saginaw Bay SW of Point Au Gres to Bay Port MI, LH
    #[doc(hidden)]
    LHZ422,
    /// Port Austin to Harbor Beach MI, LH
    #[doc(hidden)]
    LHZ441,
    /// Harbor Beach to Port Sanilac MI, LH
    #[doc(hidden)]
    LHZ442,
    /// Port Sanilac to Port Huron MI, LH
    #[doc(hidden)]
    LHZ443,
    /// Lake Huron from Port Austin to Harbor Beach 5NM Off Shore to the US/Canadian border, LH
    #[doc(hidden)]
    LHZ462,
    /// Lake Huron from Harbor Beach to Port Sanilac 5NM Off Shore to US/Canadian border, LH
    #[doc(hidden)]
    LHZ463,
    /// Lake Huron from Port Sanilac to Port Huron 5NM Off Shore to US/Canadian border, LH
    #[doc(hidden)]
    LHZ464,
    /// New Buffalo MI to St Joseph MI, LM
    #[doc(hidden)]
    LMZ043,
    /// Michigan City IN to New Buffalo MI, LM
    #[doc(hidden)]
    LMZ046,
    /// Lake Michigan Michigan City IN to St. Joseph MI 5 NM offshore to mid-line of lake., LM
    #[doc(hidden)]
    LMZ080,
    /// Green Bay North of line from Cedar River MI to Rock Island Passage, LM
    #[doc(hidden)]
    LMZ221,
    /// Seul Choix Point to Point Detour MI, LM
    #[doc(hidden)]
    LMZ248,
    /// 5NM East of a line from Fairport MI to Rock Island Passage, LM
    #[doc(hidden)]
    LMZ250,
    /// Lake Michigan from Seul Choix Point to Rock Island Passage 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ261,
    /// Grand Traverse Bay south of a line Grand Traverse Light to Norwood MI, LM
    #[doc(hidden)]
    LMZ323,
    /// Seul Choix Point to 5NM West of Mackinac Bridge, LM
    #[doc(hidden)]
    LMZ341,
    /// Norwood MI to 5NM West of Mackinac Bridge including Little Traverse Bay, LM
    #[doc(hidden)]
    LMZ342,
    /// Sleeping Bear Point to Grand Traverse Light MI, LM
    #[doc(hidden)]
    LMZ344,
    /// Point Betsie to Sleeping Bear Point MI, LM
    #[doc(hidden)]
    LMZ345,
    /// Manistee to Point Betsie MI, LM
    #[doc(hidden)]
    LMZ346,
    /// Lake Michigan South of a line from Seul Choix Point to the Mackinac Bridge and North of a line from Charlevoix MI to South Fox Island 5NM offshore, LM
    #[doc(hidden)]
    LMZ362,
    /// Lake Michigan from Charlevoix to Point Betsie MI 5NM Offshore to mid lake, LM
    #[doc(hidden)]
    LMZ364,
    /// Lake Michigan from Point Betsie to Manistee MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ366,
    /// Green Bay south of line from  Cedar River to Rock Island Passage and north of a line from Oconto WI to Little Sturgeon Bay WI, LM
    #[doc(hidden)]
    LMZ521,
    /// Green Bay south of line from  Oconto WI to Little Sturgeon Bay WI, LM
    #[doc(hidden)]
    LMZ522,
    /// Rock Island Passage to Sturgeon Bay WI, LM
    #[doc(hidden)]
    LMZ541,
    /// Sturgeon Bay to Two Rivers WI, LM
    #[doc(hidden)]
    LMZ542,
    /// Two Rivers to Sheboygan WI, LM
    #[doc(hidden)]
    LMZ543,
    /// Lake Michigan from Rock Island Passage to Sturgeon Bay WI 5NM offshore to mid lake, LM
    #[doc(hidden)]
    LMZ563,
    /// Lake Michigan from Sturgeon Bay to Two Rivers WI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ565,
    /// Lake Michigan from Two Rivers to Sheboygan WI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ567,
    /// Sheboygan to Port Washington WI, LM
    #[doc(hidden)]
    LMZ643,
    /// Port Washington to North Point Light WI, LM
    #[doc(hidden)]
    LMZ644,
    /// North Point Light to Wind Point WI, LM
    #[doc(hidden)]
    LMZ645,
    /// Wind Point WI to Winthrop Harbor IL, LM
    #[doc(hidden)]
    LMZ646,
    /// Lake Michigan from Sheboygan to Port Washington WI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ669,
    /// Lake Michigan from Port Washington to North Point Light WI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ671,
    /// Lake Michigan from North Point Light to Wind Point WI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ673,
    /// Lake Michigan from Wind Point WI to Winthrop Harbor IL 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ675,
    /// Winthrop Harbor to Wilmette Harbor IL, LM
    #[doc(hidden)]
    LMZ740,
    /// Wilmette Harbor to Northerly Island IL, LM
    #[doc(hidden)]
    LMZ741,
    /// Northerly Island to Calumet Harbor IL, LM
    #[doc(hidden)]
    LMZ742,
    /// Calumet Harbor IL to Gary IN, LM
    #[doc(hidden)]
    LMZ743,
    /// Gary to Burns Harbor IN, LM
    #[doc(hidden)]
    LMZ744,
    /// Burns Harbor to Michigan City IN, LM
    #[doc(hidden)]
    LMZ745,
    /// Lake Michigan from Winthrop Harbor to Wilmette Harbor IL 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ777,
    /// Lake Michigan from Wilmette Harbor to Michigan City in 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ779,
    /// St Joseph to South Haven MI, LM
    #[doc(hidden)]
    LMZ844,
    /// South Haven to Holland MI, LM
    #[doc(hidden)]
    LMZ845,
    /// Holland to Grand Haven MI, LM
    #[doc(hidden)]
    LMZ846,
    /// Grand Haven to Whitehall MI, LM
    #[doc(hidden)]
    LMZ847,
    /// Whitehall to Pentwater MI, LM
    #[doc(hidden)]
    LMZ848,
    /// Pentwater to Manistee MI, LM
    #[doc(hidden)]
    LMZ849,
    /// Lake Michigan from Pentwater to Manistee MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ868,
    /// Lake Michigan from Whitehall to Pentwater MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ870,
    /// Lake Michigan from Grand Haven to Whitehall MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ872,
    /// Lake Michigan from Holland to Grand Haven MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ874,
    /// Lake Michigan from South Haven to Holland MI 5NM offshore to Mid lake, LM
    #[doc(hidden)]
    LMZ876,
    /// Lake Michigan from St Joseph to South Haven MI 5NM offshore to Mid Lake, LM
    #[doc(hidden)]
    LMZ878,
    /// Lower Niagara River, LO
    #[doc(hidden)]
    LOZ030,
    /// Niagara River to Hamlin Beach NY, LO
    #[doc(hidden)]
    LOZ042,
    /// Hamlin Beach to Sodus Bay NY, LO
    #[doc(hidden)]
    LOZ043,
    /// Sodus Bay to Mexico Bay NY, LO
    #[doc(hidden)]
    LOZ044,
    /// Mexico Bay NY to the St. Lawrence River, LO
    #[doc(hidden)]
    LOZ045,
    /// Niagara River to Hamlin Beach NY beyond 5NM off shoreline to US-Canadian border, LO
    #[doc(hidden)]
    LOZ062,
    /// Hamlin Beach to Sodus Bay NY beyond 5NM off shoreline to US-Canadian border, LO
    #[doc(hidden)]
    LOZ063,
    /// Sodus Bay to Mexico Bay NY beyond 5NM off shoreline to US-Canadian border, LO
    #[doc(hidden)]
    LOZ064,
    /// Mexico Bay NY to the St. Lawrence River beyond 5NM off shoreline to US-Canadian border, LO
    #[doc(hidden)]
    LOZ065,
    /// Chequamegon Bay-Bayfield to Oak Point WI, LS
    #[doc(hidden)]
    LSZ121,
    /// Grand Portage to Grand Marais MN, LS
    #[doc(hidden)]
    LSZ140,
    /// Grand Marais to Taconite Harbor MN, LS
    #[doc(hidden)]
    LSZ141,
    /// Taconite Harbor to Silver Bay Harbor MN, LS
    #[doc(hidden)]
    LSZ142,
    /// Silver Bay Harbor to Two Harbors MN, LS
    #[doc(hidden)]
    LSZ143,
    /// Two Harbors to Duluth MN, LS
    #[doc(hidden)]
    LSZ144,
    /// Duluth MN to Port Wing WI, LS
    #[doc(hidden)]
    LSZ145,
    /// Port Wing to Sand Island WI, LS
    #[doc(hidden)]
    LSZ146,
    /// Sand Island to Bayfield WI, LS
    #[doc(hidden)]
    LSZ147,
    /// Oak Point to Saxon Harbor WI, LS
    #[doc(hidden)]
    LSZ148,
    /// Outer Apostle Islands Beyond 5 NM from Mainland, LS
    #[doc(hidden)]
    LSZ150,
    /// Lake Superior west of a line from Saxon Harbor WI to Grand Portage MN beyond 5NM, LS
    #[doc(hidden)]
    LSZ162,
    /// Saxon Harbor WI to Black River MI, LS
    #[doc(hidden)]
    LSZ240,
    /// Black River To Ontonagon MI, LS
    #[doc(hidden)]
    LSZ241,
    /// Ontonagon to Upper Entrance of Portage Canal MI, LS
    #[doc(hidden)]
    LSZ242,
    /// Upper Entrance of Portage Canal to Eagle River MI, LS
    #[doc(hidden)]
    LSZ243,
    /// Eagle River to Manitou Island MI, LS
    #[doc(hidden)]
    LSZ244,
    /// Manitou Island to Point Isabelle MI, LS
    #[doc(hidden)]
    LSZ245,
    /// Point Isabelle to Lower Entrance of Portage Canal MI, LS
    #[doc(hidden)]
    LSZ246,
    /// Portage Lake to Huron Island MI to Lower Entrance of Portage Canal To Huron Islands MI Including Keweenaw and Huron Bays, LS
    #[doc(hidden)]
    LSZ247,
    /// Huron Islands to Marquette MI, LS
    #[doc(hidden)]
    LSZ248,
    /// Marquette to Munising MI, LS
    #[doc(hidden)]
    LSZ249,
    /// Munising to Grand Marais MI, LS
    #[doc(hidden)]
    LSZ250,
    /// Grand Marais to Whitefish Point MI, LS
    #[doc(hidden)]
    LSZ251,
    /// Lake Superior from Saxon Harbor WI to Upper Entrance to Portage Canal MI 5NM off shore to the US/Canadian border including Isle Royal National Park, LS
    #[doc(hidden)]
    LSZ263,
    /// Lake Superior from Upper Entrance to Portage Canal to Manitou Island MI 5NM off shore to the US/Canadian Border, LS
    #[doc(hidden)]
    LSZ264,
    /// Lake Superior West of Line from Manitou Island to Marquette MI Beyond 5NM from shore, LS
    #[doc(hidden)]
    LSZ265,
    /// Lake Superior East of a line from Manitou Island to Marquette MI and West of a line from Grand Marais MI to the US/Canadian Border Beyond 5NM from shore, LS
    #[doc(hidden)]
    LSZ266,
    /// Lake Superior from Grand Marais MI to Whitefish Point MI 5NM off shore to the US/Canadian border, LS
    #[doc(hidden)]
    LSZ267,
    /// Whitefish Bay (U.S. Portion)/Whitefish Point to Point Iroquois MI, LS
    #[doc(hidden)]
    LSZ321,
    /// St. Marys River Point Iroquois to E. Potagannissing Bay, LS
    #[doc(hidden)]
    LSZ322,
    /// Kauai Northwest Waters, PH
    #[doc(hidden)]
    PHZ110,
    /// Kauai Windward Waters, PH
    #[doc(hidden)]
    PHZ111,
    /// Kauai Leeward Waters, PH
    #[doc(hidden)]
    PHZ112,
    /// Kauai Channel, PH
    #[doc(hidden)]
    PHZ113,
    /// Oahu Windward Waters, PH
    #[doc(hidden)]
    PHZ114,
    /// Oahu Leeward Waters, PH
    #[doc(hidden)]
    PHZ115,
    /// Kaiwi Channel, PH
    #[doc(hidden)]
    PHZ116,
    /// Maui County Windward Waters, PH
    #[doc(hidden)]
    PHZ117,
    /// Maui County Leeward Waters, PH
    #[doc(hidden)]
    PHZ118,
    /// Maalaea Bay, PH
    #[doc(hidden)]
    PHZ119,
    /// Pailolo Channel, PH
    #[doc(hidden)]
    PHZ120,
    /// Alenuihaha Channel, PH
    #[doc(hidden)]
    PHZ121,
    /// Big Island Windward Waters, PH
    #[doc(hidden)]
    PHZ122,
    /// Big Island Leeward Waters, PH
    #[doc(hidden)]
    PHZ123,
    /// Big Island Southeast Waters, PH
    #[doc(hidden)]
    PHZ124,
    /// Glacier Bay, PK
    #[doc(hidden)]
    PKZ011,
    /// Northern Lynn Canal, PK
    #[doc(hidden)]
    PKZ012,
    /// Southern Lynn Canal, PK
    #[doc(hidden)]
    PKZ013,
    /// Icy Strait, PK
    #[doc(hidden)]
    PKZ021,
    /// Cross Sound, PK
    #[doc(hidden)]
    PKZ022,
    /// Stephens Passage, PK
    #[doc(hidden)]
    PKZ031,
    /// Northern Chatham Strait, PK
    #[doc(hidden)]
    PKZ032,
    /// Southern Chatham Strait, PK
    #[doc(hidden)]
    PKZ033,
    /// Frederick Sound, PK
    #[doc(hidden)]
    PKZ034,
    /// Sumner Strait, PK
    #[doc(hidden)]
    PKZ035,
    /// Clarence Strait, PK
    #[doc(hidden)]
    PKZ036,
    /// Dixon Entrance to Cape Decision, PK
    #[doc(hidden)]
    PKZ041,
    /// Cape Decision to Cape Edgecumbe, PK
    #[doc(hidden)]
    PKZ042,
    /// Southeast Alaska Outside Waters From Cape Edgecumbe to Cape Fairweather, PK
    #[doc(hidden)]
    PKZ043,
    /// Cape Fairweather to Icy Cape, PK
    #[doc(hidden)]
    PKZ051,
    /// Icy Cape to Cape Suckling, PK
    #[doc(hidden)]
    PKZ052,
    /// Yakutat Bay, PK
    #[doc(hidden)]
    PKZ053,
    /// Cape Suckling to Cape Cleare, PK
    #[doc(hidden)]
    PKZ119,
    /// Cape Cleare to Gore Point, PK
    #[doc(hidden)]
    PKZ120,
    /// Resurrection Bay, PK
    #[doc(hidden)]
    PKZ121,
    /// Prince William Sound, PK
    #[doc(hidden)]
    PKZ125,
    /// Port of Valdez, PK
    #[doc(hidden)]
    PKZ126,
    /// Valdez Narrows, PK
    #[doc(hidden)]
    PKZ127,
    /// Valdez Arm, PK
    #[doc(hidden)]
    PKZ128,
    /// Western Prince William Sound, PK
    #[doc(hidden)]
    PKZ129,
    /// West of Barren Islands Including Kamishak Bay, PK
    #[doc(hidden)]
    PKZ130,
    /// Barren Islands East, PK
    #[doc(hidden)]
    PKZ131,
    /// Shuyak Island To Sitkinak, PK
    #[doc(hidden)]
    PKZ132,
    /// Chiniak Bay, PK
    #[doc(hidden)]
    PKZ136,
    /// Marmot Bay, PK
    #[doc(hidden)]
    PKZ137,
    /// Shelikof Strait, PK
    #[doc(hidden)]
    PKZ138,
    /// Cook Inlet Kalgin Island to Point Bede, PK
    #[doc(hidden)]
    PKZ139,
    /// Cook Inlet North Kalgin Island, PK
    #[doc(hidden)]
    PKZ140,
    /// Kachemak Bay, PK
    #[doc(hidden)]
    PKZ141,
    /// Sitkinak to Castle Cape, PK
    #[doc(hidden)]
    PKZ150,
    /// Castle Cape to Cape Sarichef, PK
    #[doc(hidden)]
    PKZ155,
    /// Bristol Bay, PK
    #[doc(hidden)]
    PKZ160,
    /// Port Heiden to Cape Sarichef, PK
    #[doc(hidden)]
    PKZ165,
    /// Cape Sarichef to Nikoski Bering Side, PK
    #[doc(hidden)]
    PKZ170,
    /// Unalaska Bay, PK
    #[doc(hidden)]
    PKZ171,
    /// Cape Sarichef to Nikoski Pacific Side, PK
    #[doc(hidden)]
    PKZ172,
    /// Nikolski to Seguam Bering Side, PK
    #[doc(hidden)]
    PKZ173,
    /// Nikolski to Seguam Pacific Side, PK
    #[doc(hidden)]
    PKZ174,
    /// Seguam to Adak Bering Side, PK
    #[doc(hidden)]
    PKZ175,
    /// Seguam to Adak Pacific Side, PK
    #[doc(hidden)]
    PKZ176,
    /// Adak to Kiska, PK
    #[doc(hidden)]
    PKZ177,
    /// Kiska to Attu, PK
    #[doc(hidden)]
    PKZ178,
    /// Pribilof Islands Nearshore Waters, PK
    #[doc(hidden)]
    PKZ179,
    /// Kuskokwim Delta and Etolin Strait, PK
    #[doc(hidden)]
    PKZ180,
    /// North and West of Nunivak Island, PK
    #[doc(hidden)]
    PKZ181,
    /// St Matthew Island Waters, PK
    #[doc(hidden)]
    PKZ185,
    /// Norton Sound, PK
    #[doc(hidden)]
    PKZ200,
    /// Etolin Strait to Dall Point, PK
    #[doc(hidden)]
    PKZ201,
    /// Dall Point to Wales, PK
    #[doc(hidden)]
    PKZ210,
    /// Kotzebue Sound, PK
    #[doc(hidden)]
    PKZ215,
    /// Wales to Cape Thompson, PK
    #[doc(hidden)]
    PKZ220,
    /// Cape Thompson to Cape Beaufort, PK
    #[doc(hidden)]
    PKZ225,
    /// Cape Beaufort to Point Franklin, PK
    #[doc(hidden)]
    PKZ230,
    /// Point Franklin to Cape Halkett, PK
    #[doc(hidden)]
    PKZ235,
    /// Cape Halkett to Flaxman Island, PK
    #[doc(hidden)]
    PKZ240,
    /// Flaxman Island to Demarcation Point, PK
    #[doc(hidden)]
    PKZ245,
    /// Guam Coastal Waters, PM
    #[doc(hidden)]
    PMZ151,
    /// Rota Coastal Waters, PM
    #[doc(hidden)]
    PMZ152,
    /// Tinian Coastal Waters, PM
    #[doc(hidden)]
    PMZ153,
    /// Saipan Coastal Waters, PM
    #[doc(hidden)]
    PMZ154,
    /// Koror Palau Coastal Waters, PM
    #[doc(hidden)]
    PMZ161,
    /// Yap Coastal Waters, PM
    #[doc(hidden)]
    PMZ171,
    /// Chuuk Coastal Waters, PM
    #[doc(hidden)]
    PMZ172,
    /// Pohnpei Coastal Waters, PM
    #[doc(hidden)]
    PMZ173,
    /// Kosrae Coastal Waters, PM
    #[doc(hidden)]
    PMZ174,
    /// Majuro Coastal Waters, PM
    #[doc(hidden)]
    PMZ181,
    /// Waters out to 40 Nautical Miles, PM
    #[doc(hidden)]
    PMZ191,
    /// Coastal waters of Tututila and Aunuu, PS
    #[doc(hidden)]
    PSZ150,
    /// Coastal waters of Manua, PS
    #[doc(hidden)]
    PSZ151,
    /// Coastal waters of Swain's Island, PS
    #[doc(hidden)]
    PSZ152,
    /// Grays Harbor Bar, PZ
    #[doc(hidden)]
    PZZ110,
    /// West Entrance U.S. Waters Strait Of Juan De Fuca, PZ
    #[doc(hidden)]
    PZZ130,
    /// Central U.S. Waters Strait Of Juan De Fuca, PZ
    #[doc(hidden)]
    PZZ131,
    /// East Entrance U.S. Waters Strait Of Juan De Fuca, PZ
    #[doc(hidden)]
    PZZ132,
    /// Northern Inland Waters Including The San Juan Islands, PZ
    #[doc(hidden)]
    PZZ133,
    /// Admiralty Inlet, PZ
    #[doc(hidden)]
    PZZ134,
    /// Puget Sound and Hood Canal, PZ
    #[doc(hidden)]
    PZZ135,
    /// Coastal Waters From Cape Flattery To James Island Out 10 Nm, PZ
    #[doc(hidden)]
    PZZ150,
    /// Coastal Waters From James Island To Point Grenville Out 10 Nm, PZ
    #[doc(hidden)]
    PZZ153,
    /// Coastal Waters From Point Grenville To Cape Shoalwater Out 10 Nm, PZ
    #[doc(hidden)]
    PZZ156,
    /// Coastal Waters From Cape Flattery To James Island 10 To 60 Nm, PZ
    #[doc(hidden)]
    PZZ170,
    /// Waters From James Island To Point Grenville 10 To 60 Nm, PZ
    #[doc(hidden)]
    PZZ173,
    /// Coastal Waters From Point Grenville To Cape Shoalwater 10 To 60 Nm, PZ
    #[doc(hidden)]
    PZZ176,
    /// Columbia River Bar, PZ
    #[doc(hidden)]
    PZZ210,
    /// Coastal waters from Cape Shoalwater WA to Cascade Head OR out 10 nm, PZ
    #[doc(hidden)]
    PZZ250,
    /// Coastal waters from Cascade Head to Florence OR out 10 nm, PZ
    #[doc(hidden)]
    PZZ255,
    /// Waters from Cape Shoalwater WA to Cascade Head OR from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ270,
    /// Waters from Cascade Head to Florence OR from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ275,
    /// Coastal waters from Florence to Cape Blanco OR out 10 nm, PZ
    #[doc(hidden)]
    PZZ350,
    /// Coastal waters from Cape Blanco OR to Pt. St. George CA out 10 nm, PZ
    #[doc(hidden)]
    PZZ356,
    /// Waters from Florence to Cape Blanco OR from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ370,
    /// Waters from Cape Blanco OR to Pt. St. George CA from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ376,
    /// Humboldt Bay Bar, PZ
    #[doc(hidden)]
    PZZ410,
    /// Humboldt Bay, PZ
    #[doc(hidden)]
    PZZ415,
    /// Coastal waters from Pt. St. George to Cape Mendocino CA out 10 nm, PZ
    #[doc(hidden)]
    PZZ450,
    /// Coastal waters from Cape Mendocino to Pt. Arena CA out 10 nm, PZ
    #[doc(hidden)]
    PZZ455,
    /// Waters from Pt. St. George to Cape Mendocino CA from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ470,
    /// Waters from Cape Mendocino to Pt. Arena CA from 10 to 60 nm, PZ
    #[doc(hidden)]
    PZZ475,
    /// San Pablo Bay, Suisun Bay, the West Delta and  the San Francisco Bay north of the Bay Bridge, PZ
    #[doc(hidden)]
    PZZ530,
    /// San Francisco Bay South of the Bay Bridge, PZ
    #[doc(hidden)]
    PZZ531,
    /// Monterey Bay, PZ
    #[doc(hidden)]
    PZZ535,
    /// Coastal Waters from Point Arena to Point Reyes California out to 10 nm, PZ
    #[doc(hidden)]
    PZZ540,
    /// Coastal Waters from Point Reyes to Pigeon Point California out to 10 nm, PZ
    #[doc(hidden)]
    PZZ545,
    /// Coastal Waters from Pigeon Point to Point Pinos California out to 10 nm, PZ
    #[doc(hidden)]
    PZZ560,
    /// Coastal Waters from Point Pinos to Point Piedras Blancas California out to 10 nm, PZ
    #[doc(hidden)]
    PZZ565,
    /// Waters from Point Arena to Point Reyes 10-60 NM, PZ
    #[doc(hidden)]
    PZZ570,
    /// Waters from Point Reyes to Pigeon Point 10-60 NM, PZ
    #[doc(hidden)]
    PZZ571,
    /// Waters from Pigeon Point to Point Pinos 10-60 NM, PZ
    #[doc(hidden)]
    PZZ575,
    /// Waters from Point Pinos to Point Piedras Blancas 10-60 NM, PZ
    #[doc(hidden)]
    PZZ576,
    /// Point Piedras Blancas to Point Sal westward out to 10 NM, PZ
    #[doc(hidden)]
    PZZ645,
    /// East Santa Barbara Channel from Pt. Conception to Pt. Mugu CA including Santa Cruz Island, PZ
    #[doc(hidden)]
    PZZ650,
    /// Inner waters from Point Mugu to San Mateo Pt. CA including Santa Catalina and Anacapa Islands, PZ
    #[doc(hidden)]
    PZZ655,
    /// Point Piedras Blancas to Point Sal from 10 to 60 NM, PZ
    #[doc(hidden)]
    PZZ670,
    /// Waters from Pt. Sal to Santa Cruz Island CA and westward 60 nm including San Miguel and Santa Rosa Islands, PZ
    #[doc(hidden)]
    PZZ673,
    /// Outer waters from Santa Cruz Island to San Clemente Island to 60 NM offshore including San Nicolas and Santa Barbara Islands, PZ
    #[doc(hidden)]
    PZZ676,
    /// Coastal Waters from San Mateo Point to the Mexican Border and out to 30 nm, PZ
    #[doc(hidden)]
    PZZ750,
    /// Waters from San Mateo point to the Mexican Border Extending 30 to 60 nm out including San Clemente Island, PZ
    #[doc(hidden)]
    PZZ775,
    /// St. Lawrence River above Ogdensburg NY, SL
    #[doc(hidden)]
    SLZ022,
    /// St. Lawrence River from Ogdensburg to St. Regis NY, SL
    #[doc(hidden)]
    SLZ024,
}
impl ::std::str::FromStr for CoastalMarineZone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "AMZ131" => Ok(CoastalMarineZone::AMZ131),
            "AMZ135" => Ok(CoastalMarineZone::AMZ135),
            "AMZ136" => Ok(CoastalMarineZone::AMZ136),
            "AMZ137" => Ok(CoastalMarineZone::AMZ137),
            "AMZ150" => Ok(CoastalMarineZone::AMZ150),
            "AMZ152" => Ok(CoastalMarineZone::AMZ152),
            "AMZ154" => Ok(CoastalMarineZone::AMZ154),
            "AMZ156" => Ok(CoastalMarineZone::AMZ156),
            "AMZ158" => Ok(CoastalMarineZone::AMZ158),
            "AMZ170" => Ok(CoastalMarineZone::AMZ170),
            "AMZ172" => Ok(CoastalMarineZone::AMZ172),
            "AMZ174" => Ok(CoastalMarineZone::AMZ174),
            "AMZ176" => Ok(CoastalMarineZone::AMZ176),
            "AMZ178" => Ok(CoastalMarineZone::AMZ178),
            "AMZ230" => Ok(CoastalMarineZone::AMZ230),
            "AMZ231" => Ok(CoastalMarineZone::AMZ231),
            "AMZ250" => Ok(CoastalMarineZone::AMZ250),
            "AMZ252" => Ok(CoastalMarineZone::AMZ252),
            "AMZ254" => Ok(CoastalMarineZone::AMZ254),
            "AMZ256" => Ok(CoastalMarineZone::AMZ256),
            "AMZ270" => Ok(CoastalMarineZone::AMZ270),
            "AMZ272" => Ok(CoastalMarineZone::AMZ272),
            "AMZ274" => Ok(CoastalMarineZone::AMZ274),
            "AMZ276" => Ok(CoastalMarineZone::AMZ276),
            "AMZ330" => Ok(CoastalMarineZone::AMZ330),
            "AMZ350" => Ok(CoastalMarineZone::AMZ350),
            "AMZ352" => Ok(CoastalMarineZone::AMZ352),
            "AMZ354" => Ok(CoastalMarineZone::AMZ354),
            "AMZ370" => Ok(CoastalMarineZone::AMZ370),
            "AMZ372" => Ok(CoastalMarineZone::AMZ372),
            "AMZ374" => Ok(CoastalMarineZone::AMZ374),
            "AMZ450" => Ok(CoastalMarineZone::AMZ450),
            "AMZ452" => Ok(CoastalMarineZone::AMZ452),
            "AMZ454" => Ok(CoastalMarineZone::AMZ454),
            "AMZ470" => Ok(CoastalMarineZone::AMZ470),
            "AMZ472" => Ok(CoastalMarineZone::AMZ472),
            "AMZ474" => Ok(CoastalMarineZone::AMZ474),
            "AMZ550" => Ok(CoastalMarineZone::AMZ550),
            "AMZ552" => Ok(CoastalMarineZone::AMZ552),
            "AMZ555" => Ok(CoastalMarineZone::AMZ555),
            "AMZ570" => Ok(CoastalMarineZone::AMZ570),
            "AMZ572" => Ok(CoastalMarineZone::AMZ572),
            "AMZ575" => Ok(CoastalMarineZone::AMZ575),
            "AMZ610" => Ok(CoastalMarineZone::AMZ610),
            "AMZ630" => Ok(CoastalMarineZone::AMZ630),
            "AMZ650" => Ok(CoastalMarineZone::AMZ650),
            "AMZ651" => Ok(CoastalMarineZone::AMZ651),
            "AMZ670" => Ok(CoastalMarineZone::AMZ670),
            "AMZ671" => Ok(CoastalMarineZone::AMZ671),
            "AMZ710" => Ok(CoastalMarineZone::AMZ710),
            "AMZ712" => Ok(CoastalMarineZone::AMZ712),
            "AMZ715" => Ok(CoastalMarineZone::AMZ715),
            "AMZ722" => Ok(CoastalMarineZone::AMZ722),
            "AMZ725" => Ok(CoastalMarineZone::AMZ725),
            "AMZ732" => Ok(CoastalMarineZone::AMZ732),
            "AMZ735" => Ok(CoastalMarineZone::AMZ735),
            "AMZ741" => Ok(CoastalMarineZone::AMZ741),
            "AMZ742" => Ok(CoastalMarineZone::AMZ742),
            "AMZ745" => Ok(CoastalMarineZone::AMZ745),
            "ANZ050" => Ok(CoastalMarineZone::ANZ050),
            "ANZ051" => Ok(CoastalMarineZone::ANZ051),
            "ANZ052" => Ok(CoastalMarineZone::ANZ052),
            "ANZ070" => Ok(CoastalMarineZone::ANZ070),
            "ANZ071" => Ok(CoastalMarineZone::ANZ071),
            "ANZ150" => Ok(CoastalMarineZone::ANZ150),
            "ANZ151" => Ok(CoastalMarineZone::ANZ151),
            "ANZ152" => Ok(CoastalMarineZone::ANZ152),
            "ANZ153" => Ok(CoastalMarineZone::ANZ153),
            "ANZ154" => Ok(CoastalMarineZone::ANZ154),
            "ANZ170" => Ok(CoastalMarineZone::ANZ170),
            "ANZ172" => Ok(CoastalMarineZone::ANZ172),
            "ANZ174" => Ok(CoastalMarineZone::ANZ174),
            "ANZ230" => Ok(CoastalMarineZone::ANZ230),
            "ANZ231" => Ok(CoastalMarineZone::ANZ231),
            "ANZ232" => Ok(CoastalMarineZone::ANZ232),
            "ANZ233" => Ok(CoastalMarineZone::ANZ233),
            "ANZ234" => Ok(CoastalMarineZone::ANZ234),
            "ANZ235" => Ok(CoastalMarineZone::ANZ235),
            "ANZ236" => Ok(CoastalMarineZone::ANZ236),
            "ANZ237" => Ok(CoastalMarineZone::ANZ237),
            "ANZ250" => Ok(CoastalMarineZone::ANZ250),
            "ANZ251" => Ok(CoastalMarineZone::ANZ251),
            "ANZ254" => Ok(CoastalMarineZone::ANZ254),
            "ANZ255" => Ok(CoastalMarineZone::ANZ255),
            "ANZ256" => Ok(CoastalMarineZone::ANZ256),
            "ANZ270" => Ok(CoastalMarineZone::ANZ270),
            "ANZ271" => Ok(CoastalMarineZone::ANZ271),
            "ANZ272" => Ok(CoastalMarineZone::ANZ272),
            "ANZ273" => Ok(CoastalMarineZone::ANZ273),
            "ANZ331" => Ok(CoastalMarineZone::ANZ331),
            "ANZ332" => Ok(CoastalMarineZone::ANZ332),
            "ANZ335" => Ok(CoastalMarineZone::ANZ335),
            "ANZ338" => Ok(CoastalMarineZone::ANZ338),
            "ANZ340" => Ok(CoastalMarineZone::ANZ340),
            "ANZ345" => Ok(CoastalMarineZone::ANZ345),
            "ANZ350" => Ok(CoastalMarineZone::ANZ350),
            "ANZ353" => Ok(CoastalMarineZone::ANZ353),
            "ANZ355" => Ok(CoastalMarineZone::ANZ355),
            "ANZ370" => Ok(CoastalMarineZone::ANZ370),
            "ANZ373" => Ok(CoastalMarineZone::ANZ373),
            "ANZ375" => Ok(CoastalMarineZone::ANZ375),
            "ANZ430" => Ok(CoastalMarineZone::ANZ430),
            "ANZ431" => Ok(CoastalMarineZone::ANZ431),
            "ANZ450" => Ok(CoastalMarineZone::ANZ450),
            "ANZ451" => Ok(CoastalMarineZone::ANZ451),
            "ANZ452" => Ok(CoastalMarineZone::ANZ452),
            "ANZ453" => Ok(CoastalMarineZone::ANZ453),
            "ANZ454" => Ok(CoastalMarineZone::ANZ454),
            "ANZ455" => Ok(CoastalMarineZone::ANZ455),
            "ANZ470" => Ok(CoastalMarineZone::ANZ470),
            "ANZ471" => Ok(CoastalMarineZone::ANZ471),
            "ANZ472" => Ok(CoastalMarineZone::ANZ472),
            "ANZ473" => Ok(CoastalMarineZone::ANZ473),
            "ANZ475" => Ok(CoastalMarineZone::ANZ475),
            "ANZ530" => Ok(CoastalMarineZone::ANZ530),
            "ANZ531" => Ok(CoastalMarineZone::ANZ531),
            "ANZ532" => Ok(CoastalMarineZone::ANZ532),
            "ANZ533" => Ok(CoastalMarineZone::ANZ533),
            "ANZ534" => Ok(CoastalMarineZone::ANZ534),
            "ANZ535" => Ok(CoastalMarineZone::ANZ535),
            "ANZ536" => Ok(CoastalMarineZone::ANZ536),
            "ANZ537" => Ok(CoastalMarineZone::ANZ537),
            "ANZ538" => Ok(CoastalMarineZone::ANZ538),
            "ANZ539" => Ok(CoastalMarineZone::ANZ539),
            "ANZ540" => Ok(CoastalMarineZone::ANZ540),
            "ANZ541" => Ok(CoastalMarineZone::ANZ541),
            "ANZ542" => Ok(CoastalMarineZone::ANZ542),
            "ANZ543" => Ok(CoastalMarineZone::ANZ543),
            "ANZ630" => Ok(CoastalMarineZone::ANZ630),
            "ANZ631" => Ok(CoastalMarineZone::ANZ631),
            "ANZ632" => Ok(CoastalMarineZone::ANZ632),
            "ANZ633" => Ok(CoastalMarineZone::ANZ633),
            "ANZ634" => Ok(CoastalMarineZone::ANZ634),
            "ANZ635" => Ok(CoastalMarineZone::ANZ635),
            "ANZ636" => Ok(CoastalMarineZone::ANZ636),
            "ANZ637" => Ok(CoastalMarineZone::ANZ637),
            "ANZ638" => Ok(CoastalMarineZone::ANZ638),
            "ANZ650" => Ok(CoastalMarineZone::ANZ650),
            "ANZ652" => Ok(CoastalMarineZone::ANZ652),
            "ANZ654" => Ok(CoastalMarineZone::ANZ654),
            "ANZ656" => Ok(CoastalMarineZone::ANZ656),
            "ANZ658" => Ok(CoastalMarineZone::ANZ658),
            "ANZ670" => Ok(CoastalMarineZone::ANZ670),
            "ANZ672" => Ok(CoastalMarineZone::ANZ672),
            "ANZ674" => Ok(CoastalMarineZone::ANZ674),
            "ANZ676" => Ok(CoastalMarineZone::ANZ676),
            "ANZ678" => Ok(CoastalMarineZone::ANZ678),
            "GMZ031" => Ok(CoastalMarineZone::GMZ031),
            "GMZ032" => Ok(CoastalMarineZone::GMZ032),
            "GMZ033" => Ok(CoastalMarineZone::GMZ033),
            "GMZ034" => Ok(CoastalMarineZone::GMZ034),
            "GMZ035" => Ok(CoastalMarineZone::GMZ035),
            "GMZ042" => Ok(CoastalMarineZone::GMZ042),
            "GMZ043" => Ok(CoastalMarineZone::GMZ043),
            "GMZ044" => Ok(CoastalMarineZone::GMZ044),
            "GMZ052" => Ok(CoastalMarineZone::GMZ052),
            "GMZ053" => Ok(CoastalMarineZone::GMZ053),
            "GMZ054" => Ok(CoastalMarineZone::GMZ054),
            "GMZ055" => Ok(CoastalMarineZone::GMZ055),
            "GMZ072" => Ok(CoastalMarineZone::GMZ072),
            "GMZ073" => Ok(CoastalMarineZone::GMZ073),
            "GMZ074" => Ok(CoastalMarineZone::GMZ074),
            "GMZ075" => Ok(CoastalMarineZone::GMZ075),
            "GMZ130" => Ok(CoastalMarineZone::GMZ130),
            "GMZ132" => Ok(CoastalMarineZone::GMZ132),
            "GMZ135" => Ok(CoastalMarineZone::GMZ135),
            "GMZ150" => Ok(CoastalMarineZone::GMZ150),
            "GMZ155" => Ok(CoastalMarineZone::GMZ155),
            "GMZ170" => Ok(CoastalMarineZone::GMZ170),
            "GMZ175" => Ok(CoastalMarineZone::GMZ175),
            "GMZ231" => Ok(CoastalMarineZone::GMZ231),
            "GMZ232" => Ok(CoastalMarineZone::GMZ232),
            "GMZ236" => Ok(CoastalMarineZone::GMZ236),
            "GMZ237" => Ok(CoastalMarineZone::GMZ237),
            "GMZ250" => Ok(CoastalMarineZone::GMZ250),
            "GMZ255" => Ok(CoastalMarineZone::GMZ255),
            "GMZ270" => Ok(CoastalMarineZone::GMZ270),
            "GMZ275" => Ok(CoastalMarineZone::GMZ275),
            "GMZ330" => Ok(CoastalMarineZone::GMZ330),
            "GMZ335" => Ok(CoastalMarineZone::GMZ335),
            "GMZ350" => Ok(CoastalMarineZone::GMZ350),
            "GMZ355" => Ok(CoastalMarineZone::GMZ355),
            "GMZ370" => Ok(CoastalMarineZone::GMZ370),
            "GMZ375" => Ok(CoastalMarineZone::GMZ375),
            "GMZ430" => Ok(CoastalMarineZone::GMZ430),
            "GMZ432" => Ok(CoastalMarineZone::GMZ432),
            "GMZ435" => Ok(CoastalMarineZone::GMZ435),
            "GMZ450" => Ok(CoastalMarineZone::GMZ450),
            "GMZ452" => Ok(CoastalMarineZone::GMZ452),
            "GMZ455" => Ok(CoastalMarineZone::GMZ455),
            "GMZ470" => Ok(CoastalMarineZone::GMZ470),
            "GMZ472" => Ok(CoastalMarineZone::GMZ472),
            "GMZ475" => Ok(CoastalMarineZone::GMZ475),
            "GMZ530" => Ok(CoastalMarineZone::GMZ530),
            "GMZ532" => Ok(CoastalMarineZone::GMZ532),
            "GMZ534" => Ok(CoastalMarineZone::GMZ534),
            "GMZ536" => Ok(CoastalMarineZone::GMZ536),
            "GMZ538" => Ok(CoastalMarineZone::GMZ538),
            "GMZ550" => Ok(CoastalMarineZone::GMZ550),
            "GMZ552" => Ok(CoastalMarineZone::GMZ552),
            "GMZ555" => Ok(CoastalMarineZone::GMZ555),
            "GMZ557" => Ok(CoastalMarineZone::GMZ557),
            "GMZ570" => Ok(CoastalMarineZone::GMZ570),
            "GMZ572" => Ok(CoastalMarineZone::GMZ572),
            "GMZ575" => Ok(CoastalMarineZone::GMZ575),
            "GMZ577" => Ok(CoastalMarineZone::GMZ577),
            "GMZ630" => Ok(CoastalMarineZone::GMZ630),
            "GMZ631" => Ok(CoastalMarineZone::GMZ631),
            "GMZ632" => Ok(CoastalMarineZone::GMZ632),
            "GMZ633" => Ok(CoastalMarineZone::GMZ633),
            "GMZ634" => Ok(CoastalMarineZone::GMZ634),
            "GMZ635" => Ok(CoastalMarineZone::GMZ635),
            "GMZ636" => Ok(CoastalMarineZone::GMZ636),
            "GMZ650" => Ok(CoastalMarineZone::GMZ650),
            "GMZ655" => Ok(CoastalMarineZone::GMZ655),
            "GMZ656" => Ok(CoastalMarineZone::GMZ656),
            "GMZ657" => Ok(CoastalMarineZone::GMZ657),
            "GMZ670" => Ok(CoastalMarineZone::GMZ670),
            "GMZ675" => Ok(CoastalMarineZone::GMZ675),
            "GMZ676" => Ok(CoastalMarineZone::GMZ676),
            "GMZ730" => Ok(CoastalMarineZone::GMZ730),
            "GMZ750" => Ok(CoastalMarineZone::GMZ750),
            "GMZ752" => Ok(CoastalMarineZone::GMZ752),
            "GMZ755" => Ok(CoastalMarineZone::GMZ755),
            "GMZ765" => Ok(CoastalMarineZone::GMZ765),
            "GMZ770" => Ok(CoastalMarineZone::GMZ770),
            "GMZ772" => Ok(CoastalMarineZone::GMZ772),
            "GMZ775" => Ok(CoastalMarineZone::GMZ775),
            "GMZ830" => Ok(CoastalMarineZone::GMZ830),
            "GMZ836" => Ok(CoastalMarineZone::GMZ836),
            "GMZ850" => Ok(CoastalMarineZone::GMZ850),
            "GMZ853" => Ok(CoastalMarineZone::GMZ853),
            "GMZ856" => Ok(CoastalMarineZone::GMZ856),
            "GMZ870" => Ok(CoastalMarineZone::GMZ870),
            "GMZ873" => Ok(CoastalMarineZone::GMZ873),
            "GMZ876" => Ok(CoastalMarineZone::GMZ876),
            "LCZ422" => Ok(CoastalMarineZone::LCZ422),
            "LCZ423" => Ok(CoastalMarineZone::LCZ423),
            "LCZ460" => Ok(CoastalMarineZone::LCZ460),
            "LEZ020" => Ok(CoastalMarineZone::LEZ020),
            "LEZ040" => Ok(CoastalMarineZone::LEZ040),
            "LEZ041" => Ok(CoastalMarineZone::LEZ041),
            "LEZ061" => Ok(CoastalMarineZone::LEZ061),
            "LEZ142" => Ok(CoastalMarineZone::LEZ142),
            "LEZ143" => Ok(CoastalMarineZone::LEZ143),
            "LEZ144" => Ok(CoastalMarineZone::LEZ144),
            "LEZ145" => Ok(CoastalMarineZone::LEZ145),
            "LEZ146" => Ok(CoastalMarineZone::LEZ146),
            "LEZ147" => Ok(CoastalMarineZone::LEZ147),
            "LEZ148" => Ok(CoastalMarineZone::LEZ148),
            "LEZ149" => Ok(CoastalMarineZone::LEZ149),
            "LEZ162" => Ok(CoastalMarineZone::LEZ162),
            "LEZ163" => Ok(CoastalMarineZone::LEZ163),
            "LEZ164" => Ok(CoastalMarineZone::LEZ164),
            "LEZ165" => Ok(CoastalMarineZone::LEZ165),
            "LEZ166" => Ok(CoastalMarineZone::LEZ166),
            "LEZ167" => Ok(CoastalMarineZone::LEZ167),
            "LEZ168" => Ok(CoastalMarineZone::LEZ168),
            "LEZ169" => Ok(CoastalMarineZone::LEZ169),
            "LEZ444" => Ok(CoastalMarineZone::LEZ444),
            "LHZ345" => Ok(CoastalMarineZone::LHZ345),
            "LHZ346" => Ok(CoastalMarineZone::LHZ346),
            "LHZ347" => Ok(CoastalMarineZone::LHZ347),
            "LHZ348" => Ok(CoastalMarineZone::LHZ348),
            "LHZ349" => Ok(CoastalMarineZone::LHZ349),
            "LHZ361" => Ok(CoastalMarineZone::LHZ361),
            "LHZ362" => Ok(CoastalMarineZone::LHZ362),
            "LHZ363" => Ok(CoastalMarineZone::LHZ363),
            "LHZ421" => Ok(CoastalMarineZone::LHZ421),
            "LHZ422" => Ok(CoastalMarineZone::LHZ422),
            "LHZ441" => Ok(CoastalMarineZone::LHZ441),
            "LHZ442" => Ok(CoastalMarineZone::LHZ442),
            "LHZ443" => Ok(CoastalMarineZone::LHZ443),
            "LHZ462" => Ok(CoastalMarineZone::LHZ462),
            "LHZ463" => Ok(CoastalMarineZone::LHZ463),
            "LHZ464" => Ok(CoastalMarineZone::LHZ464),
            "LMZ043" => Ok(CoastalMarineZone::LMZ043),
            "LMZ046" => Ok(CoastalMarineZone::LMZ046),
            "LMZ080" => Ok(CoastalMarineZone::LMZ080),
            "LMZ221" => Ok(CoastalMarineZone::LMZ221),
            "LMZ248" => Ok(CoastalMarineZone::LMZ248),
            "LMZ250" => Ok(CoastalMarineZone::LMZ250),
            "LMZ261" => Ok(CoastalMarineZone::LMZ261),
            "LMZ323" => Ok(CoastalMarineZone::LMZ323),
            "LMZ341" => Ok(CoastalMarineZone::LMZ341),
            "LMZ342" => Ok(CoastalMarineZone::LMZ342),
            "LMZ344" => Ok(CoastalMarineZone::LMZ344),
            "LMZ345" => Ok(CoastalMarineZone::LMZ345),
            "LMZ346" => Ok(CoastalMarineZone::LMZ346),
            "LMZ362" => Ok(CoastalMarineZone::LMZ362),
            "LMZ364" => Ok(CoastalMarineZone::LMZ364),
            "LMZ366" => Ok(CoastalMarineZone::LMZ366),
            "LMZ521" => Ok(CoastalMarineZone::LMZ521),
            "LMZ522" => Ok(CoastalMarineZone::LMZ522),
            "LMZ541" => Ok(CoastalMarineZone::LMZ541),
            "LMZ542" => Ok(CoastalMarineZone::LMZ542),
            "LMZ543" => Ok(CoastalMarineZone::LMZ543),
            "LMZ563" => Ok(CoastalMarineZone::LMZ563),
            "LMZ565" => Ok(CoastalMarineZone::LMZ565),
            "LMZ567" => Ok(CoastalMarineZone::LMZ567),
            "LMZ643" => Ok(CoastalMarineZone::LMZ643),
            "LMZ644" => Ok(CoastalMarineZone::LMZ644),
            "LMZ645" => Ok(CoastalMarineZone::LMZ645),
            "LMZ646" => Ok(CoastalMarineZone::LMZ646),
            "LMZ669" => Ok(CoastalMarineZone::LMZ669),
            "LMZ671" => Ok(CoastalMarineZone::LMZ671),
            "LMZ673" => Ok(CoastalMarineZone::LMZ673),
            "LMZ675" => Ok(CoastalMarineZone::LMZ675),
            "LMZ740" => Ok(CoastalMarineZone::LMZ740),
            "LMZ741" => Ok(CoastalMarineZone::LMZ741),
            "LMZ742" => Ok(CoastalMarineZone::LMZ742),
            "LMZ743" => Ok(CoastalMarineZone::LMZ743),
            "LMZ744" => Ok(CoastalMarineZone::LMZ744),
            "LMZ745" => Ok(CoastalMarineZone::LMZ745),
            "LMZ777" => Ok(CoastalMarineZone::LMZ777),
            "LMZ779" => Ok(CoastalMarineZone::LMZ779),
            "LMZ844" => Ok(CoastalMarineZone::LMZ844),
            "LMZ845" => Ok(CoastalMarineZone::LMZ845),
            "LMZ846" => Ok(CoastalMarineZone::LMZ846),
            "LMZ847" => Ok(CoastalMarineZone::LMZ847),
            "LMZ848" => Ok(CoastalMarineZone::LMZ848),
            "LMZ849" => Ok(CoastalMarineZone::LMZ849),
            "LMZ868" => Ok(CoastalMarineZone::LMZ868),
            "LMZ870" => Ok(CoastalMarineZone::LMZ870),
            "LMZ872" => Ok(CoastalMarineZone::LMZ872),
            "LMZ874" => Ok(CoastalMarineZone::LMZ874),
            "LMZ876" => Ok(CoastalMarineZone::LMZ876),
            "LMZ878" => Ok(CoastalMarineZone::LMZ878),
            "LOZ030" => Ok(CoastalMarineZone::LOZ030),
            "LOZ042" => Ok(CoastalMarineZone::LOZ042),
            "LOZ043" => Ok(CoastalMarineZone::LOZ043),
            "LOZ044" => Ok(CoastalMarineZone::LOZ044),
            "LOZ045" => Ok(CoastalMarineZone::LOZ045),
            "LOZ062" => Ok(CoastalMarineZone::LOZ062),
            "LOZ063" => Ok(CoastalMarineZone::LOZ063),
            "LOZ064" => Ok(CoastalMarineZone::LOZ064),
            "LOZ065" => Ok(CoastalMarineZone::LOZ065),
            "LSZ121" => Ok(CoastalMarineZone::LSZ121),
            "LSZ140" => Ok(CoastalMarineZone::LSZ140),
            "LSZ141" => Ok(CoastalMarineZone::LSZ141),
            "LSZ142" => Ok(CoastalMarineZone::LSZ142),
            "LSZ143" => Ok(CoastalMarineZone::LSZ143),
            "LSZ144" => Ok(CoastalMarineZone::LSZ144),
            "LSZ145" => Ok(CoastalMarineZone::LSZ145),
            "LSZ146" => Ok(CoastalMarineZone::LSZ146),
            "LSZ147" => Ok(CoastalMarineZone::LSZ147),
            "LSZ148" => Ok(CoastalMarineZone::LSZ148),
            "LSZ150" => Ok(CoastalMarineZone::LSZ150),
            "LSZ162" => Ok(CoastalMarineZone::LSZ162),
            "LSZ240" => Ok(CoastalMarineZone::LSZ240),
            "LSZ241" => Ok(CoastalMarineZone::LSZ241),
            "LSZ242" => Ok(CoastalMarineZone::LSZ242),
            "LSZ243" => Ok(CoastalMarineZone::LSZ243),
            "LSZ244" => Ok(CoastalMarineZone::LSZ244),
            "LSZ245" => Ok(CoastalMarineZone::LSZ245),
            "LSZ246" => Ok(CoastalMarineZone::LSZ246),
            "LSZ247" => Ok(CoastalMarineZone::LSZ247),
            "LSZ248" => Ok(CoastalMarineZone::LSZ248),
            "LSZ249" => Ok(CoastalMarineZone::LSZ249),
            "LSZ250" => Ok(CoastalMarineZone::LSZ250),
            "LSZ251" => Ok(CoastalMarineZone::LSZ251),
            "LSZ263" => Ok(CoastalMarineZone::LSZ263),
            "LSZ264" => Ok(CoastalMarineZone::LSZ264),
            "LSZ265" => Ok(CoastalMarineZone::LSZ265),
            "LSZ266" => Ok(CoastalMarineZone::LSZ266),
            "LSZ267" => Ok(CoastalMarineZone::LSZ267),
            "LSZ321" => Ok(CoastalMarineZone::LSZ321),
            "LSZ322" => Ok(CoastalMarineZone::LSZ322),
            "PHZ110" => Ok(CoastalMarineZone::PHZ110),
            "PHZ111" => Ok(CoastalMarineZone::PHZ111),
            "PHZ112" => Ok(CoastalMarineZone::PHZ112),
            "PHZ113" => Ok(CoastalMarineZone::PHZ113),
            "PHZ114" => Ok(CoastalMarineZone::PHZ114),
            "PHZ115" => Ok(CoastalMarineZone::PHZ115),
            "PHZ116" => Ok(CoastalMarineZone::PHZ116),
            "PHZ117" => Ok(CoastalMarineZone::PHZ117),
            "PHZ118" => Ok(CoastalMarineZone::PHZ118),
            "PHZ119" => Ok(CoastalMarineZone::PHZ119),
            "PHZ120" => Ok(CoastalMarineZone::PHZ120),
            "PHZ121" => Ok(CoastalMarineZone::PHZ121),
            "PHZ122" => Ok(CoastalMarineZone::PHZ122),
            "PHZ123" => Ok(CoastalMarineZone::PHZ123),
            "PHZ124" => Ok(CoastalMarineZone::PHZ124),
            "PKZ011" => Ok(CoastalMarineZone::PKZ011),
            "PKZ012" => Ok(CoastalMarineZone::PKZ012),
            "PKZ013" => Ok(CoastalMarineZone::PKZ013),
            "PKZ021" => Ok(CoastalMarineZone::PKZ021),
            "PKZ022" => Ok(CoastalMarineZone::PKZ022),
            "PKZ031" => Ok(CoastalMarineZone::PKZ031),
            "PKZ032" => Ok(CoastalMarineZone::PKZ032),
            "PKZ033" => Ok(CoastalMarineZone::PKZ033),
            "PKZ034" => Ok(CoastalMarineZone::PKZ034),
            "PKZ035" => Ok(CoastalMarineZone::PKZ035),
            "PKZ036" => Ok(CoastalMarineZone::PKZ036),
            "PKZ041" => Ok(CoastalMarineZone::PKZ041),
            "PKZ042" => Ok(CoastalMarineZone::PKZ042),
            "PKZ043" => Ok(CoastalMarineZone::PKZ043),
            "PKZ051" => Ok(CoastalMarineZone::PKZ051),
            "PKZ052" => Ok(CoastalMarineZone::PKZ052),
            "PKZ053" => Ok(CoastalMarineZone::PKZ053),
            "PKZ119" => Ok(CoastalMarineZone::PKZ119),
            "PKZ120" => Ok(CoastalMarineZone::PKZ120),
            "PKZ121" => Ok(CoastalMarineZone::PKZ121),
            "PKZ125" => Ok(CoastalMarineZone::PKZ125),
            "PKZ126" => Ok(CoastalMarineZone::PKZ126),
            "PKZ127" => Ok(CoastalMarineZone::PKZ127),
            "PKZ128" => Ok(CoastalMarineZone::PKZ128),
            "PKZ129" => Ok(CoastalMarineZone::PKZ129),
            "PKZ130" => Ok(CoastalMarineZone::PKZ130),
            "PKZ131" => Ok(CoastalMarineZone::PKZ131),
            "PKZ132" => Ok(CoastalMarineZone::PKZ132),
            "PKZ136" => Ok(CoastalMarineZone::PKZ136),
            "PKZ137" => Ok(CoastalMarineZone::PKZ137),
            "PKZ138" => Ok(CoastalMarineZone::PKZ138),
            "PKZ139" => Ok(CoastalMarineZone::PKZ139),
            "PKZ140" => Ok(CoastalMarineZone::PKZ140),
            "PKZ141" => Ok(CoastalMarineZone::PKZ141),
            "PKZ150" => Ok(CoastalMarineZone::PKZ150),
            "PKZ155" => Ok(CoastalMarineZone::PKZ155),
            "PKZ160" => Ok(CoastalMarineZone::PKZ160),
            "PKZ165" => Ok(CoastalMarineZone::PKZ165),
            "PKZ170" => Ok(CoastalMarineZone::PKZ170),
            "PKZ171" => Ok(CoastalMarineZone::PKZ171),
            "PKZ172" => Ok(CoastalMarineZone::PKZ172),
            "PKZ173" => Ok(CoastalMarineZone::PKZ173),
            "PKZ174" => Ok(CoastalMarineZone::PKZ174),
            "PKZ175" => Ok(CoastalMarineZone::PKZ175),
            "PKZ176" => Ok(CoastalMarineZone::PKZ176),
            "PKZ177" => Ok(CoastalMarineZone::PKZ177),
            "PKZ178" => Ok(CoastalMarineZone::PKZ178),
            "PKZ179" => Ok(CoastalMarineZone::PKZ179),
            "PKZ180" => Ok(CoastalMarineZone::PKZ180),
            "PKZ181" => Ok(CoastalMarineZone::PKZ181),
            "PKZ185" => Ok(CoastalMarineZone::PKZ185),
            "PKZ200" => Ok(CoastalMarineZone::PKZ200),
            "PKZ201" => Ok(CoastalMarineZone::PKZ201),
            "PKZ210" => Ok(CoastalMarineZone::PKZ210),
            "PKZ215" => Ok(CoastalMarineZone::PKZ215),
            "PKZ220" => Ok(CoastalMarineZone::PKZ220),
            "PKZ225" => Ok(CoastalMarineZone::PKZ225),
            "PKZ230" => Ok(CoastalMarineZone::PKZ230),
            "PKZ235" => Ok(CoastalMarineZone::PKZ235),
            "PKZ240" => Ok(CoastalMarineZone::PKZ240),
            "PKZ245" => Ok(CoastalMarineZone::PKZ245),
            "PMZ151" => Ok(CoastalMarineZone::PMZ151),
            "PMZ152" => Ok(CoastalMarineZone::PMZ152),
            "PMZ153" => Ok(CoastalMarineZone::PMZ153),
            "PMZ154" => Ok(CoastalMarineZone::PMZ154),
            "PMZ161" => Ok(CoastalMarineZone::PMZ161),
            "PMZ171" => Ok(CoastalMarineZone::PMZ171),
            "PMZ172" => Ok(CoastalMarineZone::PMZ172),
            "PMZ173" => Ok(CoastalMarineZone::PMZ173),
            "PMZ174" => Ok(CoastalMarineZone::PMZ174),
            "PMZ181" => Ok(CoastalMarineZone::PMZ181),
            "PMZ191" => Ok(CoastalMarineZone::PMZ191),
            "PSZ150" => Ok(CoastalMarineZone::PSZ150),
            "PSZ151" => Ok(CoastalMarineZone::PSZ151),
            "PSZ152" => Ok(CoastalMarineZone::PSZ152),
            "PZZ110" => Ok(CoastalMarineZone::PZZ110),
            "PZZ130" => Ok(CoastalMarineZone::PZZ130),
            "PZZ131" => Ok(CoastalMarineZone::PZZ131),
            "PZZ132" => Ok(CoastalMarineZone::PZZ132),
            "PZZ133" => Ok(CoastalMarineZone::PZZ133),
            "PZZ134" => Ok(CoastalMarineZone::PZZ134),
            "PZZ135" => Ok(CoastalMarineZone::PZZ135),
            "PZZ150" => Ok(CoastalMarineZone::PZZ150),
            "PZZ153" => Ok(CoastalMarineZone::PZZ153),
            "PZZ156" => Ok(CoastalMarineZone::PZZ156),
            "PZZ170" => Ok(CoastalMarineZone::PZZ170),
            "PZZ173" => Ok(CoastalMarineZone::PZZ173),
            "PZZ176" => Ok(CoastalMarineZone::PZZ176),
            "PZZ210" => Ok(CoastalMarineZone::PZZ210),
            "PZZ250" => Ok(CoastalMarineZone::PZZ250),
            "PZZ255" => Ok(CoastalMarineZone::PZZ255),
            "PZZ270" => Ok(CoastalMarineZone::PZZ270),
            "PZZ275" => Ok(CoastalMarineZone::PZZ275),
            "PZZ350" => Ok(CoastalMarineZone::PZZ350),
            "PZZ356" => Ok(CoastalMarineZone::PZZ356),
            "PZZ370" => Ok(CoastalMarineZone::PZZ370),
            "PZZ376" => Ok(CoastalMarineZone::PZZ376),
            "PZZ410" => Ok(CoastalMarineZone::PZZ410),
            "PZZ415" => Ok(CoastalMarineZone::PZZ415),
            "PZZ450" => Ok(CoastalMarineZone::PZZ450),
            "PZZ455" => Ok(CoastalMarineZone::PZZ455),
            "PZZ470" => Ok(CoastalMarineZone::PZZ470),
            "PZZ475" => Ok(CoastalMarineZone::PZZ475),
            "PZZ530" => Ok(CoastalMarineZone::PZZ530),
            "PZZ531" => Ok(CoastalMarineZone::PZZ531),
            "PZZ535" => Ok(CoastalMarineZone::PZZ535),
            "PZZ540" => Ok(CoastalMarineZone::PZZ540),
            "PZZ545" => Ok(CoastalMarineZone::PZZ545),
            "PZZ560" => Ok(CoastalMarineZone::PZZ560),
            "PZZ565" => Ok(CoastalMarineZone::PZZ565),
            "PZZ570" => Ok(CoastalMarineZone::PZZ570),
            "PZZ571" => Ok(CoastalMarineZone::PZZ571),
            "PZZ575" => Ok(CoastalMarineZone::PZZ575),
            "PZZ576" => Ok(CoastalMarineZone::PZZ576),
            "PZZ645" => Ok(CoastalMarineZone::PZZ645),
            "PZZ650" => Ok(CoastalMarineZone::PZZ650),
            "PZZ655" => Ok(CoastalMarineZone::PZZ655),
            "PZZ670" => Ok(CoastalMarineZone::PZZ670),
            "PZZ673" => Ok(CoastalMarineZone::PZZ673),
            "PZZ676" => Ok(CoastalMarineZone::PZZ676),
            "PZZ750" => Ok(CoastalMarineZone::PZZ750),
            "PZZ775" => Ok(CoastalMarineZone::PZZ775),
            "SLZ022" => Ok(CoastalMarineZone::SLZ022),
            "SLZ024" => Ok(CoastalMarineZone::SLZ024),
            _ => Err(()),
        }
    }
}
impl CoastalMarineZone {
    pub fn details(&self) -> crate::ZoneDetails {
        match self {
            CoastalMarineZone::AMZ131 => crate::ZoneDetails {
                state: "AM",
                zone: "131",
                zone_numeric: 131,
                name: "Alligator River",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ135 => crate::ZoneDetails {
                state: "AM",
                zone: "135",
                zone_numeric: 135,
                name: "Pamlico Sound",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ136 => crate::ZoneDetails {
                state: "AM",
                zone: "136",
                zone_numeric: 136,
                name: "Pamlico and Pungo Rivers",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ137 => crate::ZoneDetails {
                state: "AM",
                zone: "137",
                zone_numeric: 137,
                name: "Neuse and Bay Rivers",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ150 => crate::ZoneDetails {
                state: "AM",
                zone: "150",
                zone_numeric: 150,
                name: "S of Currituck Beach Light NC to Oregon Inlet NC out to 20 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ152 => crate::ZoneDetails {
                state: "AM",
                zone: "152",
                zone_numeric: 152,
                name: "S of Oregon Inlet NC to Cape Hatteras NC out to 20 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ154 => crate::ZoneDetails {
                state: "AM",
                zone: "154",
                zone_numeric: 154,
                name: "S of Cape Hatteras NC to Ocracoke Inlet NC out to 20 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ156 => crate::ZoneDetails {
                state: "AM",
                zone: "156",
                zone_numeric: 156,
                name: "S of Ocracoke Inlet NC to Cape Lookout NC out to 20 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ158 => crate::ZoneDetails {
                state: "AM",
                zone: "158",
                zone_numeric: 158,
                name: "S of Cape Lookout NC to Surf City NC out to 20 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ170 => crate::ZoneDetails {
                state: "AM",
                zone: "170",
                zone_numeric: 170,
                name: "Waters from Currituck Beach Light to Oregon Inlet NC from 20 to 40 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ172 => crate::ZoneDetails {
                state: "AM",
                zone: "172",
                zone_numeric: 172,
                name: "Waters from Oregon Inlet to Cape Hatteras NC from 20 to 40 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ174 => crate::ZoneDetails {
                state: "AM",
                zone: "174",
                zone_numeric: 174,
                name: "Waters from Cape Hatteras to Ocracoke Inlet NC from 20 to 40 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ176 => crate::ZoneDetails {
                state: "AM",
                zone: "176",
                zone_numeric: 176,
                name: "Waters fromOcracoke Inlet to Cape Lookout NC from 20 to 40 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ178 => crate::ZoneDetails {
                state: "AM",
                zone: "178",
                zone_numeric: 178,
                name: "Waters from Cape Lookout  to Surf City NC from 20 to 40 nm",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ230 => crate::ZoneDetails {
                state: "AM",
                zone: "230",
                zone_numeric: 230,
                name: "Albemarle Sound",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ231 => crate::ZoneDetails {
                state: "AM",
                zone: "231",
                zone_numeric: 231,
                name: "Croatan and Roanoke Sounds",
                wfo: "MHX",
            },
            CoastalMarineZone::AMZ250 => crate::ZoneDetails {
                state: "AM",
                zone: "250",
                zone_numeric: 250,
                name: "Coastal waters from Surf City to Cape Fear NC out 20 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ252 => crate::ZoneDetails {
                state: "AM",
                zone: "252",
                zone_numeric: 252,
                name: "Coastal waters from Cape Fear NC to Little River Inlet SC out 20 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ254 => crate::ZoneDetails {
                state: "AM",
                zone: "254",
                zone_numeric: 254,
                name: "Coastal waters from Little River Inlet to Murrells Inlet SC out 20 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ256 => crate::ZoneDetails {
                state: "AM",
                zone: "256",
                zone_numeric: 256,
                name: "Coastal waters from Murrells Inlet to South Santee River SC out 20 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ270 => crate::ZoneDetails {
                state: "AM",
                zone: "270",
                zone_numeric: 270,
                name: "Waters from Surf City to Cape Fear NC from 20 to 40 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ272 => crate::ZoneDetails {
                state: "AM",
                zone: "272",
                zone_numeric: 272,
                name: "Waters from Cape Fear NC to Little River Inlet SC from 20 to 40 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ274 => crate::ZoneDetails {
                state: "AM",
                zone: "274",
                zone_numeric: 274,
                name: "Waters from Little River Inlet to Murrells Inlet SC from 20 to 40 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ276 => crate::ZoneDetails {
                state: "AM",
                zone: "276",
                zone_numeric: 276,
                name: "Waters from Murrells Inlet NC to South Santee River SC from 20 to 40 nm",
                wfo: "ILM",
            },
            CoastalMarineZone::AMZ330 => crate::ZoneDetails {
                state: "AM",
                zone: "330",
                zone_numeric: 330,
                name: "Charleston Harbor",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ350 => crate::ZoneDetails {
                state: "AM",
                zone: "350",
                zone_numeric: 350,
                name: "Coastal waters from South Santee River to Edisto Beach SC out 20 nm",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ352 => crate::ZoneDetails {
                state: "AM",
                zone: "352",
                zone_numeric: 352,
                name: "Coastal waters from Edisto Beach SC to Savannah GA out 20 nm",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ354 => crate::ZoneDetails {
                state: "AM",
                zone: "354",
                zone_numeric: 354,
                name: "Coastal waters from Savannah GA to Altamaha Sound GA out 20 nm ...including Grays Reef National Marine Sanctuary",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ370 => crate::ZoneDetails {
                state: "AM",
                zone: "370",
                zone_numeric: 370,
                name: "Waters from South Santee River SC to Edisto Beach SC extending from 20 nm to 40 nm",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ372 => crate::ZoneDetails {
                state: "AM",
                zone: "372",
                zone_numeric: 372,
                name: "Waters from Edisto Beach SC to Savannah GA extending from 20 nm to 40 nm",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ374 => crate::ZoneDetails {
                state: "AM",
                zone: "374",
                zone_numeric: 374,
                name: "Waters from Savannah GA to Altamaha Sound GA extending from 20 nm to 60 nm",
                wfo: "CHS",
            },
            CoastalMarineZone::AMZ450 => crate::ZoneDetails {
                state: "AM",
                zone: "450",
                zone_numeric: 450,
                name: "Coastal waters from Altamaha Sound to Fernandina Beach FL out 20 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ452 => crate::ZoneDetails {
                state: "AM",
                zone: "452",
                zone_numeric: 452,
                name: "Coastal waters from Fernandina Beach to St. Augustine FL out 20 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ454 => crate::ZoneDetails {
                state: "AM",
                zone: "454",
                zone_numeric: 454,
                name: "Coastal waters from St. Augustine to Flagler Beach FL out 20 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ470 => crate::ZoneDetails {
                state: "AM",
                zone: "470",
                zone_numeric: 470,
                name: "Waters from Altamaha Sound GA to Fernandina Beach FL from 20 to 60 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ472 => crate::ZoneDetails {
                state: "AM",
                zone: "472",
                zone_numeric: 472,
                name: "Waters from Fernandina Beach to St. Augustine FL from 20 to 60 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ474 => crate::ZoneDetails {
                state: "AM",
                zone: "474",
                zone_numeric: 474,
                name: "Waters from St. Augustine to Flagler Beach FL from 20 to 60 NM",
                wfo: "JAX",
            },
            CoastalMarineZone::AMZ550 => crate::ZoneDetails {
                state: "AM",
                zone: "550",
                zone_numeric: 550,
                name: "Flagler Beach to Volusia-Brevard County Line 0-20 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ552 => crate::ZoneDetails {
                state: "AM",
                zone: "552",
                zone_numeric: 552,
                name: "Volusia-Brevard County Line to Sebastian Inlet 0-20 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ555 => crate::ZoneDetails {
                state: "AM",
                zone: "555",
                zone_numeric: 555,
                name: "Sebastian Inlet to Jupiter Inlet 0-20 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ570 => crate::ZoneDetails {
                state: "AM",
                zone: "570",
                zone_numeric: 570,
                name: "Flagler Beach to Volusia-Brevard County Line 20-60 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ572 => crate::ZoneDetails {
                state: "AM",
                zone: "572",
                zone_numeric: 572,
                name: "Volusia-Brevard County Line to Sebastian Inlet 20-60 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ575 => crate::ZoneDetails {
                state: "AM",
                zone: "575",
                zone_numeric: 575,
                name: "Sebastian Inlet to Jupiter Inlet 20-60 nm",
                wfo: "MLB",
            },
            CoastalMarineZone::AMZ610 => crate::ZoneDetails {
                state: "AM",
                zone: "610",
                zone_numeric: 610,
                name: "Lake Okeechobee",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ630 => crate::ZoneDetails {
                state: "AM",
                zone: "630",
                zone_numeric: 630,
                name: "Biscayne Bay",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ650 => crate::ZoneDetails {
                state: "AM",
                zone: "650",
                zone_numeric: 650,
                name: "Coastal waters from Jupiter Inlet to Deerfield Beach FL out 20 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ651 => crate::ZoneDetails {
                state: "AM",
                zone: "651",
                zone_numeric: 651,
                name: "Coastal waters from Deerfield Beach to Ocean Reef FL out 20 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ670 => crate::ZoneDetails {
                state: "AM",
                zone: "670",
                zone_numeric: 670,
                name: "Waters from Jupiter Inlet to Deerfield Beach FL from 20 to 60 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ671 => crate::ZoneDetails {
                state: "AM",
                zone: "671",
                zone_numeric: 671,
                name: "Waters from Deerfield Beach to Ocean Reef FL from 20 to 60 NM excluding the territorial waters of Bahamas",
                wfo: "MFL",
            },
            CoastalMarineZone::AMZ710 => crate::ZoneDetails {
                state: "AM",
                zone: "710",
                zone_numeric: 710,
                name: "Atlantic Waters of Puerto Rico AND USVI from 10 NM to 19.5N",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ712 => crate::ZoneDetails {
                state: "AM",
                zone: "712",
                zone_numeric: 712,
                name: "Coastal Waters of Northern Puerto Rico out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ715 => crate::ZoneDetails {
                state: "AM",
                zone: "715",
                zone_numeric: 715,
                name: "Coastal Waters of Northern USVI and Culebra out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ722 => crate::ZoneDetails {
                state: "AM",
                zone: "722",
                zone_numeric: 722,
                name: "Anegada Passage Southward to 17N",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ725 => crate::ZoneDetails {
                state: "AM",
                zone: "725",
                zone_numeric: 725,
                name: "Coastal Waters of Southern USVI, Vieques, and Eastern Puerto Rico out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ732 => crate::ZoneDetails {
                state: "AM",
                zone: "732",
                zone_numeric: 732,
                name: "Caribbean Waters of Puerto Rico from 10 NM to 17N",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ735 => crate::ZoneDetails {
                state: "AM",
                zone: "735",
                zone_numeric: 735,
                name: "Coastal Waters of Southern Puerto Rico out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ741 => crate::ZoneDetails {
                state: "AM",
                zone: "741",
                zone_numeric: 741,
                name: "Mona Passage Southward to 17N",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ742 => crate::ZoneDetails {
                state: "AM",
                zone: "742",
                zone_numeric: 742,
                name: "Coastal Waters OF Northwestern Puerto Rico out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::AMZ745 => crate::ZoneDetails {
                state: "AM",
                zone: "745",
                zone_numeric: 745,
                name: "Coastal Waters OF Southwestern Puerto Rico out 10 NM",
                wfo: "SJU",
            },
            CoastalMarineZone::ANZ050 => crate::ZoneDetails {
                state: "AN",
                zone: "050",
                zone_numeric: 50,
                name: "Coastal Waters from Eastport, ME to Schoodic Point, ME out 25 NM",
                wfo: "CAR",
            },
            CoastalMarineZone::ANZ051 => crate::ZoneDetails {
                state: "AN",
                zone: "051",
                zone_numeric: 51,
                name: "Coastal Waters from Schoodic Point, ME to Stonington, ME out 25 NM",
                wfo: "CAR",
            },
            CoastalMarineZone::ANZ052 => crate::ZoneDetails {
                state: "AN",
                zone: "052",
                zone_numeric: 52,
                name: "Intra Coastal Waters from Schoodic Point, ME to Stonington, ME",
                wfo: "CAR",
            },
            CoastalMarineZone::ANZ070 => crate::ZoneDetails {
                state: "AN",
                zone: "070",
                zone_numeric: 70,
                name: "Waters from Eastport ME to Schoodic Point, ME from 25 to 40 nm",
                wfo: "CAR",
            },
            CoastalMarineZone::ANZ071 => crate::ZoneDetails {
                state: "AN",
                zone: "071",
                zone_numeric: 71,
                name: "Waters from Schoodic Point, ME to Stonington ME from 25 to 40 nm",
                wfo: "CAR",
            },
            CoastalMarineZone::ANZ150 => crate::ZoneDetails {
                state: "AN",
                zone: "150",
                zone_numeric: 150,
                name: "Coastal Waters from Stonington, ME to Port Clyde, ME out 25 NM",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ151 => crate::ZoneDetails {
                state: "AN",
                zone: "151",
                zone_numeric: 151,
                name: "Penobscot Bay",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ152 => crate::ZoneDetails {
                state: "AN",
                zone: "152",
                zone_numeric: 152,
                name: "Coastal Waters from Port Clyde, ME to Cape Elizabeth, ME out 25 NM",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ153 => crate::ZoneDetails {
                state: "AN",
                zone: "153",
                zone_numeric: 153,
                name: "Casco Bay",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ154 => crate::ZoneDetails {
                state: "AN",
                zone: "154",
                zone_numeric: 154,
                name: "Coastal Waters from Cape Elizabeth, ME to Merrimack River, MA out 25 NM",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ170 => crate::ZoneDetails {
                state: "AN",
                zone: "170",
                zone_numeric: 170,
                name: "Waters from Stonington ME to Port Clyde ME from 25 to 40 nm",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ172 => crate::ZoneDetails {
                state: "AN",
                zone: "172",
                zone_numeric: 172,
                name: "Waters from Port Clyde ME to Cape Elizabeth ME from 25 to 40 nm",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ174 => crate::ZoneDetails {
                state: "AN",
                zone: "174",
                zone_numeric: 174,
                name: "Waters from Cape Elizabeth ME to Merrimack River MA from 25 to 40 nm",
                wfo: "GYX",
            },
            CoastalMarineZone::ANZ230 => crate::ZoneDetails {
                state: "AN",
                zone: "230",
                zone_numeric: 230,
                name: "Boston Harbor",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ231 => crate::ZoneDetails {
                state: "AN",
                zone: "231",
                zone_numeric: 231,
                name: "Cape Cod Bay",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ232 => crate::ZoneDetails {
                state: "AN",
                zone: "232",
                zone_numeric: 232,
                name: "Nantucket Sound",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ233 => crate::ZoneDetails {
                state: "AN",
                zone: "233",
                zone_numeric: 233,
                name: "Vineyard Sound",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ234 => crate::ZoneDetails {
                state: "AN",
                zone: "234",
                zone_numeric: 234,
                name: "Buzzards Bay",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ235 => crate::ZoneDetails {
                state: "AN",
                zone: "235",
                zone_numeric: 235,
                name: "Rhode Island Sound",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ236 => crate::ZoneDetails {
                state: "AN",
                zone: "236",
                zone_numeric: 236,
                name: "Narragansett Bay",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ237 => crate::ZoneDetails {
                state: "AN",
                zone: "237",
                zone_numeric: 237,
                name: "Block Island Sound",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ250 => crate::ZoneDetails {
                state: "AN",
                zone: "250",
                zone_numeric: 250,
                name: "Coastal waters east of Ipswich Bay and the Stellwagen Bank National Marine Sanctuary",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ251 => crate::ZoneDetails {
                state: "AN",
                zone: "251",
                zone_numeric: 251,
                name: "Massachusetts Bay and Ipswich Bay",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ254 => crate::ZoneDetails {
                state: "AN",
                zone: "254",
                zone_numeric: 254,
                name: "Coastal waters from Provincetown MA to Chatham MA to Nantucket MA out 20 nm",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ255 => crate::ZoneDetails {
                state: "AN",
                zone: "255",
                zone_numeric: 255,
                name: "Coastal Waters extending out to 25 nm South of Marthas Vineyard and Nantucket",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ256 => crate::ZoneDetails {
                state: "AN",
                zone: "256",
                zone_numeric: 256,
                name: "Coastal Waters from Montauk NY to Marthas Vineyard extending out to 20 nm South of Block Island",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ270 => crate::ZoneDetails {
                state: "AN",
                zone: "270",
                zone_numeric: 270,
                name: "Ocean Waters from the Merrimack River to Plymouth from 40 to 60 NM offshore",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ271 => crate::ZoneDetails {
                state: "AN",
                zone: "271",
                zone_numeric: 271,
                name: "Ocean Waters from Provincetown to Nantucket from 20 to 35 NM offshore",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ272 => crate::ZoneDetails {
                state: "AN",
                zone: "272",
                zone_numeric: 272,
                name: "Ocean Waters from Marthas Vineyard to Nantucket from 25 to 45 NM offshore",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ273 => crate::ZoneDetails {
                state: "AN",
                zone: "273",
                zone_numeric: 273,
                name: "Ocean Waters from Montauk NY to Marthas Vineyard from 25 to 40 NM offshore",
                wfo: "BOX",
            },
            CoastalMarineZone::ANZ331 => crate::ZoneDetails {
                state: "AN",
                zone: "331",
                zone_numeric: 331,
                name: "Long Island Sound East of New Haven CT/Port Jefferson NY to the Mouth of the Connecticut River",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ332 => crate::ZoneDetails {
                state: "AN",
                zone: "332",
                zone_numeric: 332,
                name: "Long Island Sound East of the Mouth of the Connecticut River",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ335 => crate::ZoneDetails {
                state: "AN",
                zone: "335",
                zone_numeric: 335,
                name: "Long Island Sound West of New Haven CT/Port Jefferson NY",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ338 => crate::ZoneDetails {
                state: "AN",
                zone: "338",
                zone_numeric: 338,
                name: "New York Harbor",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ340 => crate::ZoneDetails {
                state: "AN",
                zone: "340",
                zone_numeric: 340,
                name: "Peconic and Gardiners Bays",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ345 => crate::ZoneDetails {
                state: "AN",
                zone: "345",
                zone_numeric: 345,
                name: "South Shore Bays from Jones Inlet through Shinnecock Bay",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ350 => crate::ZoneDetails {
                state: "AN",
                zone: "350",
                zone_numeric: 350,
                name: "Moriches Inlet NY to Montauk Point NY out 20 nm",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ353 => crate::ZoneDetails {
                state: "AN",
                zone: "353",
                zone_numeric: 353,
                name: "Fire Island Inlet NY to Moriches Inlet NY out 20 nm",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ355 => crate::ZoneDetails {
                state: "AN",
                zone: "355",
                zone_numeric: 355,
                name: "Sandy Hook NJ to Fire Island Inlet NY out 20 nm",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ370 => crate::ZoneDetails {
                state: "AN",
                zone: "370",
                zone_numeric: 370,
                name: "Waters from Moriches Inlet NY to Montauk Point NY from 20 to 40 NM",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ373 => crate::ZoneDetails {
                state: "AN",
                zone: "373",
                zone_numeric: 373,
                name: "Waters from Fire Island Inlet NY to Moriches Inlet NY from 20 to 40 NM",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ375 => crate::ZoneDetails {
                state: "AN",
                zone: "375",
                zone_numeric: 375,
                name: "Waters from Sandy Hook NJ to Fire Island Inlet out 20 to 40 nm",
                wfo: "OKX",
            },
            CoastalMarineZone::ANZ430 => crate::ZoneDetails {
                state: "AN",
                zone: "430",
                zone_numeric: 430,
                name: "Delaware Bay waters north of East Point NJ to Slaughter Beach DE",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ431 => crate::ZoneDetails {
                state: "AN",
                zone: "431",
                zone_numeric: 431,
                name: "Delaware Bay waters south of East Point NJ to Slaughter Beach DE",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ450 => crate::ZoneDetails {
                state: "AN",
                zone: "450",
                zone_numeric: 450,
                name: "Coastal waters from Sandy Hook to Manasquan Inlet NJ out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ451 => crate::ZoneDetails {
                state: "AN",
                zone: "451",
                zone_numeric: 451,
                name: "Coastal waters from Manasquan Inlet to Little Egg Inlet NJ out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ452 => crate::ZoneDetails {
                state: "AN",
                zone: "452",
                zone_numeric: 452,
                name: "Coastal waters from Little Egg Inlet to Great Egg Inlet NJ out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ453 => crate::ZoneDetails {
                state: "AN",
                zone: "453",
                zone_numeric: 453,
                name: "Coastal waters from Great Egg Inlet to Cape May NJ out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ454 => crate::ZoneDetails {
                state: "AN",
                zone: "454",
                zone_numeric: 454,
                name: "Coastal waters from Cape May NJ to Cape Henlopen DE out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ455 => crate::ZoneDetails {
                state: "AN",
                zone: "455",
                zone_numeric: 455,
                name: "Coastal waters from Cape Henlopen to Fenwick Island DE out 20 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ470 => crate::ZoneDetails {
                state: "AN",
                zone: "470",
                zone_numeric: 470,
                name: "Waters from Sandy Hook NJ to Manasquan Inlet NJ out 20 to 40 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ471 => crate::ZoneDetails {
                state: "AN",
                zone: "471",
                zone_numeric: 471,
                name: "Waters from Manasquan Inlet NJ to Little Egg Inlet NJ out 20 to 40 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ472 => crate::ZoneDetails {
                state: "AN",
                zone: "472",
                zone_numeric: 472,
                name: "Waters from Little Egg Inlet NJ to Great Egg Inlet NJ out 20 to 40 nm from 20 to 40 NM",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ473 => crate::ZoneDetails {
                state: "AN",
                zone: "473",
                zone_numeric: 473,
                name: "Waters from Great Egg Inlet NJ to Cape May NJ out 20 to 40 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ475 => crate::ZoneDetails {
                state: "AN",
                zone: "475",
                zone_numeric: 475,
                name: "Waters from Cape May NJ to Fenwick Island DE out 20 to 40 nm",
                wfo: "PHI",
            },
            CoastalMarineZone::ANZ530 => crate::ZoneDetails {
                state: "AN",
                zone: "530",
                zone_numeric: 530,
                name: "Chesapeake Bay north of Pooles Island MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ531 => crate::ZoneDetails {
                state: "AN",
                zone: "531",
                zone_numeric: 531,
                name: "Chesapeake Bay from Pooles Island to Sandy Point MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ532 => crate::ZoneDetails {
                state: "AN",
                zone: "532",
                zone_numeric: 532,
                name: "Chesapeake Bay from Sandy Point to North Beach MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ533 => crate::ZoneDetails {
                state: "AN",
                zone: "533",
                zone_numeric: 533,
                name: "Chesapeake Bay from North Beach to Drum Point MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ534 => crate::ZoneDetails {
                state: "AN",
                zone: "534",
                zone_numeric: 534,
                name: "Chesapeake Bay from Drum Point MD to Smith Point VA",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ535 => crate::ZoneDetails {
                state: "AN",
                zone: "535",
                zone_numeric: 535,
                name: "Tidal Potomac from Key Bridge to Indian Head MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ536 => crate::ZoneDetails {
                state: "AN",
                zone: "536",
                zone_numeric: 536,
                name: "Tidal Potomac from Indian Head to Cobb Island MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ537 => crate::ZoneDetails {
                state: "AN",
                zone: "537",
                zone_numeric: 537,
                name: "Tidal Potomac from Cobb Island MD to Smith Point VA",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ538 => crate::ZoneDetails {
                state: "AN",
                zone: "538",
                zone_numeric: 538,
                name: "Patapsco River including Baltimore Harbor",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ539 => crate::ZoneDetails {
                state: "AN",
                zone: "539",
                zone_numeric: 539,
                name: "Chester River to Queenstown MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ540 => crate::ZoneDetails {
                state: "AN",
                zone: "540",
                zone_numeric: 540,
                name: "Eastern Bay",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ541 => crate::ZoneDetails {
                state: "AN",
                zone: "541",
                zone_numeric: 541,
                name: "Choptank River to Cambridge MD and the Little Choptank River",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ542 => crate::ZoneDetails {
                state: "AN",
                zone: "542",
                zone_numeric: 542,
                name: "Patuxent River to Broomes Island MD",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ543 => crate::ZoneDetails {
                state: "AN",
                zone: "543",
                zone_numeric: 543,
                name: "Tangier Sound and the inland waters surrounding Bloodsworth Island",
                wfo: "LWX",
            },
            CoastalMarineZone::ANZ630 => crate::ZoneDetails {
                state: "AN",
                zone: "630",
                zone_numeric: 630,
                name: "Chesapeake Bay from Smith Point to Windmill Point VA",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ631 => crate::ZoneDetails {
                state: "AN",
                zone: "631",
                zone_numeric: 631,
                name: "Chesapeake Bay from Windmill Point to New Point Comfort VA",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ632 => crate::ZoneDetails {
                state: "AN",
                zone: "632",
                zone_numeric: 632,
                name: "Chesapeake Bay from New Point Comfort to Little Creek VA",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ633 => crate::ZoneDetails {
                state: "AN",
                zone: "633",
                zone_numeric: 633,
                name: "Currituck Sound",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ634 => crate::ZoneDetails {
                state: "AN",
                zone: "634",
                zone_numeric: 634,
                name: "Chesapeake Bay from Little Creek VA to Cape Henry VA including the Chesapeake Bay Bridge Tunnel",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ635 => crate::ZoneDetails {
                state: "AN",
                zone: "635",
                zone_numeric: 635,
                name: "Rappahannock River from Urbanna to Windmill Point",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ636 => crate::ZoneDetails {
                state: "AN",
                zone: "636",
                zone_numeric: 636,
                name: "York River",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ637 => crate::ZoneDetails {
                state: "AN",
                zone: "637",
                zone_numeric: 637,
                name: "James River from Jamestown to the James River Bridge",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ638 => crate::ZoneDetails {
                state: "AN",
                zone: "638",
                zone_numeric: 638,
                name: "James River from James River Bridge to Hampton Roads Bridge-Tunnel",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ650 => crate::ZoneDetails {
                state: "AN",
                zone: "650",
                zone_numeric: 650,
                name: "Coastal waters from Fenwick Island DE to Chincoteague VA out 20 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ652 => crate::ZoneDetails {
                state: "AN",
                zone: "652",
                zone_numeric: 652,
                name: "Coastal waters from Chincoteague to Parramore Island VA out 20 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ654 => crate::ZoneDetails {
                state: "AN",
                zone: "654",
                zone_numeric: 654,
                name: "Coastal waters from Parramore Island to Cape Charles Light VA out 20 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ656 => crate::ZoneDetails {
                state: "AN",
                zone: "656",
                zone_numeric: 656,
                name: "Coastal Waters from Cape Charles Light to Virginia-North Carolina border out to 20 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ658 => crate::ZoneDetails {
                state: "AN",
                zone: "658",
                zone_numeric: 658,
                name: "Coastal waters from NC VA border to Currituck Beach Light NC out 20 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ670 => crate::ZoneDetails {
                state: "AN",
                zone: "670",
                zone_numeric: 670,
                name: "Waters from Fenwick Island DE to Chintoteague VA from 20 to 40 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ672 => crate::ZoneDetails {
                state: "AN",
                zone: "672",
                zone_numeric: 672,
                name: "Waters from Chincoteague VA to Parramore Island VA from 20 to 40 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ674 => crate::ZoneDetails {
                state: "AN",
                zone: "674",
                zone_numeric: 674,
                name: "Waters from Parramore Island VA to Cape Charles Light VA from 20 to 40 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ676 => crate::ZoneDetails {
                state: "AN",
                zone: "676",
                zone_numeric: 676,
                name: "Waters from Cape Charles Light to Virginia - North Carolina Border from 20 to 40 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::ANZ678 => crate::ZoneDetails {
                state: "AN",
                zone: "678",
                zone_numeric: 678,
                name: "Waters from NC VA border to Currituck Beach Light NC from 20 to 40 nm",
                wfo: "AKQ",
            },
            CoastalMarineZone::GMZ031 => crate::ZoneDetails {
                state: "GM",
                zone: "031",
                zone_numeric: 31,
                name: "Florida Bay including Barnes Sound, Blackwater Sound, and Buttonwood Sound",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ032 => crate::ZoneDetails {
                state: "GM",
                zone: "032",
                zone_numeric: 32,
                name: "Bayside and Gulf side from Craig Key to West End of Seven Mile Bridge",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ033 => crate::ZoneDetails {
                state: "GM",
                zone: "033",
                zone_numeric: 33,
                name: "Gulf waters from East Cape Sable to Chokoloskee 20 to 60 NM out and beyond 5 fathoms",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ034 => crate::ZoneDetails {
                state: "GM",
                zone: "034",
                zone_numeric: 34,
                name: "Gulf of Mexico including Dry Tortugas and Rebecca Shoal Channel",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ035 => crate::ZoneDetails {
                state: "GM",
                zone: "035",
                zone_numeric: 35,
                name: "Gulf of Mexico from West End of Seven Mile Bridge to Halfmoon Shoal out to 5 Fathoms",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ042 => crate::ZoneDetails {
                state: "GM",
                zone: "042",
                zone_numeric: 42,
                name: "Hawk Channel from Ocean Reef to Craig Key out to the reef",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ043 => crate::ZoneDetails {
                state: "GM",
                zone: "043",
                zone_numeric: 43,
                name: "Hawk Channel from Craig Key to west end of Seven Mile Bridge out to the reef",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ044 => crate::ZoneDetails {
                state: "GM",
                zone: "044",
                zone_numeric: 44,
                name: "Hawk Channel from west end of Seven Mile Bridge to Halfmoon Shoal out to the reef",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ052 => crate::ZoneDetails {
                state: "GM",
                zone: "052",
                zone_numeric: 52,
                name: "Straits of Florida from Ocean Reef to Craig Key out 20 NM",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ053 => crate::ZoneDetails {
                state: "GM",
                zone: "053",
                zone_numeric: 53,
                name: "Straits of Florida from Craig Key to west end of Seven Mile Bridge out 20 NM",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ054 => crate::ZoneDetails {
                state: "GM",
                zone: "054",
                zone_numeric: 54,
                name: "Straits of Florida from west end of Seven Mile Bridge to south of Halfmoon Shoal out 20 NM",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ055 => crate::ZoneDetails {
                state: "GM",
                zone: "055",
                zone_numeric: 55,
                name: "Straits of Florida from Halfmoon Shoal to 20 NM west of Dry Tortugas out 20 NM",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ072 => crate::ZoneDetails {
                state: "GM",
                zone: "072",
                zone_numeric: 72,
                name: "Straits of Florida from Ocean Reef to Craig Key 20 to 60 NM out",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ073 => crate::ZoneDetails {
                state: "GM",
                zone: "073",
                zone_numeric: 73,
                name: "Straits of Florida from Craig Key to west end of Seven Mile Bridge 20 to 60 NM out",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ074 => crate::ZoneDetails {
                state: "GM",
                zone: "074",
                zone_numeric: 74,
                name: "Straits of Florida from west end of Seven Mile Bridge to south of Halfmoon Shoal 20 to 60 NM out",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ075 => crate::ZoneDetails {
                state: "GM",
                zone: "075",
                zone_numeric: 75,
                name: "Straits of Florida from Halfmoon Shoal to 20 NM west of Dry Tortugas 20 to 60 NM out",
                wfo: "KEY",
            },
            CoastalMarineZone::GMZ130 => crate::ZoneDetails {
                state: "GM",
                zone: "130",
                zone_numeric: 130,
                name: "Laguna Madre From the Port Of Brownsville to the Arroyo Colorado",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ132 => crate::ZoneDetails {
                state: "GM",
                zone: "132",
                zone_numeric: 132,
                name: "Laguna Madre From The Arroyo Colorado To 5 NM North Of Port Mansfield TX",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ135 => crate::ZoneDetails {
                state: "GM",
                zone: "135",
                zone_numeric: 135,
                name: "Laguna Madre From 5 nm North Of Port Mansfield To Baffin Bay TX",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ150 => crate::ZoneDetails {
                state: "GM",
                zone: "150",
                zone_numeric: 150,
                name: "Coastal waters from Port Mansfield TX to the Rio Grande River out 20 NM",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ155 => crate::ZoneDetails {
                state: "GM",
                zone: "155",
                zone_numeric: 155,
                name: "Coastal waters from Baffin Bay to Port Mansfield TX out 20 NM",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ170 => crate::ZoneDetails {
                state: "GM",
                zone: "170",
                zone_numeric: 170,
                name: "Waters from Port Mansfield TX to the Rio Grande River from 20 to 60 NM",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ175 => crate::ZoneDetails {
                state: "GM",
                zone: "175",
                zone_numeric: 175,
                name: "Waters from Baffin Bay to Port Mansfield TX from 20 to 60 NM",
                wfo: "BRO",
            },
            CoastalMarineZone::GMZ231 => crate::ZoneDetails {
                state: "GM",
                zone: "231",
                zone_numeric: 231,
                name: "Baffin Bay and Upper Laguna Madre",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ232 => crate::ZoneDetails {
                state: "GM",
                zone: "232",
                zone_numeric: 232,
                name: "Corpus Christi and Nueces Bays",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ236 => crate::ZoneDetails {
                state: "GM",
                zone: "236",
                zone_numeric: 236,
                name: "Copano, Aransas, and Redfish Bays",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ237 => crate::ZoneDetails {
                state: "GM",
                zone: "237",
                zone_numeric: 237,
                name: "San Antonio, Mesquite, and Espiritu Santo Bays",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ250 => crate::ZoneDetails {
                state: "GM",
                zone: "250",
                zone_numeric: 250,
                name: "Coastal waters from Baffin Bay to Port Aransas out 20 NM",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ255 => crate::ZoneDetails {
                state: "GM",
                zone: "255",
                zone_numeric: 255,
                name: "Coastal waters from Port Aransas to Matagorda Ship Channel out 20 NM",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ270 => crate::ZoneDetails {
                state: "GM",
                zone: "270",
                zone_numeric: 270,
                name: "Waters from Baffin Bay to Port Aransas from 20 to 60 NM",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ275 => crate::ZoneDetails {
                state: "GM",
                zone: "275",
                zone_numeric: 275,
                name: "Waters from Port Aransas to Matagorda Ship Channel from 20 to 60 NM",
                wfo: "CRP",
            },
            CoastalMarineZone::GMZ330 => crate::ZoneDetails {
                state: "GM",
                zone: "330",
                zone_numeric: 330,
                name: "Matagorda Bay",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ335 => crate::ZoneDetails {
                state: "GM",
                zone: "335",
                zone_numeric: 335,
                name: "Galveston Bay",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ350 => crate::ZoneDetails {
                state: "GM",
                zone: "350",
                zone_numeric: 350,
                name: "Coastal waters from Freeport to Matagorda Ship Channel TX out 20 NM",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ355 => crate::ZoneDetails {
                state: "GM",
                zone: "355",
                zone_numeric: 355,
                name: "Coastal waters from High Island to Freeport TX out 20 NM",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ370 => crate::ZoneDetails {
                state: "GM",
                zone: "370",
                zone_numeric: 370,
                name: "Waters from Freeport to Matagorda Ship Channel TX from 20 to 60 NM",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ375 => crate::ZoneDetails {
                state: "GM",
                zone: "375",
                zone_numeric: 375,
                name: "Waters from High Island to Freeport TX from 20 to 60 NM",
                wfo: "HGX",
            },
            CoastalMarineZone::GMZ430 => crate::ZoneDetails {
                state: "GM",
                zone: "430",
                zone_numeric: 430,
                name: "Sabine Lake",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ432 => crate::ZoneDetails {
                state: "GM",
                zone: "432",
                zone_numeric: 432,
                name: "Calcasieu Lake",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ435 => crate::ZoneDetails {
                state: "GM",
                zone: "435",
                zone_numeric: 435,
                name: "Vermilion Bay",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ450 => crate::ZoneDetails {
                state: "GM",
                zone: "450",
                zone_numeric: 450,
                name: "Coastal waters from Cameron LA to High Island TX out 20 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ452 => crate::ZoneDetails {
                state: "GM",
                zone: "452",
                zone_numeric: 452,
                name: "Coastal waters from Intracoastal City to Cameron LA out 20 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ455 => crate::ZoneDetails {
                state: "GM",
                zone: "455",
                zone_numeric: 455,
                name: "Coastal waters from Lower Atchafalaya River to Intracoastal City LA out 20 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ470 => crate::ZoneDetails {
                state: "GM",
                zone: "470",
                zone_numeric: 470,
                name: "Waters from Cameron LA to High Island TX from 20 to 60 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ472 => crate::ZoneDetails {
                state: "GM",
                zone: "472",
                zone_numeric: 472,
                name: "Waters from Intracoastal City to Cameron LA from 20 to 60 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ475 => crate::ZoneDetails {
                state: "GM",
                zone: "475",
                zone_numeric: 475,
                name: "Waters from Lower Atchafalaya River to Intracoastal City LA from 20 to 60 NM",
                wfo: "LCH",
            },
            CoastalMarineZone::GMZ530 => crate::ZoneDetails {
                state: "GM",
                zone: "530",
                zone_numeric: 530,
                name: "Lake Pontchartrain and Lake Maurepas",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ532 => crate::ZoneDetails {
                state: "GM",
                zone: "532",
                zone_numeric: 532,
                name: "Mississippi Sound",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ534 => crate::ZoneDetails {
                state: "GM",
                zone: "534",
                zone_numeric: 534,
                name: "Lake Borgne",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ536 => crate::ZoneDetails {
                state: "GM",
                zone: "536",
                zone_numeric: 536,
                name: "Chandeleur Sound",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ538 => crate::ZoneDetails {
                state: "GM",
                zone: "538",
                zone_numeric: 538,
                name: "Breton Sound",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ550 => crate::ZoneDetails {
                state: "GM",
                zone: "550",
                zone_numeric: 550,
                name: "Coastal Waters from Port Fourchon LA to Lower Atchafalaya River LA out 20 nm",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ552 => crate::ZoneDetails {
                state: "GM",
                zone: "552",
                zone_numeric: 552,
                name: "Coastal waters from the Southwest Pass of the Mississippi River to Port Fourchon Louisiana out 20 NM",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ555 => crate::ZoneDetails {
                state: "GM",
                zone: "555",
                zone_numeric: 555,
                name: "Coastal Waters from Boothville LA to Southwest Pass of the Mississippi River out 20 nm",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ557 => crate::ZoneDetails {
                state: "GM",
                zone: "557",
                zone_numeric: 557,
                name: "Coastal waters from Pascagoula Mississippi to Stake Island out 20 NM",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ570 => crate::ZoneDetails {
                state: "GM",
                zone: "570",
                zone_numeric: 570,
                name: "Coastal waters from Port Fourchon Louisiana to Lower Atchafalaya River LA from 20 to 60 NM",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ572 => crate::ZoneDetails {
                state: "GM",
                zone: "572",
                zone_numeric: 572,
                name: "Coastal waters from Southwest Pass of the Mississippi River to Port Fourchon Louisiana from 20 to 60 NM",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ575 => crate::ZoneDetails {
                state: "GM",
                zone: "575",
                zone_numeric: 575,
                name: "Coastal Waters from Stake Island LA to Southwest Pass of the Mississippi River from 20 to 60 nm",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ577 => crate::ZoneDetails {
                state: "GM",
                zone: "577",
                zone_numeric: 577,
                name: "Coastal waters from Pascagoula Mississippi to Stake Island Louisiana out 20 to 60 NM",
                wfo: "LIX",
            },
            CoastalMarineZone::GMZ630 => crate::ZoneDetails {
                state: "GM",
                zone: "630",
                zone_numeric: 630,
                name: "North Mobile Bay",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ631 => crate::ZoneDetails {
                state: "GM",
                zone: "631",
                zone_numeric: 631,
                name: "South Mobile Bay",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ632 => crate::ZoneDetails {
                state: "GM",
                zone: "632",
                zone_numeric: 632,
                name: "Mississippi Sound",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ633 => crate::ZoneDetails {
                state: "GM",
                zone: "633",
                zone_numeric: 633,
                name: "Perdido Bay Area",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ634 => crate::ZoneDetails {
                state: "GM",
                zone: "634",
                zone_numeric: 634,
                name: "Pensacola Bay Area including Santa Rosa Sound",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ635 => crate::ZoneDetails {
                state: "GM",
                zone: "635",
                zone_numeric: 635,
                name: "Western Choctawhatchee Bay",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ636 => crate::ZoneDetails {
                state: "GM",
                zone: "636",
                zone_numeric: 636,
                name: "Eastern Choctawhatchee Bay",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ650 => crate::ZoneDetails {
                state: "GM",
                zone: "650",
                zone_numeric: 650,
                name: "Coastal waters from Pensacola FL to Pascagoula MS out 20 NM",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ655 => crate::ZoneDetails {
                state: "GM",
                zone: "655",
                zone_numeric: 655,
                name: "Coastal waters from Okaloosa-Walton County Line to Pensacola FL out 20 NM",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ656 => crate::ZoneDetails {
                state: "GM",
                zone: "656",
                zone_numeric: 656,
                name: "Coastal waters from Chokoloskee to Bonita Beach FL out 20 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::GMZ657 => crate::ZoneDetails {
                state: "GM",
                zone: "657",
                zone_numeric: 657,
                name: "Coastal waters from East Cape Sable to Chokoloskee FL out 20 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::GMZ670 => crate::ZoneDetails {
                state: "GM",
                zone: "670",
                zone_numeric: 670,
                name: "Waters from Pensacola FL to Pascagoula MS from 20 to 60 NM",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ675 => crate::ZoneDetails {
                state: "GM",
                zone: "675",
                zone_numeric: 675,
                name: "Waters from Okaloosa-Walton County Line to Pensacola FL from 20 to 60 NM",
                wfo: "MOB",
            },
            CoastalMarineZone::GMZ676 => crate::ZoneDetails {
                state: "GM",
                zone: "676",
                zone_numeric: 676,
                name: "Waters from Chokoloskee to Bonita Beach FL from 20 to 60 NM",
                wfo: "MFL",
            },
            CoastalMarineZone::GMZ730 => crate::ZoneDetails {
                state: "GM",
                zone: "730",
                zone_numeric: 730,
                name: "Apalachee Bay or Coastal Waters From Keaton Beach to Ochlockonee River Fl out to 20 Nm",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ750 => crate::ZoneDetails {
                state: "GM",
                zone: "750",
                zone_numeric: 750,
                name: "Coastal waters from Okaloosa-Walton County Line to Mexico Beach out 20 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ752 => crate::ZoneDetails {
                state: "GM",
                zone: "752",
                zone_numeric: 752,
                name: "Coastal Waters from Mexico Beach to Apalachicola out 20 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ755 => crate::ZoneDetails {
                state: "GM",
                zone: "755",
                zone_numeric: 755,
                name: "Coastal Waters From  Ochlockonee River to Apalachicola FL out to 20 Nm",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ765 => crate::ZoneDetails {
                state: "GM",
                zone: "765",
                zone_numeric: 765,
                name: "Coastal waters from  Suwannee River to Keaton Beach out 20 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ770 => crate::ZoneDetails {
                state: "GM",
                zone: "770",
                zone_numeric: 770,
                name: "Waters from Okaloosa-Walton County Line to Mexico Beach from 20 to 60 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ772 => crate::ZoneDetails {
                state: "GM",
                zone: "772",
                zone_numeric: 772,
                name: "Waters from Mexico Beach to Apalachicola FL from 20 to 60 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ775 => crate::ZoneDetails {
                state: "GM",
                zone: "775",
                zone_numeric: 775,
                name: "Waters from Suwannee River to Apalachicola FL from 20 to 60 NM",
                wfo: "TAE",
            },
            CoastalMarineZone::GMZ830 => crate::ZoneDetails {
                state: "GM",
                zone: "830",
                zone_numeric: 830,
                name: "Tampa Bay waters",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ836 => crate::ZoneDetails {
                state: "GM",
                zone: "836",
                zone_numeric: 836,
                name: "Charlotte Harbor and Pine Island Sound",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ850 => crate::ZoneDetails {
                state: "GM",
                zone: "850",
                zone_numeric: 850,
                name: "Coastal waters from Tarpon Springs to Suwannee River FL out 20 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ853 => crate::ZoneDetails {
                state: "GM",
                zone: "853",
                zone_numeric: 853,
                name: "Coastal waters from Englewood to Tarpon Springs FL out 20 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ856 => crate::ZoneDetails {
                state: "GM",
                zone: "856",
                zone_numeric: 856,
                name: "Coastal waters from Bonita Beach to Englewood FL out 20 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ870 => crate::ZoneDetails {
                state: "GM",
                zone: "870",
                zone_numeric: 870,
                name: "Waters from Tarpon Springs to Suwannee River FL out 20 to 60 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ873 => crate::ZoneDetails {
                state: "GM",
                zone: "873",
                zone_numeric: 873,
                name: "Waters from Englewood to Tarpon Springs FL out 20 to 60 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::GMZ876 => crate::ZoneDetails {
                state: "GM",
                zone: "876",
                zone_numeric: 876,
                name: "Waters from Bonita Beach to Englewood FL out 20 to 60 NM",
                wfo: "TBW",
            },
            CoastalMarineZone::LCZ422 => crate::ZoneDetails {
                state: "LC",
                zone: "422",
                zone_numeric: 422,
                name: "St. Clair River",
                wfo: "DTX",
            },
            CoastalMarineZone::LCZ423 => crate::ZoneDetails {
                state: "LC",
                zone: "423",
                zone_numeric: 423,
                name: "Detroit River",
                wfo: "DTX",
            },
            CoastalMarineZone::LCZ460 => crate::ZoneDetails {
                state: "LC",
                zone: "460",
                zone_numeric: 460,
                name: "Lake St. Clair Open Lake (U.S. Portion)",
                wfo: "DTX",
            },
            CoastalMarineZone::LEZ020 => crate::ZoneDetails {
                state: "LE",
                zone: "020",
                zone_numeric: 20,
                name: "Upper Niagara River and Buffalo Harbor",
                wfo: "BUF",
            },
            CoastalMarineZone::LEZ040 => crate::ZoneDetails {
                state: "LE",
                zone: "040",
                zone_numeric: 40,
                name: "Ripley to Dunkirk NY",
                wfo: "BUF",
            },
            CoastalMarineZone::LEZ041 => crate::ZoneDetails {
                state: "LE",
                zone: "041",
                zone_numeric: 41,
                name: "Dunkirk to Buffalo NY",
                wfo: "BUF",
            },
            CoastalMarineZone::LEZ061 => crate::ZoneDetails {
                state: "LE",
                zone: "061",
                zone_numeric: 61,
                name: "Ripley to Buffalo NY extending from 5NM off shoreline to US-Canadian border",
                wfo: "BUF",
            },
            CoastalMarineZone::LEZ142 => crate::ZoneDetails {
                state: "LE",
                zone: "142",
                zone_numeric: 142,
                name: "Maumee Bay to Reno Beach OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ143 => crate::ZoneDetails {
                state: "LE",
                zone: "143",
                zone_numeric: 143,
                name: "Reno Beach to The Islands OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ144 => crate::ZoneDetails {
                state: "LE",
                zone: "144",
                zone_numeric: 144,
                name: "The Islands to Vermilion OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ145 => crate::ZoneDetails {
                state: "LE",
                zone: "145",
                zone_numeric: 145,
                name: "Vermilion to Avon Point OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ146 => crate::ZoneDetails {
                state: "LE",
                zone: "146",
                zone_numeric: 146,
                name: "Avon Point to Willowick OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ147 => crate::ZoneDetails {
                state: "LE",
                zone: "147",
                zone_numeric: 147,
                name: "Willowick to Geneva-on-the Lake OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ148 => crate::ZoneDetails {
                state: "LE",
                zone: "148",
                zone_numeric: 148,
                name: "Geneva-on-the-Lake to Conneaut OH",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ149 => crate::ZoneDetails {
                state: "LE",
                zone: "149",
                zone_numeric: 149,
                name: "Conneaut OH to Ripley NY",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ162 => crate::ZoneDetails {
                state: "LE",
                zone: "162",
                zone_numeric: 162,
                name: "Detroit River Lt. to Maumee Bay OH to Reno Beach OH beyond 5NM offshoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ163 => crate::ZoneDetails {
                state: "LE",
                zone: "163",
                zone_numeric: 163,
                name: "Reno Beach to The Islands OH beyond 5NM off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ164 => crate::ZoneDetails {
                state: "LE",
                zone: "164",
                zone_numeric: 164,
                name: "The Islands to Vermilion OH beyond 5 nm off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ165 => crate::ZoneDetails {
                state: "LE",
                zone: "165",
                zone_numeric: 165,
                name: "Vermilion to Avon Point OH beyond 5 nm off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ166 => crate::ZoneDetails {
                state: "LE",
                zone: "166",
                zone_numeric: 166,
                name: "Avon Point to Willowick OH beyond 5 nm off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ167 => crate::ZoneDetails {
                state: "LE",
                zone: "167",
                zone_numeric: 167,
                name: "Willowick to Geneva-on-the-Lake OH beyond 5NM off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ168 => crate::ZoneDetails {
                state: "LE",
                zone: "168",
                zone_numeric: 168,
                name: "Geneva-on-the-Lake to Conneaut OH beyond 5 nm off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ169 => crate::ZoneDetails {
                state: "LE",
                zone: "169",
                zone_numeric: 169,
                name: "Conneaut OH to Ripley NY beyond 5 nm off shoreline to US-Canadian border",
                wfo: "CLE",
            },
            CoastalMarineZone::LEZ444 => crate::ZoneDetails {
                state: "LE",
                zone: "444",
                zone_numeric: 444,
                name: "Michigan Waters of Lake Erie from Detroit River to North Cape MI",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ345 => crate::ZoneDetails {
                state: "LH",
                zone: "345",
                zone_numeric: 345,
                name: "Straits of Mackinac within 5 nm of Mackinac Bridge including Mackinac Island",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ346 => crate::ZoneDetails {
                state: "LH",
                zone: "346",
                zone_numeric: 346,
                name: "St Ignace to False Detour Channel",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ347 => crate::ZoneDetails {
                state: "LH",
                zone: "347",
                zone_numeric: 347,
                name: "5NM East of Mackinac Bridge to Presque Isle Light MI including Bois Blanc Island",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ348 => crate::ZoneDetails {
                state: "LH",
                zone: "348",
                zone_numeric: 348,
                name: "Presque Isle Light to Sturgeon Pt MI Including Thunder Bay National Marine Sanctuary",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ349 => crate::ZoneDetails {
                state: "LH",
                zone: "349",
                zone_numeric: 349,
                name: "Sturgeon Pt to Alabaster MI",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ361 => crate::ZoneDetails {
                state: "LH",
                zone: "361",
                zone_numeric: 361,
                name: "Lake Huron from 5NM east of Mackinac Bridge to Presque Isle Lt  to the US/Canadian border beyond 5 NM from shore",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ362 => crate::ZoneDetails {
                state: "LH",
                zone: "362",
                zone_numeric: 362,
                name: "Lake Huron from Presque Isle Lt. to Sturgeon Point  MI 5NM off shore to US/Canadian border",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ363 => crate::ZoneDetails {
                state: "LH",
                zone: "363",
                zone_numeric: 363,
                name: "Lake Huron from Sturgeon Point to Alabaster MI 5NM off shore to US/Canadian border",
                wfo: "APX",
            },
            CoastalMarineZone::LHZ421 => crate::ZoneDetails {
                state: "LH",
                zone: "421",
                zone_numeric: 421,
                name: "Outer Saginaw Bay SW of Alabaster to Port Austin MI to Inner Saginaw Bay",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ422 => crate::ZoneDetails {
                state: "LH",
                zone: "422",
                zone_numeric: 422,
                name: "Inner Saginaw Bay SW of Point Au Gres to Bay Port MI",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ441 => crate::ZoneDetails {
                state: "LH",
                zone: "441",
                zone_numeric: 441,
                name: "Port Austin to Harbor Beach MI",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ442 => crate::ZoneDetails {
                state: "LH",
                zone: "442",
                zone_numeric: 442,
                name: "Harbor Beach to Port Sanilac MI",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ443 => crate::ZoneDetails {
                state: "LH",
                zone: "443",
                zone_numeric: 443,
                name: "Port Sanilac to Port Huron MI",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ462 => crate::ZoneDetails {
                state: "LH",
                zone: "462",
                zone_numeric: 462,
                name: "Lake Huron from Port Austin to Harbor Beach 5NM Off Shore to the US/Canadian border",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ463 => crate::ZoneDetails {
                state: "LH",
                zone: "463",
                zone_numeric: 463,
                name: "Lake Huron from Harbor Beach to Port Sanilac 5NM Off Shore to US/Canadian border",
                wfo: "DTX",
            },
            CoastalMarineZone::LHZ464 => crate::ZoneDetails {
                state: "LH",
                zone: "464",
                zone_numeric: 464,
                name: "Lake Huron from Port Sanilac to Port Huron 5NM Off Shore to US/Canadian border",
                wfo: "DTX",
            },
            CoastalMarineZone::LMZ043 => crate::ZoneDetails {
                state: "LM",
                zone: "043",
                zone_numeric: 43,
                name: "New Buffalo MI to St Joseph MI",
                wfo: "IWX",
            },
            CoastalMarineZone::LMZ046 => crate::ZoneDetails {
                state: "LM",
                zone: "046",
                zone_numeric: 46,
                name: "Michigan City IN to New Buffalo MI",
                wfo: "IWX",
            },
            CoastalMarineZone::LMZ080 => crate::ZoneDetails {
                state: "LM",
                zone: "080",
                zone_numeric: 80,
                name: "Lake Michigan Michigan City IN to St. Joseph MI 5 NM offshore to mid-line of lake.",
                wfo: "IWX",
            },
            CoastalMarineZone::LMZ221 => crate::ZoneDetails {
                state: "LM",
                zone: "221",
                zone_numeric: 221,
                name: "Green Bay North of line from Cedar River MI to Rock Island Passage",
                wfo: "MQT",
            },
            CoastalMarineZone::LMZ248 => crate::ZoneDetails {
                state: "LM",
                zone: "248",
                zone_numeric: 248,
                name: "Seul Choix Point to Point Detour MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LMZ250 => crate::ZoneDetails {
                state: "LM",
                zone: "250",
                zone_numeric: 250,
                name: "5NM East of a line from Fairport MI to Rock Island Passage",
                wfo: "MQT",
            },
            CoastalMarineZone::LMZ261 => crate::ZoneDetails {
                state: "LM",
                zone: "261",
                zone_numeric: 261,
                name: "Lake Michigan from Seul Choix Point to Rock Island Passage 5NM offshore to Mid Lake",
                wfo: "MQT",
            },
            CoastalMarineZone::LMZ323 => crate::ZoneDetails {
                state: "LM",
                zone: "323",
                zone_numeric: 323,
                name: "Grand Traverse Bay south of a line Grand Traverse Light to Norwood MI",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ341 => crate::ZoneDetails {
                state: "LM",
                zone: "341",
                zone_numeric: 341,
                name: "Seul Choix Point to 5NM West of Mackinac Bridge",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ342 => crate::ZoneDetails {
                state: "LM",
                zone: "342",
                zone_numeric: 342,
                name: "Norwood MI to 5NM West of Mackinac Bridge including Little Traverse Bay",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ344 => crate::ZoneDetails {
                state: "LM",
                zone: "344",
                zone_numeric: 344,
                name: "Sleeping Bear Point to Grand Traverse Light MI",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ345 => crate::ZoneDetails {
                state: "LM",
                zone: "345",
                zone_numeric: 345,
                name: "Point Betsie to Sleeping Bear Point MI",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ346 => crate::ZoneDetails {
                state: "LM",
                zone: "346",
                zone_numeric: 346,
                name: "Manistee to Point Betsie MI",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ362 => crate::ZoneDetails {
                state: "LM",
                zone: "362",
                zone_numeric: 362,
                name: "Lake Michigan South of a line from Seul Choix Point to the Mackinac Bridge and North of a line from Charlevoix MI to South Fox Island 5NM offshore",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ364 => crate::ZoneDetails {
                state: "LM",
                zone: "364",
                zone_numeric: 364,
                name: "Lake Michigan from Charlevoix to Point Betsie MI 5NM Offshore to mid lake",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ366 => crate::ZoneDetails {
                state: "LM",
                zone: "366",
                zone_numeric: 366,
                name: "Lake Michigan from Point Betsie to Manistee MI 5NM offshore to Mid Lake",
                wfo: "APX",
            },
            CoastalMarineZone::LMZ521 => crate::ZoneDetails {
                state: "LM",
                zone: "521",
                zone_numeric: 521,
                name: "Green Bay south of line from  Cedar River to Rock Island Passage and north of a line from Oconto WI to Little Sturgeon Bay WI",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ522 => crate::ZoneDetails {
                state: "LM",
                zone: "522",
                zone_numeric: 522,
                name: "Green Bay south of line from  Oconto WI to Little Sturgeon Bay WI",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ541 => crate::ZoneDetails {
                state: "LM",
                zone: "541",
                zone_numeric: 541,
                name: "Rock Island Passage to Sturgeon Bay WI",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ542 => crate::ZoneDetails {
                state: "LM",
                zone: "542",
                zone_numeric: 542,
                name: "Sturgeon Bay to Two Rivers WI",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ543 => crate::ZoneDetails {
                state: "LM",
                zone: "543",
                zone_numeric: 543,
                name: "Two Rivers to Sheboygan WI",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ563 => crate::ZoneDetails {
                state: "LM",
                zone: "563",
                zone_numeric: 563,
                name: "Lake Michigan from Rock Island Passage to Sturgeon Bay WI 5NM offshore to mid lake",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ565 => crate::ZoneDetails {
                state: "LM",
                zone: "565",
                zone_numeric: 565,
                name: "Lake Michigan from Sturgeon Bay to Two Rivers WI 5NM offshore to Mid Lake",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ567 => crate::ZoneDetails {
                state: "LM",
                zone: "567",
                zone_numeric: 567,
                name: "Lake Michigan from Two Rivers to Sheboygan WI 5NM offshore to Mid Lake",
                wfo: "GRB",
            },
            CoastalMarineZone::LMZ643 => crate::ZoneDetails {
                state: "LM",
                zone: "643",
                zone_numeric: 643,
                name: "Sheboygan to Port Washington WI",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ644 => crate::ZoneDetails {
                state: "LM",
                zone: "644",
                zone_numeric: 644,
                name: "Port Washington to North Point Light WI",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ645 => crate::ZoneDetails {
                state: "LM",
                zone: "645",
                zone_numeric: 645,
                name: "North Point Light to Wind Point WI",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ646 => crate::ZoneDetails {
                state: "LM",
                zone: "646",
                zone_numeric: 646,
                name: "Wind Point WI to Winthrop Harbor IL",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ669 => crate::ZoneDetails {
                state: "LM",
                zone: "669",
                zone_numeric: 669,
                name: "Lake Michigan from Sheboygan to Port Washington WI 5NM offshore to Mid Lake",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ671 => crate::ZoneDetails {
                state: "LM",
                zone: "671",
                zone_numeric: 671,
                name: "Lake Michigan from Port Washington to North Point Light WI 5NM offshore to Mid Lake",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ673 => crate::ZoneDetails {
                state: "LM",
                zone: "673",
                zone_numeric: 673,
                name: "Lake Michigan from North Point Light to Wind Point WI 5NM offshore to Mid Lake",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ675 => crate::ZoneDetails {
                state: "LM",
                zone: "675",
                zone_numeric: 675,
                name: "Lake Michigan from Wind Point WI to Winthrop Harbor IL 5NM offshore to Mid Lake",
                wfo: "MKX",
            },
            CoastalMarineZone::LMZ740 => crate::ZoneDetails {
                state: "LM",
                zone: "740",
                zone_numeric: 740,
                name: "Winthrop Harbor to Wilmette Harbor IL",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ741 => crate::ZoneDetails {
                state: "LM",
                zone: "741",
                zone_numeric: 741,
                name: "Wilmette Harbor to Northerly Island IL",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ742 => crate::ZoneDetails {
                state: "LM",
                zone: "742",
                zone_numeric: 742,
                name: "Northerly Island to Calumet Harbor IL",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ743 => crate::ZoneDetails {
                state: "LM",
                zone: "743",
                zone_numeric: 743,
                name: "Calumet Harbor IL to Gary IN",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ744 => crate::ZoneDetails {
                state: "LM",
                zone: "744",
                zone_numeric: 744,
                name: "Gary to Burns Harbor IN",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ745 => crate::ZoneDetails {
                state: "LM",
                zone: "745",
                zone_numeric: 745,
                name: "Burns Harbor to Michigan City IN",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ777 => crate::ZoneDetails {
                state: "LM",
                zone: "777",
                zone_numeric: 777,
                name: "Lake Michigan from Winthrop Harbor to Wilmette Harbor IL 5NM offshore to Mid Lake",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ779 => crate::ZoneDetails {
                state: "LM",
                zone: "779",
                zone_numeric: 779,
                name: "Lake Michigan from Wilmette Harbor to Michigan City in 5NM offshore to Mid Lake",
                wfo: "LOT",
            },
            CoastalMarineZone::LMZ844 => crate::ZoneDetails {
                state: "LM",
                zone: "844",
                zone_numeric: 844,
                name: "St Joseph to South Haven MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ845 => crate::ZoneDetails {
                state: "LM",
                zone: "845",
                zone_numeric: 845,
                name: "South Haven to Holland MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ846 => crate::ZoneDetails {
                state: "LM",
                zone: "846",
                zone_numeric: 846,
                name: "Holland to Grand Haven MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ847 => crate::ZoneDetails {
                state: "LM",
                zone: "847",
                zone_numeric: 847,
                name: "Grand Haven to Whitehall MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ848 => crate::ZoneDetails {
                state: "LM",
                zone: "848",
                zone_numeric: 848,
                name: "Whitehall to Pentwater MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ849 => crate::ZoneDetails {
                state: "LM",
                zone: "849",
                zone_numeric: 849,
                name: "Pentwater to Manistee MI",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ868 => crate::ZoneDetails {
                state: "LM",
                zone: "868",
                zone_numeric: 868,
                name: "Lake Michigan from Pentwater to Manistee MI 5NM offshore to Mid Lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ870 => crate::ZoneDetails {
                state: "LM",
                zone: "870",
                zone_numeric: 870,
                name: "Lake Michigan from Whitehall to Pentwater MI 5NM offshore to Mid Lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ872 => crate::ZoneDetails {
                state: "LM",
                zone: "872",
                zone_numeric: 872,
                name: "Lake Michigan from Grand Haven to Whitehall MI 5NM offshore to Mid Lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ874 => crate::ZoneDetails {
                state: "LM",
                zone: "874",
                zone_numeric: 874,
                name: "Lake Michigan from Holland to Grand Haven MI 5NM offshore to Mid Lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ876 => crate::ZoneDetails {
                state: "LM",
                zone: "876",
                zone_numeric: 876,
                name: "Lake Michigan from South Haven to Holland MI 5NM offshore to Mid lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LMZ878 => crate::ZoneDetails {
                state: "LM",
                zone: "878",
                zone_numeric: 878,
                name: "Lake Michigan from St Joseph to South Haven MI 5NM offshore to Mid Lake",
                wfo: "GRR",
            },
            CoastalMarineZone::LOZ030 => crate::ZoneDetails {
                state: "LO",
                zone: "030",
                zone_numeric: 30,
                name: "Lower Niagara River",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ042 => crate::ZoneDetails {
                state: "LO",
                zone: "042",
                zone_numeric: 42,
                name: "Niagara River to Hamlin Beach NY",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ043 => crate::ZoneDetails {
                state: "LO",
                zone: "043",
                zone_numeric: 43,
                name: "Hamlin Beach to Sodus Bay NY",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ044 => crate::ZoneDetails {
                state: "LO",
                zone: "044",
                zone_numeric: 44,
                name: "Sodus Bay to Mexico Bay NY",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ045 => crate::ZoneDetails {
                state: "LO",
                zone: "045",
                zone_numeric: 45,
                name: "Mexico Bay NY to the St. Lawrence River",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ062 => crate::ZoneDetails {
                state: "LO",
                zone: "062",
                zone_numeric: 62,
                name: "Niagara River to Hamlin Beach NY beyond 5NM off shoreline to US-Canadian border",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ063 => crate::ZoneDetails {
                state: "LO",
                zone: "063",
                zone_numeric: 63,
                name: "Hamlin Beach to Sodus Bay NY beyond 5NM off shoreline to US-Canadian border",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ064 => crate::ZoneDetails {
                state: "LO",
                zone: "064",
                zone_numeric: 64,
                name: "Sodus Bay to Mexico Bay NY beyond 5NM off shoreline to US-Canadian border",
                wfo: "BUF",
            },
            CoastalMarineZone::LOZ065 => crate::ZoneDetails {
                state: "LO",
                zone: "065",
                zone_numeric: 65,
                name: "Mexico Bay NY to the St. Lawrence River beyond 5NM off shoreline to US-Canadian border",
                wfo: "BUF",
            },
            CoastalMarineZone::LSZ121 => crate::ZoneDetails {
                state: "LS",
                zone: "121",
                zone_numeric: 121,
                name: "Chequamegon Bay-Bayfield to Oak Point WI",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ140 => crate::ZoneDetails {
                state: "LS",
                zone: "140",
                zone_numeric: 140,
                name: "Grand Portage to Grand Marais MN",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ141 => crate::ZoneDetails {
                state: "LS",
                zone: "141",
                zone_numeric: 141,
                name: "Grand Marais to Taconite Harbor MN",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ142 => crate::ZoneDetails {
                state: "LS",
                zone: "142",
                zone_numeric: 142,
                name: "Taconite Harbor to Silver Bay Harbor MN",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ143 => crate::ZoneDetails {
                state: "LS",
                zone: "143",
                zone_numeric: 143,
                name: "Silver Bay Harbor to Two Harbors MN",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ144 => crate::ZoneDetails {
                state: "LS",
                zone: "144",
                zone_numeric: 144,
                name: "Two Harbors to Duluth MN",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ145 => crate::ZoneDetails {
                state: "LS",
                zone: "145",
                zone_numeric: 145,
                name: "Duluth MN to Port Wing WI",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ146 => crate::ZoneDetails {
                state: "LS",
                zone: "146",
                zone_numeric: 146,
                name: "Port Wing to Sand Island WI",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ147 => crate::ZoneDetails {
                state: "LS",
                zone: "147",
                zone_numeric: 147,
                name: "Sand Island to Bayfield WI",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ148 => crate::ZoneDetails {
                state: "LS",
                zone: "148",
                zone_numeric: 148,
                name: "Oak Point to Saxon Harbor WI",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ150 => crate::ZoneDetails {
                state: "LS",
                zone: "150",
                zone_numeric: 150,
                name: "Outer Apostle Islands Beyond 5 NM from Mainland",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ162 => crate::ZoneDetails {
                state: "LS",
                zone: "162",
                zone_numeric: 162,
                name: "Lake Superior west of a line from Saxon Harbor WI to Grand Portage MN beyond 5NM",
                wfo: "DLH",
            },
            CoastalMarineZone::LSZ240 => crate::ZoneDetails {
                state: "LS",
                zone: "240",
                zone_numeric: 240,
                name: "Saxon Harbor WI to Black River MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ241 => crate::ZoneDetails {
                state: "LS",
                zone: "241",
                zone_numeric: 241,
                name: "Black River To Ontonagon MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ242 => crate::ZoneDetails {
                state: "LS",
                zone: "242",
                zone_numeric: 242,
                name: "Ontonagon to Upper Entrance of Portage Canal MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ243 => crate::ZoneDetails {
                state: "LS",
                zone: "243",
                zone_numeric: 243,
                name: "Upper Entrance of Portage Canal to Eagle River MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ244 => crate::ZoneDetails {
                state: "LS",
                zone: "244",
                zone_numeric: 244,
                name: "Eagle River to Manitou Island MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ245 => crate::ZoneDetails {
                state: "LS",
                zone: "245",
                zone_numeric: 245,
                name: "Manitou Island to Point Isabelle MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ246 => crate::ZoneDetails {
                state: "LS",
                zone: "246",
                zone_numeric: 246,
                name: "Point Isabelle to Lower Entrance of Portage Canal MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ247 => crate::ZoneDetails {
                state: "LS",
                zone: "247",
                zone_numeric: 247,
                name: "Portage Lake to Huron Island MI to Lower Entrance of Portage Canal To Huron Islands MI Including Keweenaw and Huron Bays",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ248 => crate::ZoneDetails {
                state: "LS",
                zone: "248",
                zone_numeric: 248,
                name: "Huron Islands to Marquette MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ249 => crate::ZoneDetails {
                state: "LS",
                zone: "249",
                zone_numeric: 249,
                name: "Marquette to Munising MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ250 => crate::ZoneDetails {
                state: "LS",
                zone: "250",
                zone_numeric: 250,
                name: "Munising to Grand Marais MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ251 => crate::ZoneDetails {
                state: "LS",
                zone: "251",
                zone_numeric: 251,
                name: "Grand Marais to Whitefish Point MI",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ263 => crate::ZoneDetails {
                state: "LS",
                zone: "263",
                zone_numeric: 263,
                name: "Lake Superior from Saxon Harbor WI to Upper Entrance to Portage Canal MI 5NM off shore to the US/Canadian border including Isle Royal National Park",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ264 => crate::ZoneDetails {
                state: "LS",
                zone: "264",
                zone_numeric: 264,
                name: "Lake Superior from Upper Entrance to Portage Canal to Manitou Island MI 5NM off shore to the US/Canadian Border",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ265 => crate::ZoneDetails {
                state: "LS",
                zone: "265",
                zone_numeric: 265,
                name: "Lake Superior West of Line from Manitou Island to Marquette MI Beyond 5NM from shore",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ266 => crate::ZoneDetails {
                state: "LS",
                zone: "266",
                zone_numeric: 266,
                name: "Lake Superior East of a line from Manitou Island to Marquette MI and West of a line from Grand Marais MI to the US/Canadian Border Beyond 5NM from shore",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ267 => crate::ZoneDetails {
                state: "LS",
                zone: "267",
                zone_numeric: 267,
                name: "Lake Superior from Grand Marais MI to Whitefish Point MI 5NM off shore to the US/Canadian border",
                wfo: "MQT",
            },
            CoastalMarineZone::LSZ321 => crate::ZoneDetails {
                state: "LS",
                zone: "321",
                zone_numeric: 321,
                name: "Whitefish Bay (U.S. Portion)/Whitefish Point to Point Iroquois MI",
                wfo: "APX",
            },
            CoastalMarineZone::LSZ322 => crate::ZoneDetails {
                state: "LS",
                zone: "322",
                zone_numeric: 322,
                name: "St. Marys River Point Iroquois to E. Potagannissing Bay",
                wfo: "APX",
            },
            CoastalMarineZone::PHZ110 => crate::ZoneDetails {
                state: "PH",
                zone: "110",
                zone_numeric: 110,
                name: "Kauai Northwest Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ111 => crate::ZoneDetails {
                state: "PH",
                zone: "111",
                zone_numeric: 111,
                name: "Kauai Windward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ112 => crate::ZoneDetails {
                state: "PH",
                zone: "112",
                zone_numeric: 112,
                name: "Kauai Leeward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ113 => crate::ZoneDetails {
                state: "PH",
                zone: "113",
                zone_numeric: 113,
                name: "Kauai Channel",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ114 => crate::ZoneDetails {
                state: "PH",
                zone: "114",
                zone_numeric: 114,
                name: "Oahu Windward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ115 => crate::ZoneDetails {
                state: "PH",
                zone: "115",
                zone_numeric: 115,
                name: "Oahu Leeward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ116 => crate::ZoneDetails {
                state: "PH",
                zone: "116",
                zone_numeric: 116,
                name: "Kaiwi Channel",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ117 => crate::ZoneDetails {
                state: "PH",
                zone: "117",
                zone_numeric: 117,
                name: "Maui County Windward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ118 => crate::ZoneDetails {
                state: "PH",
                zone: "118",
                zone_numeric: 118,
                name: "Maui County Leeward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ119 => crate::ZoneDetails {
                state: "PH",
                zone: "119",
                zone_numeric: 119,
                name: "Maalaea Bay",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ120 => crate::ZoneDetails {
                state: "PH",
                zone: "120",
                zone_numeric: 120,
                name: "Pailolo Channel",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ121 => crate::ZoneDetails {
                state: "PH",
                zone: "121",
                zone_numeric: 121,
                name: "Alenuihaha Channel",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ122 => crate::ZoneDetails {
                state: "PH",
                zone: "122",
                zone_numeric: 122,
                name: "Big Island Windward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ123 => crate::ZoneDetails {
                state: "PH",
                zone: "123",
                zone_numeric: 123,
                name: "Big Island Leeward Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PHZ124 => crate::ZoneDetails {
                state: "PH",
                zone: "124",
                zone_numeric: 124,
                name: "Big Island Southeast Waters",
                wfo: "HFO",
            },
            CoastalMarineZone::PKZ011 => crate::ZoneDetails {
                state: "PK",
                zone: "011",
                zone_numeric: 11,
                name: "Glacier Bay",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ012 => crate::ZoneDetails {
                state: "PK",
                zone: "012",
                zone_numeric: 12,
                name: "Northern Lynn Canal",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ013 => crate::ZoneDetails {
                state: "PK",
                zone: "013",
                zone_numeric: 13,
                name: "Southern Lynn Canal",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ021 => crate::ZoneDetails {
                state: "PK",
                zone: "021",
                zone_numeric: 21,
                name: "Icy Strait",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ022 => crate::ZoneDetails {
                state: "PK",
                zone: "022",
                zone_numeric: 22,
                name: "Cross Sound",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ031 => crate::ZoneDetails {
                state: "PK",
                zone: "031",
                zone_numeric: 31,
                name: "Stephens Passage",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ032 => crate::ZoneDetails {
                state: "PK",
                zone: "032",
                zone_numeric: 32,
                name: "Northern Chatham Strait",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ033 => crate::ZoneDetails {
                state: "PK",
                zone: "033",
                zone_numeric: 33,
                name: "Southern Chatham Strait",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ034 => crate::ZoneDetails {
                state: "PK",
                zone: "034",
                zone_numeric: 34,
                name: "Frederick Sound",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ035 => crate::ZoneDetails {
                state: "PK",
                zone: "035",
                zone_numeric: 35,
                name: "Sumner Strait",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ036 => crate::ZoneDetails {
                state: "PK",
                zone: "036",
                zone_numeric: 36,
                name: "Clarence Strait",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ041 => crate::ZoneDetails {
                state: "PK",
                zone: "041",
                zone_numeric: 41,
                name: "Dixon Entrance to Cape Decision",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ042 => crate::ZoneDetails {
                state: "PK",
                zone: "042",
                zone_numeric: 42,
                name: "Cape Decision to Cape Edgecumbe",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ043 => crate::ZoneDetails {
                state: "PK",
                zone: "043",
                zone_numeric: 43,
                name: "Southeast Alaska Outside Waters From Cape Edgecumbe to Cape Fairweather",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ051 => crate::ZoneDetails {
                state: "PK",
                zone: "051",
                zone_numeric: 51,
                name: "Cape Fairweather to Icy Cape",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ052 => crate::ZoneDetails {
                state: "PK",
                zone: "052",
                zone_numeric: 52,
                name: "Icy Cape to Cape Suckling",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ053 => crate::ZoneDetails {
                state: "PK",
                zone: "053",
                zone_numeric: 53,
                name: "Yakutat Bay",
                wfo: "AJK",
            },
            CoastalMarineZone::PKZ119 => crate::ZoneDetails {
                state: "PK",
                zone: "119",
                zone_numeric: 119,
                name: "Cape Suckling to Cape Cleare",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ120 => crate::ZoneDetails {
                state: "PK",
                zone: "120",
                zone_numeric: 120,
                name: "Cape Cleare to Gore Point",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ121 => crate::ZoneDetails {
                state: "PK",
                zone: "121",
                zone_numeric: 121,
                name: "Resurrection Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ125 => crate::ZoneDetails {
                state: "PK",
                zone: "125",
                zone_numeric: 125,
                name: "Prince William Sound",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ126 => crate::ZoneDetails {
                state: "PK",
                zone: "126",
                zone_numeric: 126,
                name: "Port of Valdez",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ127 => crate::ZoneDetails {
                state: "PK",
                zone: "127",
                zone_numeric: 127,
                name: "Valdez Narrows",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ128 => crate::ZoneDetails {
                state: "PK",
                zone: "128",
                zone_numeric: 128,
                name: "Valdez Arm",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ129 => crate::ZoneDetails {
                state: "PK",
                zone: "129",
                zone_numeric: 129,
                name: "Western Prince William Sound",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ130 => crate::ZoneDetails {
                state: "PK",
                zone: "130",
                zone_numeric: 130,
                name: "West of Barren Islands Including Kamishak Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ131 => crate::ZoneDetails {
                state: "PK",
                zone: "131",
                zone_numeric: 131,
                name: "Barren Islands East",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ132 => crate::ZoneDetails {
                state: "PK",
                zone: "132",
                zone_numeric: 132,
                name: "Shuyak Island To Sitkinak",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ136 => crate::ZoneDetails {
                state: "PK",
                zone: "136",
                zone_numeric: 136,
                name: "Chiniak Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ137 => crate::ZoneDetails {
                state: "PK",
                zone: "137",
                zone_numeric: 137,
                name: "Marmot Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ138 => crate::ZoneDetails {
                state: "PK",
                zone: "138",
                zone_numeric: 138,
                name: "Shelikof Strait",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ139 => crate::ZoneDetails {
                state: "PK",
                zone: "139",
                zone_numeric: 139,
                name: "Cook Inlet Kalgin Island to Point Bede",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ140 => crate::ZoneDetails {
                state: "PK",
                zone: "140",
                zone_numeric: 140,
                name: "Cook Inlet North Kalgin Island",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ141 => crate::ZoneDetails {
                state: "PK",
                zone: "141",
                zone_numeric: 141,
                name: "Kachemak Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ150 => crate::ZoneDetails {
                state: "PK",
                zone: "150",
                zone_numeric: 150,
                name: "Sitkinak to Castle Cape",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ155 => crate::ZoneDetails {
                state: "PK",
                zone: "155",
                zone_numeric: 155,
                name: "Castle Cape to Cape Sarichef",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ160 => crate::ZoneDetails {
                state: "PK",
                zone: "160",
                zone_numeric: 160,
                name: "Bristol Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ165 => crate::ZoneDetails {
                state: "PK",
                zone: "165",
                zone_numeric: 165,
                name: "Port Heiden to Cape Sarichef",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ170 => crate::ZoneDetails {
                state: "PK",
                zone: "170",
                zone_numeric: 170,
                name: "Cape Sarichef to Nikoski Bering Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ171 => crate::ZoneDetails {
                state: "PK",
                zone: "171",
                zone_numeric: 171,
                name: "Unalaska Bay",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ172 => crate::ZoneDetails {
                state: "PK",
                zone: "172",
                zone_numeric: 172,
                name: "Cape Sarichef to Nikoski Pacific Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ173 => crate::ZoneDetails {
                state: "PK",
                zone: "173",
                zone_numeric: 173,
                name: "Nikolski to Seguam Bering Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ174 => crate::ZoneDetails {
                state: "PK",
                zone: "174",
                zone_numeric: 174,
                name: "Nikolski to Seguam Pacific Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ175 => crate::ZoneDetails {
                state: "PK",
                zone: "175",
                zone_numeric: 175,
                name: "Seguam to Adak Bering Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ176 => crate::ZoneDetails {
                state: "PK",
                zone: "176",
                zone_numeric: 176,
                name: "Seguam to Adak Pacific Side",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ177 => crate::ZoneDetails {
                state: "PK",
                zone: "177",
                zone_numeric: 177,
                name: "Adak to Kiska",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ178 => crate::ZoneDetails {
                state: "PK",
                zone: "178",
                zone_numeric: 178,
                name: "Kiska to Attu",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ179 => crate::ZoneDetails {
                state: "PK",
                zone: "179",
                zone_numeric: 179,
                name: "Pribilof Islands Nearshore Waters",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ180 => crate::ZoneDetails {
                state: "PK",
                zone: "180",
                zone_numeric: 180,
                name: "Kuskokwim Delta and Etolin Strait",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ181 => crate::ZoneDetails {
                state: "PK",
                zone: "181",
                zone_numeric: 181,
                name: "North and West of Nunivak Island",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ185 => crate::ZoneDetails {
                state: "PK",
                zone: "185",
                zone_numeric: 185,
                name: "St Matthew Island Waters",
                wfo: "AFC",
            },
            CoastalMarineZone::PKZ200 => crate::ZoneDetails {
                state: "PK",
                zone: "200",
                zone_numeric: 200,
                name: "Norton Sound",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ201 => crate::ZoneDetails {
                state: "PK",
                zone: "201",
                zone_numeric: 201,
                name: "Etolin Strait to Dall Point",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ210 => crate::ZoneDetails {
                state: "PK",
                zone: "210",
                zone_numeric: 210,
                name: "Dall Point to Wales",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ215 => crate::ZoneDetails {
                state: "PK",
                zone: "215",
                zone_numeric: 215,
                name: "Kotzebue Sound",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ220 => crate::ZoneDetails {
                state: "PK",
                zone: "220",
                zone_numeric: 220,
                name: "Wales to Cape Thompson",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ225 => crate::ZoneDetails {
                state: "PK",
                zone: "225",
                zone_numeric: 225,
                name: "Cape Thompson to Cape Beaufort",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ230 => crate::ZoneDetails {
                state: "PK",
                zone: "230",
                zone_numeric: 230,
                name: "Cape Beaufort to Point Franklin",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ235 => crate::ZoneDetails {
                state: "PK",
                zone: "235",
                zone_numeric: 235,
                name: "Point Franklin to Cape Halkett",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ240 => crate::ZoneDetails {
                state: "PK",
                zone: "240",
                zone_numeric: 240,
                name: "Cape Halkett to Flaxman Island",
                wfo: "AFG",
            },
            CoastalMarineZone::PKZ245 => crate::ZoneDetails {
                state: "PK",
                zone: "245",
                zone_numeric: 245,
                name: "Flaxman Island to Demarcation Point",
                wfo: "AFG",
            },
            CoastalMarineZone::PMZ151 => crate::ZoneDetails {
                state: "PM",
                zone: "151",
                zone_numeric: 151,
                name: "Guam Coastal Waters",
                wfo: "GUM",
            },
            CoastalMarineZone::PMZ152 => crate::ZoneDetails {
                state: "PM",
                zone: "152",
                zone_numeric: 152,
                name: "Rota Coastal Waters",
                wfo: "GUM",
            },
            CoastalMarineZone::PMZ153 => crate::ZoneDetails {
                state: "PM",
                zone: "153",
                zone_numeric: 153,
                name: "Tinian Coastal Waters",
                wfo: "GUM",
            },
            CoastalMarineZone::PMZ154 => crate::ZoneDetails {
                state: "PM",
                zone: "154",
                zone_numeric: 154,
                name: "Saipan Coastal Waters",
                wfo: "GUM",
            },
            CoastalMarineZone::PMZ161 => crate::ZoneDetails {
                state: "PM",
                zone: "161",
                zone_numeric: 161,
                name: "Koror Palau Coastal Waters",
                wfo: "PQW",
            },
            CoastalMarineZone::PMZ171 => crate::ZoneDetails {
                state: "PM",
                zone: "171",
                zone_numeric: 171,
                name: "Yap Coastal Waters",
                wfo: "PQW",
            },
            CoastalMarineZone::PMZ172 => crate::ZoneDetails {
                state: "PM",
                zone: "172",
                zone_numeric: 172,
                name: "Chuuk Coastal Waters",
                wfo: "PQW",
            },
            CoastalMarineZone::PMZ173 => crate::ZoneDetails {
                state: "PM",
                zone: "173",
                zone_numeric: 173,
                name: "Pohnpei Coastal Waters",
                wfo: "PQE",
            },
            CoastalMarineZone::PMZ174 => crate::ZoneDetails {
                state: "PM",
                zone: "174",
                zone_numeric: 174,
                name: "Kosrae Coastal Waters",
                wfo: "PQE",
            },
            CoastalMarineZone::PMZ181 => crate::ZoneDetails {
                state: "PM",
                zone: "181",
                zone_numeric: 181,
                name: "Majuro Coastal Waters",
                wfo: "PQE",
            },
            CoastalMarineZone::PMZ191 => crate::ZoneDetails {
                state: "PM",
                zone: "191",
                zone_numeric: 191,
                name: "Waters out to 40 Nautical Miles",
                wfo: "GUM",
            },
            CoastalMarineZone::PSZ150 => crate::ZoneDetails {
                state: "PS",
                zone: "150",
                zone_numeric: 150,
                name: "Coastal waters of Tututila and Aunuu",
                wfo: "STU",
            },
            CoastalMarineZone::PSZ151 => crate::ZoneDetails {
                state: "PS",
                zone: "151",
                zone_numeric: 151,
                name: "Coastal waters of Manua",
                wfo: "STU",
            },
            CoastalMarineZone::PSZ152 => crate::ZoneDetails {
                state: "PS",
                zone: "152",
                zone_numeric: 152,
                name: "Coastal waters of Swain's Island",
                wfo: "STU",
            },
            CoastalMarineZone::PZZ110 => crate::ZoneDetails {
                state: "PZ",
                zone: "110",
                zone_numeric: 110,
                name: "Grays Harbor Bar",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ130 => crate::ZoneDetails {
                state: "PZ",
                zone: "130",
                zone_numeric: 130,
                name: "West Entrance U.S. Waters Strait Of Juan De Fuca",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ131 => crate::ZoneDetails {
                state: "PZ",
                zone: "131",
                zone_numeric: 131,
                name: "Central U.S. Waters Strait Of Juan De Fuca",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ132 => crate::ZoneDetails {
                state: "PZ",
                zone: "132",
                zone_numeric: 132,
                name: "East Entrance U.S. Waters Strait Of Juan De Fuca",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ133 => crate::ZoneDetails {
                state: "PZ",
                zone: "133",
                zone_numeric: 133,
                name: "Northern Inland Waters Including The San Juan Islands",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ134 => crate::ZoneDetails {
                state: "PZ",
                zone: "134",
                zone_numeric: 134,
                name: "Admiralty Inlet",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ135 => crate::ZoneDetails {
                state: "PZ",
                zone: "135",
                zone_numeric: 135,
                name: "Puget Sound and Hood Canal",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ150 => crate::ZoneDetails {
                state: "PZ",
                zone: "150",
                zone_numeric: 150,
                name: "Coastal Waters From Cape Flattery To James Island Out 10 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ153 => crate::ZoneDetails {
                state: "PZ",
                zone: "153",
                zone_numeric: 153,
                name: "Coastal Waters From James Island To Point Grenville Out 10 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ156 => crate::ZoneDetails {
                state: "PZ",
                zone: "156",
                zone_numeric: 156,
                name: "Coastal Waters From Point Grenville To Cape Shoalwater Out 10 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ170 => crate::ZoneDetails {
                state: "PZ",
                zone: "170",
                zone_numeric: 170,
                name: "Coastal Waters From Cape Flattery To James Island 10 To 60 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ173 => crate::ZoneDetails {
                state: "PZ",
                zone: "173",
                zone_numeric: 173,
                name: "Waters From James Island To Point Grenville 10 To 60 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ176 => crate::ZoneDetails {
                state: "PZ",
                zone: "176",
                zone_numeric: 176,
                name: "Coastal Waters From Point Grenville To Cape Shoalwater 10 To 60 Nm",
                wfo: "SEW",
            },
            CoastalMarineZone::PZZ210 => crate::ZoneDetails {
                state: "PZ",
                zone: "210",
                zone_numeric: 210,
                name: "Columbia River Bar",
                wfo: "PQR",
            },
            CoastalMarineZone::PZZ250 => crate::ZoneDetails {
                state: "PZ",
                zone: "250",
                zone_numeric: 250,
                name: "Coastal waters from Cape Shoalwater WA to Cascade Head OR out 10 nm",
                wfo: "PQR",
            },
            CoastalMarineZone::PZZ255 => crate::ZoneDetails {
                state: "PZ",
                zone: "255",
                zone_numeric: 255,
                name: "Coastal waters from Cascade Head to Florence OR out 10 nm",
                wfo: "PQR",
            },
            CoastalMarineZone::PZZ270 => crate::ZoneDetails {
                state: "PZ",
                zone: "270",
                zone_numeric: 270,
                name: "Waters from Cape Shoalwater WA to Cascade Head OR from 10 to 60 nm",
                wfo: "PQR",
            },
            CoastalMarineZone::PZZ275 => crate::ZoneDetails {
                state: "PZ",
                zone: "275",
                zone_numeric: 275,
                name: "Waters from Cascade Head to Florence OR from 10 to 60 nm",
                wfo: "PQR",
            },
            CoastalMarineZone::PZZ350 => crate::ZoneDetails {
                state: "PZ",
                zone: "350",
                zone_numeric: 350,
                name: "Coastal waters from Florence to Cape Blanco OR out 10 nm",
                wfo: "MFR",
            },
            CoastalMarineZone::PZZ356 => crate::ZoneDetails {
                state: "PZ",
                zone: "356",
                zone_numeric: 356,
                name: "Coastal waters from Cape Blanco OR to Pt. St. George CA out 10 nm",
                wfo: "MFR",
            },
            CoastalMarineZone::PZZ370 => crate::ZoneDetails {
                state: "PZ",
                zone: "370",
                zone_numeric: 370,
                name: "Waters from Florence to Cape Blanco OR from 10 to 60 nm",
                wfo: "MFR",
            },
            CoastalMarineZone::PZZ376 => crate::ZoneDetails {
                state: "PZ",
                zone: "376",
                zone_numeric: 376,
                name: "Waters from Cape Blanco OR to Pt. St. George CA from 10 to 60 nm",
                wfo: "MFR",
            },
            CoastalMarineZone::PZZ410 => crate::ZoneDetails {
                state: "PZ",
                zone: "410",
                zone_numeric: 410,
                name: "Humboldt Bay Bar",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ415 => crate::ZoneDetails {
                state: "PZ",
                zone: "415",
                zone_numeric: 415,
                name: "Humboldt Bay",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ450 => crate::ZoneDetails {
                state: "PZ",
                zone: "450",
                zone_numeric: 450,
                name: "Coastal waters from Pt. St. George to Cape Mendocino CA out 10 nm",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ455 => crate::ZoneDetails {
                state: "PZ",
                zone: "455",
                zone_numeric: 455,
                name: "Coastal waters from Cape Mendocino to Pt. Arena CA out 10 nm",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ470 => crate::ZoneDetails {
                state: "PZ",
                zone: "470",
                zone_numeric: 470,
                name: "Waters from Pt. St. George to Cape Mendocino CA from 10 to 60 nm",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ475 => crate::ZoneDetails {
                state: "PZ",
                zone: "475",
                zone_numeric: 475,
                name: "Waters from Cape Mendocino to Pt. Arena CA from 10 to 60 nm",
                wfo: "EKA",
            },
            CoastalMarineZone::PZZ530 => crate::ZoneDetails {
                state: "PZ",
                zone: "530",
                zone_numeric: 530,
                name: "San Pablo Bay, Suisun Bay, the West Delta and  the San Francisco Bay north of the Bay Bridge",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ531 => crate::ZoneDetails {
                state: "PZ",
                zone: "531",
                zone_numeric: 531,
                name: "San Francisco Bay South of the Bay Bridge",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ535 => crate::ZoneDetails {
                state: "PZ",
                zone: "535",
                zone_numeric: 535,
                name: "Monterey Bay",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ540 => crate::ZoneDetails {
                state: "PZ",
                zone: "540",
                zone_numeric: 540,
                name: "Coastal Waters from Point Arena to Point Reyes California out to 10 nm",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ545 => crate::ZoneDetails {
                state: "PZ",
                zone: "545",
                zone_numeric: 545,
                name: "Coastal Waters from Point Reyes to Pigeon Point California out to 10 nm",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ560 => crate::ZoneDetails {
                state: "PZ",
                zone: "560",
                zone_numeric: 560,
                name: "Coastal Waters from Pigeon Point to Point Pinos California out to 10 nm",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ565 => crate::ZoneDetails {
                state: "PZ",
                zone: "565",
                zone_numeric: 565,
                name: "Coastal Waters from Point Pinos to Point Piedras Blancas California out to 10 nm",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ570 => crate::ZoneDetails {
                state: "PZ",
                zone: "570",
                zone_numeric: 570,
                name: "Waters from Point Arena to Point Reyes 10-60 NM",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ571 => crate::ZoneDetails {
                state: "PZ",
                zone: "571",
                zone_numeric: 571,
                name: "Waters from Point Reyes to Pigeon Point 10-60 NM",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ575 => crate::ZoneDetails {
                state: "PZ",
                zone: "575",
                zone_numeric: 575,
                name: "Waters from Pigeon Point to Point Pinos 10-60 NM",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ576 => crate::ZoneDetails {
                state: "PZ",
                zone: "576",
                zone_numeric: 576,
                name: "Waters from Point Pinos to Point Piedras Blancas 10-60 NM",
                wfo: "MTR",
            },
            CoastalMarineZone::PZZ645 => crate::ZoneDetails {
                state: "PZ",
                zone: "645",
                zone_numeric: 645,
                name: "Point Piedras Blancas to Point Sal westward out to 10 NM",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ650 => crate::ZoneDetails {
                state: "PZ",
                zone: "650",
                zone_numeric: 650,
                name: "East Santa Barbara Channel from Pt. Conception to Pt. Mugu CA including Santa Cruz Island",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ655 => crate::ZoneDetails {
                state: "PZ",
                zone: "655",
                zone_numeric: 655,
                name: "Inner waters from Point Mugu to San Mateo Pt. CA including Santa Catalina and Anacapa Islands",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ670 => crate::ZoneDetails {
                state: "PZ",
                zone: "670",
                zone_numeric: 670,
                name: "Point Piedras Blancas to Point Sal from 10 to 60 NM",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ673 => crate::ZoneDetails {
                state: "PZ",
                zone: "673",
                zone_numeric: 673,
                name: "Waters from Pt. Sal to Santa Cruz Island CA and westward 60 nm including San Miguel and Santa Rosa Islands",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ676 => crate::ZoneDetails {
                state: "PZ",
                zone: "676",
                zone_numeric: 676,
                name: "Outer waters from Santa Cruz Island to San Clemente Island to 60 NM offshore including San Nicolas and Santa Barbara Islands",
                wfo: "LOX",
            },
            CoastalMarineZone::PZZ750 => crate::ZoneDetails {
                state: "PZ",
                zone: "750",
                zone_numeric: 750,
                name: "Coastal Waters from San Mateo Point to the Mexican Border and out to 30 nm",
                wfo: "SGX",
            },
            CoastalMarineZone::PZZ775 => crate::ZoneDetails {
                state: "PZ",
                zone: "775",
                zone_numeric: 775,
                name: "Waters from San Mateo point to the Mexican Border Extending 30 to 60 nm out including San Clemente Island",
                wfo: "SGX",
            },
            CoastalMarineZone::SLZ022 => crate::ZoneDetails {
                state: "SL",
                zone: "022",
                zone_numeric: 22,
                name: "St. Lawrence River above Ogdensburg NY",
                wfo: "BUF",
            },
            CoastalMarineZone::SLZ024 => crate::ZoneDetails {
                state: "SL",
                zone: "024",
                zone_numeric: 24,
                name: "St. Lawrence River from Ogdensburg to St. Regis NY",
                wfo: "BUF",
            },
        }
    }
    pub fn new(two: &str, numeric: u16) -> Option<Self> {
        match two {
            "AM" => match numeric {
                131 => Some(CoastalMarineZone::AMZ131),
                135 => Some(CoastalMarineZone::AMZ135),
                136 => Some(CoastalMarineZone::AMZ136),
                137 => Some(CoastalMarineZone::AMZ137),
                150 => Some(CoastalMarineZone::AMZ150),
                152 => Some(CoastalMarineZone::AMZ152),
                154 => Some(CoastalMarineZone::AMZ154),
                156 => Some(CoastalMarineZone::AMZ156),
                158 => Some(CoastalMarineZone::AMZ158),
                170 => Some(CoastalMarineZone::AMZ170),
                172 => Some(CoastalMarineZone::AMZ172),
                174 => Some(CoastalMarineZone::AMZ174),
                176 => Some(CoastalMarineZone::AMZ176),
                178 => Some(CoastalMarineZone::AMZ178),
                230 => Some(CoastalMarineZone::AMZ230),
                231 => Some(CoastalMarineZone::AMZ231),
                250 => Some(CoastalMarineZone::AMZ250),
                252 => Some(CoastalMarineZone::AMZ252),
                254 => Some(CoastalMarineZone::AMZ254),
                256 => Some(CoastalMarineZone::AMZ256),
                270 => Some(CoastalMarineZone::AMZ270),
                272 => Some(CoastalMarineZone::AMZ272),
                274 => Some(CoastalMarineZone::AMZ274),
                276 => Some(CoastalMarineZone::AMZ276),
                330 => Some(CoastalMarineZone::AMZ330),
                350 => Some(CoastalMarineZone::AMZ350),
                352 => Some(CoastalMarineZone::AMZ352),
                354 => Some(CoastalMarineZone::AMZ354),
                370 => Some(CoastalMarineZone::AMZ370),
                372 => Some(CoastalMarineZone::AMZ372),
                374 => Some(CoastalMarineZone::AMZ374),
                450 => Some(CoastalMarineZone::AMZ450),
                452 => Some(CoastalMarineZone::AMZ452),
                454 => Some(CoastalMarineZone::AMZ454),
                470 => Some(CoastalMarineZone::AMZ470),
                472 => Some(CoastalMarineZone::AMZ472),
                474 => Some(CoastalMarineZone::AMZ474),
                550 => Some(CoastalMarineZone::AMZ550),
                552 => Some(CoastalMarineZone::AMZ552),
                555 => Some(CoastalMarineZone::AMZ555),
                570 => Some(CoastalMarineZone::AMZ570),
                572 => Some(CoastalMarineZone::AMZ572),
                575 => Some(CoastalMarineZone::AMZ575),
                610 => Some(CoastalMarineZone::AMZ610),
                630 => Some(CoastalMarineZone::AMZ630),
                650 => Some(CoastalMarineZone::AMZ650),
                651 => Some(CoastalMarineZone::AMZ651),
                670 => Some(CoastalMarineZone::AMZ670),
                671 => Some(CoastalMarineZone::AMZ671),
                710 => Some(CoastalMarineZone::AMZ710),
                712 => Some(CoastalMarineZone::AMZ712),
                715 => Some(CoastalMarineZone::AMZ715),
                722 => Some(CoastalMarineZone::AMZ722),
                725 => Some(CoastalMarineZone::AMZ725),
                732 => Some(CoastalMarineZone::AMZ732),
                735 => Some(CoastalMarineZone::AMZ735),
                741 => Some(CoastalMarineZone::AMZ741),
                742 => Some(CoastalMarineZone::AMZ742),
                745 => Some(CoastalMarineZone::AMZ745),
                _ => None,
            },
            "AN" => match numeric {
                050 => Some(CoastalMarineZone::ANZ050),
                051 => Some(CoastalMarineZone::ANZ051),
                052 => Some(CoastalMarineZone::ANZ052),
                070 => Some(CoastalMarineZone::ANZ070),
                071 => Some(CoastalMarineZone::ANZ071),
                150 => Some(CoastalMarineZone::ANZ150),
                151 => Some(CoastalMarineZone::ANZ151),
                152 => Some(CoastalMarineZone::ANZ152),
                153 => Some(CoastalMarineZone::ANZ153),
                154 => Some(CoastalMarineZone::ANZ154),
                170 => Some(CoastalMarineZone::ANZ170),
                172 => Some(CoastalMarineZone::ANZ172),
                174 => Some(CoastalMarineZone::ANZ174),
                230 => Some(CoastalMarineZone::ANZ230),
                231 => Some(CoastalMarineZone::ANZ231),
                232 => Some(CoastalMarineZone::ANZ232),
                233 => Some(CoastalMarineZone::ANZ233),
                234 => Some(CoastalMarineZone::ANZ234),
                235 => Some(CoastalMarineZone::ANZ235),
                236 => Some(CoastalMarineZone::ANZ236),
                237 => Some(CoastalMarineZone::ANZ237),
                250 => Some(CoastalMarineZone::ANZ250),
                251 => Some(CoastalMarineZone::ANZ251),
                254 => Some(CoastalMarineZone::ANZ254),
                255 => Some(CoastalMarineZone::ANZ255),
                256 => Some(CoastalMarineZone::ANZ256),
                270 => Some(CoastalMarineZone::ANZ270),
                271 => Some(CoastalMarineZone::ANZ271),
                272 => Some(CoastalMarineZone::ANZ272),
                273 => Some(CoastalMarineZone::ANZ273),
                331 => Some(CoastalMarineZone::ANZ331),
                332 => Some(CoastalMarineZone::ANZ332),
                335 => Some(CoastalMarineZone::ANZ335),
                338 => Some(CoastalMarineZone::ANZ338),
                340 => Some(CoastalMarineZone::ANZ340),
                345 => Some(CoastalMarineZone::ANZ345),
                350 => Some(CoastalMarineZone::ANZ350),
                353 => Some(CoastalMarineZone::ANZ353),
                355 => Some(CoastalMarineZone::ANZ355),
                370 => Some(CoastalMarineZone::ANZ370),
                373 => Some(CoastalMarineZone::ANZ373),
                375 => Some(CoastalMarineZone::ANZ375),
                430 => Some(CoastalMarineZone::ANZ430),
                431 => Some(CoastalMarineZone::ANZ431),
                450 => Some(CoastalMarineZone::ANZ450),
                451 => Some(CoastalMarineZone::ANZ451),
                452 => Some(CoastalMarineZone::ANZ452),
                453 => Some(CoastalMarineZone::ANZ453),
                454 => Some(CoastalMarineZone::ANZ454),
                455 => Some(CoastalMarineZone::ANZ455),
                470 => Some(CoastalMarineZone::ANZ470),
                471 => Some(CoastalMarineZone::ANZ471),
                472 => Some(CoastalMarineZone::ANZ472),
                473 => Some(CoastalMarineZone::ANZ473),
                475 => Some(CoastalMarineZone::ANZ475),
                530 => Some(CoastalMarineZone::ANZ530),
                531 => Some(CoastalMarineZone::ANZ531),
                532 => Some(CoastalMarineZone::ANZ532),
                533 => Some(CoastalMarineZone::ANZ533),
                534 => Some(CoastalMarineZone::ANZ534),
                535 => Some(CoastalMarineZone::ANZ535),
                536 => Some(CoastalMarineZone::ANZ536),
                537 => Some(CoastalMarineZone::ANZ537),
                538 => Some(CoastalMarineZone::ANZ538),
                539 => Some(CoastalMarineZone::ANZ539),
                540 => Some(CoastalMarineZone::ANZ540),
                541 => Some(CoastalMarineZone::ANZ541),
                542 => Some(CoastalMarineZone::ANZ542),
                543 => Some(CoastalMarineZone::ANZ543),
                630 => Some(CoastalMarineZone::ANZ630),
                631 => Some(CoastalMarineZone::ANZ631),
                632 => Some(CoastalMarineZone::ANZ632),
                633 => Some(CoastalMarineZone::ANZ633),
                634 => Some(CoastalMarineZone::ANZ634),
                635 => Some(CoastalMarineZone::ANZ635),
                636 => Some(CoastalMarineZone::ANZ636),
                637 => Some(CoastalMarineZone::ANZ637),
                638 => Some(CoastalMarineZone::ANZ638),
                650 => Some(CoastalMarineZone::ANZ650),
                652 => Some(CoastalMarineZone::ANZ652),
                654 => Some(CoastalMarineZone::ANZ654),
                656 => Some(CoastalMarineZone::ANZ656),
                658 => Some(CoastalMarineZone::ANZ658),
                670 => Some(CoastalMarineZone::ANZ670),
                672 => Some(CoastalMarineZone::ANZ672),
                674 => Some(CoastalMarineZone::ANZ674),
                676 => Some(CoastalMarineZone::ANZ676),
                678 => Some(CoastalMarineZone::ANZ678),
                _ => None,
            },
            "GM" => match numeric {
                031 => Some(CoastalMarineZone::GMZ031),
                032 => Some(CoastalMarineZone::GMZ032),
                033 => Some(CoastalMarineZone::GMZ033),
                034 => Some(CoastalMarineZone::GMZ034),
                035 => Some(CoastalMarineZone::GMZ035),
                042 => Some(CoastalMarineZone::GMZ042),
                043 => Some(CoastalMarineZone::GMZ043),
                044 => Some(CoastalMarineZone::GMZ044),
                052 => Some(CoastalMarineZone::GMZ052),
                053 => Some(CoastalMarineZone::GMZ053),
                054 => Some(CoastalMarineZone::GMZ054),
                055 => Some(CoastalMarineZone::GMZ055),
                072 => Some(CoastalMarineZone::GMZ072),
                073 => Some(CoastalMarineZone::GMZ073),
                074 => Some(CoastalMarineZone::GMZ074),
                075 => Some(CoastalMarineZone::GMZ075),
                130 => Some(CoastalMarineZone::GMZ130),
                132 => Some(CoastalMarineZone::GMZ132),
                135 => Some(CoastalMarineZone::GMZ135),
                150 => Some(CoastalMarineZone::GMZ150),
                155 => Some(CoastalMarineZone::GMZ155),
                170 => Some(CoastalMarineZone::GMZ170),
                175 => Some(CoastalMarineZone::GMZ175),
                231 => Some(CoastalMarineZone::GMZ231),
                232 => Some(CoastalMarineZone::GMZ232),
                236 => Some(CoastalMarineZone::GMZ236),
                237 => Some(CoastalMarineZone::GMZ237),
                250 => Some(CoastalMarineZone::GMZ250),
                255 => Some(CoastalMarineZone::GMZ255),
                270 => Some(CoastalMarineZone::GMZ270),
                275 => Some(CoastalMarineZone::GMZ275),
                330 => Some(CoastalMarineZone::GMZ330),
                335 => Some(CoastalMarineZone::GMZ335),
                350 => Some(CoastalMarineZone::GMZ350),
                355 => Some(CoastalMarineZone::GMZ355),
                370 => Some(CoastalMarineZone::GMZ370),
                375 => Some(CoastalMarineZone::GMZ375),
                430 => Some(CoastalMarineZone::GMZ430),
                432 => Some(CoastalMarineZone::GMZ432),
                435 => Some(CoastalMarineZone::GMZ435),
                450 => Some(CoastalMarineZone::GMZ450),
                452 => Some(CoastalMarineZone::GMZ452),
                455 => Some(CoastalMarineZone::GMZ455),
                470 => Some(CoastalMarineZone::GMZ470),
                472 => Some(CoastalMarineZone::GMZ472),
                475 => Some(CoastalMarineZone::GMZ475),
                530 => Some(CoastalMarineZone::GMZ530),
                532 => Some(CoastalMarineZone::GMZ532),
                534 => Some(CoastalMarineZone::GMZ534),
                536 => Some(CoastalMarineZone::GMZ536),
                538 => Some(CoastalMarineZone::GMZ538),
                550 => Some(CoastalMarineZone::GMZ550),
                552 => Some(CoastalMarineZone::GMZ552),
                555 => Some(CoastalMarineZone::GMZ555),
                557 => Some(CoastalMarineZone::GMZ557),
                570 => Some(CoastalMarineZone::GMZ570),
                572 => Some(CoastalMarineZone::GMZ572),
                575 => Some(CoastalMarineZone::GMZ575),
                577 => Some(CoastalMarineZone::GMZ577),
                630 => Some(CoastalMarineZone::GMZ630),
                631 => Some(CoastalMarineZone::GMZ631),
                632 => Some(CoastalMarineZone::GMZ632),
                633 => Some(CoastalMarineZone::GMZ633),
                634 => Some(CoastalMarineZone::GMZ634),
                635 => Some(CoastalMarineZone::GMZ635),
                636 => Some(CoastalMarineZone::GMZ636),
                650 => Some(CoastalMarineZone::GMZ650),
                655 => Some(CoastalMarineZone::GMZ655),
                656 => Some(CoastalMarineZone::GMZ656),
                657 => Some(CoastalMarineZone::GMZ657),
                670 => Some(CoastalMarineZone::GMZ670),
                675 => Some(CoastalMarineZone::GMZ675),
                676 => Some(CoastalMarineZone::GMZ676),
                730 => Some(CoastalMarineZone::GMZ730),
                750 => Some(CoastalMarineZone::GMZ750),
                752 => Some(CoastalMarineZone::GMZ752),
                755 => Some(CoastalMarineZone::GMZ755),
                765 => Some(CoastalMarineZone::GMZ765),
                770 => Some(CoastalMarineZone::GMZ770),
                772 => Some(CoastalMarineZone::GMZ772),
                775 => Some(CoastalMarineZone::GMZ775),
                830 => Some(CoastalMarineZone::GMZ830),
                836 => Some(CoastalMarineZone::GMZ836),
                850 => Some(CoastalMarineZone::GMZ850),
                853 => Some(CoastalMarineZone::GMZ853),
                856 => Some(CoastalMarineZone::GMZ856),
                870 => Some(CoastalMarineZone::GMZ870),
                873 => Some(CoastalMarineZone::GMZ873),
                876 => Some(CoastalMarineZone::GMZ876),
                _ => None,
            },
            "LC" => match numeric {
                422 => Some(CoastalMarineZone::LCZ422),
                423 => Some(CoastalMarineZone::LCZ423),
                460 => Some(CoastalMarineZone::LCZ460),
                _ => None,
            },
            "LE" => match numeric {
                020 => Some(CoastalMarineZone::LEZ020),
                040 => Some(CoastalMarineZone::LEZ040),
                041 => Some(CoastalMarineZone::LEZ041),
                061 => Some(CoastalMarineZone::LEZ061),
                142 => Some(CoastalMarineZone::LEZ142),
                143 => Some(CoastalMarineZone::LEZ143),
                144 => Some(CoastalMarineZone::LEZ144),
                145 => Some(CoastalMarineZone::LEZ145),
                146 => Some(CoastalMarineZone::LEZ146),
                147 => Some(CoastalMarineZone::LEZ147),
                148 => Some(CoastalMarineZone::LEZ148),
                149 => Some(CoastalMarineZone::LEZ149),
                162 => Some(CoastalMarineZone::LEZ162),
                163 => Some(CoastalMarineZone::LEZ163),
                164 => Some(CoastalMarineZone::LEZ164),
                165 => Some(CoastalMarineZone::LEZ165),
                166 => Some(CoastalMarineZone::LEZ166),
                167 => Some(CoastalMarineZone::LEZ167),
                168 => Some(CoastalMarineZone::LEZ168),
                169 => Some(CoastalMarineZone::LEZ169),
                444 => Some(CoastalMarineZone::LEZ444),
                _ => None,
            },
            "LH" => match numeric {
                345 => Some(CoastalMarineZone::LHZ345),
                346 => Some(CoastalMarineZone::LHZ346),
                347 => Some(CoastalMarineZone::LHZ347),
                348 => Some(CoastalMarineZone::LHZ348),
                349 => Some(CoastalMarineZone::LHZ349),
                361 => Some(CoastalMarineZone::LHZ361),
                362 => Some(CoastalMarineZone::LHZ362),
                363 => Some(CoastalMarineZone::LHZ363),
                421 => Some(CoastalMarineZone::LHZ421),
                422 => Some(CoastalMarineZone::LHZ422),
                441 => Some(CoastalMarineZone::LHZ441),
                442 => Some(CoastalMarineZone::LHZ442),
                443 => Some(CoastalMarineZone::LHZ443),
                462 => Some(CoastalMarineZone::LHZ462),
                463 => Some(CoastalMarineZone::LHZ463),
                464 => Some(CoastalMarineZone::LHZ464),
                _ => None,
            },
            "LM" => match numeric {
                043 => Some(CoastalMarineZone::LMZ043),
                046 => Some(CoastalMarineZone::LMZ046),
                080 => Some(CoastalMarineZone::LMZ080),
                221 => Some(CoastalMarineZone::LMZ221),
                248 => Some(CoastalMarineZone::LMZ248),
                250 => Some(CoastalMarineZone::LMZ250),
                261 => Some(CoastalMarineZone::LMZ261),
                323 => Some(CoastalMarineZone::LMZ323),
                341 => Some(CoastalMarineZone::LMZ341),
                342 => Some(CoastalMarineZone::LMZ342),
                344 => Some(CoastalMarineZone::LMZ344),
                345 => Some(CoastalMarineZone::LMZ345),
                346 => Some(CoastalMarineZone::LMZ346),
                362 => Some(CoastalMarineZone::LMZ362),
                364 => Some(CoastalMarineZone::LMZ364),
                366 => Some(CoastalMarineZone::LMZ366),
                521 => Some(CoastalMarineZone::LMZ521),
                522 => Some(CoastalMarineZone::LMZ522),
                541 => Some(CoastalMarineZone::LMZ541),
                542 => Some(CoastalMarineZone::LMZ542),
                543 => Some(CoastalMarineZone::LMZ543),
                563 => Some(CoastalMarineZone::LMZ563),
                565 => Some(CoastalMarineZone::LMZ565),
                567 => Some(CoastalMarineZone::LMZ567),
                643 => Some(CoastalMarineZone::LMZ643),
                644 => Some(CoastalMarineZone::LMZ644),
                645 => Some(CoastalMarineZone::LMZ645),
                646 => Some(CoastalMarineZone::LMZ646),
                669 => Some(CoastalMarineZone::LMZ669),
                671 => Some(CoastalMarineZone::LMZ671),
                673 => Some(CoastalMarineZone::LMZ673),
                675 => Some(CoastalMarineZone::LMZ675),
                740 => Some(CoastalMarineZone::LMZ740),
                741 => Some(CoastalMarineZone::LMZ741),
                742 => Some(CoastalMarineZone::LMZ742),
                743 => Some(CoastalMarineZone::LMZ743),
                744 => Some(CoastalMarineZone::LMZ744),
                745 => Some(CoastalMarineZone::LMZ745),
                777 => Some(CoastalMarineZone::LMZ777),
                779 => Some(CoastalMarineZone::LMZ779),
                844 => Some(CoastalMarineZone::LMZ844),
                845 => Some(CoastalMarineZone::LMZ845),
                846 => Some(CoastalMarineZone::LMZ846),
                847 => Some(CoastalMarineZone::LMZ847),
                848 => Some(CoastalMarineZone::LMZ848),
                849 => Some(CoastalMarineZone::LMZ849),
                868 => Some(CoastalMarineZone::LMZ868),
                870 => Some(CoastalMarineZone::LMZ870),
                872 => Some(CoastalMarineZone::LMZ872),
                874 => Some(CoastalMarineZone::LMZ874),
                876 => Some(CoastalMarineZone::LMZ876),
                878 => Some(CoastalMarineZone::LMZ878),
                _ => None,
            },
            "LO" => match numeric {
                030 => Some(CoastalMarineZone::LOZ030),
                042 => Some(CoastalMarineZone::LOZ042),
                043 => Some(CoastalMarineZone::LOZ043),
                044 => Some(CoastalMarineZone::LOZ044),
                045 => Some(CoastalMarineZone::LOZ045),
                062 => Some(CoastalMarineZone::LOZ062),
                063 => Some(CoastalMarineZone::LOZ063),
                064 => Some(CoastalMarineZone::LOZ064),
                065 => Some(CoastalMarineZone::LOZ065),
                _ => None,
            },
            "LS" => match numeric {
                121 => Some(CoastalMarineZone::LSZ121),
                140 => Some(CoastalMarineZone::LSZ140),
                141 => Some(CoastalMarineZone::LSZ141),
                142 => Some(CoastalMarineZone::LSZ142),
                143 => Some(CoastalMarineZone::LSZ143),
                144 => Some(CoastalMarineZone::LSZ144),
                145 => Some(CoastalMarineZone::LSZ145),
                146 => Some(CoastalMarineZone::LSZ146),
                147 => Some(CoastalMarineZone::LSZ147),
                148 => Some(CoastalMarineZone::LSZ148),
                150 => Some(CoastalMarineZone::LSZ150),
                162 => Some(CoastalMarineZone::LSZ162),
                240 => Some(CoastalMarineZone::LSZ240),
                241 => Some(CoastalMarineZone::LSZ241),
                242 => Some(CoastalMarineZone::LSZ242),
                243 => Some(CoastalMarineZone::LSZ243),
                244 => Some(CoastalMarineZone::LSZ244),
                245 => Some(CoastalMarineZone::LSZ245),
                246 => Some(CoastalMarineZone::LSZ246),
                247 => Some(CoastalMarineZone::LSZ247),
                248 => Some(CoastalMarineZone::LSZ248),
                249 => Some(CoastalMarineZone::LSZ249),
                250 => Some(CoastalMarineZone::LSZ250),
                251 => Some(CoastalMarineZone::LSZ251),
                263 => Some(CoastalMarineZone::LSZ263),
                264 => Some(CoastalMarineZone::LSZ264),
                265 => Some(CoastalMarineZone::LSZ265),
                266 => Some(CoastalMarineZone::LSZ266),
                267 => Some(CoastalMarineZone::LSZ267),
                321 => Some(CoastalMarineZone::LSZ321),
                322 => Some(CoastalMarineZone::LSZ322),
                _ => None,
            },
            "PH" => match numeric {
                110 => Some(CoastalMarineZone::PHZ110),
                111 => Some(CoastalMarineZone::PHZ111),
                112 => Some(CoastalMarineZone::PHZ112),
                113 => Some(CoastalMarineZone::PHZ113),
                114 => Some(CoastalMarineZone::PHZ114),
                115 => Some(CoastalMarineZone::PHZ115),
                116 => Some(CoastalMarineZone::PHZ116),
                117 => Some(CoastalMarineZone::PHZ117),
                118 => Some(CoastalMarineZone::PHZ118),
                119 => Some(CoastalMarineZone::PHZ119),
                120 => Some(CoastalMarineZone::PHZ120),
                121 => Some(CoastalMarineZone::PHZ121),
                122 => Some(CoastalMarineZone::PHZ122),
                123 => Some(CoastalMarineZone::PHZ123),
                124 => Some(CoastalMarineZone::PHZ124),
                _ => None,
            },
            "PK" => match numeric {
                011 => Some(CoastalMarineZone::PKZ011),
                012 => Some(CoastalMarineZone::PKZ012),
                013 => Some(CoastalMarineZone::PKZ013),
                021 => Some(CoastalMarineZone::PKZ021),
                022 => Some(CoastalMarineZone::PKZ022),
                031 => Some(CoastalMarineZone::PKZ031),
                032 => Some(CoastalMarineZone::PKZ032),
                033 => Some(CoastalMarineZone::PKZ033),
                034 => Some(CoastalMarineZone::PKZ034),
                035 => Some(CoastalMarineZone::PKZ035),
                036 => Some(CoastalMarineZone::PKZ036),
                041 => Some(CoastalMarineZone::PKZ041),
                042 => Some(CoastalMarineZone::PKZ042),
                043 => Some(CoastalMarineZone::PKZ043),
                051 => Some(CoastalMarineZone::PKZ051),
                052 => Some(CoastalMarineZone::PKZ052),
                053 => Some(CoastalMarineZone::PKZ053),
                119 => Some(CoastalMarineZone::PKZ119),
                120 => Some(CoastalMarineZone::PKZ120),
                121 => Some(CoastalMarineZone::PKZ121),
                125 => Some(CoastalMarineZone::PKZ125),
                126 => Some(CoastalMarineZone::PKZ126),
                127 => Some(CoastalMarineZone::PKZ127),
                128 => Some(CoastalMarineZone::PKZ128),
                129 => Some(CoastalMarineZone::PKZ129),
                130 => Some(CoastalMarineZone::PKZ130),
                131 => Some(CoastalMarineZone::PKZ131),
                132 => Some(CoastalMarineZone::PKZ132),
                136 => Some(CoastalMarineZone::PKZ136),
                137 => Some(CoastalMarineZone::PKZ137),
                138 => Some(CoastalMarineZone::PKZ138),
                139 => Some(CoastalMarineZone::PKZ139),
                140 => Some(CoastalMarineZone::PKZ140),
                141 => Some(CoastalMarineZone::PKZ141),
                150 => Some(CoastalMarineZone::PKZ150),
                155 => Some(CoastalMarineZone::PKZ155),
                160 => Some(CoastalMarineZone::PKZ160),
                165 => Some(CoastalMarineZone::PKZ165),
                170 => Some(CoastalMarineZone::PKZ170),
                171 => Some(CoastalMarineZone::PKZ171),
                172 => Some(CoastalMarineZone::PKZ172),
                173 => Some(CoastalMarineZone::PKZ173),
                174 => Some(CoastalMarineZone::PKZ174),
                175 => Some(CoastalMarineZone::PKZ175),
                176 => Some(CoastalMarineZone::PKZ176),
                177 => Some(CoastalMarineZone::PKZ177),
                178 => Some(CoastalMarineZone::PKZ178),
                179 => Some(CoastalMarineZone::PKZ179),
                180 => Some(CoastalMarineZone::PKZ180),
                181 => Some(CoastalMarineZone::PKZ181),
                185 => Some(CoastalMarineZone::PKZ185),
                200 => Some(CoastalMarineZone::PKZ200),
                201 => Some(CoastalMarineZone::PKZ201),
                210 => Some(CoastalMarineZone::PKZ210),
                215 => Some(CoastalMarineZone::PKZ215),
                220 => Some(CoastalMarineZone::PKZ220),
                225 => Some(CoastalMarineZone::PKZ225),
                230 => Some(CoastalMarineZone::PKZ230),
                235 => Some(CoastalMarineZone::PKZ235),
                240 => Some(CoastalMarineZone::PKZ240),
                245 => Some(CoastalMarineZone::PKZ245),
                _ => None,
            },
            "PM" => match numeric {
                151 => Some(CoastalMarineZone::PMZ151),
                152 => Some(CoastalMarineZone::PMZ152),
                153 => Some(CoastalMarineZone::PMZ153),
                154 => Some(CoastalMarineZone::PMZ154),
                161 => Some(CoastalMarineZone::PMZ161),
                171 => Some(CoastalMarineZone::PMZ171),
                172 => Some(CoastalMarineZone::PMZ172),
                173 => Some(CoastalMarineZone::PMZ173),
                174 => Some(CoastalMarineZone::PMZ174),
                181 => Some(CoastalMarineZone::PMZ181),
                191 => Some(CoastalMarineZone::PMZ191),
                _ => None,
            },
            "PS" => match numeric {
                150 => Some(CoastalMarineZone::PSZ150),
                151 => Some(CoastalMarineZone::PSZ151),
                152 => Some(CoastalMarineZone::PSZ152),
                _ => None,
            },
            "PZ" => match numeric {
                110 => Some(CoastalMarineZone::PZZ110),
                130 => Some(CoastalMarineZone::PZZ130),
                131 => Some(CoastalMarineZone::PZZ131),
                132 => Some(CoastalMarineZone::PZZ132),
                133 => Some(CoastalMarineZone::PZZ133),
                134 => Some(CoastalMarineZone::PZZ134),
                135 => Some(CoastalMarineZone::PZZ135),
                150 => Some(CoastalMarineZone::PZZ150),
                153 => Some(CoastalMarineZone::PZZ153),
                156 => Some(CoastalMarineZone::PZZ156),
                170 => Some(CoastalMarineZone::PZZ170),
                173 => Some(CoastalMarineZone::PZZ173),
                176 => Some(CoastalMarineZone::PZZ176),
                210 => Some(CoastalMarineZone::PZZ210),
                250 => Some(CoastalMarineZone::PZZ250),
                255 => Some(CoastalMarineZone::PZZ255),
                270 => Some(CoastalMarineZone::PZZ270),
                275 => Some(CoastalMarineZone::PZZ275),
                350 => Some(CoastalMarineZone::PZZ350),
                356 => Some(CoastalMarineZone::PZZ356),
                370 => Some(CoastalMarineZone::PZZ370),
                376 => Some(CoastalMarineZone::PZZ376),
                410 => Some(CoastalMarineZone::PZZ410),
                415 => Some(CoastalMarineZone::PZZ415),
                450 => Some(CoastalMarineZone::PZZ450),
                455 => Some(CoastalMarineZone::PZZ455),
                470 => Some(CoastalMarineZone::PZZ470),
                475 => Some(CoastalMarineZone::PZZ475),
                530 => Some(CoastalMarineZone::PZZ530),
                531 => Some(CoastalMarineZone::PZZ531),
                535 => Some(CoastalMarineZone::PZZ535),
                540 => Some(CoastalMarineZone::PZZ540),
                545 => Some(CoastalMarineZone::PZZ545),
                560 => Some(CoastalMarineZone::PZZ560),
                565 => Some(CoastalMarineZone::PZZ565),
                570 => Some(CoastalMarineZone::PZZ570),
                571 => Some(CoastalMarineZone::PZZ571),
                575 => Some(CoastalMarineZone::PZZ575),
                576 => Some(CoastalMarineZone::PZZ576),
                645 => Some(CoastalMarineZone::PZZ645),
                650 => Some(CoastalMarineZone::PZZ650),
                655 => Some(CoastalMarineZone::PZZ655),
                670 => Some(CoastalMarineZone::PZZ670),
                673 => Some(CoastalMarineZone::PZZ673),
                676 => Some(CoastalMarineZone::PZZ676),
                750 => Some(CoastalMarineZone::PZZ750),
                775 => Some(CoastalMarineZone::PZZ775),
                _ => None,
            },
            "SL" => match numeric {
                022 => Some(CoastalMarineZone::SLZ022),
                024 => Some(CoastalMarineZone::SLZ024),
                _ => None,
            },
            _ => None,
        }
    }
}
