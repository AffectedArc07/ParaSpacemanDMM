//! DreamChecker, a robust static analysis and typechecking engine for
//! DreamMaker.

extern crate dreammaker as dm;
extern crate dreamchecker;

// ----------------------------------------------------------------------------
// Command-line interface

fn main() {
    // command-line args
    let mut environment = None;
    let mut config_file = None;

    let mut args = std::env::args();
    let _ = args.next();  // skip executable name
    while let Some(arg) = args.next() {
        if arg == "-V" || arg == "--version" {
            println!(
                "dreamchecker {}  Copyright (C) 2017-2019  Tad Hardesty",
                env!("CARGO_PKG_VERSION")
            );
            println!("{}", include_str!(concat!(env!("OUT_DIR"), "/build-info.txt")));
            println!("This program comes with ABSOLUTELY NO WARRANTY. This is free software,");
            println!("and you are welcome to redistribute it under the conditions of the GNU");
            println!("General Public License version 3.");
            return;
        } else if arg == "-e" {
            environment = Some(args.next().expect("must specify a value for -e"));
        } else if arg == "-c" {
            config_file = Some(args.next().expect("must specify a file for -c"));
        } else {
            eprintln!("unknown argument: {}", arg);
            return;
        }
    }

    let dme = environment
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| dm::detect_environment_default()
            .expect("error detecting .dme")
            .expect("no .dme found"));

    let mut context = dm::Context::default();
    if let Some(filepath) = config_file {
        context.force_config(filepath.as_ref());
    } else {
        context.autodetect_config(&dme);
    }
    context.set_print_severity(Some(dm::Severity::Info));

    println!("============================================================");
    println!("Parsing {}...\n", dme.display());
    let pp = dm::preprocessor::Preprocessor::new(&context, dme)
        .expect("i/o error opening .dme");
    let indents = dm::indents::IndentProcessor::new(&context, pp);
    let mut parser = dm::parser::Parser::new(&context, indents);
    parser.enable_procs();
    let tree = parser.parse_object_tree();

    dreamchecker::run_cli(&context, &tree);

    println!("============================================================");
    let errors = context.errors().iter().filter(|each| each.severity() <= dm::Severity::Info).count();
    println!("Found {} diagnostics", errors);
    std::process::exit(if errors > 0 { 1 } else { 0 });
}
