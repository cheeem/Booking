use axum::http::StatusCode;
use axum::extract::Path;
use axum::response::Html;
use askama::Template;
use chrono::Datelike;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Month {
    year: i32,
    month: u32,
}

#[derive(Template)]
#[template(path = "calendar/month.html")] 
struct MonthTemplate<'a> {
    year: i32, 
    month: &'a str,
    first_day_of_week: u32,
    last_day: u32,
    previous_month: u32,
    previous_month_year: i32,
    next_month: u32,
    next_month_year: i32,
}

impl<'a> MonthTemplate<'a> {

    fn new(year: i32, month: u32) -> Self {

        assert!(month <= 12);

        let first_day_of_week: u32 = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .weekday()
            .number_from_sunday();

        let last_day: u32 = chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or(chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
            .day();

        let (previous_month, previous_month_year) = if month == 1 { (12, year - 1) } else { (month - 1, year) };
        let (next_month, next_month_year) = if month == 12 { (1, year + 1) } else { (month + 1, year) };

        let month: &str = chrono::Month::try_from(month as u8).unwrap().name();

        Self { 
            year,
            month,
            first_day_of_week,
            last_day,
            previous_month,
            previous_month_year,
            next_month,
            next_month_year,
        }

    }

    fn to_calendar_template(self) -> CalendarTemplate<'a> {
        
        CalendarTemplate {
            path: "/calendar/",
            year: self.year,
            month: self.month,
            first_day_of_week: self.first_day_of_week,
            last_day: self.last_day,
            previous_month: self.previous_month,
            previous_month_year: self.previous_month_year,
            next_month: self.next_month,
            next_month_year: self.next_month_year,
        }

    }
    
}

pub async fn month(Path(Month { year, month }): Path<Month>) -> Result<Html<String>, StatusCode> {
    
    let month: MonthTemplate = MonthTemplate::new(year, month);

    let html: String = month.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    return Ok(Html(html))

}

#[derive(Template)]
#[template(path = "calendar/calendar.html")] 
struct CalendarTemplate<'a> { 
    path: &'static str,
    year: i32,
    month: &'a str,
    first_day_of_week: u32,
    last_day: u32,
    previous_month: u32,
    previous_month_year: i32,
    next_month: u32,
    next_month_year: i32,
}

pub async fn calendar(Path(Month { year, month }): Path<Month>) -> Result<Html<String>, StatusCode> {

    let calendar: CalendarTemplate = MonthTemplate::new(year, month).to_calendar_template();
    
    let html: String = calendar.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    return Ok(Html(html))

}