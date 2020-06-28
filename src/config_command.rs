use clap::{App, Arg, ArgMatches};

pub fn get_command() -> ArgMatches<'static> {
    App::new("Quota manage system parameter configmation")
        .version("0.1.0")
        .author("luo4lu <luo4lu@163.com>")
        .about("Go to the server and request the address")
        .arg(
            Arg::with_name("qms")
                .short("q")
                .long("qms")
                .help("set self Quota manage system IP addr and port")
                .takes_value(true),
        )
        .get_matches()
}
