use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct APIResponse<T> {
    pub data: T,
    #[serde(default)]
    pub abilities: Option<Abilities>,
}

#[derive(Debug, Deserialize)]
pub struct Abilities {
    #[serde(flatten)]
    pub base: BaseAbilities,
    #[serde(default)]
    pub group_user: BaseAbilities,
    #[serde(default)]
    pub repo: BaseAbilities,
}

#[derive(Debug, Deserialize, Default)]
pub struct BaseAbilities {
    #[serde(default)]
    pub read: Option<bool>,
    #[serde(default)]
    pub update: Option<bool>,
    #[serde(default)]
    pub destroy: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Object {
    pub id: u32,
    #[serde(rename(deserialize = "type"), default)]
    pub typ: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub _serializer: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct UserSerializer {
    #[serde(flatten)]
    pub object: Object,
    pub name: String,
    pub login: String,
    pub avatar_url: String,
    #[serde(default)]
    pub books_count: Option<u32>,
    #[serde(default)]
    pub public_books_count: Option<u32>,
    pub followers_count: u32,
    pub following_count: u32,
    #[serde(default)]
    pub public: Option<u8>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct GroupSerializer {
    #[serde(flatten)]
    pub object: Object,
    pub name: String,
    pub login: String,
    pub avatar_url: String,
    #[serde(default)]
    pub owner_id: Option<u32>,
    pub books_count: Option<u32>,
    pub public_books_count: Option<u32>,
    pub topics_count: u32,
    pub public_topics_count: u32,
    pub members_count: u32,
    pub public: u8,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct GroupUserSerializer {
    #[serde(flatten)]
    pub object: Object,
    pub group_id: u32,
    pub user_id: u32,
    #[serde(default)]
    pub group: Option<UserSerializer>,
    pub user: UserSerializer,
    pub role: u8,
    pub visibility: u8,
    pub status: u8,
}

#[derive(Debug, Deserialize, Default)]
pub struct BookSerializer {
    #[serde(flatten)]
    pub object: Object,
    pub slug: String,
    pub name: String,
    pub user_id: u32,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub creator_id: Option<u32>,
    pub public: u8,
    pub items_count: u32,
    pub likes_count: u32,
    pub watches_count: u32,
    #[serde(default)]
    pub content_updated_at: Option<String>,
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default)]
    pub user: Option<UserSerializer>,

    #[serde(default)]
    pub toc: Option<String>,
    #[serde(default)]
    pub toc_yml: Option<String>,
    #[serde(default)]
    pub pinned_at: Option<String>,
    #[serde(default)]
    pub archived_at: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct DocSerializer {
    #[serde(flatten)]
    pub object: Object,
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub user_id: u32,
    pub book_id: u32,
    pub format: String,
    pub public: u8,
    pub status: u8,
    pub view_status: u8,
    pub read_status: u8,
    #[serde(default)]
    pub likes_count: Option<u32>,
    #[serde(default)]
    pub read_count: Option<u32>,
    #[serde(default)]
    pub comments_count: Option<u32>,
    pub content_updated_at: String,
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub first_published_at: Option<String>,
    #[serde(default)]
    pub draft_version: Option<u16>,
    #[serde(default)]
    pub last_editor_id: Option<u32>,
    pub word_count: u32,
    #[serde(default)]
    pub cover: Option<String>,
    #[serde(default)]
    pub custom_description: Option<String>,
    #[serde(default)]
    pub last_editor: Option<UserSerializer>,
    #[serde(default)]
    pub book: Option<BookSerializer>,
    #[serde(default)]
    pub hits: Option<u32>,
    #[serde(default)]
    pub creator: Option<UserSerializer>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub body_draft: Option<String>,
    #[serde(default)]
    pub body_html: Option<String>,
    #[serde(default)]
    pub body_lake: Option<String>,
    #[serde(default)]
    pub body_draft_lake: Option<String>,
    #[serde(default)]
    pub deleted_at: Option<String>,
}
