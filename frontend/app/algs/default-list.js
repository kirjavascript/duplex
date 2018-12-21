export default [
    { name: 'sune', moves: `R U R' U R U2 R'`, mirror: true, invert: true },
    { name: 'dblsune', moves: `R U R' U R U' R' U R U2 R'`, mirror: true, invert: true },
    { name: 'widesune', moves: `r U R' U R U2 r'`, mirror: true, invert: true },
    { name: 'dblwidesune', moves: `r U R' U R U' R' U R U2 r'`, mirror: true, invert: true },
    { name: 'pureflip', moves: `RUR2FRF'y'R'UR`, mirror: true, invert: true },
    { name: 'suneflip', moves: `RU2R'U2R'FRF'`, mirror: true, invert: true },
    { name: 'fruruf', moves: `FRUR'U'F'`, mirror: true, invert: true },
    { name: 'dlbfruruf', moves: `F R U R' U' R U R' U' F'`, mirror: true, invert: true },
    { name: 'aperm', moves: `l'UR'D2RU'R'D2lR`, mirror: true, invert: true },
    { name: 'tperm', moves: `RUR'U'R'FR2U'R'U'RUR'F'`, mirror: false, invert: false },
    { name: 'sexysledge', moves: `R U R' U' R' F R F'`, mirror: true, invert: true },
    { name: 'uperm', moves: `M2 U'MU2M'U'M2`, mirror: true, invert: true },
];

// L F' L' U' L F L' F' U F
// R' U' R' F R F' U R
// F R U' R D R' U2 R D' R2 U' F'
// R U R' U F' L' U L F U' R U' R'
// L F R' F R F L' F R' F' L F L' R
// R2 D R' U2 R D' R' U2 R'
// R' F' R U R' U' R' F R2 U' R' U2 R
// F R2 D R' U R D' R2 U' F'
// R U' L' U R' U L U L' U L
// R' U' R U' R' U F' U F R
// R' U' R' F R F' R U' R' U2 R
// R U R' U' L R' F R F' L'
// R' L F R L' U2 R' L F R L'
// R U2 R' U2 R' F R2 U R' U' F'
// F R U' R' U' R U2 R' U' F'
// R' U' R U R' F' R U R' U' R' F R2
// L F L' R U R' U' L F' L'
// R U R' U R U' R' U' R' F R F'
// F U R U2 R' U' R U R' F'
// F R U R' U' R U' R' U R U R' F'
// F R U R2 U2 R2 U R2 U R F'
// F R' U' R2 F R' F' R2 U R U F'
