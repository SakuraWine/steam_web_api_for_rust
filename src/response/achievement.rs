use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Achievement {
    pub achievementpercentages: Option<AchievementPercentages>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AchievementPercentages {
    pub achievements: Vec<AchievementData>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AchievementData {
    pub name: String,
    pub percent: f64,
}