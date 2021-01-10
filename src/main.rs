use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String
}

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
        .data(tera)
        .route("/", web::get().to(index))
        .route("/signup", web::get().to(signup))
        .route("/signup", web::post().to(process_signup))
        .route("/login", web::get().to(login))
        .route("/login", web::post().to(process_login))
        .route("/submission", web::get().to(submission))
        .route("/submission", web::post().to(process_submission))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://github.com/shohiebsense"),
            author: String::from("Shohieb")
        },
        Post {
            title: String::from("The Second Link"),
            link: String::from("https://twitter.com/shohiebsense"),
            author: String::from("Nasruddin")
        },
    ];

    data.insert("title", "Shohiebsense");
    data.insert("name", "Shohieb Nasruddin");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_signup(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title","Login");
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged In: {}", data.username))
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");
    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}",data);
    HttpResponse::Ok().body(format!("Posted Submission: {}", data.title))
}