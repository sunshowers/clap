use crate::utils;

use clap::{arg, App, AppSettings, Arg, ErrorKind};

#[test]
fn flag_subcommand_normal() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            ),
        )
        .try_get_matches_from(vec!["myprog", "some", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_normal_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .long_flag("S")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .alias("result"),
        )
        .try_get_matches_from(vec!["myprog", "result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            ),
        )
        .try_get_matches_from(vec!["myprog", "-S", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_args() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            ),
        )
        .try_get_matches_from(vec!["myprog", "-St"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_alias('M')
                .short_flag_alias('B'),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_alias_same_as_short_flag() {
    let matches = App::new("test")
        .subcommand(App::new("some").short_flag('S').short_flag_alias('S'))
        .try_get_matches_from(vec!["myprog", "-S"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_long_with_alias_same_as_long_flag() {
    let matches = App::new("test")
        .subcommand(App::new("some").long_flag("sync").long_flag_alias("sync"))
        .try_get_matches_from(vec!["myprog", "--sync"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases_vis_and_hidden() {
    let app = App::new("test").subcommand(
        App::new("some")
            .short_flag('S')
            .arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            )
            .visible_short_flag_aliases(&['M', 'B'])
            .short_flag_alias('C'),
    );
    let app1 = app.clone();
    let matches1 = app1.try_get_matches_from(vec!["test", "-M"]).unwrap();
    assert_eq!(matches1.subcommand_name().unwrap(), "some");

    let app2 = app.clone();
    let matches2 = app2.try_get_matches_from(vec!["test", "-C"]).unwrap();
    assert_eq!(matches2.subcommand_name().unwrap(), "some");

    let app3 = app.clone();
    let matches3 = app3.try_get_matches_from(vec!["test", "-B"]).unwrap();
    assert_eq!(matches3.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_aliases(&['M', 'B']),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_alias_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_alias('-'),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_aliases_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_aliases(&['-', '-', '-']),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
}

#[test]
fn flag_subcommand_short_after_long_arg() {
    let m = App::new("pacman")
        .subcommand(
            App::new("sync")
                .short_flag('S')
                .arg(Arg::new("clean").short('c')),
        )
        .arg(Arg::new("arg").long("arg").takes_value(true))
        .try_get_matches_from(vec!["pacman", "--arg", "foo", "-Sc"])
        .unwrap();
    let subm = m.subcommand_matches("sync");
    assert!(subm.is_some());
    let subm = subm.unwrap();
    assert!(subm.is_present("clean"));
}

#[test]
fn flag_subcommand_long() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            ),
        )
        .try_get_matches_from(vec!["myprog", "--some", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .long_flag_alias("result"),
        )
        .try_get_matches_from(vec!["myprog", "--result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long_with_aliases() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .long_flag_aliases(&["result", "someall"]),
        )
        .try_get_matches_from(vec!["myprog", "--result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_multiple() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .long_flag("some")
                .arg(arg!(-f --flag "some flag"))
                .arg(arg!(-p --print "print something"))
                .subcommand(
                    App::new("result")
                        .short_flag('R')
                        .long_flag("result")
                        .arg(arg!(-f --flag "some flag"))
                        .arg(arg!(-p --print "print something")),
                ),
        )
        .try_get_matches_from(vec!["myprog", "-SfpRfp"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("flag"));
    assert!(sub_matches.is_present("print"));
    assert_eq!(sub_matches.subcommand_name().unwrap(), "result");
    let result_matches = sub_matches.subcommand_matches("result").unwrap();
    assert!(result_matches.is_present("flag"));
    assert!(result_matches.is_present("print"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_short_conflict_with_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .subcommand(App::new("result").short_flag('t').short_flag_alias('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_long_conflict_with_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").long_flag("flag"))
        .subcommand(App::new("result").long_flag("test").long_flag_alias("flag"))
        .try_get_matches_from(vec!["myprog", "--flag"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('t').short_alias('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--some\' long flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").long("test").alias("some"))
        .try_get_matches_from(vec!["myprog", "--some"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag for the \'flag\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('a').long_flag("flag"))
        .arg(Arg::new("flag").long("flag"))
        .try_get_matches_from(vec!["myprog", "--flag"])
        .unwrap();
}

#[test]
fn flag_subcommand_conflict_with_help() {
    let _ = App::new("test")
        .subcommand(App::new("help").short_flag('h').long_flag("help"))
        .try_get_matches_from(vec!["myprog", "--help"])
        .unwrap();
}

#[test]
fn flag_subcommand_conflict_with_version() {
    let _ = App::new("test")
        .subcommand(App::new("ver").short_flag('V').long_flag("version"))
        .try_get_matches_from(vec!["myprog", "--version"])
        .unwrap();
}

#[test]
fn flag_subcommand_long_infer_pass() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .try_get_matches_from(vec!["prog", "--te"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[cfg(feature = "suggestions")]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn flag_subcommand_long_infer_pass_close() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--tes"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn flag_subcommand_long_infer_exact_match() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("testa").long_flag("testa"))
        .subcommand(App::new("testb").long_flag("testb"))
        .try_get_matches_from(vec!["prog", "--test"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

static FLAG_SUBCOMMAND_HELP: &str = "pacman-query 
Query the package database.

USAGE:
    pacman {query, --query, -Q} [OPTIONS]

OPTIONS:
    -h, --help                  Print help information
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings
";

#[test]
fn flag_subcommand_long_short_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman -Qh",
        FLAG_SUBCOMMAND_HELP,
        false
    ));
}

static FLAG_SUBCOMMAND_NO_SHORT_HELP: &str = "pacman-query 
Query the package database.

USAGE:
    pacman {query, --query} [OPTIONS]

OPTIONS:
    -h, --help                  Print help information
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings
";

#[test]
fn flag_subcommand_long_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_SHORT_HELP,
        false
    ));
}

static FLAG_SUBCOMMAND_NO_LONG_HELP: &str = "pacman-query 
Query the package database.

USAGE:
    pacman {query, -Q} [OPTIONS]

OPTIONS:
    -h, --help                  Print help information
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings
";

#[test]
fn flag_subcommand_short_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .short_flag('Q')
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_LONG_HELP,
        false
    ));
}
