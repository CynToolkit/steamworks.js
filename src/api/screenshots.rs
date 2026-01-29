use napi_derive::napi;

#[napi]
pub mod screenshots {
    use napi::bindgen_prelude::Error;
    use std::path::Path;

    /// Triggers the Steam overlay to take a screenshot.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamScreenshots#TriggerScreenshot}
    #[napi]
    pub fn trigger_screenshot() {
        let client = crate::client::get_client();
        let screenshots = client.screenshots();
        screenshots.trigger_screenshot();
    }

    /// Adds a screenshot to the user's Steam screenshot library from disk.
    ///
    /// @param filename - The absolute path to the screenshot image file
    /// @param thumbnail_filename - Optional path to a thumbnail image (can be null/undefined)
    /// @param width - Width of the screenshot in pixels
    /// @param height - Height of the screenshot in pixels
    /// @returns The screenshot handle, or throws an error if the operation fails
    ///
    /// This call is asynchronous. The screenshot will be processed and added to the library.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamScreenshots#AddScreenshotToLibrary}
    #[napi]
    pub fn add_screenshot_to_library(
        filename: String,
        thumbnail_filename: Option<String>,
        width: i32,
        height: i32,
    ) -> Result<u32, Error> {
        let client = crate::client::get_client();
        let screenshots = client.screenshots();

        let path = Path::new(&filename);
        let thumbnail_path = thumbnail_filename.as_ref().map(|s| Path::new(s.as_str()));

        match screenshots.add_screenshot_to_library(path, thumbnail_path, width, height) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(Error::from_reason(e.to_string())),
        }
    }

}
