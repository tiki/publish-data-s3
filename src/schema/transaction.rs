/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub userid: String,
    pub transactionid: Option<String>,
    pub authorized_date: Option<String>,
    pub transaction_date: Option<String>,
    pub clean_merchant_name: Option<String>,
    pub merchant_name: Option<String>,
    pub transaction_name: Option<String>,
    pub amount: Option<f32>,
    pub payment_channel: Option<String>,
    pub category_level1: Option<String>,
    pub category_level2: Option<String>,
    pub category_level3: Option<String>,
    pub num_levels: Option<u32>,
    pub primary_personal_finance_category: Option<String>,
    pub detailed_personal_finance_category: Option<String>,
    pub merchant_address: Option<String>,
    pub merchant_city: Option<String>,
    pub merchant_state: Option<String>,
    pub merchant_zip: Option<String>,
}
