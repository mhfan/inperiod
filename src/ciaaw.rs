
use super::{ChemElem, AtomicWeight::{self, *}};
impl ChemElem { // Auto-generated by syncd.rs, DO NOT EDIT.
    pub const fn atomic_weight(&self) -> &AtomicWeight { &MASS[self.atomic_number() as usize] }
}

const MASS: [AtomicWeight; ChemElem::MAX as usize] = [ MassNumber(0),
    Abridged { value: 1.0080, uncert: 0.0002 },
    Abridged { value: 4.0026, uncert: 0.0001 },
    Abridged { value: 6.94  , uncert: 0.06 },
    Abridged { value: 9.0122, uncert: 0.0001 },
    Abridged { value: 10.81 , uncert: 0.02 },
    Abridged { value: 12.011, uncert: 0.002 },
    Abridged { value: 14.007, uncert: 0.001 },
    Abridged { value: 15.999, uncert: 0.001 },
    Abridged { value: 18.998, uncert: 0.001 },
    Abridged { value: 20.180, uncert: 0.001 },
    Abridged { value: 22.990, uncert: 0.001 },
    Abridged { value: 24.305, uncert: 0.002 },
    Abridged { value: 26.982, uncert: 0.001 },
    Abridged { value: 28.085, uncert: 0.001 },
    Abridged { value: 30.974, uncert: 0.001 },
    Abridged { value: 32.06 , uncert: 0.02 },
    Abridged { value: 35.45 , uncert: 0.01 },
    Abridged { value: 39.95 , uncert: 0.16 },
    Abridged { value: 39.098, uncert: 0.001 },
    Abridged { value: 40.078, uncert: 0.004 },
    Abridged { value: 44.956, uncert: 0.001 },
    Abridged { value: 47.867, uncert: 0.001 },
    Abridged { value: 50.942, uncert: 0.001 },
    Abridged { value: 51.996, uncert: 0.001 },
    Abridged { value: 54.938, uncert: 0.001 },
    Abridged { value: 55.845, uncert: 0.002 },
    Abridged { value: 58.933, uncert: 0.001 },
    Abridged { value: 58.693, uncert: 0.001 },
    Abridged { value: 63.546, uncert: 0.003 },
    Abridged { value: 65.38 , uncert: 0.02 },
    Abridged { value: 69.723, uncert: 0.001 },
    Abridged { value: 72.630, uncert: 0.008 },
    Abridged { value: 74.922, uncert: 0.001 },
    Abridged { value: 78.971, uncert: 0.008 },
    Abridged { value: 79.904, uncert: 0.003 },
    Abridged { value: 83.798, uncert: 0.002 },
    Abridged { value: 85.468, uncert: 0.001 },
    Abridged { value: 87.62 , uncert: 0.01 },
    Abridged { value: 88.906, uncert: 0.001 },
    Abridged { value: 91.222, uncert: 0.003 },
    Abridged { value: 92.906, uncert: 0.001 },
    Abridged { value: 95.95 , uncert: 0.01 },
    MassNumber(97),
    Abridged { value: 101.07, uncert: 0.02 },
    Abridged { value: 102.91, uncert: 0.01 },
    Abridged { value: 106.42, uncert: 0.01 },
    Abridged { value: 107.87, uncert: 0.01 },
    Abridged { value: 112.41, uncert: 0.01 },
    Abridged { value: 114.82, uncert: 0.01 },
    Abridged { value: 118.71, uncert: 0.01 },
    Abridged { value: 121.76, uncert: 0.01 },
    Abridged { value: 127.60, uncert: 0.03 },
    Abridged { value: 126.90, uncert: 0.01 },
    Abridged { value: 131.29, uncert: 0.01 },
    Abridged { value: 132.91, uncert: 0.01 },
    Abridged { value: 137.33, uncert: 0.01 },
    Abridged { value: 138.91, uncert: 0.01 },
    Abridged { value: 140.12, uncert: 0.01 },
    Abridged { value: 140.91, uncert: 0.01 },
    Abridged { value: 144.24, uncert: 0.01 },
    MassNumber(145),
    Abridged { value: 150.36, uncert: 0.02 },
    Abridged { value: 151.96, uncert: 0.01 },
    Abridged { value: 157.25, uncert: 0.01 },
    Abridged { value: 158.93, uncert: 0.01 },
    Abridged { value: 162.50, uncert: 0.01 },
    Abridged { value: 164.93, uncert: 0.01 },
    Abridged { value: 167.26, uncert: 0.01 },
    Abridged { value: 168.93, uncert: 0.01 },
    Abridged { value: 173.05, uncert: 0.02 },
    Abridged { value: 174.97, uncert: 0.01 },
    Abridged { value: 178.49, uncert: 0.01 },
    Abridged { value: 180.95, uncert: 0.01 },
    Abridged { value: 183.84, uncert: 0.01 },
    Abridged { value: 186.21, uncert: 0.01 },
    Abridged { value: 190.23, uncert: 0.03 },
    Abridged { value: 192.22, uncert: 0.01 },
    Abridged { value: 195.08, uncert: 0.02 },
    Abridged { value: 196.97, uncert: 0.01 },
    Abridged { value: 200.59, uncert: 0.01 },
    Abridged { value: 204.38, uncert: 0.01 },
    Abridged { value: 207.2 , uncert: 1.1 },
    Abridged { value: 208.98, uncert: 0.01 },
    MassNumber(209),
    MassNumber(210),
    MassNumber(222),
    MassNumber(223),
    MassNumber(226),
    MassNumber(227),
    Abridged { value: 232.04, uncert: 0.01 },
    Abridged { value: 231.04, uncert: 0.01 },
    Abridged { value: 238.03, uncert: 0.01 },
    MassNumber(237),
    MassNumber(244),
    MassNumber(243),
    MassNumber(247),
    MassNumber(247),
    MassNumber(251),
    MassNumber(252),
    MassNumber(257),
    MassNumber(258),
    MassNumber(259),
    MassNumber(266),
    MassNumber(267),
    MassNumber(268),
    MassNumber(269),
    MassNumber(270),
    MassNumber(269),
    MassNumber(277),
    MassNumber(281),
    MassNumber(282),
    MassNumber(285),
    MassNumber(286),
    MassNumber(290),
    MassNumber(290),
    MassNumber(293),
    MassNumber(294),
    MassNumber(294),
];

