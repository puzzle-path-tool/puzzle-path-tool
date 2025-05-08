fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .compile()
            .expect("Failed to compile Windows resources");
    }

    puzzle_core_build::rerun_if_changed("assets/**/*");
}
