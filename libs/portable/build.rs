fn main() {
    println!("cargo:rerun-if-env-changed=ODSREMOTE_EDITION");
    println!("cargo:rerun-if-env-changed=ODSREMOTE_VERSION");
    #[cfg(windows)]
    {
        use std::io::Write;
        let mut res = winres::WindowsResource::new();
        if let Ok(edition) = std::env::var("ODSREMOTE_EDITION") {
            let version = std::env::var("ODSREMOTE_VERSION")
                .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_owned());
            let filename = if edition == "qs" {
                format!("ODSremote-{version}-windows-x64-qs.exe")
            } else {
                format!("ODSremote-{version}-windows-x64-agent-install.exe")
            };
            res.set("CompanyName", "OneDot Systems")
                .set("ProductName", "ODSremote")
                .set("FileDescription", "ODSremote Remote Support")
                .set("OriginalFilename", &filename);
        }
        res.set_icon("../../res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("../../res/manifest.xml");
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
