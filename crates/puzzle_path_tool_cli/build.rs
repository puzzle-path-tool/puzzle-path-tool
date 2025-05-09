#![allow(clippy::unnecessary_wraps)]

use puzzle_core_build as cb;

fn main() {
    cb::handle_build_result(compile_resources());
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
