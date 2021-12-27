use actix_files::Directory;
use actix_web::post;
use actix_web::{
    dev::ServiceResponse, error, web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde_json::json;
use std::fs::{self};
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
        .app_data::<Data<TinyTemplate<'_>>>()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("directory-page.html", DIRECTORY_PAGE)
            .unwrap();

        App::new().data(tt).service(create_new_directory).service(
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

static DIRECTORY_PAGE: &str = include_str!("../pages/directory-page.html");
