use ndarray::array;
use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    let data = array![
    	[1, 2, 3, 4, 5], 
    	[2, 5, 6, 7, 8], 
    	[3, 11, 12, 13, 14],
    	];
    let dim = 5;

    pre::Data::new(data.iter(), dim)
        .title("My Title")
        .save(1)?;

    Ok(())
}
