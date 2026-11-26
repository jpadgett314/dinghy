use crate::api::BookmarkChanges;
use crate::models::BookmarkCardModel;

use std::collections::HashSet;

pub enum TagOperation {
    Add,
    Remove,
    Replace,
}

pub struct BatchUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tag_operation: TagOperation,
    pub tags: Vec<String>,
}

impl BatchUpdate {
    pub fn for_bookmark(&self, bookmark: &BookmarkCardModel) -> Option<BookmarkChanges> {
        let current_tags: Vec<String> = bookmark
            .tag_names()
            .iter()
            .map(ToString::to_string)
            .collect();

        let tags = match self.tag_operation {
            TagOperation::Add =>
                set_union(current_tags.clone(), self.tags.clone()),
            TagOperation::Remove =>
                set_difference(current_tags.clone(), self.tags.clone()),
            TagOperation::Replace => self.tags.clone(),
        };

        let title = self.title.clone();
        let description = self.description.clone();
        let tag_names = (tags != current_tags).then_some(tags);

        if title.is_none() && description.is_none() && tag_names.is_none() {
            return None;
        }

        Some(BookmarkChanges {
            id: bookmark.id(),
            title,
            description,
            tag_names,
            ..Default::default()
        })
    }
}

fn set_union(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    v1.into_iter()
        .chain(v2)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn set_difference(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    let set1: HashSet<String> = v1.into_iter().collect();
    let set2: HashSet<String> = v2.into_iter().collect();

    set1.difference(&set2)
        .cloned()
        .collect()
}

