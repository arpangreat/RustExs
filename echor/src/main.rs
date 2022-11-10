use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .author("Swastik Acharyya")
        .version("1.0.0")
        .about("rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print new line")
                .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }

    print!("{}{}", text.join(" "), ending);

    println!("{:#?}", matches);
}
