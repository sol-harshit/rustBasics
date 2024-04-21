// the current program takes the input from an HTML page form for number n and m 
// and returns the greatest common divisor of the two numbers
// the program uses the actix-web crate to create a web server

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("serving on http://localhost:3000...");

    server.bind("127.0.0.1:3000").expect("error binding server to the address")
        .run().await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <html>
                <head>
                    <title> GCD Calculator </title>
                </head>
                <body>
                    <form action="/gcd" method="post">
                        <input type="text" name="n" />
                        <input type="text" name="m" />
                        <button type="submit">Compute GCD</button>
                    </form>
                </body>
            </html>
            "#,
        )
}

async fn post_gcd(form: web::Form<FormData>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("cannot compute gcd for zero");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

// definition of the gcd function can be found in the gcd/src/main.rs file
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m!=0 && m!=0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m%n;
    }
    n
}

// writing tests for the gcd function
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

