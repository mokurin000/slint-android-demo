slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    AppWindow::new()?.run()?;
    Ok(())
}
