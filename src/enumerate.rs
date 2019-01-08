
// ULB UBR URF UFL
// UB, UR, UF, UL

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::cube::*;

static CO: [[u8; 4]; 8] = [
    [0, 0, 0, 0], // solved
    [2, 2, 2, 0], // sune
    [0, 1, 1, 1], // antisune
    [0, 1, 0, 2], // L
    [2, 1, 0, 0], // U
    [1, 2, 0, 0], // T
    [1, 1, 2, 2], // Bruno
    [1, 2, 1, 2], // H
];

static CP: [(usize, usize); 6] = [
    (0, 0),
    (0, 1), // back
    (1, 2), // right
    (2, 3), // front
    (3, 0), // left
    (0, 2), // diag
];


static EO: [[u8; 4]; 8] = [
    [0, 0, 0, 0],
    [0, 1, 0, 1],
    [1, 0, 1, 0],
    [1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 1, 1],
    [1, 0, 0, 1],
    [1, 1, 1, 1],
];

macro_rules! edge_cycles {
    ( $( $x:expr ),* ) => {
        Transform {
            edge_cycles: vec![$($x),*],
            edge_flips: vec![],
            corner_cycles: vec![vec![]],
            corner_twists: vec![],
            centre_cycles: vec![],
        }
    };
}

#[allow(non_snake_case)]
fn get_EP() -> ([Transform; 12], [Transform; 12]) {
    let EP = [
        // solved
        edge_cycles![vec![]],
        // 1x H perm
        edge_cycles![vec![0, 2], vec![1, 3]],
        // 2x Z perm
        edge_cycles![vec![0, 3], vec![1, 2]],
        edge_cycles![vec![1, 0], vec![2, 3]],
        // 8* U perm
        edge_cycles![vec![0, 1, 2]],
        edge_cycles![vec![2, 1, 0]],
        edge_cycles![vec![1, 2, 3]],
        edge_cycles![vec![3, 2, 1]],
        edge_cycles![vec![2, 3, 0]],
        edge_cycles![vec![0, 3, 2]],
        edge_cycles![vec![3, 0, 1]],
        edge_cycles![vec![1, 0, 3]],
    ];
    let EP_PARITY = [
        // 4* adj swap
        edge_cycles![vec![0, 1]],
        edge_cycles![vec![1, 2]],
        edge_cycles![vec![2, 3]],
        edge_cycles![vec![3, 0]],
        // 2* opp swap
        edge_cycles![vec![0, 2]],
        edge_cycles![vec![1, 3]],
        // 2* rotation
        edge_cycles![vec![0, 1, 2, 3]],
        edge_cycles![vec![3, 2, 1, 0]],
        // 4* weird swap
        edge_cycles![vec![0, 2, 3, 1]],
        edge_cycles![vec![0, 2, 1, 3]],
        edge_cycles![vec![2, 0, 1, 3]],
        edge_cycles![vec![2, 0, 3, 1]],
    ];
    (EP, EP_PARITY)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    pub index: String,
    edges: Vec<Edge>,
    corners: Vec<Corner>,
}

