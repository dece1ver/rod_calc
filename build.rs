fn main() {
    if cfg!(target_os = "windows") {
        let res = winres::WindowsResource::new();
        res.compile().unwrap();
    }

    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
