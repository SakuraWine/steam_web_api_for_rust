use std::time::Duration;
use std::error::Error;

use response::achievement;
use response::news::NewsItem;
use response::News;
use response::Achievement;
use response::achievement::AchievementData;
mod response;

pub struct SteamAPI {
    client: reqwest::blocking::Client,
}


impl SteamAPI {
    const URL_API_BASE: &'static str =  "http://api.steampowered.com";
    const MAX_LENGTH_NEWS: i32 = 300;

    pub fn new() -> Result<SteamAPI, Box<dyn Error>> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(SteamAPI{client})
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // API creation.
        let id_ender_lilies = 1369630;
        let id_ender_magnolia = 2725260;
        let api = SteamAPI::new().unwrap();
        // Test get news
        let news = api.get_news_for_app(id_ender_lilies, 10);
        assert!(news.is_ok());
        // let news = news.unwrap();
        // println!("{news:?}");
        // Test get achievements
        // for game which have achievements.
        let achievements = api.get_global_achievement_percentages_for_app(id_ender_lilies);
        assert!(achievements.is_ok());
        let achievements = achievements.unwrap();
        assert_ne!(achievements.len(), 0);
        // for game which have NO achievements.
        let achievements = api.get_global_achievement_percentages_for_app(id_ender_magnolia);
        assert!(achievements.is_ok());
    }
}
