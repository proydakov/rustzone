use axum::{response::Html, routing::get, Router};

async fn hello() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<body>

<h1>Evgeny Proydakov Personal Page</h1>

<p>C++ engineer and technical leader from Rußland. Пишу на крестах ††.</p>
<p>Part time speaker and writer. Life is to short for a malloc.</p>
<span>Web Browser Mini Game: </span><a href="https://proydakov.github.io/zeptobird/">zeptobird</a><span>. You are welcome.</span>
<p></p>

<iframe src="https://vk.com/video_ext.php?oid=-228047553&id=456239030&hd=2" width="853" height="480" allow="autoplay; encrypted-media; fullscreen; picture-in-picture;" frameborder="0" allowfullscreen></iframe>

</body>
</html>"#)
}

#[tokio::main] // This macro starts the Tokio runtime
async fn main() {
    // 1. Define our app routes
    let app = Router::new().route("/", get(hello));

    // 2. Define the address to listen on
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    
    println!("Listening on http://{}", listener.local_addr().unwrap());

    // 3. Start the server
    ax_serve(listener, app).await.unwrap();
}

// Helper to serve the app (Axum 0.7 style)
async fn ax_serve(listener: tokio::net::TcpListener, app: Router) -> std::io::Result<()> {
    axum::serve(listener, app).await
}

