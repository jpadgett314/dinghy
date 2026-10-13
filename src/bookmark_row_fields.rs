// NOT YET IN USE (in the future, it'll be used to get linkding::Bookmark off of the public
// interface of BookmarkModel, which will make it easier to implement and use BookmarkModel
// methods that will be implemented in the future)
pub struct BookmarkRowFields {
    pub description: String,
    pub icon_path: Option<String>,
    pub tags: Vec<String>,
    pub title: String,
}

