use napi_derive::napi;

#[napi]
pub mod screenshots {
    use napi::bindgen_prelude::Error;
    use std::path::Path;

    /// Toggles whether the overlay handles screenshots when the user presses the screenshot hotkey,
    /// or if the game handles them.
    ///
    /// Hooking is disabled by default, and only ever enabled if you do so with this function.
    ///
    /// If hooking is enabled, then the ScreenshotRequested callback will be sent if the user presses
    /// the hotkey or when triggerScreenshot is called, and then the game is expected to call
    /// addScreenshotToLibrary in response.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamScreenshots#HookScreenshots}
    #[napi]
    pub fn hook_screenshots(hook: bool) {
        let client = crate::client::get_client();
        let screenshots = client.screenshots();
        screenshots.hook_screenshots(hook);
    }

    /// Checks if the app is hooking screenshots, or if the Steam Overlay is handling them.
    ///
    /// @returns true if the game is hooking screenshots and is expected to handle them; otherwise, false.
    ///
    /// {@link https://partner.steamgames.com/doc/api/ISteamScreenshots#IsScreenshotsHooked}
    #[napi]
    pub fn is_screenshots_hooked() -> bool {
        let client = crate::client::get_client();
        let screenshots = client.screenshots();
        screenshots.is_screenshots_hooked()
    }

    /// Either causes the Steam Overlay to take a screenshot, or tells your screenshot manager
    /// that a screenshot needs to be taken, depending on whether hooking is enabled.
    ///
    /// If hooking is disabled (default):
    /// - Steam overlay takes the screenshot automatically
    /// - Screenshot is saved to Steam's screenshot folder
    /// - Can be viewed in Steam > View > Screenshots
    ///
    /// If hooking is enabled via hookScreenshots(true):
    /// - A ScreenshotRequested callback is triggered
    /// - Your game must handle the screenshot by calling addScreenshotToLibrary
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
