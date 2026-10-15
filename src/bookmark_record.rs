use linkding::{Bookmark, CreateBookmarkBody, UpdateBookmarkBody};

// linkding::Bookmark fails to implement these traits...
#[derive(Clone, Default, Debug, PartialEq)]
pub struct BookmarkRecord {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: String,
    pub notes: String,
    pub web_archive_snapshot_url: String,
    pub favicon_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub is_archived: bool,
    pub unread: bool,
    pub shared: bool,
    pub tag_names: Vec<String>,
    pub date_added: String,
    pub date_modified: String,
    pub website_title: Option<String>,
    pub website_description: Option<String>,
}

impl From<Bookmark> for BookmarkRecord {
    fn from(b: Bookmark) -> BookmarkRecord {
        BookmarkRecord {
            id: b.id,
            url: b.url,
            title: b.title,
            description: b.description,
            notes: b.notes,
            web_archive_snapshot_url: b.web_archive_snapshot_url,
            favicon_url: b.favicon_url,
            preview_image_url: b.preview_image_url,
            is_archived: b.is_archived,
            unread: b.unread,
            shared: b.shared,
            tag_names: b.tag_names,
            date_added: b.date_added,
            date_modified: b.date_modified,
            website_title: b.website_title,
            website_description: b.website_description,
        }
    }
}

pub fn to_internal(external: &Bookmark) -> BookmarkRecord {
    BookmarkRecord {
        id: external.id.clone(),
        url: external.url.clone(),
        title: external.title.clone(),
        description: external.description.clone(),
        notes: external.notes.clone(),
        web_archive_snapshot_url: external.web_archive_snapshot_url.clone(),
        favicon_url: external.favicon_url.clone(),
        preview_image_url: external.preview_image_url.clone(),
        is_archived: external.is_archived.clone(),
        unread: external.unread.clone(),
        shared: external.shared.clone(),
        tag_names: external.tag_names.clone(),
        date_added: external.date_added.clone(),
        date_modified: external.date_modified.clone(),
        website_title: external.website_title.clone(),
        website_description: external.website_description.clone(),
    }
}

#[allow(unused)]
pub fn to_external(internal: &BookmarkRecord) -> Bookmark {
    Bookmark {
        id: internal.id.clone(),
        url: internal.url.clone(),
        title: internal.title.clone(),
        description: internal.description.clone(),
        notes: internal.notes.clone(),
        web_archive_snapshot_url: internal.web_archive_snapshot_url.clone(),
        favicon_url: internal.favicon_url.clone(),
        preview_image_url: internal.preview_image_url.clone(),
        is_archived: internal.is_archived.clone(),
        unread: internal.unread.clone(),
        shared: internal.shared.clone(),
        tag_names: internal.tag_names.clone(),
        date_added: internal.date_added.clone(),
        date_modified: internal.date_modified.clone(),
        website_title: internal.website_title.clone(),
        website_description: internal.website_description.clone(),
    }
}

// TODO: check empty strings
pub fn to_create_req(internal: &BookmarkRecord) -> CreateBookmarkBody {
    CreateBookmarkBody {
        url: internal.url.clone(),
        title: Some(internal.title.clone()),
        description: Some(internal.description.clone()),
        notes: Some(internal.notes.clone()),
        web_archive_snapshot_url: Some(internal.web_archive_snapshot_url.clone()),
        favicon_url: internal.favicon_url.clone(),
        preview_image_url: internal.preview_image_url.clone(),
        is_archived: Some(internal.is_archived.clone()),
        unread: Some(internal.unread.clone()),
        shared: Some(internal.shared.clone()),
        tag_names: Some(internal.tag_names.clone()),
        date_added: Some(internal.date_added.clone()),
        date_modified: Some(internal.date_modified.clone()),
        website_title: internal.website_title.clone(),
        website_description: internal.website_description.clone(),
    }
}

pub fn to_update_req(internal: &BookmarkRecord) -> UpdateBookmarkBody {
    UpdateBookmarkBody {
        title: Some(internal.title.clone()),
        description: Some(internal.description.clone()),
        notes: Some(internal.notes.clone()),
        web_archive_snapshot_url: Some(internal.web_archive_snapshot_url.clone()),
        favicon_url: internal.favicon_url.clone(),
        preview_image_url: internal.preview_image_url.clone(),
        is_archived: Some(internal.is_archived.clone()),
        unread: Some(internal.unread.clone()),
        shared: Some(internal.shared.clone()),
        tag_names: Some(internal.tag_names.clone()),
        date_added: Some(internal.date_added.clone()),
        date_modified: Some(internal.date_modified.clone()),
        website_title: internal.website_title.clone(),
        website_description: internal.website_description.clone(),
    }
}

