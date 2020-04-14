use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    // Simpliest use

    let data = 0..10;
    data.preexplore().plot("id_1")?;

    // Data is an iterator already

    let data = 0..10;
    let mut plotting = data.preexplore();
    plotting.title("My Title").logx(3).plot("id_2")?;

    // Data is a full array

    let data = vec![0, 1, 2, 3, 4];
    let mut plotting = (&data).preexplore();
    plotting.title("My Title 2").logx(2).plot("id_3")?;

    println!("{:?}", data);

    Ok(())
}
