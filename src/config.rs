use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use std::convert::From;
use std::fs::File;

#[derive(Debug)]
pub struct Configuration {
    file: String,
    nbytes: Nbytes,
    endian: Endianess,
}

impl Configuration {
    pub fn new(file: String, nbytes: Nbytes, endian: Endianess) -> Self {
        Self {
            file,
            nbytes,
            endian,
        }
    }
}

impl From<Configuration> for Vec<f64> {
    fn from(c: Configuration) -> Vec<f64> {
        let mut f = File::open(c.file).unwrap();
        let meta = f.metadata().unwrap();
        let filesize = meta.len() as usize;

        let bufsize = match c.nbytes {
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

        match c.endian {
            Endianess::Native => f.read_f64_into::<NativeEndian>(&mut buf).unwrap(),
            Endianess::Big => f.read_f64_into::<BigEndian>(&mut buf).unwrap(),
            Endianess::Little => f.read_f64_into::<LittleEndian>(&mut buf).unwrap(),
        }

        buf
    }
}

#[derive(Debug)]
pub enum Nbytes {
    Whole,
    Bytes(usize),
}

#[derive(Debug)]
pub enum Endianess {
    Big,
    Little,
    Native,
}
