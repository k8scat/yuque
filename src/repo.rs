use std::fmt::{Display, self};
use anyhow::Result;
use serde::Serialize;

use crate::{yuque::Yuque, serializer::{BookSerializer, APIResponse}};

pub enum ObjType {
    Group,
    User,
}

impl Display for ObjType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            ObjType::Group => "groups",
            ObjType::User => "users",
        })?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub enum RepoType {
    Book,
    Design
}

impl Display for RepoType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            RepoType::Book => "Book",
            RepoType::Design => "Design",
        })?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum RepoPublic {
    Private, // 私密
    Public, // 所有人可见
    GroupMember, // 空间成员可见
    GroupAll, // 空间所有人（含外部联系人）可见
    RepoMember, // 知识库成员可见
}

impl From<RepoPublic> for u8 {
    fn from(p: RepoPublic) -> u8 {
        match p {
            RepoPublic::Private => 0,
            RepoPublic::Public => 1,
            RepoPublic::GroupMember => 2,
            RepoPublic::GroupAll => 3,
            RepoPublic::RepoMember => 4,
        }
    }
}

impl Serialize for RepoPublic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            RepoPublic::Private => "0",
            RepoPublic::Public => "1",
            RepoPublic::GroupMember => "2",
            RepoPublic::GroupAll => "3",
            RepoPublic::RepoMember => "4",
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CreateRepoRequest {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub public: RepoPublic,
    #[serde(rename = "type")]
    pub typ: RepoType,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateRepoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<RepoPublic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc: Option<String>,
}

impl Yuque {
    pub async fn list_repos<T: Display>(&self, obj_type: ObjType, obj: T, repo_type: Option<RepoType>, offset: Option<u32>) -> Result<Vec<BookSerializer>> {
        let endpoint = format!("/{}/{}/repos", obj_type, obj);
        let api = Yuque::build_api(&endpoint, None);

        let mut query = Vec::new();
        if let Some(repo_type) = repo_type {
            query.push(("type", repo_type.to_string()));
        }
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }

        let resp = self.client.get(api).query(&query).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<Vec<BookSerializer>>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn get_repo<T: Display>(&self, repo: T) -> Result<APIResponse<BookSerializer>> {
        let endpoint = format!("/repos/{}", repo);
        let api = Yuque::build_api(&endpoint, None);

        let resp = self.client.get(api).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<BookSerializer>>().await?;
            Ok(resp)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn delete_repo<T: Display>(&self, repo: T) -> Result<()> {
        let endpoint = format!("/repos/{}", repo);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.delete(api).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn create_repo<T: Display>(&self, obj_type: ObjType, obj: T, req: &CreateRepoRequest) -> Result<BookSerializer> {
        let endpoint = format!("/{}/{}/repos", obj_type, obj);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.post(api).json(req).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<BookSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn update_repo<T: Display>(&self, repo: T, req: &UpdateRepoRequest) -> Result<BookSerializer> {
        let endpoint = format!("/repos/{}", repo);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.put(api).json(req).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<BookSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::yuque::Yuque;
    use crate::repo::{ObjType, RepoType, CreateRepoRequest, RepoPublic, UpdateRepoRequest};
    use crate::utils::gen_rand_str;

    #[tokio::test]
    async fn test_list_repos() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        assert_eq!(user.name, "K8sCat");

        let repos = yuque.list_repos(ObjType::User, user.object.id, Some(RepoType::Book), None).await.unwrap();
        assert_ne!(repos.len(), 0);
    }

    #[tokio::test]
    async fn test_get_repo() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let namespace = "k8scat/programming";
        let repo = yuque.get_repo(namespace).await.unwrap();
        assert_eq!(repo.data.namespace.unwrap(), namespace);
    }

    #[tokio::test]
    async fn test_create_repo() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        assert_eq!(user.name, "K8sCat");

        let rand_str = gen_rand_str();
        let req = CreateRepoRequest {
            name: rand_str.clone(),
            slug: rand_str.clone(),
            description: rand_str.clone(),
            public: RepoPublic::Private,
            typ: RepoType::Book,
        };
        let repo = yuque.create_repo(ObjType::User, user.login, &req).await.unwrap();
        assert_eq!(repo.name, rand_str);

        yuque.delete_repo(repo.object.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_repo() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let user = yuque.get_auth_user().await.unwrap();
        assert_eq!(user.name, "K8sCat");

        let rand_str = gen_rand_str();
        let req = CreateRepoRequest {
            name: rand_str.clone(),
            slug: rand_str.clone(),
            description: rand_str.clone(),
            public: RepoPublic::Private,
            typ: RepoType::Book,
        };
        let repo = yuque.create_repo(ObjType::User, user.login, &req).await.unwrap();

        let rand_str = gen_rand_str();
        let req = UpdateRepoRequest {
            name: Some(rand_str.clone()),
            slug: Some(rand_str.clone()),
            description: Some(rand_str.clone()),
            public: Some(RepoPublic::Public),
            ..UpdateRepoRequest::default()
        };
        let repo = yuque.update_repo(repo.object.id, &req).await.unwrap();
        assert_eq!(repo.name, rand_str);
        assert_eq!(repo.slug, rand_str);
        if let Some(desc) = &repo.description {
            assert_eq!(desc, &rand_str);
        }
        let p: u8 = RepoPublic::Public.into();
        assert_eq!(repo.public, p);

        yuque.delete_repo(repo.object.id).await.unwrap();
    }
}