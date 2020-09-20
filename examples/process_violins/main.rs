use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let domain_1: Vec<f64> = (1..4).map(|i| (500. * i as f64).sqrt()).collect();
    let image_1: Vec<Vec<f64>> = (1..4)
        .map(|i| {
            (-5..10)
                .map(|j| {
                    let j = j as f64;
                    let i = i as f64;
                    // Some computation
                    3. * i + 3. * j.cos() / i
                })
                .collect()
        })
        .collect();

    let domain_2: Vec<f64> = (1..7).map(|i| (500. * i as f64).sqrt()).collect();
    let image_2: Vec<Vec<f64>> = (1..7)
        .map(|i| {
            (0..10)
                .map(|j| {
                    let j = j as f64;
                    let i = i as f64;
                    // Some computation
                    -3. * i + 3. * j.sin() / i
                })
                .collect()
        })
        .collect();

    (pre::ProcessViolin::new(domain_1, image_1)
        .set_title("first")
        .to_owned()
        + pre::ProcessViolin::new(domain_2, image_2)
            .set_title("second")
            .to_owned())
    .set_xlabel("index")
    .set_ylabel("value")
    .set_title("Overall title")
    .plot("my_identifier")?;

    Ok(())
}
