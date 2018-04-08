use hyper::{Client, Uri, client::HttpConnector};

/// A client for the RonFS protocol.
#[derive(Debug)]
pub struct RonFSClient {
    client: Client<HttpConnector>,
    base_uri: Uri,
}

impl RonFSClient {
    /// Creates a new filesystem.
    pub fn new(client: Client<HttpConnector>, base_uri: Uri) -> RonFSClient {
        RonFSClient { client, base_uri }
    }
}