#[allow(non_snake_case)]
pub fn get_cases() -> Vec<Case> {

    let mut positions = Vec::new();
    let (EP, EP_PARITY) = get_EP();

    let mut index = 0;
    for (co_i, co) in CO.iter().enumerate() {
        for (cp_i, cp) in CP.iter().enumerate() {
            for ep_i in 0..12 {
                let ep = if cp_i == 0 { &EP } else { &EP_PARITY };
                let ep_transform = &ep[ep_i];
                for eo in &EO {

                    let mut cube = Cube::new();

                    // do cp
                    let (a, b) = cp;
                    if a != b {
                        cube.corners.swap(*a, *b);
                    }
                    // do co
                    for (i, rot) in co.iter().enumerate() {
                        match rot {
                            2 => cube.corners[i].twist(Twist::Acw),
                            1 => cube.corners[i].twist(Twist::Cw),
                            _ => {},
                        }
                    }
                    // do ep
                    cube.do_transform(ep_transform);
                    // do eo
                    for (i, flip) in eo.iter().enumerate() {
                        if flip == &1 {
                            cube.edges[i].flip();
                        }
                    }


                    let valid_solved = co_i != 0 || (cp_i != 1 && cp_i != 3 && cp_i != 4);
                    let valid_h = co_i != 7 || (cp_i != 1 && cp_i != 4);

                    if valid_solved && valid_h &&
                        !ROTATE_INDEX.contains(&cube.get_ll_index())  {
                            positions.push((index, cube));
                            index += 1;
                    }
                }
            }
        }
    }

    // get unique indexes
    let mut map = HashMap::new();

    let pos_len = positions.len();
    for (index, cube) in &mut positions {
        map.insert(cube.get_ll_index(), (*index, cube.clone()));
    }
    console!("LL cases {:?}", (pos_len, map.len()));

    // convert to ordered vec
    let mut vec: Vec<(&u64, &(usize, Cube))> = map.iter().collect();
    vec.sort_by(|a, b| (a.1).0.cmp(&(b.1).0));
    let mut vec: Vec<Case> = vec.iter()
        .map(|(k, (_, v))| Case {
            index: format!("{}", *k),
            edges: v.edges[..4].iter().fold(vec![], |mut acc, cur| {
                acc.push(cur.clone());
                acc
            }),
            corners: v.corners[..4].iter().fold(vec![], |mut acc, cur| {
                acc.push(cur.clone());
                acc
            }),
        })
        .collect();

    // remove solved case
    vec.swap_remove(0);

    vec
}

// Code to generate indices;

// fn cmp_auf_cube(this: &Cube, case: &Cube) -> bool {
//     let is_same = |edges: &[Edge], corners: &[Corner]| {
//         edges == &case.edges[..4] && corners == &case.corners[..4]
//     };
//     let edges = &mut this.edges.clone()[..4];
//     let corners = &mut this.corners.clone()[..4];
//     for _ in 0..4 {
//         rotate_stickers(edges, corners);
//         for _ in 0..4 {
//             rotate_edges(edges, corners);
//             if is_same(edges, corners) {
//                 return true
//             }
//         }
//     }
//     false
// }

// fn rotate_stickers(edges: &mut [Edge], corners: &mut [Corner]) {
//     for edge in edges.iter_mut() {
//         rotate_sticker(&mut edge.0);
//         rotate_sticker(&mut edge.1);
//     }
//     for corner in corners.iter_mut() {
//         rotate_sticker(&mut corner.0);
//         rotate_sticker(&mut corner.1);
//         rotate_sticker(&mut corner.2);
//     }
// }

// fn rotate_edges(edges: &mut [Edge], corners: &mut [Corner]) {
//     edges.rotate_right(1);
//     corners.rotate_right(1);
// }


// fn rotate_sticker(sticker: &mut Face) {
//     use crate::cube::Face::*;
//     std::mem::replace(sticker, match sticker {
//         U => U,
//         B => R,
//         R => F,
//         F => L,
//         L => B,
//         _ => unreachable!()
//     });
// }

