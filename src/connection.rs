use bookmark_model::{BookmarkModel}
use linkding::{LinkDingClient}

mod imp {
    pub struct ServerConnection {
        url: str,
        auth_token: str,
        linkding_api: LinkDingClient
    }

    impl ServerConnection {

    }
}

// purpose: store connection, report status, construct models
impl ServerConnection {
    pub fn new(url: str, auth_token: str) -> Self {
        Self { url, auth_token, LinkDingClient::new(url, auth_token) }
    }

    pub fn get_bookmark_model(&self) {
        BookmarkModel::new( self.linkding_api )
    }
}

