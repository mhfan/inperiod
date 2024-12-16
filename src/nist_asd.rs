
use super::ChemElem::{self, *};
impl ChemElem { // Auto-generated by syncd.rs, DO NOT EDIT.
    pub const fn ionization_energy(&self) -> Option<(f64, f64)> {
        Some(match *self {
            H  => (13.598434599702, 0.000000000012),
            He => (24.587389011, 0.000000025),
            Li => (5.391714996, 0.000000022),
            Be => (9.322699, 0.000007),
            B  => (8.298019, 0.000003),
            C  => (11.2602880, 0.0000011),
            N  => (14.53413, 0.00004),
            O  => (13.618055, 0.000007),
            F  => (17.42282, 0.00005),
            Ne => (21.564541, 0.000007),
            Na => (5.13907696, 0.00000025),
            Mg => (7.646236, 0.000004),
            Al => (5.985769, 0.000003),
            Si => (8.15168, 0.00003),
            P  => (10.486686, 0.000015),
            S  => (10.3600167, 0.0000014),
            Cl => (12.967633, 0.000016),
            Ar => (15.7596119, 0.0000005),
            K  => (4.34066373, 0.00000009),
            Ca => (6.1131549210, 0.0000000005),
            Sc => (6.56149, 0.00006),
            Ti => (6.828120, 0.000012),
            V  => (6.746187, 0.000021),
            Cr => (6.76651, 0.00004),
            Mn => (7.4340380, 0.0000012),
            Fe => (7.9024681, 0.0000012),
            Co => (7.88101, 0.00012),
            Ni => (7.639878, 0.000017),
            Cu => (7.726380, 0.000004),
            Zn => (9.394197, 0.000006),
            Ga => (5.9993020, 0.0000012),
            Ge => (7.899435, 0.000012),
            As => (9.78855, 0.00025),
            Se => (9.752368, 0.000006),
            Br => (11.81381, 0.00006),
            Kr => (13.9996055, 0.0000020),
            Rb => (4.1771281, 0.0000012),
            Sr => (5.69486745, 0.00000012),
            Y  => (6.21726, 0.00010),
            Zr => (6.634126, 0.000005),
            Nb => (6.75885, 0.00004),
            Mo => (7.09243, 0.00004),
            Tc => (7.11938, 0.00003),
            Ru => (7.36050, 0.00005),
            Rh => (7.45890, 0.00005),
            Pd => (8.336839, 0.000010),
            Ag => (7.576234, 0.000025),
            Cd => (8.993820, 0.000016),
            In => (5.7863558, 0.0000005),
            Sn => (7.343918, 0.000012),
            Sb => (8.608389, 0.000012),
            Te => (9.009808, 0.000006),
            I  => (10.451236, 0.000025),
            Xe => (12.1298437, 0.0000015),
            Cs => (3.89390572743, 0.00000000017),
            Ba => (5.2116646, 0.0000012),
            La => (5.5769, 0.0006),
            Ce => (5.5386, 0.0004),
            Pr => (5.4702, 0.0004),
            Nd => (5.52475, 0.00005),
            Pm => (5.58187, 0.00004),
            Sm => (5.643722, 0.000021),
            Eu => (5.670385, 0.000005),
            Gd => (6.14980, 0.00004),
            Tb => (5.8638, 0.0006),
            Dy => (5.939061, 0.000006),
            Ho => (6.0215, 0.0006),
            Er => (6.1077, 0.0010),
            Tm => (6.184402, 0.000009),
            Yb => (6.254160, 0.000012),
            Lu => (5.425871, 0.000012),
            Hf => (6.825070, 0.000012),
            Ta => (7.549571, 0.000025),
            W  => (7.86403, 0.00010),
            Re => (7.83352, 0.00011),
            Os => (8.43823, 0.00020),
            Ir => (8.96702, 0.00022),
            Pt => (8.95883, 0.00010),
            Au => (9.225554, 0.000004),
            Hg => (10.437504, 0.000006),
            Tl => (6.1082873, 0.0000012),
            Pb => (7.4166799, 0.0000006),
            Bi => (7.285516, 0.000006),
            Po => (8.418070, 0.000004),
            At => (9.31751, 0.00008),
            Rn => (10.74850, 0.),
            Fr => (4.0727411, 0.0000011),
            Ra => (5.2784239, 0.0000025),
            Ac => (5.380235, 0.000012),
            Th => (6.30670, 0.00025),
            Pa => (5.89, 0.12),
            U  => (6.19405, 0.00006),
            Np => (6.265608, 0.000019),
            Pu => (6.02576, 0.00025),
            Am => (5.97381, 0.00025),
            Cm => (5.992241, 0.000020),
            Bk => (6.19785, 0.00025),
            Cf => (6.281878, 0.000006),
            Es => (6.36840, 0.00006),
            Fm => (6.50, 0.07),
            Md => (6.58, 0.07),
            No => (6.62621, 0.00005),
            Lr => (4.96, 0.05),
            Rf => (6.02, 0.04),
            Db => (6.8, 0.5),
            Sg => (7.8, 0.5),
            Bh => (7.7, 0.5),
            Hs => (7.6, 0.5),
            _  => return None
        })
    }
}

