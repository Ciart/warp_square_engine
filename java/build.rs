use std::{env, path::{Path, PathBuf}, time::Instant};

use flapigen::{DotNetConfig, JavaConfig, LanguageConfig};

fn main() {
    if env::var("CARGO_FEATURE_JAVA").is_ok() {
        build_java();
    } else if env::var("CARGO_FEATURE_DOTNET").is_ok() {
        build_dotnet();
    } else {
        panic!("No target specified");
    }
}

fn build_java() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let jni_c_headers_rs = Path::new(&out_dir).join("jni_c_header.rs");
    gen_jni_bindings(&jni_c_headers_rs);

    let java_cfg = JavaConfig::new(
        Path::new("lib")
            .join("src")
            .join("main")
            .join("java")
            .join("club")
            .join("gamza")
            .join("warpsquare")
            .join("engine"),
        "club.gamza.warpsquare.engine".into(),
    );

    let in_src = Path::new("src").join("java_glue.in.rs");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_src = Path::new(&out_dir).join("glue.rs");
    let flap_gen =
        flapigen::Generator::new(LanguageConfig::JavaConfig(java_cfg)).rustfmt_bindings(true);

    flap_gen.expand("java bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
}

fn build_dotnet() {
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

fn gen_jni_bindings(jni_c_headers_rs: &Path) {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME env variable not settted");
    let java_include_dir = Path::new(&java_home).join("include");
    let target = env::var("TARGET").expect("target env var not setted");

    let java_sys_include_dir = java_include_dir.join(if target.contains("windows") {
        "win32"
    } else if target.contains("darwin") {
        "darwin"
    } else {
        "linux"
    });

    let include_dirs = [java_include_dir, java_sys_include_dir];
    println!("jni include dirs {:?}", include_dirs);

    let jni_h_path =
        search_file_in_directory(&include_dirs[..], "jni.h").expect("Can not find jni.h");
    println!("cargo:rerun-if-changed={}", jni_h_path.display());

    gen_binding(&include_dirs[..], &jni_h_path, jni_c_headers_rs).expect("gen_binding failed");
}

fn search_file_in_directory<P: AsRef<Path>>(dirs: &[P], file: &str) -> Result<PathBuf, ()> {
    for dir in dirs {
        let dir = dir.as_ref().to_path_buf();
        let file_path = dir.join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P: AsRef<Path>>(
    include_dirs: &[P],
    c_file_path: &Path,
    output_rust: &Path,
) -> Result<(), String> {
    let mut bindings: bindgen::Builder = bindgen::builder().header(c_file_path.to_str().unwrap());

    bindings = include_dirs.iter().fold(bindings, |acc, x| {
        acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
    });

    let generated_bindings = bindings
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;

    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn flapigen_expand(from: &Path, out: &Path) {
    println!("Run flapigen_expand");
    let config = DotNetConfig::new("WarpSquareEngine".to_owned(), "WarpSquareEngine".into()).native_lib_name("warp_square_engine".to_owned());
    let swig_gen = flapigen::Generator::new(LanguageConfig::DotNetConfig(config));
    swig_gen.expand("warp_square_engine", from, out);
}
