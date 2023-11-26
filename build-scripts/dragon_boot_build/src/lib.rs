#[macro_use]
extern crate lazy_static;
extern crate cc;

mod cfiles;
mod constant;
mod utils;

/// 运行构建
pub fn run() {
    println!("cargo:rustc-link-search=src");

    crate::cfiles::CFilesBuilder::build();
    // 设置链接参数
    crate::utils::ld_scripts::setup();
}
