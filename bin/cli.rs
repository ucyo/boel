use boel::config::{Configuration, Endianess, Nbytes};
use clap::{App, Arg};

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
    println!("C {:?}", config);
    let v: Vec<f64> = Vec::from(config);
    println!("V {:?}", v);
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
}
