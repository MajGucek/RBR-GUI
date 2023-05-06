use winres::WindowsResource;

fn main() {
    WindowsResource::new()
        .set("FileDescription", "RBR G29 LED RPM")
        .set("ProductVersion", "1.0.0")
        .set("OriginalFilename", "RBR2G29")
        .set("ProductName", "RBR2G29")        
        .compile()
        .expect("winres");
}
