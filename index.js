const { platform, arch } = process

/** @typedef {typeof import('./client.d')} Client */
/** @type {Client} */
let nativeBinding = undefined

if (platform === 'win32' && arch === 'x64') {
    nativeBinding = require('./dist/win64/steamworksjs.win32-x64-msvc.node')
} else if (platform === 'linux' && arch === 'x64') {
    nativeBinding = require('./dist/linux64/steamworksjs.linux-x64-gnu.node')
} else if (platform === 'darwin') {
    if (arch === 'x64') {
        nativeBinding = require('./dist/osx/steamworksjs.darwin-x64.node')
    } else if (arch === 'arm64') {
        nativeBinding = require('./dist/osx/steamworksjs.darwin-arm64.node')
    }
} else {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

let runCallbacksInterval = undefined
let steamOverlayInitialized = false

/**
 * Initialize the steam client or throw an error if it fails
 * @param {number} [appId] - App ID of the game to load, if undefined, will search for a steam_appid.txt file
 * @returns {Omit<Client, 'init' | 'runCallbacks'>}
*/
module.exports.init = (appId) => {
    const { init: internalInit, runCallbacks, restartAppIfNecessary, ...api } = nativeBinding

    internalInit(appId)

    clearInterval(runCallbacksInterval)
    runCallbacksInterval = setInterval(runCallbacks, 1000 / 30)

    return api
}

/**
 * @param {number} appId - App ID of the game to load
 * {@link https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary}
 * @returns {boolean}
 */
module.exports.restartAppIfNecessary = (appId) => nativeBinding.restartAppIfNecessary(appId);

/**
 * Enable the steam overlay on electron
 * @param {boolean} [disableEachFrameInvalidation] - Should attach a single pixel to be rendered each frame
*/
module.exports.electronEnableSteamOverlay = (disableEachFrameInvalidation) => {
    const electron = require('electron')
    if (!electron) {
        throw new Error('Electron module not found')
    }

    electron.app.commandLine.appendSwitch('in-process-gpu')
    electron.app.commandLine.appendSwitch('disable-direct-composition')

    if (!disableEachFrameInvalidation) {
        /** @param {electron.BrowserWindow} browserWindow */
        const attachFrameInvalidator = (browserWindow) => {
            browserWindow.steamworksRepaintInterval = setInterval(() => {
                if (browserWindow.isDestroyed()) {
                    clearInterval(browserWindow.steamworksRepaintInterval)
                } else if (!browserWindow.webContents.isPainting()) {
                    browserWindow.webContents.invalidate()
                }
            }, 1000 / 60)
        }

        electron.BrowserWindow.getAllWindows().forEach(attachFrameInvalidator)
        electron.app.on('browser-window-created', (_, bw) => attachFrameInvalidator(bw))
    }
}

module.exports.electronEnableSteamOverlay2 = (fpsLimit = 30) => {
    if (steamOverlayInitialized) {
        console.log('Steam overlay already initialized');
        return;
    }

    const electron = require('electron')
    if (!electron) {
        throw new Error('Electron module not found')
    }
    const app = electron.app

    // Check if app is ready
    if (!app.isReady()) {
        throw new Error('Electron app is not ready');
    }

    // Wrap switch appending in try-catch
    try {
        app.commandLine.appendSwitch('in-process-gpu');
        app.commandLine.appendSwitch('disable-direct-composition');
        app.commandLine.appendSwitch('disable-gpu-shader-disk-cache');
        app.commandLine.appendSwitch('disable-http-cache');
        app.commandLine.appendSwitch('use-angle', 'd3d11');
        app.commandLine.appendSwitch('disable-renderer-backgrounding');
        app.commandLine.appendSwitch('disable-background-timer-throttling');
    } catch (error) {
        console.error('Error appending command line switches:', error);
        throw error;
    }

    /** @param {electron.BrowserWindow} browserWindow */
    const attachOverlayInvalidator = (window) => {
        // Validate window and webContents existence
        if (!window || !window.webContents) {
            console.error('Invalid window or webContents');
            return;
        }

        const FPS_LIMIT = fpsLimit;

        try {
            const intervalId = setInterval(() => {
                try {
                    if (!window.isDestroyed() && window.isVisible()) {
                        // Invalidation "douce" sans forcer si le compositeur est déjà occupé
                        const { webContents } = window;
                        if (webContents && !webContents.isPainting()) {
                            webContents.invalidate();
                        }
                    }
                } catch (error) {
                    console.error('Error in overlay invalidator interval:', error);
                }
            }, 1000 / FPS_LIMIT);

            window.once('closed', () => {
                clearInterval(intervalId);
            });
        } catch (error) {
            console.error('Error setting up overlay invalidator:', error);
        }
    };

    try {
        electron.BrowserWindow.getAllWindows().forEach(attachOverlayInvalidator)
        electron.app.on('browser-window-created', (_, bw) => attachOverlayInvalidator(bw))
    } catch (error) {
        console.error('Error attaching overlay invalidator:', error);
        throw error;
    }

    steamOverlayInitialized = true;
}

const SteamCallback = nativeBinding.callback.SteamCallback
module.exports.SteamCallback = SteamCallback