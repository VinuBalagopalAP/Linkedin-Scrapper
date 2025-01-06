// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub summary: String,
    pub experience: Vec<Experience>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: i32,
    pub title: String,
    pub company_name: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub work_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub rating: i32,
}
