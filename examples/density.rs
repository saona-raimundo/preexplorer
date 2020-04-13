use preexplorer::prelude::*;


fn main() {
    let values = (1..20).chain(1..10).chain(1..10);

    pre::Density::new(values)
        .title("My Title")
        .logx(2)
        .plot("1")
        .unwrap();
}


// fn main() {
//     let values = (0..200).chain(0..50);

//     pre::Density::new(values)
//         .title("My Title")
//         // .logx(2)
//         .plot("1")
//         .unwrap();
// }
