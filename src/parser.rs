use cube::*;

use nom::types::CompleteStr as Input;
use nom::*;

named!(layer<Input, Layer>,
    map!(
        one_of!("UFRLBDMESufrlbdxyz"),
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
            'x' => Layer::X,
            'y' => Layer::Y,
            'z' => Layer::Z,
            _ => unreachable!()
        }
    )
);

named!(order<Input, Order>,
    map!(
        opt!(one_of!("'2")),
        |or| match or {
            Some('\'') => Order::Prime,
            Some('2') => Order::Double,
            None => Order::Normal,
            _ => unreachable!()
        }
    )
);

named!(move_<Input, Move>,
    do_parse!(
        multispace0 >> layer: layer >>
        multispace0 >> order: order >>
        (Move { order, layer })
    )
);

named!(moves<Input, Vec<Move>>,
    many0!( move_ )
);

pub fn parse_moves(data: &str) -> Vec<Move> {
    moves(Input(data.trim()))
}
