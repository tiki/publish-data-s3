/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub userid: String,
    pub ipaddress: Option<String>,
    pub maid: Option<String>,
    #[serde(deserialize_with = "or_none")]
    pub estimated_age: Option<u32>,
    pub age_range: Option<String>,
    pub age_generation: Option<String>,
    pub race_ethnicity: Option<String>,
    pub income_range: Option<String>,
    pub income_bucket: Option<String>,
    pub gender: Option<String>,
    pub user_zip: Option<String>,
    pub user_state: Option<String>,
    #[serde(deserialize_with = "or_none")]
    pub presence_of_children: Option<bool>,
    #[serde(deserialize_with = "or_none")]
    pub quant_of_children: Option<u32>,
    pub education: Option<String>,
    pub living_situation: Option<String>,
    #[serde(deserialize_with = "or_none")]
    pub hh_size: Option<u32>,
    pub marital_status: Option<String>,
    pub employment_status: Option<String>,
    pub sexual_orientation: Option<String>,
}

fn or_none<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: serde::Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    Ok(Deserialize::deserialize(de).ok())
}
