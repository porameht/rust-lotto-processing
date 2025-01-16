use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct ListResponse {
    status: String,
    response: Vec<LottoEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LottoEntry {
    id: String,
    url: String,
    date: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LottoDetailResponse {
    status: String,
    response: LottoDetail,
}
#[derive(Debug, Deserialize, Serialize)]
struct LottoDetail {
    date: String,
    endpoint: String,
    prizes: Vec<Prize>
}

#[derive(Debug, Deserialize, Serialize)]
struct Prize {
    id: String,
    name: String,
    reward: String,
    amount: i32,
    number: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NumberEntry {
    number: String,
    date: String,
    prize: String,
    reward: String,
}

async fn fetch_lotto_list() -> Result<Vec<LottoEntry>, Box<dyn Error>> {
    const BASE_URL: &str = "https://lotto.api.rayriffy.com/list";
    let client: reqwest::Client = reqwest::Client::new();
    let mut lotto_entries: Vec<LottoEntry> = Vec::new();
    let mut page: i32 = 1;

    loop {
        let url: String = format!("{}/{}", BASE_URL, page);
        let response: ListResponse = client
            .get(&url)
            .send()
            .await?
            .json::<ListResponse>()
            .await?;

        if response.response.is_empty() {
            break;
        }

        println!("page: {}", page);
        lotto_entries.extend(response.response);

        // write to json file
        let json_string = serde_json::to_string_pretty(&lotto_entries)?;
        std::fs::write("lotto_entries.json", json_string)?;

        page += 1;
    }

    Ok(lotto_entries)
}
async fn fetch_lotto_detail() -> Result<(), Box<dyn Error>> {
    let read_from_file: bool = true;
    let mut lotto_entries: Vec<LottoEntry> = Vec::new();
    let mut lotto_details: Vec<LottoDetailResponse> = Vec::new();

    if read_from_file {
        let file_content = std::fs::read_to_string("lotto_entries.json")?;
        lotto_entries = serde_json::from_str(&file_content)?;

        println!("Processing {} entries", lotto_entries.len());
        for i in 0..lotto_entries.len() {
            println!("Processing entry {}: {}", i + 1, lotto_entries[i].id);

            let client: reqwest::Client = reqwest::Client::new();
            let url: String = format!("https://lotto.api.rayriffy.com/lotto/{}", lotto_entries[i].id);

            let response: LottoDetailResponse = client
                .get(&url)
                .send()
                .await?
                .json::<LottoDetailResponse>()
                .await?;

            lotto_details.push(response);
        }

        // Write raw details to JSON file
        let json_string = serde_json::to_string_pretty(&lotto_details)?;
        std::fs::write("lotto_details.json", json_string)?;
    }
        
    Ok(())
}


fn data_prep_search_number(lotto_details: Vec<LottoDetailResponse>) -> Result<(), Box<dyn Error>> {
    let mut number_entries: Vec<NumberEntry> = Vec::new();
    
    // Process each lottery draw
    for detail in lotto_details {
        let date_id = detail.response.endpoint
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();
            
        // Process each prize category
        for prize in detail.response.prizes {
            // Process each number in the prize category
            for number in prize.number {
                number_entries.push(NumberEntry {
                    number,
                    date: date_id.clone(),
                    prize: prize.id.clone(),
                    reward: prize.reward.clone(),
                });
            }
        }
    }

    // Sort by number to make searching easier
    number_entries.sort_by(|a, b| a.number.cmp(&b.number));

    // Write to new JSON file
    let json_string = serde_json::to_string_pretty(&number_entries)?;
    std::fs::write("lotto_numbers.json", json_string)?;

    Ok(())
}


mod worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Add this line to check for latest lottery data
    worker::check_and_fetch_latest().await?;

    let file_content = std::fs::read_to_string("lotto_details.json")?;
    let lotto_details: Vec<LottoDetailResponse> = serde_json::from_str(&file_content)?;

    data_prep_search_number(lotto_details)?;

    Ok(())
}
