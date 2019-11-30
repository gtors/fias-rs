use fias;
use env_logger;
use failure::{Error};

#[tokio::main]
async fn main() -> Result<(), Error>{
    env_logger::init();

    let version_data = fias::actual::read_version_date().await?;
    println!("{}", version_data);
    Ok(())
}
