use std::collections::HashMap;
use actix_web::{error, web, Error, HttpResponse, Result};
use vulcan_salute::util::*;
pub struct Index {}
impl Index {
    pub async fn index(
        tmpl: web::Data<tera::Tera>,
        query: web::Query<HashMap<String, String>>,
        // item: impl super::Controller,
    ) -> Result<HttpResponse, Error> {
        let s = if let Some(name) = query.get("username") {
            let mut ctx = tera::Context::new();
            ctx.insert("name", &name.to_owned());
            let welcome = "Welcome!".to_string();
            ctx.insert("text", &welcome);
            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        } else {
            let mut ctx = tera::Context::new();
            ctx.insert("os", &get_os_info());
            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        };
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}
