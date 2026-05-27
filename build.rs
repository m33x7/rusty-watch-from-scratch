use std::env;

fn main() {
    embuild::espidf::sysenv::output();

    match dotenvy::dotenv() {
        Ok(path) => println!("cargo:warning=.env loaded from {:?}", path),
        Err(e) => panic!("dotenv error: {:?}", e),
    }

    let wifi =  env::var("WIFI").unwrap();
    let wifi_pwd = env::var("WIFI_PWD").unwrap();

    println!("cargo:rustc-env=WIFI={}", wifi);
    println!("cargo:rustc-env=WIFI_PWD={}", wifi_pwd);
}
