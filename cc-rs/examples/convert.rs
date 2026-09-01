use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: convert [fcl2zl|zl2fcl] <input> <output>");
        std::process::exit(2);
    }
    let (mode, input, output) = if args.len() >= 3 {
        (args[0].as_str(), args[1].as_str(), args[2].as_str())
    } else {
        ("fcl2zl", args[0].as_str(), args[1].as_str())
    };
    let source = match cc::jsonio::load_json_file(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            std::process::exit(1);
        }
    };
    let mut ctx = cc::context::ConversionContext::new();
    let result = match mode {
        "zl2fcl" => cc::zl_to_fcl::convert_zl_to_fcl(&mut ctx, &source, false),
        _ => cc::fcl_to_zl::convert_fcl_to_zl(&mut ctx, &source, false, false, 16.0 / 9.0, true, false),
    };
    if let Err(e) = cc::jsonio::write_json_file_opts(output, &result, false, mode != "zl2fcl") {
        eprintln!("failed to write output file: {}", e);
        std::process::exit(1);
    }
    if let Some(summary) = ctx.substitution_summary() {
        eprintln!("{}", summary);
    }
    let _ = std::io::stdout().flush();
}
