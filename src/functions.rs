use leptos::*;
#[server(MyServerFunction)]
pub async fn my_server_fn(x: u8, y: u8) -> Result<u8, ServerFnError> {
    Ok(x.overflowing_add(y).0)
}
