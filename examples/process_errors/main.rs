use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let times: Vec<i32> = (0..10).map(|i: i32| i.pow(2)).collect();
    let values: Vec<Vec<f64>> = (0..10).map(|i| {
	    	(0..10).map(|j| {
	    		// Some computation
	    		(i as f64).sin() * j as f64
	    	}).collect()
	    })
    	.collect();

    let pro_error_1 = pre::ProcessError::new(&times, values)
        .set_title("variable error")
        .to_owned();

    let values: Vec<Vec<f64>> = (0..10).map(|i| {
            (0..10).map(|j| {
                // Some other computation
                (30 - i * j) as f64
            }).collect()
        })
        .collect();

    let pro_error_2 = pre::ProcessError::new(&times, values)
        .set_title("increasing error")
        .to_owned();

    let mut comparison = pro_error_1 + pro_error_2;

    comparison.set_title("Various processes with error margin")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}
