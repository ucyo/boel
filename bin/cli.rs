use boel::{base::Base, base::Data, shape::Shape};
use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, ReadBytesExt};
use clap::{App, AppSettings, Arg, ArgMatches};
use log::{debug, info};
use ndarray::{Array, Ix1, Ix2};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;
use std::convert::From;
use std::fs::File;
use std::io::Write;

fn main() {
    env_logger::init();
    let matches = configure_app().get_matches();
    match matches.subcommand() {
        ("window", Some(args)) => run_window(args),
        ("rand", Some(args)) => run_rand(args),
        _ => (),
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

fn configure_app() -> App<'static, 'static> {
    App::new("boel")
        .version("0.1.0")
        .author("ucyo <cayoglu@me.com>")
        .about("Iterates over data via windows or chunks")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(configure_window())
        .subcommand(configure_rand())
}

fn configure_window() -> App<'static, 'static> {
    App::new("window")
        .about("Iterate over the data using a windows")
        .arg(
            Arg::with_name("FILE")
                .help("File to be read")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("nbytes")
                .help("Number of bytes to be read from file [defaults to whole file]")
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
            Arg::with_name("type")
                .help("f32 or f64")
                .short("t")
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

fn run_window(matches: &ArgMatches) {
    let filename = matches.value_of("FILE").unwrap();
    info!("{:?}", matches);

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

    // Read data using given type
    let dt = match matches.value_of("type").unwrap() {
        "f32" => Datatype::Float,
        _ => Datatype::Double,
    };

    // Setup configuration
    let config = Configuration::new(String::from(filename), nbytes, endian, dt);

    // Print result
    info!("{:?}", config);

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

fn configure_rand() -> App<'static, 'static> {
    App::new("rand")
        .version("0.1.0")
        .author("ucyo <cayoglu@me.com>")
        .about("Generate random arrays")
        .arg(
            Arg::with_name("FILE")
                .help("File to be written")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("distribution")
                .help("Distribuiton to be used")
                .short("d")
                .default_value("normal")
                .takes_value(true)
                .multiple(false)
                .possible_values(&["normal", "uniform"]),
        )
        .arg(
            Arg::with_name("mean")
                .help("Mean of distribution")
                .takes_value(true)
                .required_if("distribution", "normal")
                .default_value("10")
                .multiple(false)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("std")
                .help("Standard deviation of distribution")
                .takes_value(true)
                .required_if("distribution", "normal")
                .default_value("1")
                .multiple(false)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("shape")
                .help("Shape of array")
                .short("s")
                .long("shape")
                .takes_value(true)
                .min_values(2)
                .max_values(2)
                .required(true)
                .validator(is_usize_and_non_null),
        )
        .arg(
            Arg::with_name("minimum")
                .help("Minimum of value range")
                .required_if("distribution", "uniform")
                .takes_value(true)
                .default_value("0")
                .multiple(false)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("maximum")
                .help("Maximum of value range")
                .required_if("distribution", "uniform")
                .takes_value(true)
                .default_value("1")
                .multiple(false)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("type")
                .help("f32 or f64")
                .short("t")
                .default_value("f64")
                .takes_value(true)
                .possible_values(&["f64", "f32"])
                .max_values(1),
        )
        .arg(
            Arg::with_name("endian")
                .help("Endianess of file")
                .short("e")
                .long("endian")
                .default_value("native")
                .takes_value(true)
                .possible_values(&["native", "little", "big"])
                .multiple(false),
        )
}

fn run_rand(matches: &ArgMatches) {
    let filename = matches.value_of("FILE").unwrap();
    let size = matches
        .values_of("shape")
        .unwrap()
        .fold(1, |acc, x| x.parse::<usize>().unwrap() * acc);
    info!(
        "#Elements: {:?} ({:?})",
        size,
        matches.values_of("shape").unwrap().collect::<Vec<_>>()
    );
    let dtype = matches.value_of("type").unwrap();
    info!("Datatype: {:?}", dtype);
    let distr = matches.value_of("distribution").unwrap();
    info!("Distribution: {:?}", distr);
    let endian = match matches.value_of("endian").unwrap() {
        "big" => Endianess::Big,
        "little" => Endianess::Little,
        _ => Endianess::Native,
    };
    info!("Endianess: {:?}", endian);

    let mut f = File::create(filename).expect("Unable to create file");
    let mut rng = rand::thread_rng();
    match (dtype, distr) {
        ("f64", "uniform") => {
            let maximum = matches
                .value_of("maximum")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let minimum = matches
                .value_of("minimum")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            info!("Interval: [{:?};{:?})", minimum, maximum);
            let step = Uniform::new(minimum as f64, maximum as f64);
            let choices: Vec<_> = step.sample_iter(&mut rng).take(size).collect();
            debug!("Data: {:?}", choices);

            let mut buf = vec![0_u8; choices.len() * 8];
            match endian {
                Endianess::Native => NativeEndian::write_f64_into(&choices, &mut buf),
                Endianess::Little => LittleEndian::write_f64_into(&choices, &mut buf),
                Endianess::Big => BigEndian::write_f64_into(&choices, &mut buf),
            }
            f.write_all(&buf[..]).unwrap();
            f.sync_all().unwrap();
            info!("Written to {:?}", filename);
        }
        ("f32", "uniform") => {
            let maximum = matches
                .value_of("maximum")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let minimum = matches
                .value_of("minimum")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            info!("Interval: [{:?};{:?})", minimum, maximum);
            let step = Uniform::new(minimum as f32, maximum as f32);
            let choices: Vec<_> = step.sample_iter(&mut rng).take(size).collect();
            debug!("Data: {:?}", choices);

            let mut buf = vec![0_u8; choices.len() * 4];
            match endian {
                Endianess::Native => NativeEndian::write_f32_into(&choices, &mut buf),
                Endianess::Little => LittleEndian::write_f32_into(&choices, &mut buf),
                Endianess::Big => BigEndian::write_f32_into(&choices, &mut buf),
            }
            f.write_all(&buf[..]).unwrap();
            f.sync_all().unwrap();
            info!("Written to {:?}", filename);
        }
        ("f32", "normal") => {
            let mean = matches.value_of("mean").unwrap().parse::<usize>().unwrap();
            let std = matches.value_of("std").unwrap().parse::<usize>().unwrap();
            info!("Distribution {:?} +- {:?}", mean, std);
            let step = Normal::new(mean as f32, std as f32).unwrap();
            let choices: Vec<_> = step.sample_iter(&mut rng).take(size).collect();
            debug!("Data: {:?}", choices);

            let mut buf = vec![0_u8; choices.len() * 4];
            match endian {
                Endianess::Native => NativeEndian::write_f32_into(&choices, &mut buf),
                Endianess::Little => LittleEndian::write_f32_into(&choices, &mut buf),
                Endianess::Big => BigEndian::write_f32_into(&choices, &mut buf),
            }
            f.write_all(&buf[..]).unwrap();
            f.sync_all().unwrap();
            info!("Written to {:?}", filename);
        }
        ("f64", "normal") => {
            let mean = matches.value_of("mean").unwrap().parse::<usize>().unwrap();
            let std = matches.value_of("std").unwrap().parse::<usize>().unwrap();
            info!("Distribution {:?} +- {:?}", mean, std);
            let step = Normal::new(mean as f64, std as f64).unwrap();
            let choices: Vec<_> = step.sample_iter(&mut rng).take(size).collect();
            debug!("Data: {:?}", choices);

            let mut buf = vec![0_u8; choices.len() * 8];
            match endian {
                Endianess::Native => NativeEndian::write_f64_into(&choices, &mut buf),
                Endianess::Little => LittleEndian::write_f64_into(&choices, &mut buf),
                Endianess::Big => BigEndian::write_f64_into(&choices, &mut buf),
            }
            f.write_all(&buf[..]).unwrap();
            f.sync_all().unwrap();
            info!("Written to {:?}", filename);
        }
        _ => panic!("Not implemented yet"),
    };
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
