use view::CreateView;
mod view;
mod widgets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut view = CreateView {};
    view.run()?;
    Ok(())
}
