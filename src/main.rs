use std::io::{self, Read, Write};
use std::io::{Error, ErrorKind};
use structopt::StructOpt;

extern crate base64;

#[derive(Debug, StructOpt)]
#[structopt(name = "b64url", about = "Base 64 URL encoder/decoder.")]
struct Cli {
    #[structopt(short, long)]
    decode: bool,

    #[structopt(short, long)]
    encode: bool,
}

fn encode<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let result = base64::encode_config(input, base64::URL_SAFE_NO_PAD);
    return result.as_bytes().to_vec();
}

fn decode<T: AsRef<[u8]>>(input: T) -> io::Result<Vec<u8>> {
    let inputstr = std::str::from_utf8(input.as_ref())
        .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;

    base64::decode_config(inputstr, base64::URL_SAFE_NO_PAD)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}

fn main() -> io::Result<()> {
    let mut opt = Cli::from_args();
    if opt.decode == false && opt.encode == false {
        opt.encode = true;
    }
    if opt.decode && opt.encode {
        let result = Cli::clap().print_help();
        eprintln!("\n"); // print_help does not print a trailing newline.
        match result {
            Ok(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Cannot use --encode and --decode at the same time!",
                ))
            }
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
        }
    }

    let mut buffer = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    while match buffer.last() {
        Some(b) => *b == b'\r' || *b == b'\n',
        None => false,
    } {
        buffer.truncate(buffer.len() - 1);
    }

    if opt.decode {
        match decode(buffer) {
            Ok(result) => {
                io::stdout()
                    .write_all(&result)
                    .and_then(|_| io::stdout().write_all(b"\n"))
            }
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
        }
    } else {
        io::stdout()
            .write_all(&encode(buffer))
            .and_then(|_| io::stdout().write_all(b"\n"))
    }
}
