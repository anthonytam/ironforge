use super::{types::achievements::{AchievementCateogoryIndex, Achievement, AchievementMedia, AchievementsIndex, AchivementSummaryCategory}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_achievement_categories_index(&self) -> Result<AchievementCateogoryIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/achievement-category/index"), "static")
                                .await?
                                .json::<AchievementCateogoryIndex>()
                                .await?;

            Ok(response)
    }

    pub async fn get_achievement_category(&self, id: u32) -> Result<AchivementSummaryCategory> {
        let response = self.client
                                .send_request(format!("/data/wow/achievement-category/{}", id), "static")
                                .await?
                                .json::<AchivementSummaryCategory>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_achievements_index(&self) -> Result<AchievementsIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/achievement/index"), "static")
                                .await?
                                .json::<AchievementsIndex>()
                                .await?;
        
        Ok(response)
    }

    pub async fn get_achivement(&self, id: u32) -> Result<Achievement> {
        let response = self.client
                                .send_request(format!("/data/wow/achievement/{}", id), "static")
                                .await?
                                .json::<Achievement>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_achivement_media(&self, id: u32) -> Result<AchievementMedia> {
        let response = self.client
                                .send_request(format!("/data/wow/media/achievement/{}", id), "static")
                                .await?
                                .json::<AchievementMedia>()
                                .await?;

                                Ok(response)
    }
}