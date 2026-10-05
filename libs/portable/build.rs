fn main() {
    println!("cargo:rerun-if-env-changed=ODSREMOTE_EDITION");
    #[cfg(windows)]
    {
        use std::io::Write;
        let mut res = winres::WindowsResource::new();
        if let Ok(edition) = std::env::var("ODSREMOTE_EDITION") {
            res.set("CompanyName", "OneDot Systems")
                .set("ProductName", "ODSremote")
                .set("FileDescription", "ODSremote Remote Support")
                .set("OriginalFilename", if edition == "qs" { "ODSremote-qs.exe" } else { "ODSremote-agent-install.exe" });
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
