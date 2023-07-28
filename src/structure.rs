use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TabEndpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TabRenderer {
    pub endpoint: Option<TabEndpoint>,
    // unselectable: Option<> // ??
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub tab_renderer: TabRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextRenderer {
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextTabbedResultsRenderer {
    pub watch_next_tabbed_results_renderer: WatchNextRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabbedRenderer {
    pub tabbed_renderer: WatchNextTabbedResultsRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleColumnMusicWatchNextResultsRenderer {
    pub single_column_music_watch_next_results_renderer: TabbedRenderer,
}

#[derive(Debug, Deserialize)]
pub struct NextEndpoint {
    pub contents: SingleColumnMusicWatchNextResultsRenderer,
}
