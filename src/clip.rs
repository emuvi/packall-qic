use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new("Packall")
        .version(clap::crate_version!())
        .about("Packall is a command program that eats all the files you feed and keeps them organized, first in the belly, then in the body (indexed), for future searches.")
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .value_name("DIR")
                .takes_value(true)
                .required(true)
                .help("Where all books I eat ends up")
        )
        .arg(
            Arg::with_name("speed")
                .short("e")
                .long("speed")
                .value_name("NUMBER")
                .default_value("4")
                .takes_value(true)
                .required(false)
                .help("How fast should I go")
        )
        .arg(
            Arg::with_name("clean")
                .short("c")
                .long("clean")
                .takes_value(false)
                .required(false)
                .help("Removes from the origin and/or the already digested.")
        )
        .arg(
            Arg::with_name("feed")
                .short("f")
                .long("feed")
                .value_name("PATH")
                .takes_value(true)
                .required(false)
                .help("Yami! More files for me to eat")
        )
        .arg(
            Arg::with_name("digest")
                .short("d")
                .long("digest")
                .takes_value(false)
                .required(false)
                .help("Digests the food in my belly")
        )
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("WORDS")
                .takes_value(true)
                .required(false)
                .help("Searches in the digested words")
        )
        .arg(
            Arg::with_name("lend")
                .short("l")
                .long("lend")
                .value_name("PATH")
                .takes_value(true)
                .required(false)
                .help("Copies the founds inside me")
        )
        .arg(
            Arg::with_name("give")
                .short("g")
                .long("give")
                .value_name("PATH")
                .takes_value(true)
                .required(false)
                .help("Moves the founds inside me")
        )
        .arg(
            Arg::with_name("junk")
                .short("j")
                .long("junk")
                .takes_value(false)
                .required(false)
                .help("Discards the founds inside me")
        )
        .arg(
            Arg::with_name("open")
                .short("o")
                .long("open")
                .takes_value(false)
                .required(false)
                .help("Opens the founds inside me")
        )
        .get_matches()
}
