use cube::*;

use nom::types::CompleteStr as Input;
use nom::*;
// remove whitespace first

named!(layer<Input, Layer>,
    map!(
        one_of!("UFRLBDMESufrlbdXYZ"),
        |ly| match ly {
            'U' => Layer::U,
            'F' => Layer::F,
            'R' => Layer::R,
            'L' => Layer::L,
            'B' => Layer::B,
            'D' => Layer::D,
            'M' => Layer::M,
            'E' => Layer::E,
            'S' => Layer::S,
            'u' => Layer::Uw,
            'f' => Layer::Fw,
            'r' => Layer::Rw,
            'l' => Layer::Lw,
            'b' => Layer::Bw,
            'd' => Layer::Dw,
            'X' => Layer::X,
            'Y' => Layer::Y,
            'Z' => Layer::Z,
            _ => unreachable!()
        }
    )
);

// move
// alg
