use serde::{Deserialize, Serialize};
use std::error::Error;
use supabase_rs::{SupabaseClient, postgrest::PostgrestBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct LottoNumberRecord {
    pub number: String,
    pub date: String,
    pub prize: String,
    pub reward: String,
}

pub struct Database {
    client: SupabaseClient,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let supabase_url = std::env::var("SUPABASE_URL")
            .expect("SUPABASE_URL must be set");
        let supabase_key = std::env::var("SUPABASE_KEY")
            .expect("SUPABASE_KEY must be set");

        let client = SupabaseClient::new(&supabase_url, &supabase_key);

        Ok(Database { client })
    }

    pub async fn upsert_lotto_numbers(&self, numbers: Vec<LottoNumberRecord>) -> Result<(), Box<dyn Error>> {
        for number in numbers {
            self.client
                .from("lotto_numbers")
                .insert(serde_json::json!({
                    "number": number.number,
                    "date": number.date,
                    "prize": number.prize,
                    "reward": number.reward
                }))
                .on_conflict("number,date")
                .execute()
                .await?;
        }
        Ok(())
    }
}