use tide::{Request, Result};

pub async fn ac3(req: Request<()>) -> Result<String> {
    let board = req.param("board").unwrap_or("");

    Ok(board.to_string())
}

pub async fn backtracking(req: Request<()>) -> Result<String> {
    let board = req.param("board").unwrap_or("");

    Ok(board.to_string())
}
