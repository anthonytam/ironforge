use super::{types::achievements::{AchievementCateogoryIndex, Achievement, AchievementMedia, AchievementsIndex, AchivementSummaryCategory}, WorldOfWarcraftClient};

impl WorldOfWarcraftClient {
    pub async fn get_achievement_categories_index(&self) -> AchievementCateogoryIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement-category/index"), "static")
                                .await
                                .json::<AchievementCateogoryIndex>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_achievement_category(&self, id: u32) -> AchivementSummaryCategory {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement-category/{}", id), "static")
                                .await
                                .json::<AchivementSummaryCategory>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_achievements_index(&self) -> AchievementsIndex {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement/index"), "static")
                                .await
                                .json::<AchievementsIndex>()
                                .await;
        println!("{:?}", response_result);
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_achivement(&self, id: u32) -> Achievement {
        let response_result = self.client
                                .send_request(format!("/data/wow/achievement/{}", id), "static")
                                .await
                                .json::<Achievement>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }

    pub async fn get_achivement_media(&self, id: u32) -> AchievementMedia {
        let response_result = self.client
                                .send_request(format!("/data/wow/media/achievement/{}", id), "static")
                                .await
                                .json::<AchievementMedia>()
                                .await;
        match response_result {
            Ok(response) => response,
            Err(e) => panic!("Failed to get a response. {:?}", e)
        }
    }
}