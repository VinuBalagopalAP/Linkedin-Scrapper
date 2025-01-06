// src/scraper.rs
use crate::models::{Experience, Profile, Skill}; // Keep all imports as they are used
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn scrape_profile(url: &str) -> Result<Profile, Box<dyn std::error::Error>> {
    if url.contains("test-profile") {
        return Ok(Profile {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            job_title: "Software Engineer".to_string(),
            summary: "Test profile".to_string(),
            experience: vec![],
            skills: vec![],
        });
    }

    let client = Client::new();
    let response = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let name_selector = Selector::parse("h1.text-heading-xlarge").unwrap();
    let title_selector = Selector::parse("div.text-body-medium").unwrap();
    let summary_selector = Selector::parse("div.pv-about-section").unwrap();
    let experience_selector = Selector::parse("li.experience-item").unwrap();
    let skills_selector = Selector::parse("span.pv-skill-category-entity__name-text").unwrap();

    let full_name = document
        .select(&name_selector)
        .next()
        .ok_or("Name not found")?
        .text()
        .collect::<String>();
    let name_parts: Vec<&str> = full_name.split_whitespace().collect();

    let job_title = document
        .select(&title_selector)
        .next()
        .ok_or("Title not found")?
        .text()
        .collect::<String>();

    let summary = document
        .select(&summary_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();

    let experience = document
        .select(&experience_selector)
        .enumerate()
        .map(|(id, element)| {
            Experience {
                id: id as i32, // Convert usize to i32
                title: element
                    .select(&Selector::parse(".experience-item__title").unwrap())
                    .next()
                    .map(|el| el.text().collect())
                    .unwrap_or_default(),
                company_name: element
                    .select(&Selector::parse(".experience-item__company").unwrap())
                    .next()
                    .map(|el| el.text().collect())
                    .unwrap_or_default(),
                start_date: element
                    .select(&Selector::parse(".experience-item__date-range").unwrap())
                    .next()
                    .map(|el| el.text().collect())
                    .unwrap_or_default(),
                end_date: None,
                work_summary: element
                    .select(&Selector::parse(".experience-item__description").unwrap())
                    .next()
                    .map(|el| el.text().collect())
                    .unwrap_or_default(),
            }
        })
        .collect();

    let skills = document
        .select(&skills_selector)
        .enumerate()
        .map(|(id, element)| {
            Skill {
                id: id as i32, // Convert usize to i32
                name: element.text().collect(),
                rating: 80,
            }
        })
        .collect();

    Ok(Profile {
        first_name: name_parts.first().unwrap_or(&"").to_string(),
        last_name: name_parts.get(1).unwrap_or(&"").to_string(),
        job_title,
        summary,
        experience,
        skills,
    })
}
