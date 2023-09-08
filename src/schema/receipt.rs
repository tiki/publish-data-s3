/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub userid: String,
    #[serde(rename = "receiptid")]
    pub receiptid: Option<String>,
    pub receipt_date: Option<String>,
    pub merchant_name: Option<String>,
    pub merchant_address: Option<String>,
    pub merchant_city: Option<String>,
    pub merchant_state: Option<String>,
    pub merchant_zip: Option<String>,
    pub channel: Option<String>,
    pub amount: Option<f32>,
    pub brand: Option<String>,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub quantity: Option<u32>,
    pub unit_price: Option<f32>,
    pub total_price: Option<f32>,
    pub category_level1: Option<String>,
    pub category_level2: Option<String>,
    pub category_level3: Option<String>,
    pub size: Option<String>,
    pub upc: Option<String>,
}
