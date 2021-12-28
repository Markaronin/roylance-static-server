use actix_files::Directory;
use actix_web::dev::ServiceResponse;
use actix_web::{delete, error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::{get, post, put};
use futures::StreamExt;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;
use tinytemplate::TinyTemplate;

fn directory_renderer(
    dir: &Directory,
    req: &HttpRequest,
) -> Result<ServiceResponse, std::io::Error> {
    let relative_path_string = dir
        .path
        .strip_prefix(dir.base.clone())
        .unwrap()
        .to_str()
        .unwrap();
    let tt = req
        .app_data::<web::Data<TinyTemplate<'_>>>()
        .map(|t| t.get_ref())
        .unwrap();

    let mut sub_files = fs::read_dir(dir.path.clone())
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    sub_files.sort_unstable();

    let ctx = json!({
        "path" : relative_path_string,
        "subfiles": sub_files
    });

    let s = tt
        .render("directory-page.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok().content_type("text/html").body(s),
    ))
}

#[post("/{tail:.*}")]
async fn create_new_directory(path: actix_web::web::Path<String>) -> impl Responder {
    if path.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    let new_dir_path = Path::new(&std::env::current_dir().unwrap())
        .join("files")
        .join(path.as_str());
    let create_dir_result = fs::create_dir(new_dir_path);
    if create_dir_result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[put("/{tail:.*}")]
async fn create_new_file(
    mut payload: web::Payload,
    path: actix_web::web::Path<String>,
) -> Result<HttpResponse, Error> {
    let new_file_path = Path::new(&std::env::current_dir().unwrap())
        .join("files")
        .join(path.as_str());

    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(new_file_path)).await?;

    // payload is a stream of Bytes objects
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
    }
    Ok(HttpResponse::Ok().into())
}

#[delete("/{tail:.*}")]
async fn delete_file(path: actix_web::web::Path<String>) -> impl Responder {
    let deleted_file_path = Path::new(&std::env::current_dir().unwrap())
        .join("files")
        .join(path.as_str());

    if !deleted_file_path.exists() {
        return HttpResponse::NotFound();
    }

    if fs::metadata(&deleted_file_path).unwrap().is_dir() {
        fs::remove_dir(deleted_file_path).unwrap();
    } else {
        fs::remove_file(deleted_file_path).unwrap();
    }

    HttpResponse::Ok().into()
}

#[get("/directory-page.js")]
async fn directory_page_js() -> impl Responder {
    HttpResponse::Ok().body(DIRECTORY_PAGE_JS)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("directory-page.html", DIRECTORY_PAGE_HTML)
            .unwrap();

        App::new()
            .data(tt)
            .service(directory_page_js)
            .service(create_new_directory)
            .service(create_new_file)
            .service(delete_file)
            .service(
                actix_files::Files::new("/", "./files")
                    .show_files_listing()
                    .redirect_to_slash_directory()
                    .files_listing_renderer(directory_renderer),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

static DIRECTORY_PAGE_HTML: &str = include_str!("../pages/directory-page.html");
static DIRECTORY_PAGE_JS: &str = include_str!("../pages/directory-page.js");
