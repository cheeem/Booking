use axum::response::Html;
use axum::http::StatusCode;
use askama::Template;

pub const NAV_TAB: crate::NavTab = crate::NavTab {
    path: "/",
    display: "Home",
};

#[derive(Template)]
#[template(path = "home/home.html")] 
struct HomeTemplate<'a> { 
    path: &'a str,
}

pub async fn home() -> Result<Html<String>, StatusCode> {
    
    let home: HomeTemplate<'_> = HomeTemplate { 
        path: NAV_TAB.path, 
    };
    
    let html: String = home.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    return Ok(Html(html))

}