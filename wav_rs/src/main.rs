mod header;
use header::format::Format;
use header::master::{self, Master};
use std::fs::File;
use std::io::{BufReader, Read};
fn main() -> std::io::Result<()> {
    let mut wav = File::open("/home/octavia/Projects/algos/wav_rs/perfect_girl.wav")?;
    let mut reader = BufReader::new(wav);

    let mut master_buf = [0; 12];
    //fix to work with read_exact
    reader.read_exact(&mut master_buf)?;
    let master = Master::from_slice(&master_buf)?;
    println!("{:x?}", master);

    let mut format_buf = [0; 24];
    reader.read_exact(&mut format_buf)?;
    let format = Format::from_slice(format_buf)?;
    println!("{:x?}", format);
    Ok(())
}
