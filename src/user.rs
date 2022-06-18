use std::fmt::Display;

use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::yuque::Yuque;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserSerializer {
    pub id: u64,
    #[serde(rename(deserialize = "type"))]
    pub user_type: String,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GetUserResponse {
    pub data: UserSerializer,
}

impl Yuque {
    pub async fn get_auth_user(&self) -> Result<UserSerializer> {
        let api = Yuque::build_api("/user", None);
        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<GetUserResponse>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn get_user<T: Display>(&self, id: T) -> Result<UserSerializer> {
        let endpoint = format!("/users/{}", id);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<GetUserResponse>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::yuque::Yuque;

    #[tokio::test]
    async fn test_get_auth_user() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default().to_string();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        println!("{:?}", user)
    }

    #[tokio::test]
    async fn test_get_user() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default().to_string();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_user(565457).await.unwrap();
        println!("{:?}", user)
    }
}
