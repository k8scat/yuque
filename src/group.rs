use crate::serializer::{APIResponse, GroupSerializer, GroupUserSerializer};
use crate::yuque::Yuque;
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Display;

impl Yuque {
    pub async fn list_user_groups<T: Display>(&self, user: T) -> Result<Vec<GroupSerializer>> {
        let endpoint = format!("/users/{}/groups", user);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<Vec<GroupSerializer>>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn list_public_groups(&self, offset: Option<u32>) -> Result<Vec<GroupSerializer>> {
        let endpoint = format!("/groups");
        let api = Yuque::build_api(&endpoint, None);
        let mut query = Vec::new();
        if let Some(offset) = offset {
            query.push(("offset", offset));
        }
        let resp = self.client.get(api).query(&query).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<Vec<GroupSerializer>>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn list_group_users<T: Display>(
        &self,
        group: T,
        offset: Option<u32>,
        limit: Option<u8>,
        role: Option<u8>,
    ) -> Result<Vec<GroupUserSerializer>> {
        let endpoint = format!("/groups/{}/users", group);
        let api = Yuque::build_api(&endpoint, None);

        let mut query = Vec::<(&str, u32)>::new();
        if let Some(role) = role {
            query.push(("role", role.into()));
        }
        if let Some(offset) = offset {
            query.push(("offset", offset));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.into()));
        }

        let resp = self.client.get(api).query(&query).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<Vec<GroupUserSerializer>>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn get_group<T: Display>(&self, group: T) -> Result<APIResponse<GroupSerializer>> {
        let endpoint = format!("/groups/{}", group);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<GroupSerializer>>().await?;
            Ok(resp)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn create_group(
        &self,
        name: &str,
        login: &str,
        desc: &str,
    ) -> Result<GroupSerializer> {
        let endpoint = format!("/groups");
        let api = Yuque::build_api(&endpoint, None);
        let resp = self
            .client
            .post(api)
            .json(&json!({
                "name": name,
                "login": login,
                "description": desc,
            }))
            .send()
            .await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<GroupSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn update_group<T: Display>(
        &self,
        group: T,
        name: Option<&str>,
        login: Option<&str>,
        desc: Option<&str>,
    ) -> Result<GroupSerializer> {
        let endpoint = format!("/groups/{}", group);
        let api = Yuque::build_api(&endpoint, None);

        let mut payload = HashMap::new();
        if let Some(name) = name {
            payload.insert("name", name);
        }
        if let Some(login) = login {
            payload.insert("login", login);
        }
        if let Some(desc) = desc {
            payload.insert("description", desc);
        }

        let resp = self.client.put(api).json(&payload).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<GroupSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn delete_group<T: Display>(&self, group: T) -> Result<()> {
        let endpoint = format!("/groups/{}", group);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.delete(api).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn add_group_user<T: Display>(&self, group: T, user: T, role: u8) -> Result<GroupUserSerializer> {
        let endpoint = format!("/groups/{}/users/{}", group, user);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self
            .client
            .put(api)
            .json(&json!({
                "role": role,
            }))
            .send()
            .await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<GroupUserSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn delete_group_user<T: Display>(&self, group: T, user: T) -> Result<()> {
        let endpoint = format!("/groups/{}/users/{}", group, user);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.delete(api).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::yuque::Yuque;
    use crate::utils::gen_rand_str;

    #[tokio::test]
    async fn test_list_user_groups() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        assert_eq!(user.name, "K8sCat");
        let groups = yuque.list_user_groups(user.object.id).await.unwrap();
        assert_ne!(groups.len(), 0);
    }

    #[tokio::test]
    async fn test_list_public_groups() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let groups = yuque.list_public_groups(None).await.unwrap();
        assert_ne!(groups.len(), 0);
    }

    #[tokio::test]
    async fn test_list_group_users() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let users = yuque
            .list_group_users("huayin.io", None, None, None)
            .await
            .unwrap();
        assert_ne!(users.len(), 0);
    }

    #[tokio::test]
    async fn test_get_group() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let resp = yuque.get_group("huayin.io").await.unwrap();
        assert_eq!(resp.data.login, "huayin.io");
    }

    #[tokio::test]
    async fn test_create_group() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let rand_str = gen_rand_str();
        let group = yuque
            .create_group(&rand_str, &rand_str, &rand_str)
            .await
            .unwrap();
        assert_eq!(group.name, rand_str);

        yuque.delete_group(&group.object.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_group() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let rand_str = gen_rand_str();
        let group = yuque
            .create_group(&rand_str, &rand_str, &rand_str)
            .await
            .unwrap();
        assert_eq!(group.name, rand_str);

        let rand_str = gen_rand_str();
        let group = yuque
            .update_group(group.object.id, Some(&rand_str), Some(&rand_str), Some(&rand_str))
            .await
            .unwrap();
        assert_eq!(group.name, rand_str);
        assert_eq!(group.login, rand_str);
        assert_eq!(group.description.unwrap_or_default(), rand_str);

        yuque.delete_group(&group.object.id).await.unwrap();
    }
}
