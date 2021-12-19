use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut f = File::open("Cargo.toml")?;
    let meta = f.metadata()?;
    let filesize = meta.len();
    let nbytes = Nbytes::Bytes(8);

    let bufsize = match nbytes {
        Nbytes::Whole => filesize,
        Nbytes::Bytes(n) => {
            if n <= filesize {
                n
            } else {
                filesize
            }
        }
    };
    assert_eq!(bufsize % 8, 0);
    let mut buf = vec![0.0; bufsize as usize / 8];

    let end = Endianess::Native;
    match end {
        Endianess::Native => f.read_f64_into::<NativeEndian>(&mut buf)?,
        Endianess::Big => f.read_f64_into::<BigEndian>(&mut buf)?,
        Endianess::Little => f.read_f64_into::<LittleEndian>(&mut buf)?,
    }
    println!("{:?}", buf);
    Ok(())
}

enum Nbytes {
    Whole,
    Bytes(u64),
}

enum Endianess {
    Big,
    Little,
    Native,
}
