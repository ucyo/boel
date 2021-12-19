use clap::{App, AppSettings, Arg};

fn main() {
    let matches = configure().get_matches();
    println!("{:?}", matches)
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
