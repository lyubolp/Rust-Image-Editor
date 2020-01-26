extern crate cmake;
use cmake::Config;

fn main()
{
    //let dst = Config::new("core_cpp").build();
    let dst = cmake::build("core_cpp");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=core_cpp");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
