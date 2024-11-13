
// Auto-generated by syncd.rs, DO NOT EDIT.

impl super::ChemElem {
    pub const fn oxidation_states(&self) -> (&'static [i8], &'static [i8]) {
        let all: &'static [i8] = match self.atomic_number() {
              1 => &[             -1,    1,                        ],

              3 => &[                    1,                        ],
              4 => &[                 0, 1, 2,                     ],
              5 => &[ -5,         -1, 0, 1, 2, 3,                  ],
              6 => &[    -4,-3,-2,-1, 0, 1, 2, 3, 4,               ],
              7 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5,            ],
              8 => &[          -2,-1, 0, 1, 2,                     ],
              9 => &[             -1,                              ],

             11 => &[             -1, 0, 1,                        ],
             12 => &[                 0, 1, 2,                     ],
             13 => &[          -2,-1, 0, 1, 2, 3,                  ],
             14 => &[    -4,-3,-2,-1, 0, 1, 2, 3, 4,               ],
             15 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5,            ],
             16 => &[          -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             17 => &[             -1,    1, 2, 3, 4, 5, 6, 7,      ],

             19 => &[             -1,    1,                        ],
             20 => &[                    1, 2,                     ],
             21 => &[                 0, 1, 2, 3,                  ],
             22 => &[          -2,-1, 0, 1, 2, 3, 4,               ],
             23 => &[       -3,   -1, 0, 1, 2, 3, 4, 5,            ],
             24 => &[    -4,   -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             25 => &[       -3,   -1, 0, 1, 2, 3, 4, 5, 6, 7,      ],
             26 => &[    -4,   -2,-1, 0, 1, 2, 3, 4, 5, 6, 7,      ],
             27 => &[       -3,   -1, 0, 1, 2, 3, 4, 5,            ],
             28 => &[          -2,-1, 0, 1, 2, 3, 4,               ],
             29 => &[          -2,    0, 1, 2, 3, 4,               ],
             30 => &[          -2,    0, 1, 2,                     ],
             31 => &[ -5,-4,-3,-2,-1, 0, 1, 2, 3,                  ],
             32 => &[    -4,-3,-2,-1, 0, 1, 2, 3, 4,               ],
             33 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5,            ],
             34 => &[          -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             35 => &[             -1,    1, 2, 3, 4, 5,    7,      ],
             36 => &[                    1, 2,                     ],
             37 => &[             -1,    1,                        ],
             38 => &[                    1, 2,                     ],
             39 => &[                 0, 1, 2, 3,                  ],
             40 => &[                    1, 2, 3, 4,               ],
             41 => &[       -3,   -1, 0, 1, 2, 3, 4, 5,            ],
             42 => &[    -4,   -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             43 => &[       -3,   -1,    1, 2, 3, 4, 5, 6, 7,      ],
             44 => &[    -4,   -2,       1, 2, 3, 4, 5, 6, 7, 8,   ],
             45 => &[       -3,   -1,    1, 2, 3, 4, 5, 6, 7,      ],
             46 => &[                    1, 2, 3, 4, 5,            ],
             47 => &[          -2,-1, 0, 1, 2, 3,                  ],
             48 => &[          -2,       1, 2,                     ],
             49 => &[ -5,      -2,-1, 0, 1, 2, 3,                  ],
             50 => &[    -4,-3,-2,-1, 0, 1, 2, 3, 4,               ],
             51 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5,            ],
             52 => &[          -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             53 => &[             -1,    1, 2, 3, 4, 5, 6, 7,      ],
             54 => &[                       2,    4,    6,    8,   ],
             55 => &[             -1,    1,                        ],
             56 => &[                    1, 2,                     ],
             57 => &[                 0, 1, 2, 3,                  ],
             58 => &[                    1, 2, 3, 4,               ],
             59 => &[                 0, 1, 2, 3, 4, 5,            ],
             60 => &[                 0,    2, 3, 4,               ],
             61 => &[                       2, 3,                  ],
             62 => &[                 0, 1, 2, 3,                  ],
             63 => &[                 0,    2, 3,                  ],
             64 => &[                 0, 1, 2, 3,                  ],
             65 => &[                 0, 1, 2, 3, 4,               ],
             66 => &[                 0, 1, 2, 3, 4,               ],
             67 => &[                 0, 1, 2, 3,                  ],
             68 => &[                 0, 1, 2, 3,                  ],
             69 => &[                 0, 1, 2, 3,                  ],
             70 => &[                 0, 1, 2, 3,                  ],
             71 => &[                 0, 1, 2, 3,                  ],
             72 => &[          -2,    0, 1, 2, 3, 4,               ],
             73 => &[       -3,   -1, 0, 1, 2, 3, 4, 5,            ],
             74 => &[    -4,   -2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             75 => &[       -3,   -1, 0, 1, 2, 3, 4, 5, 6, 7,      ],
             76 => &[    -4,   -2,-1, 0, 1, 2, 3, 4, 5, 6, 7, 8,   ],
             77 => &[       -3,-2,-1,    1, 2, 3, 4, 5, 6, 7, 8, 9,],
             78 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5, 6,         ],
             79 => &[       -3,-2,-1, 0, 1, 2, 3,    5,            ],
             80 => &[          -2,       1, 2,                     ],
             81 => &[ -5,      -2,-1,    1, 2, 3,                  ],
             82 => &[    -4,   -2,-1, 0, 1, 2, 3, 4,               ],
             83 => &[       -3,-2,-1, 0, 1, 2, 3, 4, 5,            ],
             84 => &[          -2,          2,    4, 5, 6,         ],
             85 => &[             -1,    1,    3,    5,    7,      ],
             86 => &[                       2,          6,         ],
             87 => &[                    1,                        ],
             88 => &[                       2,                     ],
             89 => &[                          3,                  ],
             90 => &[             -1,    1, 2, 3, 4,               ],
             91 => &[                       2, 3, 4, 5,            ],
             92 => &[             -1,    1, 2, 3, 4, 5, 6,         ],
             93 => &[                       2, 3, 4, 5, 6, 7,      ],
             94 => &[                       2, 3, 4, 5, 6, 7, 8,   ],
             95 => &[                       2, 3, 4, 5, 6, 7,      ],
             96 => &[                          3, 4, 5, 6,         ],
             97 => &[                       2, 3, 4, 5,            ],
             98 => &[                       2, 3, 4, 5,            ],
             99 => &[                       2, 3, 4,               ],
            100 => &[                       2, 3,                  ],
            101 => &[                       2, 3,                  ],
            102 => &[                       2, 3,                  ],
            103 => &[                          3,                  ],
            104 => &[                          3, 4,               ],
            105 => &[                          3, 4, 5,            ],
            106 => &[                          3, 4, 5, 6,         ],
            107 => &[                          3, 4, 5,    7,      ],
            108 => &[                          3, 4,    6,    8,   ],
            109 => &[                    1,    3,       6,         ],
            110 => &[                       2,    4,    6,         ],
            111 => &[             -1,          3,    5,            ],
            112 => &[                       2,    4,               ],



            116 => &[          -2,                4,               ],
            117 => &[             -1,                5,            ],
            118 => &[             -1,    1, 2,    4,    6,         ],
            _   => &[]
        };

        let main: &'static [i8] = match self.atomic_number() {
              1 => &[ -1, 1, ],
              3 => &[  1, ],
              4 => &[  2, ],
              5 => &[  3, ],
              6 => &[ -4, 4, ],
              7 => &[ -3, 3, 5, ],
              8 => &[ -2, ],
              9 => &[ -1, ],
             11 => &[  1, ],
             12 => &[  2, ],
             13 => &[  3, ],
             14 => &[ -4, 4, ],
             15 => &[ -3, 3, 5, ],
             16 => &[ -2, 2, 4, 6, ],
             17 => &[ -1, 1, 3, 5, 7, ],
             19 => &[  1, ],
             20 => &[  2, ],
             21 => &[  3, ],
             22 => &[  4, ],
             23 => &[  5, ],
             24 => &[  3, 6, ],
             25 => &[  2, 4, 7, ],
             26 => &[  2, 3, ],
             27 => &[  2, 3, ],
             28 => &[  2, ],
             29 => &[  2, ],
             30 => &[  2, ],
             31 => &[  3, ],
             32 => &[ -4, 2, 4, ],
             33 => &[ -3, 3, 5, ],
             34 => &[ -2, 2, 4, 6, ],
             35 => &[ -1, 1, 3, 5, ],
             36 => &[  2, ],
             37 => &[  1, ],
             38 => &[  2, ],
             39 => &[  3, ],
             40 => &[  4, ],
             41 => &[  5, ],
             42 => &[  4, 6, ],
             43 => &[  4, 7, ],
             44 => &[  3, 4, ],
             45 => &[  3, ],
             46 => &[  2, 4, ],
             47 => &[  1, ],
             48 => &[  2, ],
             49 => &[  3, ],
             50 => &[ -4, 2, 4, ],
             51 => &[ -3, 3, 5, ],
             52 => &[ -2, 2, 4, 6, ],
             53 => &[ -1, 1, 3, 5, 7, ],
             54 => &[  2, 4, 6, ],
             55 => &[  1, ],
             56 => &[  2, ],
             57 => &[  3, ],
             58 => &[  3, 4, ],
             59 => &[  3, ],
             60 => &[  3, ],
             61 => &[  3, ],
             62 => &[  3, ],
             63 => &[  2, 3, ],
             64 => &[  3, ],
             65 => &[  3, ],
             66 => &[  3, ],
             67 => &[  3, ],
             68 => &[  3, ],
             69 => &[  3, ],
             70 => &[  3, ],
             71 => &[  3, ],
             72 => &[  4, ],
             73 => &[  5, ],
             74 => &[  4, 6, ],
             75 => &[  4, ],
             76 => &[  4, ],
             77 => &[  3, 4, ],
             78 => &[  2, 4, ],
             79 => &[  3, ],
             80 => &[  1, 2, ],
             81 => &[  1, 3, ],
             82 => &[  2, 4, ],
             83 => &[  3, ],
             84 => &[ -2, 2, 4, ],
             85 => &[ -1, 1, ],
             87 => &[  1, ],
             88 => &[  2, ],
             89 => &[  3, ],
             90 => &[  4, ],
             91 => &[  5, ],
             92 => &[  6, ],
             93 => &[  5, ],
             94 => &[  4, ],
             95 => &[  3, ],
             96 => &[  3, ],
             97 => &[  3, ],
             98 => &[  3, ],
             99 => &[  3, ],
            100 => &[  3, ],
            101 => &[  3, ],
            102 => &[  3, ],
            103 => &[  3, ],
            104 => &[  4, ],
            _   => &[]
        };

        (main, all)
    }
}

