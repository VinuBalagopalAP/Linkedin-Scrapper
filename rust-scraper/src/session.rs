use rand::seq::SliceRandom;

pub struct SessionManager {
    cookies: Vec<String>,
    user_agents: Vec<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            cookies: vec![
                "li_at=XXXXXX".to_string(),
                // Add more cookies
            ],
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
                // Add more user agents
            ],
        }
    }

    pub fn rotate_session(&self) -> (String, String) {
        let cookie = self.cookies.choose(&mut rand::thread_rng()).unwrap();
        let user_agent = self.user_agents.choose(&mut rand::thread_rng()).unwrap();
        (cookie.clone(), user_agent.clone())
    }
}