static ROTATE_INDEX: [u64; 212] = [
    4816698677396563u64, 836810182231123u64,
    809043218662483u64, 4815216913679443u64,
    2425046138618963u64, 504680361231443u64,
    421207671833683u64, 2421975237002323u64,
    613321558983763u64, 3618338377303123u64,
    223123780142163u64, 334492282127443u64,
    1227373033292883u64, 1336379303265363u64,
    336768614794323u64, 225486012154963u64,
    1225096700626003u64, 1338741535278163u64,
    813144912430163u64, 868829163422803u64,
    4823184078013523u64, 4876527571829843u64,
    871126970926163u64, 815485669606483u64,
    4820886270510163u64, 4878868329006163u64,
    818406247367763u64, 901943361274963u64,
    4826899224724563u64, 4909620294845523u64,
    902673505715283u64, 819222291154003u64,
    4826169080284243u64, 4910436338631763u64,
    406819531392083u64, 434736818816083u64,
    2412998755353683u64, 2438510861091923u64,
    436970201810003u64, 409224713077843u64,
    2410765372359763u64, 2440916042777683u64,
    607351554442323u64, 635225892193363u64,
    3616233843328083u64, 3640972854953043u64,
    638275318973523u64, 610486880568403u64,
    3613184416547923u64, 3644108181079123u64,
    217411473638483u64, 300991537218643u64,
    1220887632675923u64, 1302835608683603u64,
    302451826099283u64, 219043561210963u64,
    1219427343795283u64, 1304467696256083u64,
    623736854676563u64, 735083881825363u64,
    3631072955335763u64, 3740852319421523u64,
    736608595215443u64, 625304517739603u64,
    3629548241945683u64, 3742419982484563u64,
    807582784741647u64, 835457122492687u64,
    4817621950325007u64, 4843134056063247u64,
    837733455159567u64, 809945016754447u64,
    4815345617658127u64, 4845496288076047u64,
    607050761691407u64, 634968049115407u64,
    3614386862350607u64, 3640672062202127u64,
    636428337996047u64, 608682849263887u64,
    3612926573469967u64, 3642304149774607u64,
    2413922028282127u64, 437893474738447u64,
    410126511169807u64, 2410894076338447u64,
    4827822497653007u64, 903596778643727u64,
    820124089245967u64, 4826297784262927u64,
    214404851491087u64, 1214015539962127u64,
    623586385781007u64, 734954887766287u64,
    3630149392326927u64, 3740701850525967u64,
    735685032206607u64, 624402429567247u64,
    3629419247886607u64, 3741517894312207u64,
    423505334296847u64, 534852361445647u64,
    2429684558258447u64, 2538690828230927u64,
    537150168949007u64, 425846091473167u64,
    2427386750755087u64, 2541031585407247u64,
    417642703937807u64, 501222767517967u64,
    2422275739672847u64, 2504996809793807u64,
    501909962285327u64, 418501697397007u64,
    2421588544905487u64, 2505855803253007u64,
    217711976309007u64, 301249090216207u64,
    1222734323573007u64, 1303136111354127u64,
    304298516996367u64, 220847302435087u64,
    1219684896792847u64, 1306271437480207u64,
    612913392050447u64, 668597643043087u64,
    3621795680936207u64, 3674366080639247u64,
    671668544659727u64, 616027243340047u64,
    3618724779319567u64, 3677479931928847u64,
    809043270702368u64, 4815216965719328u64,
    421207723873568u64, 2421975289042208u64,
    613321611023648u64, 3618338429343008u64,
    225486064194848u64, 1225096752665888u64,
    406819583431968u64, 434736870855968u64,
    2412998807393568u64, 2438510913131808u64,
    436970253849888u64, 409224765117728u64,
    2410765424399648u64, 2440916094817568u64,
    607351606482208u64, 635225944233248u64,
    3616233895367968u64, 3640972906992928u64,
    638275371013408u64, 610486932608288u64,
    3613184468587808u64, 3644108233119008u64,
    217411525678368u64, 300991589258528u64,
    1220887684715808u64, 1302835660723488u64,
    302451878139168u64, 219043613250848u64,
    1219427395835168u64, 1304467748295968u64,
    623736906716448u64, 735083933865248u64,
    3631073007375648u64, 3740852371461408u64,
    736608647255328u64, 625304569779488u64,
    3629548293985568u64, 3742420034524448u64,
    807582836781532u64, 835457174532572u64,
    4817622002364892u64, 4843134108103132u64,
    837733507199452u64, 809945068794332u64,
    4815345669698012u64, 4845496340115932u64,
    607050813731292u64, 634968101155292u64,
    3614386914390492u64, 3640672114242012u64,
    636428390035932u64, 608682901303772u64,
    3612926625509852u64, 3642304201814492u64,
    410126563209692u64, 2410894128378332u64,
    820124141285852u64, 4826297836302812u64,
    214404903530972u64, 1214015592002012u64,
    624402481607132u64, 3629419299926492u64,
    217712028348892u64, 301249142256092u64,
    1222734375612892u64, 1303136163394012u64,
    304298569036252u64, 220847354474972u64,
    1219684948832732u64, 1306271489520092u64,
    612913444090332u64, 668597695082972u64,
    3621795732976092u64, 3674366132679132u64,
    671668596699612u64, 616027295379932u64,
    3618724831359452u64, 3677479983968732u64,
];
