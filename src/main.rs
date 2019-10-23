use external_gnuplot::errors::SavingError;
use failure::Fail;

fn main() {
    let e = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
    println!("{:?}", e);

    println!("{:?}", SavingError::new(e).cause());
}
