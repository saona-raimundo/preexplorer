use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let data: Vec<Vec<f64>> = (1..15).map(|i| {
	    	(0..10).map(|j| {
                let j = j as f64;
                let i = i as f64; 
	    		// Some computation
	    		i + j / i
	    	}).collect()
	    })
    	.collect();

    pre::SequenceViolin::new(data)
        .set_title("Numerical results through violins")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}
