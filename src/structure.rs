use serde::Deserialize;

// =============================[ Youtube Music API Structs ]==============================
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackRun {
    pub text: String,
    pub navigation_endpoint: Option<Endpoint>,
}

#[derive(Debug, Deserialize)]
pub struct LineTextRuns {
    pub runs: Vec<TrackRun>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Thumbnail {
    pub width: i32,
    pub height: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailStore {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Text {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct PropRuns {
    pub runs: Vec<Text>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoRenderer {
    pub video_id: String,
    pub title: PropRuns,
    pub length_text: Option<PropRuns>, // Taking a Guess
    pub thumbnail: ThumbnailStore,
    // pub navigation_endpoint: NavEndpoint, Not Needed
    pub long_byline_text: LineTextRuns,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryRenderer {
    pub playlist_panel_video_renderer: Option<PlaylistPanelVideoRenderer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoWrapperRenderer {
    pub primary_renderer: PrimaryRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackResult {
    pub playlist_panel_video_wrapper_renderer: Option<PlaylistPanelVideoWrapperRenderer>,
    pub playlist_panel_video_renderer: Option<PlaylistPanelVideoRenderer>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistPanelRenderer {
    pub contents: Vec<TrackResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueContent {
    pub playlist_panel_renderer: PlaylistPanelRenderer,
}

#[derive(Debug, Deserialize)]
pub struct MusicQueueRenderer {
    pub content: QueueContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabContent {
    pub music_queue_renderer: MusicQueueRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Deserialize)]
pub struct TabRenderer {
    pub content: Option<TabContent>,
    pub endpoint: Option<Endpoint>,
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
