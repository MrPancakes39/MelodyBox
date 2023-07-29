use serde::Deserialize;

// =============================[ Youtube Music API Structs ]==============================
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TabContent {
//     music_queue_renderer:
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabEndpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Deserialize)]
pub struct TabRenderer {
    // pub content: Option<TabContent>
    pub endpoint: Option<TabEndpoint>,
    pub unselectable: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub tab_renderer: TabRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextRenderer {
    pub tabs: [Tab; 3],
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
// ========================================================================================
