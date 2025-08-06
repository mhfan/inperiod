
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

![Elements Periodic Table](https://github.com/user-attachments/assets/e467d1da-d49c-40d9-aa81-69da7afbe9e1)
![Chinese Periodic Table](https://github.com/user-attachments/assets/0e2c2102-7f85-4f81-b42f-9796ee8b1d0c)
![Elements cosmic origin](https://github.com/user-attachments/assets/838189ca-977d-4c50-9634-d7ccb8e11c9f)
![Elements flame test](https://github.com/user-attachments/assets/f4bd4a74-91e2-4848-b613-3a33de595d95)

![Show electron configuration diagram](https://github.com/user-attachments/assets/931590e3-34e0-44d8-9209-7febac64306a)
![Show crystal structure](https://github.com/user-attachments/assets/b9c0c651-5d3c-43be-af42-8cd374c5aa07)

Additionally, this project also targets to provide a set of API in Rust for accessing
all kinds of information and data about the elements.

## Instructions

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind cli: <https://tailwindcss.com/docs/installation/tailwind-cli>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npm install tailwindcss @tailwindcss/cli #-D -g

npx @tailwindcss/cli -i input.css -o assets/tailwind.css -w #-m
```

Launch the Dioxus Web/Desktop/Mobile app:

```bash
dx serve --platform web #--verbose

dx serve --platform desktop #rm -rf dist
#cd dist && cargo r -F desktop

dx serve --platform android #ios
```

Open the browser to <http://localhost:8080/inperiod>

Note: Recommended to `Print to PDF` on Chrome, or `Export to PDF` on Safari.

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
* <https://www.halcyonmaps.com/periodic-table-of-the-elements/>
* <https://www.futurity.org/periodic-table-new-elements-1087782-2/>
* <https://iupac.org/what-we-do/periodic-table-of-elements/>
