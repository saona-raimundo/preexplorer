use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let data: Vec<Vec<f64>> = (0..10).map(|i| {
	    	(0..10).map(|j| {
	    		// Some computation
	    		(i as f64).sin() * j as f64
	    	}).collect()
	    })
    	.collect();

    let seq_error_1 = pre::SequenceError::new(data)
        .set_title("variable error")
        .to_owned();

    let data: Vec<Vec<f64>> = (0..10).map(|i| {
            (0..10).map(|j| {
                // Some other computation
                (30 - i * j) as f64
            }).collect()
        })
        .collect();

    let seq_error_2 = pre::SequenceError::new(data)
        .set_title("increasing error")
        .to_owned();

    let mut comparison = seq_error_1 + seq_error_2;

    comparison.set_title("Various sequences with error margin")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}
