mod downloader;
mod serialiser;
use std::time::Instant;
use azul::orlude::*;
use azul::widgets::{button::Button, label::Label};

struct DataModel {
    counter: usize, 
}

extern "C" 
fn render_dom(data: &mut RefAny, _: &mut LayoutInfo) -> StyledDom {

    let data = data.downcast_ref::<DataModel>()?;

    let label = Dom::text(format!("{}", data.counter))
        .with_inline_style("font-size: 50px;");
        
    let button = Button::new("Increment counter")
        .onmouseup(increment_counter, data.clone());

    Dom::body()
    .with_child(label)
    .with_child(button.dom())
    .style(Css::empty())
}

extern "C" 
fn increment_counter(data: &mut RefAny, _: &mut CallbackInfo) -> Update {
    let mut data = data.downcast_mut::<DataModel>()?;
    data.counter += 1;
    Update::RefreshDom // call render_dom() again
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    downloader::download_file(
        String::from("test.zip"),
        String::from("http://speedcheck.cdn.on.net/10meg.test"),
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
