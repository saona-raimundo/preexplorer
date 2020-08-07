use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let times = (0..10).map(|i: i32| i.pow(2));
    let values: Vec<Vec<f64>> = (0..10).map(|i| {
	    	(0..10).map(|j| {
	    		// Some computation
	    		(i as f64).sin() * j as f64
	    	}).collect()
	    })
    	.collect();

    pre::ProcessError::new(times, values)
        .set_title("Numerical results with variable error")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}
