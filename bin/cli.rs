use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use clap::{App, Arg};
use std::convert::From;
use std::fs::File;

fn main() {
    let matches = configure().get_matches();
    let filename = matches.value_of("FILE").unwrap();
    let nbytes = if matches.is_present("nbytes") {
        Nbytes::Bytes(
            matches
                .value_of("nbytes")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        )
    } else {
        Nbytes::Whole
    };
    let endian = match matches.value_of("endian").unwrap() {
        "big" => Endianess::Big,
        "little" => Endianess::Little,
        _ => Endianess::Native,
    };
    let config = Configuration::new(String::from(filename), nbytes, endian);

    let v = match matches.value_of("datatype").unwrap() {
        "f32" => Data::Float(Vec::from(config)),
        _ => Data::Double(Vec::from(config)),
    };
    println!("{:?}", v);
}

fn is_usize(input: String) -> Result<(), String> {
    match input.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Can not parse {:?} as number", input)),
    }
}

fn configure() -> App<'static, 'static> {
    App::new("boel")
        .version("0.1.0")
        .author("ucyo <cayoglu@me.com>")
        .about("Iterates over data via windows or chunks")
        .arg(
            Arg::with_name("FILE")
                .help("File to be read")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("nbytes")
                .help("Number of bytes to be read from file")
                .short("b")
                .long("nbytes")
                .takes_value(true)
                .max_values(1)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("endian")
                .help("Endianess of file")
                .short("e")
                .long("endian")
                .default_value("native")
                .takes_value(true)
                .possible_values(&["native", "little", "big"])
                .max_values(1),
        )
        .arg(
            Arg::with_name("datatype")
                .help("f32 or f64")
                .short("d")
                .long("datatype")
                .default_value("f64")
                .takes_value(true)
                .possible_values(&["f64", "f32"])
                .max_values(1),
        )
}

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

impl From<Configuration> for Vec<f32> {
    fn from(c: Configuration) -> Vec<f32> {
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
        assert_eq!(bufsize % 4, 0);
        let mut buf = vec![0.0; bufsize as usize / 4];

        match c.endian {
            Endianess::Native => f.read_f32_into::<NativeEndian>(&mut buf).unwrap(),
            Endianess::Big => f.read_f32_into::<BigEndian>(&mut buf).unwrap(),
            Endianess::Little => f.read_f32_into::<LittleEndian>(&mut buf).unwrap(),
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

#[derive(Debug)]
pub enum Data {
    Float(Vec<f32>),
    Double(Vec<f64>),
}
