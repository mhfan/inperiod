
// Auto-generated by syncd.rs, DO NOT EDIT.

use super::{ChemElem, AtomicWeight::{self, *}};

impl ChemElem {
    pub const fn atomic_weight(&self) -> &AtomicWeight { &MASS[self.atomic_number() as usize] }
}

const MASS: [AtomicWeight; ChemElem::MAX as usize] = [ MassNumber(0),
    Abridged { value: 1.0080, uncertainty: 0.0002 },
    Abridged { value: 4.0026, uncertainty: 0.0001 },
    Abridged { value: 6.94  , uncertainty: 0.06 },
    Abridged { value: 9.0122, uncertainty: 0.0001 },
    Abridged { value: 10.81 , uncertainty: 0.02 },
    Abridged { value: 12.011, uncertainty: 0.002 },
    Abridged { value: 14.007, uncertainty: 0.001 },
    Abridged { value: 15.999, uncertainty: 0.001 },
    Abridged { value: 18.998, uncertainty: 0.001 },
    Abridged { value: 20.180, uncertainty: 0.001 },
    Abridged { value: 22.990, uncertainty: 0.001 },
    Abridged { value: 24.305, uncertainty: 0.002 },
    Abridged { value: 26.982, uncertainty: 0.001 },
    Abridged { value: 28.085, uncertainty: 0.001 },
    Abridged { value: 30.974, uncertainty: 0.001 },
    Abridged { value: 32.06 , uncertainty: 0.02 },
    Abridged { value: 35.45 , uncertainty: 0.01 },
    Abridged { value: 39.95 , uncertainty: 0.16 },
    Abridged { value: 39.098, uncertainty: 0.001 },
    Abridged { value: 40.078, uncertainty: 0.004 },
    Abridged { value: 44.956, uncertainty: 0.001 },
    Abridged { value: 47.867, uncertainty: 0.001 },
    Abridged { value: 50.942, uncertainty: 0.001 },
    Abridged { value: 51.996, uncertainty: 0.001 },
    Abridged { value: 54.938, uncertainty: 0.001 },
    Abridged { value: 55.845, uncertainty: 0.002 },
    Abridged { value: 58.933, uncertainty: 0.001 },
    Abridged { value: 58.693, uncertainty: 0.001 },
    Abridged { value: 63.546, uncertainty: 0.003 },
    Abridged { value: 65.38 , uncertainty: 0.02 },
    Abridged { value: 69.723, uncertainty: 0.001 },
    Abridged { value: 72.630, uncertainty: 0.008 },
    Abridged { value: 74.922, uncertainty: 0.001 },
    Abridged { value: 78.971, uncertainty: 0.008 },
    Abridged { value: 79.904, uncertainty: 0.003 },
    Abridged { value: 83.798, uncertainty: 0.002 },
    Abridged { value: 85.468, uncertainty: 0.001 },
    Abridged { value: 87.62 , uncertainty: 0.01 },
    Abridged { value: 88.906, uncertainty: 0.001 },
    Abridged { value: 91.222, uncertainty: 0.003 },
    Abridged { value: 92.906, uncertainty: 0.001 },
    Abridged { value: 95.95 , uncertainty: 0.01 },
    MassNumber(97),
    Abridged { value: 101.07, uncertainty: 0.02 },
    Abridged { value: 102.91, uncertainty: 0.01 },
    Abridged { value: 106.42, uncertainty: 0.01 },
    Abridged { value: 107.87, uncertainty: 0.01 },
    Abridged { value: 112.41, uncertainty: 0.01 },
    Abridged { value: 114.82, uncertainty: 0.01 },
    Abridged { value: 118.71, uncertainty: 0.01 },
    Abridged { value: 121.76, uncertainty: 0.01 },
    Abridged { value: 127.60, uncertainty: 0.03 },
    Abridged { value: 126.90, uncertainty: 0.01 },
    Abridged { value: 131.29, uncertainty: 0.01 },
    Abridged { value: 132.91, uncertainty: 0.01 },
    Abridged { value: 137.33, uncertainty: 0.01 },
    Abridged { value: 138.91, uncertainty: 0.01 },
    Abridged { value: 140.12, uncertainty: 0.01 },
    Abridged { value: 140.91, uncertainty: 0.01 },
    Abridged { value: 144.24, uncertainty: 0.01 },
    MassNumber(145),
    Abridged { value: 150.36, uncertainty: 0.02 },
    Abridged { value: 151.96, uncertainty: 0.01 },
    Abridged { value: 157.25, uncertainty: 0.01 },
    Abridged { value: 158.93, uncertainty: 0.01 },
    Abridged { value: 162.50, uncertainty: 0.01 },
    Abridged { value: 164.93, uncertainty: 0.01 },
    Abridged { value: 167.26, uncertainty: 0.01 },
    Abridged { value: 168.93, uncertainty: 0.01 },
    Abridged { value: 173.05, uncertainty: 0.02 },
    Abridged { value: 174.97, uncertainty: 0.01 },
    Abridged { value: 178.49, uncertainty: 0.01 },
    Abridged { value: 180.95, uncertainty: 0.01 },
    Abridged { value: 183.84, uncertainty: 0.01 },
    Abridged { value: 186.21, uncertainty: 0.01 },
    Abridged { value: 190.23, uncertainty: 0.03 },
    Abridged { value: 192.22, uncertainty: 0.01 },
    Abridged { value: 195.08, uncertainty: 0.02 },
    Abridged { value: 196.97, uncertainty: 0.01 },
    Abridged { value: 200.59, uncertainty: 0.01 },
    Abridged { value: 204.38, uncertainty: 0.01 },
    Abridged { value: 207.2 , uncertainty: 1.1 },
    Abridged { value: 208.98, uncertainty: 0.01 },
    MassNumber(209),
    MassNumber(210),
    MassNumber(222),
    MassNumber(223),
    MassNumber(226),
    MassNumber(227),
    Abridged { value: 232.04, uncertainty: 0.01 },
    Abridged { value: 231.04, uncertainty: 0.01 },
    Abridged { value: 238.03, uncertainty: 0.01 },
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

