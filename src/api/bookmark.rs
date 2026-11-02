use linkding::{CreateBookmarkBody, UpdateBookmarkBody};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Bookmark {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: String,
    pub favicon_url: Option<String>,
    pub tag_names: Vec<String>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BookmarkChanges {
    pub id: i32,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tag_names: Option<Vec<String>>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BookmarkToAdd {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tag_names: Option<Vec<String>>,
}

pub fn to_internal(external: &linkding::Bookmark) -> Bookmark {
    Bookmark {
        id: external.id.clone(),
        url: external.url.clone(),
        title: external.title.clone(),
        description: external.description.clone(),
        favicon_url: external.favicon_url.clone(),
        tag_names: external.tag_names.clone(),
    }
}

pub fn to_create_req(internal: &BookmarkToAdd) -> CreateBookmarkBody {
    CreateBookmarkBody {
        url: internal.url.clone(),
        title: internal.title.clone(),
        description: internal.description.clone(),
        notes: None,
        web_archive_snapshot_url: None,
        favicon_url: None,
        preview_image_url: None,
        is_archived: None,
        unread: None,
        shared: None,
        tag_names: internal.tag_names.clone(),
        date_added: None,
        date_modified: None,
        website_title: None,
        website_description: None,
    }
}

pub fn to_update_req(internal: &BookmarkChanges) -> UpdateBookmarkBody {
    UpdateBookmarkBody {
        // `url` field is not updatable by this implementation of the API
        title: internal.title.clone(),
        description: internal.description.clone(),
        notes: None,
        web_archive_snapshot_url: None,
        favicon_url: None,
        preview_image_url: None,
        is_archived: None,
        unread: None,
        shared: None,
        tag_names: internal.tag_names.clone(),
        date_added: None,
        date_modified: None,
        website_title: None,
        website_description: None,
    }
}

