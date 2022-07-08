use sudoku_backend::*;
use tide::{Request, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();

    app.at("/").get(say_hello);
    app.at("/:name").get(say_hello);
    app.at("/ac3/:board").get(ac3);
    app.at("/backtrack/:board").get(backtracking);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn say_hello(req: Request<()>) -> Result<String> {
    let name = req.param("name").unwrap_or("you");

    Ok(format!(
        "Don't even dare to say hello! Hi {name}! Go to /backtrack/:board or /ac3/:board for more information",
    ))
}
