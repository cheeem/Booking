use axum::response::Html;
use axum::http::StatusCode;
use askama::Template;

pub const NAV_TAB: crate::NavTab = crate::NavTab {
    path: "/venues/",
    display: "Venues",
};

#[derive(Template)]
#[template(path = "venues/venues.html")] 
struct VenuesTemplate<'a> { 
    path: &'a str,
}

pub async fn venues() -> Result<Html<String>, StatusCode> {
    
    let venues: VenuesTemplate<'_> = VenuesTemplate { 
        path: NAV_TAB.path, 
    };
    
    let html: String = venues.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    return Ok(Html(html))

}