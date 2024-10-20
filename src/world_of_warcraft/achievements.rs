use super::{types::achievements::{AchievementCateogoryIndex, Achievement, AchievementMedia, AchievementsIndex, AchivementSummaryCategory}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_achievement_categories_index(&self) -> Result<AchievementCateogoryIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement-category/index"), "static")
                                .await?
                                .json::<AchievementCateogoryIndex>()
                                .await;

            response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_achievement_category(&self, id: u32) -> Result<AchivementSummaryCategory> {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement-category/{}", id), "static")
                                .await?
                                .json::<AchivementSummaryCategory>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_achievements_index(&self) -> Result<AchievementsIndex> {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement/index"), "static")
                                .await?
                                .json::<AchievementsIndex>()
                                .await;
        
        response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_achivement(&self, id: u32) -> Result<Achievement> {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement/{}", id), "static")
                                .await?
                                .json::<Achievement>()
                                .await;
        
                                response_result.map_err(anyhow::Error::from)
    }

    pub async fn get_achivement_media(&self, id: u32) -> Result<AchievementMedia> {
        let response_result = self.client
                                .send_request(format!("/data/wow/media/achievement/{}", id), "static")
                                .await?
                                .json::<AchievementMedia>()
                                .await;

                                response_result.map_err(anyhow::Error::from)
    }
}