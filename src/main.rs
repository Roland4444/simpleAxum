use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Создаем приложение с маршрутами
    let app = Router::new()
        .route("/test", get(hello_handler))
        .route("/", get(index_handler))
        // Раздаем статические файлы из папки "static"
        .nest_service("/static", ServeDir::new("static"))
        .fallback(fallback_handler);

    let PORT = 3000;    

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    println!(          "*********************************************************");
    println!(          "*********************************************************");
    println!(          "***  ***************************************    ****  ***");
    println!(          "***  *******                           *****  ** ***  ***");
    println!(          "***  *******STARTUP SERVER AT PORT {}*****  *** **  ***", PORT);
    println!(          "***  *******                           *****  **** *  ***");
    println!(          "***  ***************************************  *****   ***");
    println!(          "***       **********************************  ******  ***");
    println!(          "*********************************************************");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Обработчик для /test
async fn hello_handler() -> &'static str {
    "hello world"
}

// Обработчик для / (index.html)
async fn index_handler() -> impl IntoResponse {
    match std::fs::read_to_string("static/index.html") {
        Ok(content) => Html(content),
        Err(_) => {
            // Если файл не найден, возвращаем простую HTML-страницу
            Html(
                r#"
              !!!ERROR!!!
                "#
                .to_string(),
            )
        }
    }
}

// Обработчик для неизвестных маршрутов
async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Страница не найдена")
}


// use axum::{
//     routing::get,
//     Router,
//     response::{Html, IntoResponse},
//     http::StatusCode,
// };
// use std::net::SocketAddr;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let app = Router::new()
//         .route("/test", get(hello_handler))
//         .route("/", get(index_handler))
//         .nest_service("/static", ServeDir::new("static"))
//         .fallback(fallback_handler);

//     let PORT = 3000;    

//     let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
//     println!(          "*********************************************************");
//     println!(          "*********************************************************");
//     println!(          "***  ***************************************    ****  ***");
//     println!(          "***  *******                           *****  ** ***  ***");
//     println!(          "***  *******STARTUP SERVER AT PORT {}*****  *** **  ***", PORT);
//     println!(          "***  *******                           *****  **** *  ***");
//     println!(          "***  ***************************************  *****   ***");
//     println!(          "***       **********************************  ******  ***");
//     println!(          "*********************************************************");



//     let listener = TcpListener::bind(addr).await?;
//     axum::serve(listener, app).await?;

//     Ok(())
// }

// async fn hello_handler() -> &'static str {
//     "hello world"
// }

// async fn index_handler() -> impl IntoResponse {
//     match std::fs::read_to_string("static/index.html") {
//         Ok(content) => Html(content),
//         Err(_) => {
//             // Если файл не найден, возвращаем простую HTML-страницу
//             Html(
//                 r#"
// !!!ERROR!!!
//                 "#
//                 .to_string(),
//             )
//         }
//     }
// }

// async fn fallback_handler() -> impl IntoResponse {
//     (StatusCode::NOT_FOUND, "Страница не найдена")
// }