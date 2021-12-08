// build.rs

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("test.ico")
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, 1);
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}