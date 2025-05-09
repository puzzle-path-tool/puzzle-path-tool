use puzzle_core_build as cb;
use puzzle_core_build::BuildResult;

fn main() {
    compile_resources().unwrap_build_result();
}

fn compile_resources() -> anyhow::Result<()> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico").compile()?;
    }

    cb::rerun_if_changed("assets/**/*");
    Ok(())
}
