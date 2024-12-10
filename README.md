
![Build status](https://github.com/mhfan/inperiod/actions/workflows/publish.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/inperiod.svg)](https://crates.io/crates/inperiod)
[![dependency status](https://deps.rs/repo/github/mhfan/inperiod/status.svg)](https://deps.rs/repo/github/mhfan/inperiod)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

There are plenty of periodic tables available online,
including [web pages](https://ptable.com),
[spreadsheet](https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.htm),
[JPEG/PNG/SVG images](https://commons.wikimedia.org/wiki/File:Periodic_table_large.svg),
[mobile APP](https://github.com/baotlake/periodic-table-pro),
[Python-generated](https://github.com/lmmentel/mendeleev),
and even those can be purchased from various e-commerce platforms,
but none of them fully meet my expectations.

What I want is a periodic table that could comprehensively showcase all aspects of the periodic
trends of chemical elements, provide rich and layered information, synchronize with the latest
authoritative data in real-time, and include interactive features as a WebAPP or native/mobile
APP. Moreover, it had to feature Chinese names and pinyin and be printable on A4/A3 paper for
use by sixth and ninth-grade students as an educational and memorization aid.
Thus, this project was born.

<https://mhfan.github.io/inperiod>

![Elements Periodic Table](https://github.com/user-attachments/assets/34632195-1344-4377-a02e-d654e88a0afb)
![Show electron configuration diagram](https://github.com/user-attachments/assets/e6a46fba-04cf-4275-b1e0-db78aecf307d)
![Show crystal structure](https://github.com/user-attachments/assets/32518da9-e1b5-4314-95bc-cffa43ec6fc4)

Additionally, this project also targets to provide a set of API in Rust for accessing
all kinds of information and data about the elements.

## Instructions

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npm install tailwindcss -D -g

npx tailwindcss -i tailwind_base.css -o assets/tailwind.css -w #-m
```

Launch the Dioxus Web/Desktop/Mobile app:

```bash
dx serve --platform web #--verbose

dx serve --platform desktop #rm -rf dist
#cd dist && cargo r -F desktop

dx serve --platform android #ios
```

Open the browser to <http://localhost:8080/inperiod>

Note: Recommended to print as PDF on Chrome browser.

## References

* <https://en.wikipedia.org/wiki/Periodic_table>
* <https://www.webelements.com/periodicity/contents/>
* <https://en.wikipedia.org/wiki/Category:Chemical_element_data_pages>
* <https://physics.nist.gov/PhysRefData/ASD/ionEnergy.html>
* <https://pubchem.ncbi.nlm.nih.gov/periodic-table/>
* <https://www.nist.gov/pml/periodic-table-elements>
* <https://ciaaw.org/atomic-weights.htm>
* <https://github.com/Bowserinator/Periodic-Table-JSON>
* <https://periodictable.com/Properties/A/HumanAbundance.html>
* <https://www.futurity.org/periodic-table-new-elements-1087782-2/>
* <https://iupac.org/what-we-do/periodic-table-of-elements/>
