
There are plenty of periodic tables available online,
including [web pages](https://ptable.com),
[spreadsheet](https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.htm),
[JPEG/PNG/SVG images](https://commons.wikimedia.org/wiki/File:Periodic_table_large.svg),
[mobile APP](https://github.com/baotlake/periodic-table-pro),
[Python-generated](https://github.com/lmmentel/mendeleev),
and even those can be purchased from various e-commerce platforms,
but none of them fully meet my expectations.

I wanted a periodic table that could comprehensively showcase all aspects of the periodic
trends of chemical elements, provide rich and layered information, synchronize with the latest
authoritative data in real-time, and include interactive features as a WebAPP or native/mobile
APP. Moreover, it had to feature Chinese names and pinyin and be printable on A4/A3 paper for
use by sixth and ninth-grade students as an educational and memorization aid.
Thus, this project was born.

<https://mhfan.github.io/inperiod>

Additionally, this project also target to provide a set of API in Rust for accessing
all kinds of information and data about the elements.

## Instructions

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind_base.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Web/Desktop app:

```bash
dx serve #--verbose
dx serve --platform web

dx serve --platform desktop #rm -rf dist
cd dist && cargo r --features desktop
```

Note: Recommended to print as PDF on Chrome browser.

![Elements Periodic Table](https://github.com/user-attachments/assets/89435148-8b88-4308-a61b-90b1ad04b2ab)
![Show electron configuration diagram](https://github.com/user-attachments/assets/ddbb6ddc-dcc6-4cdb-b07d-a8b6fac10937)
![Show crystal structure](https://github.com/user-attachments/assets/5ae493e0-a247-4860-a9c1-521f94077e7d)

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
