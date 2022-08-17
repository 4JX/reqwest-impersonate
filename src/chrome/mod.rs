use std::sync::Arc;

use boring::ssl::SslConnectorBuilder;
use http::HeaderMap;

mod ver;

pub(crate) fn build_chrome(ver: ChromeVersion) -> ChromeVersionData {
    match ver {
        ChromeVersion::V104 => ver::v104::get_data(),
    }
}

/// Defines the Chrome version to mimic when setting up a builder
#[derive(Debug)]
#[allow(missing_docs)]
pub enum ChromeVersion {
    V104,
}

pub(crate) struct ChromeVersionData {
    pub tls_builder_func: Arc<dyn Fn() -> SslConnectorBuilder + Send + Sync>,
    pub http2: Http2Data,
    pub headers: HeaderMap,
    pub gzip: bool,
    pub brotli: bool,
}

pub(crate) struct Http2Data {
    pub initial_stream_window_size: u32,
    pub initial_connection_window_size: u32,
    pub max_concurrent_streams: u32,
    pub max_header_list_size: u32,
    pub header_table_size: u32,
}
