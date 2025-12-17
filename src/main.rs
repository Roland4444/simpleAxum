use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse},
    http::StatusCode,
    Form
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Создаем приложение с маршрутами
    let app = Router::new()
        .route("/test", get(hello_handler))
        .route("/", get(index_handler))
        // Раздаем статические файлы из папки "static"
        .nest_service("/static", ServeDir::new("static"))
        .route("/api/language/:lang", get(change_language_handler))
        .route("/submit", post(submit_survey_handler))

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
    println!(" Опрос доступен на русском и фарси");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}


 async fn change_language_handler() -> impl IntoResponse {
     (
         axum::response::AppendHeaders([("content-type", "application/json")]),
         r#"{"status": "success", "message": "Язык изменен"}"#,
     )
 }


// Структура для данных опроса
#[derive(Debug, Deserialize)]
struct SurveyData {
    name: Option<String>,
    linux_experience: String,
    attitude: String,
   docs_flag: Option<String>,
    email_flag: Option<String>,
   c1flag: Option<String>,
    special_flag: Option<String>,
    special_software: Option<String>,
    will_test: Option<String>,
    contact_preference: Option<String>,
    comments: Option<String>,
    language: String,
}


// Обработчик для отправки опроса
async fn submit_survey_handler(Form(data): Form<SurveyData>) -> impl IntoResponse {
   // Сохраняем данные в CSV файл
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    let record = format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        timestamp,
        data.name.unwrap_or_default(),
        data.linux_experience,
        data.attitude,
        data.docs_flag.unwrap_or_default(),
        data.email_flag.unwrap_or_default(),
        data.c1flag.unwrap_or_default(),
        data.special_flag.unwrap_or_default(),
        data.special_software.unwrap_or_default(),
        data.will_test.unwrap_or_default(),
        data.contact_preference.unwrap_or_default(),
        data.comments.unwrap_or_default(),
        data.language
    );
    
    // Сохраняем в файл
    let file_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open("survey_results.csv");
    
    match file_result {
        Ok(mut file) => {
            // Записываем заголовок если файл пустой
            if file.metadata().map(|m| m.len() == 0).unwrap_or(true) {
                let header = "timestamp,name,linux_experience,attitude,docs_flag,email_flag,c1flag,special_flag,special_software,will_test,contact_preference,comments,language\n";
                let _ = file.write_all(header.as_bytes());
            }
            
            match file.write_all(record.as_bytes()) {
                Ok(_) => {
                    let message = match data.language.as_str() {
                        "fa" => "نظر شما با موفقیت ثبت شد. سپاسگزاریم!",
                        _ => "Ваш ответ успешно отправлен. Спасибо!"
                    };
                    (StatusCode::OK, message)
                },
                Err(e) => {
                    eprintln!("Ошибка записи в файл: {}", e);
                    let message = match data.language.as_str() {
                        "fa" => "خطا در ثبت اطلاعات",
                        _ => "Ошибка при сохранении данных"
                   };
                    (StatusCode::INTERNAL_SERVER_ERROR, message)
                }
           }
        },
       Err(e) => {
            eprintln!("Ошибка открытия файла: {}", e);
            let message = match data.language.as_str() {
                "fa" => "خطا در سیستم ذخیره‌سازی",
                _ => "Ошибка системы сохранения"
           };
            (StatusCode::INTERNAL_SERVER_ERROR, message)
       }
    }
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