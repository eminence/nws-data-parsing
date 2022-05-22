# WMO Abbreviated Headings

This crate has parsers for the data designators in WMO Abbreviated Headings, specifcally for Attachment II-5
in [WMO manual 386](https://library.wmo.int/doc_num.php?explnum_id=10469).

These abbreviated headings are often referenced by their fields TTAAii, and can be seen in
NWS bulletings and EMWIN filenames.  For example, consider the first two lines of an
[NWS forecast](https://web.archive.org/web/20220521191347/https://forecast.weather.gov/product.php?site=NWS&issuedby=BOX&product=ZFP&format=CI&version=1&glossary=0):

    FPUS51 KBOX 211708
    ZFPBOX

The `FPUS51` part is the WMO abbreviated heading and can be decoded using tables A through D of WMO manual 386.
In this particular example, the `F` indicates a Forecast, the `P` indicates a public forecast, the `US` part
indicates "United States", and the `51` part is an NWS specific code that indicates it was issues from the Northeast US.

## License

The wmo-abbrev-headings library is licensed under the  MIT license ([LICENSE-MIT](LICENSE-MIT.txt)
or http://opensource.org/licenses/MIT)

# Note

This crate uses data published by the World Meteorological Organization, but is otherwise unaffiliated with the
WMO, and is not an offical WMO library.