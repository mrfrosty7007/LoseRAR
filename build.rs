fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        res.set_icon("assets/icon.ico");

        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile windows resources: {}", e);
        }
    }
}