use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // Simpliest use

    let data = 0..10;
    data.preexplore().plot("id_1")?;

    // Data is an iterator already

    let data = 0..10;
    let mut plotting = data.preexplore();
    plotting.set_title("My Title").set_logx(3).plot("id_2")?;

    // Data is a full array

    let data = vec![0, 1, 2, 3, 4];
    let mut plotting = (&data).preexplore();
    plotting.set_title("My Title 2").set_logx(2).plot("id_3")?;

    println!("{:?}", data);

    Ok(())
}
