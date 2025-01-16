// use serde::{Deserialize, Serialize};
// // use crate::db::LottoNumberRecord;
// use std::error::Error;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct LottoDetail {
//     pub date: String,
//     pub endpoint: String,
//     pub prizes: Vec<Prize>
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Prize {
//     pub id: String,
//     pub name: String,
//     pub reward: String,
//     pub amount: i32,
//     pub number: Vec<String>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct LottoDetailResponse {
//     pub status: String,
//     pub response: LottoDetail,
// }

// pub fn data_prep_search_number(lotto_details: Vec<LottoDetailResponse>) -> Result<(), Box<dyn Error>> {
//     let mut number_entries: Vec<LottoNumberRecord> = Vec::new();
    
//     for detail in lotto_details {
//         let date_id = detail.response.endpoint
//             .split('/')
//             .last()
//             .unwrap_or("")
//             .to_string();
            
//         for prize in detail.response.prizes {
//             for number in prize.number {
//                 number_entries.push(LottoNumberRecord {
//                     number,
//                     date: date_id.clone(),
//                     prize: prize.id.clone(),
//                     reward: prize.reward.clone(),
//                 });
//             }
//         }
//     }

//     number_entries.sort_by(|a, b| a.number.cmp(&b.number));

//     let db = crate::db::Database::new()?;
//     tokio::spawn(async move {
//         if let Err(e) = db.upsert_lotto_numbers(number_entries).await {
//             eprintln!("Error saving to database: {}", e);
//         }
//     });

//     Ok(())
// } 