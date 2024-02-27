use view::CreateView;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut view = CreateView {};
    view.run()?;
    Ok(())
}
