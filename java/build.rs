use std::{env, path::Path, time::Instant};

use flapigen::{DotNetConfig, LanguageConfig};

fn main() {
    // env_logger::init();

    let now = Instant::now();

    let out_dir = env::var("OUT_DIR").unwrap();
    flapigen_expand(
        Path::new("src/cs_glue.in.rs"),
        &Path::new(&out_dir).join("glue.rs"),
    );
    let expand_time = now.elapsed();
    println!(
        "flapigen expand time: {}",
        expand_time.as_secs() as f64 + (expand_time.subsec_nanos() as f64) / 1_000_000_000.
    );
    println!("cargo:rerun-if-changed=cs_glue.in.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn flapigen_expand(from: &Path, out: &Path) {
    println!("Run flapigen_expand");
    let config = DotNetConfig::new("WarpSquareEngine".to_owned(), "WarpSquareEngine".into()).native_lib_name("warp_square_engine".to_owned());
    let swig_gen = flapigen::Generator::new(LanguageConfig::DotNetConfig(config));
    swig_gen.expand("warp_square_engine", from, out);
}
