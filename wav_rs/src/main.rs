mod header;
use header::master::Master;
use std::fs::File;
use std::io::{BufReader, Read};
fn main() -> std::io::Result<()> {
    let wav = File::open("./perfect_girl.wav")?;
    let reader = BufReader::new(wav);

    println!("{:x?}", Master::from_bufreader(reader));
    Ok(())
}
