# NWS Forecast Zones

This crate contains information on the [NWS Public Forecast Zones](https://www.weather.gov/gis/PublicZones).
A forecast zone is, to quote NWS:

> The NWS issues forecasts and some watches and warnings for public zones
> which usually are the same as counties but in many cases are subsets of
> counties.  Counties are subset into zones to allow for more accurate
> forecasts because of the differences in weather within a county due to
> such things as elevation or proximity to large bodies of water.

This crate has an enum that represents all 3882 zones.  It can also parse a range of forecast zones, like this:

    MAZ017>019-RIZ001>003

These ranges are very common in NWS text products.  The parsed range can be used to test if a specific zone
is part of the range or not.

## License

The nws-forecast-zones library is licensed under the  MIT license ([LICENSE-MIT](LICENSE-MIT.txt)
or http://opensource.org/licenses/MIT)

## Note

This crate uses data published from NWS, but is otherwise unaffiliated with the National Weather Service,
and is not an official NWS library.