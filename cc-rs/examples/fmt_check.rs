use std::io::{BufRead, Write};

fn main() {
    let path = std::env::args().nth(1).expect("usage: fmt_check <go_out.txt>");
    let file = std::fs::File::open(&path).expect("open failed");
    let mut mismatches = 0usize;
    let mut total = 0usize;
    let mut out = std::io::BufWriter::new(std::io::stdout());
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 4 {
            continue;
        }
        let bits = u64::from_str_radix(parts[0], 16).unwrap();
        let v = f64::from_bits(bits);
        let is_nan = parts[1] == "NaN";
        let py_ok = cc::utils::py_float_format(v) == parts[2];
        let json_ok = is_nan || cc::utils::go_json_float_format(v) == parts[3];
        if !py_ok || !json_ok {
            mismatches += 1;
            if mismatches <= 20 {
                let _ = writeln!(
                    out,
                    "MISMATCH bits={} v={} nan={} go_py={} rust_py={} go_json={} rust_json={}",
                    parts[0],
                    if is_nan { "NaN".to_string() } else { format!("{}", v) },
                    is_nan,
                    parts[2],
                    cc::utils::py_float_format(v),
                    parts[3],
                    cc::utils::go_json_float_format(v),
                );
            }
        }
        total += 1;
    }
    let _ = writeln!(out, "total={} mismatches={}", total, mismatches);
    let _ = out.flush();
    if mismatches > 0 {
        std::process::exit(1);
    }
}
