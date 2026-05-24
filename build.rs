fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        // Set icon when we have one
        // res.set_icon("assets/icon.ico");
        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile windows resources: {}", e);
        }
    }
}
