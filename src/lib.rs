use std::time::Duration;
use std::error::Error;

use response::news::NewsItem;
use response::News;
use response::Achievement;
use response::achievement::AchievementData;
mod response;

pub struct SteamAPI {
    client: reqwest::blocking::Client,
    api_key: String,
}


impl SteamAPI {
    const URL_API_BASE: &'static str =  "http://api.steampowered.com";
    const MAX_LENGTH_NEWS: i32 = 300;

    pub fn new(api_key: String) -> Result<SteamAPI, Box<dyn Error>> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(SteamAPI{client, api_key})
    }

    fn create_base_url(&self, interface: &str, method: &str, version: &str) -> String {
        format!("{}/{}/{}/v{}/", SteamAPI::URL_API_BASE, interface, method, version)
    }

    pub fn get_news_for_app(&self, app_id: i32, count: i32) -> Result<Vec<NewsItem>, Box<dyn Error>> {
        let base_url = self.create_base_url("ISteamNews", "GetNewsForApp", "0002");
        let param_url = format!("?appid={}&count={}&maxlength={}&format=json", app_id, count, SteamAPI::MAX_LENGTH_NEWS);
        let url = format!("{}{}", base_url, param_url);
        let response = self.client.get(url)
            .send()
            .unwrap();
        if response.status().is_success() {
            println!("Getting news successful.");
        }
        let news = response.json::<News>().unwrap();
        let items = news.appnews.newsitems;
        let num_news = items.len();
        println!("Got {num_news} news");
        Ok(items)
    }

    pub fn get_global_achievement_percentages_for_app(&self, app_id: i32) -> Result<Vec<AchievementData>, Box<dyn Error>> {
        let base_url = self.create_base_url("ISteamUserStats", "GetGlobalAchievementPercentagesForApp", "0002");
        let param_url = format!("?gameid={}&format=json", app_id);
        let url = format!("{}{}", base_url, param_url);
        let response = self.client.get(url)
            .send()
            .unwrap();
        if response.status().is_success() {
            println!("Getting global achievement percentages successful.");
        }
        let achievements = response.json::<Achievement>().unwrap();
        let achievements = achievements.achievementpercentages;
        match achievements {
            Some(achievements) => Ok(achievements.achievements),
            _ => {
                println!("No achievements for this APP");
                Ok(vec![])
            }
        }
    }

    pub fn get_player_summaries(&self, steam_id: i64) {
        let base_url = self.create_base_url("ISteamUser", "GetPlayerSummaries", "0002");
        let param_url = format!("?key={}&steamids={}", self.api_key, steam_id);
        let url = format!("{}{}", base_url, param_url);
        println!("{url}");
        let response = self.client.get(url)
            .send()
            .unwrap();
        if response.status().is_success() {
            println!("Getting player summaries successful.");
        }
        println!("{response:?}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const ID_ENDER_LILIES: i32 = 1369630;
    const ID_ENDER_MAGNOLIA: i32 = 2725260;
    const MY_PLAYER_ID: i64 = 76561198065075223;

    fn parse_arg() -> Result<String, Box<dyn Error>>{
        let api_key = env::args().nth(2).unwrap_or("".to_string());
        Ok(api_key)
    }

    fn create_api() -> Result<SteamAPI, Box<dyn Error>> {
        let api_key = parse_arg().unwrap();
        let api = SteamAPI::new(api_key).unwrap();
        Ok(api)
    }

    #[test]
    fn get_news() {
        // create API
        let api = create_api().unwrap();
        let news = api.get_news_for_app(ID_ENDER_LILIES, 10);
        assert!(news.is_ok());
        println!("get news test have passed.");
    }

    #[test]
    fn get_achievements() {
        // create API
        let api = create_api().unwrap();
        // for game which have achievements.
        let achievements = api.get_global_achievement_percentages_for_app(ID_ENDER_LILIES);
        assert!(achievements.is_ok());
        let achievements = achievements.unwrap();
        assert_ne!(achievements.len(), 0);
        // for game which have NO achievements.
        let achievements = api.get_global_achievement_percentages_for_app(ID_ENDER_MAGNOLIA);
        assert!(achievements.is_ok());
        println!("get achievements test have passed.");
    }

    #[test]
    fn get_player_summaries() {
        // create API
        let api = create_api().unwrap();
        api.get_player_summaries(MY_PLAYER_ID);
    }
}
