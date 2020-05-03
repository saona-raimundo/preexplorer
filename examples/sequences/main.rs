use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    comparing_many()?;

    increasing_comparison()?;

    Ok(())
}

fn comparing_many() -> anyhow::Result<()> {
    // Computing the data

    let data_1 = vec![0. as f32, 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Define plotables

    let seq_1 = data_1.preexplore();
    let seq_2 = data_2.preexplore();

    // Create comparison and plot

    pre::Sequences::new(vec![seq_1, seq_2])
        .set_title("All together")
        .plot("1")?;

    Ok(())
}

fn increasing_comparison() -> anyhow::Result<()> {
    // First Sequence

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let mut main_plot = data_1
        .preexplore()
        .set_title("First")
        .to_owned()
        .to_comparison();

    // Another sequence

    let data_2 = vec![0., 1.4, 10., 4.];
    let plot2 = data_2.preexplore().set_title("Second").to_owned();

    main_plot.add(plot2);

    // Add many

    let data_3 = vec![0.1, 1.5, 7., 5.];
    let group_of_plottings = vec![data_3.preexplore().set_title("Third").to_owned()];

    main_plot.add_many(group_of_plottings);

    // Change some settings

    main_plot.set_title("More comparisons");

    // Plot everything

    main_plot.plot("my_id")?;

    Ok(())
}
