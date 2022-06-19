use std::{fmt::Display};
use anyhow::Result;
use serde::Serialize;

use crate::{serializer::{DocSerializer, APIResponse}, yuque::Yuque};

#[derive(Debug)]
pub enum DocFormat {
    Markdown,
    Lake,
    Html,
}

impl Serialize for DocFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            DocFormat::Markdown => "markdown",
            DocFormat::Lake => "lake",
            DocFormat::Html => "html",
        })
    }
}

#[derive(Debug, Serialize, Default)]
pub struct CreateDocRequest {
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DocFormat>,
    pub body: String,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateDocRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _force_asl: Option<u8>,
}

impl Yuque {
    pub async fn list_docs<T: Display>(&self, repo: T, offset: Option<u32>, limit: Option<u8>, optional_properties: Option<Vec<String>>) -> Result<Vec<DocSerializer>> {
        let endpoint = format!("/repos/{}/docs", repo);
        let api = Yuque::build_api(&endpoint, None);

        let mut query = Vec::new();
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(optional_properties) = &optional_properties {
            let props = optional_properties.join(",");
            query.push(("optional_properties", props));
        }

        let resp = self.client.get(api).query(&query).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<Vec<DocSerializer>>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn get_doc<T: Display>(&self, repo: T, doc: T, raw: Option<bool>) -> Result<DocSerializer> {
        let endpoint = format!("/repos/{}/docs/{}", repo, doc);
        let api = Yuque::build_api(&endpoint, None);

        let mut query = Vec::new();
        if let Some(raw) = raw {
            if raw {
                query.push(("raw", 1));
            }
        }

        let resp = self.client.get(api).query(&query).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<DocSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn create_doc<T: Display>(&self, repo: T, req: &CreateDocRequest) -> Result<DocSerializer> {
        let endpoint = format!("/repos/{}/docs", repo);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.post(api).json(req).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<DocSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn delete_doc<T: Display>(&self, repo: T, doc_id: u32) -> Result<()> {
        let endpoint = format!("/repos/{}/docs/{}", repo, doc_id);
        let api = Yuque::build_api(&endpoint, None);
        let resp = self.client.delete(api).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn update_doc<T: Display>(&self, repo: T, doc_id: u32, req: &mut UpdateDocRequest) -> Result<DocSerializer> {
        let endpoint = format!("/repos/{}/docs/{}", repo, doc_id);
        let api = Yuque::build_api(&endpoint, None);
        if let Some(force_asl) = req._force_asl {
            if force_asl > 0 {
                req._force_asl = Some(1);
            } else {
                req._force_asl = None;
            }
        }
        let resp = self.client.put(api).json(req).send().await?;
        if resp.status().is_success() {
            let resp = resp.json::<APIResponse<DocSerializer>>().await?;
            Ok(resp.data)
        } else {
            Err(anyhow::anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::gen_rand_str;
    use crate::yuque::Yuque;
    use crate::doc::{CreateDocRequest, UpdateDocRequest};

    #[tokio::test]
    async fn test_list_repos() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let repo = "k8scat/programming";
        let repos = yuque.list_docs(repo, None, None, Some(vec!["hits".to_string()])).await.unwrap();
        assert_ne!(repos.len(), 0);
        assert_ne!(repos[0].hits, None);
    }

    #[tokio::test]
    async fn test_get_repo() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let repo_slug = "yuque";
        let repo = "k8scat/opensource";
        let repo = yuque.get_doc(repo, repo_slug, Some(true)).await.unwrap();
        assert_eq!(repo.slug, repo_slug);
    }

    #[tokio::test]
    async fn test_create_repo() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let repo = "k8scat/opensource";
        let rand_str = gen_rand_str();
        let content = "# Hello World";
        let req = CreateDocRequest {
            title: rand_str.clone(),
            slug: rand_str.clone(),
            body: content.to_string(),
            ..CreateDocRequest::default()
        };
        let doc = yuque.create_doc(repo, &req).await.unwrap();
        assert_eq!(doc.title, rand_str);
        assert_eq!(doc.slug, rand_str);
        assert_ne!(doc.body, None);
        if let Some(body) = &doc.body {
            assert_eq!(body, content);
        }
        
        yuque.delete_doc(repo, doc.object.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_doc() {
        let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
        let yuque = Yuque::new(token).unwrap();
        let repo = "k8scat/opensource";
        let rand_str = gen_rand_str();
        let content = "# Hello World";
        let req = CreateDocRequest {
            title: rand_str.clone(),
            slug: rand_str.clone(),
            body: content.to_string(),
            ..CreateDocRequest::default()
        };
        let doc = yuque.create_doc(repo, &req).await.unwrap();
        assert_eq!(doc.title, rand_str);
        assert_eq!(doc.slug, rand_str);
        assert_ne!(doc.body, None);
        if let Some(body) = &doc.body {
            assert_eq!(body, content);
        }

        let rand_str = gen_rand_str();
        let content = "# Updated";
        let mut req = UpdateDocRequest {
            title: Some(rand_str.clone()),
            slug: Some(rand_str.clone()),
            body: Some(content.to_string()),
            ..UpdateDocRequest::default()
        };
        let doc = yuque.update_doc(repo, doc.object.id, &mut req).await.unwrap();
        assert_eq!(doc.title, rand_str);
        assert_eq!(doc.slug, rand_str);
        assert_ne!(doc.body, None);
        if let Some(body) = &doc.body {
            assert_eq!(body, content);
        }
        
        yuque.delete_doc(repo, doc.object.id).await.unwrap();
    }
}