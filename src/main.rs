use wall::xlib;

fn main() -> anyhow::Result<()> {
    let xlib = xlib::Xlib::new()?;
    xlib.set("pepe.jpeg", None)?;
    // xlib.set("screenshot.png", None)?;
    Ok(())
}
