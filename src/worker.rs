use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::{LottoDetail, LottoDetailResponse, data_prep_search_number};


#[derive(Debug, Deserialize, Serialize)]
struct LatestResponse {
    status: String,
    response: LottoDetail,
}

pub async fn check_and_fetch_latest() -> Result<(), Box<dyn Error>> {
    let today = Local::now();
    let year_be = today.year() + 543; // Convert CE to BE
    let month = today.month();
    let day = today.day();

    // Determine which date to check based on current day
    let target_day = if day < 16 {
        // First half of month - check day 1 or 2
        if is_valid_date(year_be, month, 1) {
            "01"
        } else {
            "02"
        }
    } else {
        // Second half of month - check day 16 or 17
        if is_valid_date(year_be, month, 16) {
            "16"
        } else {
            "17"
        }
    };

    // Format the date string for API call
    let date_str = format!(
        "{:02}{:02}{}", 
        target_day,
        month,
        year_be
    );

    // Fetch latest lottery data
    let client = reqwest::Client::new();
    let url = "https://lotto.api.rayriffy.com/latest";
    
    let response = client
        .get(url)
        .send()
        .await?
        .json::<LatestResponse>()
        .await?;

    // Convert LatestResponse to LottoDetailResponse
    let detail_response = LottoDetailResponse {
        status: "success".to_string(),
        response: response.response,
    };

    // Process directly without saving to files
    data_prep_search_number(vec![detail_response])?;

    Ok(())
}

fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    NaiveDate::from_ymd_opt(year, month, day).is_some()
} 