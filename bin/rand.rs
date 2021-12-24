use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian};
use clap::{App, Arg};
use log::{debug, info};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;
use std::fs::File;
use std::io::Write;

fn main() {
    env_logger::init();
    let matches = configure().get_matches();
    debug!("CLI Input: {:?}", matches);
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
                .long("minimum")
                .required_if("distribution", "uniform")
                .takes_value(true)
                .default_value("0")
                .multiple(false)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("maximum")
                .help("Maximum of value range")
                .long("maximum")
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

#[derive(Debug)]
pub enum Endianess {
    Big,
    Little,
    Native,
}
