Programs that scan sfgov data and print statistics

Prerequisites:

* [rust language](https://www.rust-lang.org/learn/get-started)
(tested on rustc 1.32).
This provides the `cargo` command
* [PROJ.4](https://proj4.org/) (v5.2.x)
shared library for projections
(Mac: `brew install proj`)

Other dependencies are downloaded automatically by `cargo`.

## parcelscan

Prints a histogram of the number of residential units per 2500 sq. ft. of lot space in the city of San Francisco.

File needed:
* [LandUse2016.csv](https://data.sfgov.org/Housing-and-Buildings/Land-Use/us3s-fp9q)
All parcels in San Francisco

Usage:

```sh
cargo build --release
RUST_LOG=info target/release/parcelscan --input ~/Downloads/LandUse2016.csv density-historgram
```

Sample output:
```
…
[2019-03-25T09:30:48Z INFO  parcelscan] 0.75 units: 29808 records e.g. 360-360 COUNTRY CLUB DR (7272038, 5203), 1946-1946 43RD AVE (2092041, 4893), 566-566 48TH AVE (1497016T, 3987), 2122-2122 36TH AVE (2182016A, 2950), 125-125 STAPLES AVE (3156043, 2300)
[2019-03-25T09:30:48Z INFO  parcelscan] 1 units: 30286 records e.g. 234-234 MOLIMO DR (2998010, 5953), 661-661 46TH AVE (1594001A, 4101), 1440-1440 VALLEJO ST (0549029, 2104), 2920-2920 ANZA ST (1524017, 2087), 2077-2077 BANCROFT AVE (5426030, 2016)
[2019-03-25T09:30:48Z INFO  parcelscan] 1.25 units: 9864 records e.g. 928-928 SHOTWELL ST (3641087, 2015), 1710-1710 DIAMOND ST (7535103, 2014), 60-62 PERALTA AVE (5512031, 2014), 162-162 BREWSTER ST (5556019, 2014), 213-213 LOS PALMOS DR (3027A118, 2014)
…
```

## parcelscanchicago

Prints the number of residential units per 2500 sq. ft. of lot space in the city of Chicago.
I don’t really trust the data though; it’s a lot messier than SF.

Files needed:
* [buildings.csv](https://data.cityofchicago.org/Buildings/Building-Footprints-current-/hz9b-7nh8)
building footprints in the city of Chicago
* [ccgisdata_-_Parcels_2016.csv](https://datacatalog.cookcountyil.gov/GIS-Maps/ccgisdata-Parcels-2016/a33b-b59u)
parcels in Cook County

Usage:

```sh
cargo build --release
RUST_LOG=info target/release/parcelscanchicago --buildings ~/Downloads/buildings.csv --parcels ~/Downloads/ccgisdata_-_Parcels_2016.csv  density-historgram
```

Sample output:
```
…
[2019-03-25T09:45:09Z INFO  parcelscanchicago] 0.25 units: 14023 records e.g. 1481-1481 107TH ST (891022, 2010) 1 units, parcel area 11203 MULTIPOLYGON(((-87.65973047170833 41.6991044075244,-87.6603547608075 41.69923496223231,-87.66031880691396 41.69933283428024,-87.65964341354685 41.69934117145256,-87.65973047170833 41.6991044075244))), 14-14 90TH ST (889352, 2009) 1 units, parcel area 12367.375 MULTIPOLYGON(((-87.62321029938778 41.73100077302029,-87.62388810452757 41.73099689178087,-87.62389326876136 41.731180134095986,-87.62358184297119 41.731181942881584,-87.62334369535458 41.731183270852995,-87.62321531506531 41.73118401444456,-87.62321029938778 41.73100077302029))), 3560-3560 FEDERAL ST (888855, 2009) 1 units, parcel area 6714.8125 MULTIPOLYGON(((-87.62899655322434 41.82942390765379,-87.62899448934796 41.82934708899332,-87.62899242671844 41.82927026673808,-87.62898999562704 41.829179727217614,-87.62900882567699 41.829179481644445,-87.62919175644068 41.82917709906575,-87.62920075460059 41.8295118208632,-87.6289989831159 41.829514448063314,-87.62899655322434 41.82942390765379))), 3540-3540 FEDERAL ST (888848, 2009) 1 units, parcel area 6715.125 MULTIPOLYGON(((-87.62900686164035 41.829808017979715,-87.62900480139625 41.82973119574538,-87.6290027399521 41.829654373502635,-87.62900030883904 41.82956383398818,-87.62901864416347 41.82956359528377,-87.62920208168255 41.829561206794345,-87.62920451547753 41.82965174722178,-87.62920657960987 41.829728565876415,-87.62920864491014 41.82980538813854,-87.62921107872874 41.829895927661845,-87.62900929159301 41.82989855478209,-87.62900686164035 41.829808017979715))), 3548-3548 FEDERAL ST (888850, 2009) 1 units, parcel area 6715.125 MULTIPOLYGON(((-87.62900686164035 41.829808017979715,-87.62900480139625 41.82973119574538,-87.6290027399521 41.829654373502635,-87.62900030883904 41.82956383398818,-87.62901864416347 41.82956359528377,-87.62920208168255 41.829561206794345,-87.62920451547753 41.82965174722178,-87.62920657960987 41.829728565876415,-87.62920864491014 41.82980538813854,-87.62921107872874 41.829895927661845,-87.62900929159301 41.82989855478209,-87.62900686164035 41.829808017979715)))
…
```

## peskinexpansionsimpact

Prints stats about which past residential expansions in the city of San Francisco
would probably be prohibited by
[Leg Ver1](https://sfgov.legistar.com/View.ashx?M=F&ID=6838135&GUID=08C9052E-3A30-445F-B11C-CF4A07130B99)
of Aaron Peskin’s demolition, mergers, and expansion prohibitions
(Board File [181216](https://sfgov.legistar.com/LegislationDetail.aspx?ID=3781286&GUID=3E5F18E7-DD20-436B-A63F-954036D210F0)).

Methodology: The Peskin prohibited projects that this program can detect are those that expanded the residential area,
the project building’s FAR was greater than the neighborhood FAR,
the size of the expansion was greater than 360 sq. ft.
(the limit for “limited horizontal expansions” on a typical 25 ft. lot),
and the project is a “major expansion” (building grows ≥20%) or demolition.
The neighborhood FAR is calculated by looking at parcels whose centroid is within 300 ft. of the centroid of the project parcel,
and dividing the `BLDGSQFT` by `SHAPE_Area` in the `LandUse` file.
The project building FAR is calculated by summing all the `LAND_USE_*_PROP` columns
(e.g. `LAND_USE_RESIDENTIAL_PROP`) in the SF Planning Acela PPTS file,
and dividing it by the area of the `the_geom` column.
Note that since the FAR is calculated from different data sources,
there is potential for discrepancies.

This is a subset of the projects that the Peskin bill would prohibit.
It also prohibits other projects (e.g. expansions that contain new parking,
or expansions that do not build the maximum number of units that are permitted,
or expansions in which any unit is greater than 1200 sq. ft.).
It also requires Conditional Use Authorization for more projects
(e.g. replacement of 25% of internal walls).
So this program identifies only a fraction of the projects that are affected by the bill.

Files needed:

* [LandUse2016.csv](https://data.sfgov.org/Housing-and-Buildings/Land-Use/us3s-fp9q)
* [PPTS_Records_data.csv](https://data.sfgov.org/Housing-and-Buildings/PPTS-Records/7yuw-98m5)
Permits from SF Planning dept since 2013

Usage:

```sh
cargo build --release
RUST_LOG=peskinexpansionsimpact=info target/release/peskinexpansionsimpact --planning ~/Downloads/PPTS_Records_data.csv --land-use ~/Downloads/LandUse2016.csv expansions
```

Sample output:
```
…
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] Prohibited: 12/06/2017 address: 1552 35TH AVE 94122, far 1.02, neighbor far: 0.65 (58 neighbors), bldg growth 810sqft (32%)
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] Probably OK: 05/13/2016 address: 3930 FOLSOM ST 94110, far 1.21, neighbor far: 0.65 (62 neighbors), bldg growth 325sqft (18%)
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] Probably OK: 04/07/2017 address: 30 KRONQUIST CT 94131, far 1.13, neighbor far: 0.72 (45 neighbors), bldg growth 187sqft (7%)
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] Probably OK: 10/11/2016 address: 1983 JEFFERSON ST 94123, far 0.86, neighbor far: 1.22 (46 neighbors), bldg growth 478sqft (25%)
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] Probably OK: 01/10/2018 address: 1478 06TH AVE 94122, far 0.63, neighbor far: 0.81 (54 neighbors), bldg growth 190sqft (11%)
[2019-03-25T11:26:29Z INFO  peskinexpansionsimpact] ok expansions: 836; prohibited expansions: 935 (52%)
```

For full output see [this gist](https://gist.github.com/yonran/445a1d6c8fcbcf9fd81f954f831e6fff)