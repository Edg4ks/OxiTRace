mod client; // Importa tu modulo client
use client::http::fetch_url; // Trae la funcion que acabamos de crear
#[tokio::main] // Active el runtime async de Rust
async fn main() {
    //URL de prueba (puede cambiarla luego)
    let url = "https://example.com";

    // llamamos al cliente HTTP
    match fetch_url(url).await {
        // en caso exitoso
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers recibidos correctamente");
        }
        // caso de error
        Err(e) => {
            println!("Error haciendo request: {}", e);
        }
    }
}
