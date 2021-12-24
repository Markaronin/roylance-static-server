use actix_files::Directory;
use std::fs;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, error, web::{Data, self}, dev::ServiceResponse, Responder,
};
use serde_json::json;
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
        .map(|dir_entry| dir_entry
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
        )
        .collect::<Vec<String>>();
    sub_files.sort_unstable();
    
    let ctx = json!({
        "path" : relative_path_string,
        "subfiles": sub_files
    });

    let s = tt.render("directory-page.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(
        ServiceResponse::new(
            req.clone(),
            HttpResponse::Ok().content_type("text/html").body(s)
        )
    )
}

async fn create_new_directory() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("directory-page.html", DIRECTORY_PAGE).unwrap();

        App::new()
            .data(tt)
            .service(
                web::scope("/admin")
                .route("/create-new-directory", web::put().to(create_new_directory)),
            )
            .service(
            actix_files::Files::new("/", "./files")
                    .show_files_listing()
                    .redirect_to_slash_directory()
                    .files_listing_renderer(directory_renderer),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

static DIRECTORY_PAGE: &str = include_str!("../pages/directory-page.html");
