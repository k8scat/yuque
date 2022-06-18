use crate::serializer::{UserSerializer, APIResponse};
use crate::yuque::Yuque;
use anyhow::Result;
use std::fmt::Display;

impl Yuque {
    pub async fn get_auth_user(&self) -> Result<UserSerializer> {
        let api = Yuque::build_api("/user", None);
        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<UserSerializer>>().await?;
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
            let resp = resp.json::<APIResponse<UserSerializer>>().await?;
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
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        assert_eq!(user.name, "K8sCat");
    }

    #[tokio::test]
    async fn test_get_user() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_user(565457).await.unwrap();
        assert_eq!(user.name, "K8sCat");
    }
}
