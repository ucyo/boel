use boel::{base::Base, base::Data, shape::Shape};
use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use clap::{App, Arg};
use ndarray::{Array, Ix1, Ix2};
use std::convert::From;
use std::fs::File;

fn main() {
    let matches = configure().get_matches();
    let filename = matches.value_of("FILE").unwrap();

    // Check number of bytes to be read
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

    // Check endianess
    let endian = match matches.value_of("endian").unwrap() {
        "big" => Endianess::Big,
        "little" => Endianess::Little,
        _ => Endianess::Native,
    };

    // Setup configuration
    // Read data using given datatype
    let dt = match matches.value_of("datatype").unwrap() {
        "f32" => Datatype::Float,
        _ => Datatype::Double,
    };

    // Setup configuration
    let config = Configuration::new(String::from(filename), nbytes, endian, dt);

    // Print result
    println!("{:?}", v);

    // Generate Base
    let base_shape = if matches.is_present("shape") {
        matches
            .values_of("shape")
            .unwrap()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    } else {
        match nbytes {
            Nbytes::Whole => vec![File::open(filename).unwrap().metadata().unwrap().len() as usize],
            Nbytes::Bytes(m) => vec![m],
        }
    };
    let shape = match base_shape.len() {
        1 => {
            let mut ret = [0usize; 1];
            ret[0] = base_shape[0];
            Shape::D1(ret)
        }
        2 => {
            let mut ret = [0usize; 2];
            ret[0] = base_shape[0];
            ret[1] = base_shape[1];
            Shape::D2(ret)
        }
        _ => panic!("Only supporting up to two dimensions"),
    };

    match (&config.dt, &shape) {
        (Datatype::Float, Shape::D2(_)) => {
            let raw: Data<f32> = Data::from(config);
            let base = Base::new(raw, shape);
            let arr = Array::<f32, Ix2>::from(base);
            println!("{:?}", arr);
        }
        (Datatype::Float, Shape::D1(_)) => {
            let raw: Data<f32> = Data::from(config);
            let base = Base::new(raw, shape);
            let arr = Array::<f32, Ix1>::from(base);
            println!("{:?}", arr);
        }
        (Datatype::Double, Shape::D2(_)) => {
            let raw: Data<f64> = Data::from(config);
            let base = Base::new(raw, shape);
            let arr = Array::<f64, Ix2>::from(base);
            println!("{:?}", arr);
        }
        (Datatype::Double, Shape::D1(_)) => {
            let raw: Data<f64> = Data::from(config);
            let base = Base::new(raw, shape);
            let arr = Array::<f64, Ix1>::from(base);
            println!("{:?}", arr);
        }
    }
}

fn is_usize(input: String) -> Result<(), String> {
    match input.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Can not parse {:?} as number", input)),
    }
}

fn is_usize_and_non_null(input: String) -> Result<(), String> {
    match input.parse::<usize>() {
        Ok(n) if n > 0 => Ok(()),
        Ok(_) => Err(format!("Expected input to be > 0, got {:?}", input)),
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
        .arg(
            Arg::with_name("shape")
                .help("Shape of array")
                .short("s")
                .long("shape")
                .takes_value(true)
                .min_values(2)
                .max_values(2)
                .validator(is_usize_and_non_null),
        )
}

#[derive(Debug)]
struct Configuration {
    file: String,
    nbytes: Nbytes,
    endian: Endianess,
    dt: Datatype,
}

impl Configuration {
    pub fn new(file: String, nbytes: Nbytes, endian: Endianess, dt: Datatype) -> Self {
        Self {
            file,
            nbytes,
            endian,
            dt,
        }
    }
}

impl From<Configuration> for Data<f64> {
    fn from(c: Configuration) -> Data<f64> {
        assert_eq!(
            c.dt,
            Datatype::Double,
            "Expected data type Double, got {:?}",
            c.dt
        );

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
        let unit_size = std::mem::size_of::<f64>();
        assert_eq!(
            bufsize % unit_size,
            0,
            "Buffersize {:?} is not complementary with unit size {:?}",
            bufsize,
            unit_size
        );
        let mut buf = vec![0.0; bufsize as usize / unit_size];

        match c.endian {
            Endianess::Native => f.read_f64_into::<NativeEndian>(&mut buf).unwrap(),
            Endianess::Big => f.read_f64_into::<BigEndian>(&mut buf).unwrap(),
            Endianess::Little => f.read_f64_into::<LittleEndian>(&mut buf).unwrap(),
        }
        buf
    }
}

impl From<Configuration> for Data<f32> {
    fn from(c: Configuration) -> Data<f32> {
        assert_eq!(
            c.dt,
            Datatype::Float,
            "Expected data type Float, got {:?}",
            c.dt
        );

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
        let unit_size = std::mem::size_of::<f32>();
        assert_eq!(
            bufsize % unit_size,
            0,
            "Buffersize {:?} is not complementary with unit size {:?}",
            bufsize,
            unit_size
        );
        let mut buf = vec![0.0; bufsize as usize / unit_size];

        match c.endian {
            Endianess::Native => f.read_f32_into::<NativeEndian>(&mut buf).unwrap(),
            Endianess::Big => f.read_f32_into::<BigEndian>(&mut buf).unwrap(),
            Endianess::Little => f.read_f32_into::<LittleEndian>(&mut buf).unwrap(),
        }
        buf
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub enum Datatype {
    Float,
    Double,
}
