# Pipelab's steamworks.js

> **Note**: This is an actively maintained community fork of [steamworks.js](https://github.com/ceifa/steamworks.js).
> We're keeping the project active by reviewing and merging pending PRs and addressing issues.

## Why This Fork?

- ✅ Actively reviewing and merging PRs
- ✅ Regular updates and bug fixes
- ✅ Community-driven development
- ✅ NW.js support improvements
- ✅ Incorporating pending PRs from upstream

This fork is maintained by [Pipelab](https://pipelab.app).

## Installation

```bash
# npm - use our scoped package
npm install @pipelab/steamworks.js

# Or reference directly from GitHub
npm install github:CynToolkit/steamworks.js

# For Electron/NW.js projects
npm install github:CynToolkit/steamworks.js --runtime=electron --target=27.0.0
npm install github:CynToolkit/steamworks.js --runtime=node-webkit --target=0.75.0
```

[![Build Status](https://github.com/CynToolkit/steamworks.js/actions/workflows/publish.yml/badge.svg)](https://github.com/CynToolkit/steamworks.js/actions/workflows/publish.yml)
[![npm](https://img.shields.io/npm/v/@pipelab/steamworks.js.svg)](https://npmjs.com/package/@pipelab/steamworks.js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

To make the steam overlay working, call the `electronEnableSteamOverlay` on the end of your `main.js` file:

```js
require('@pipelab/steamworks.js').electronEnableSteamOverlay()
```

For the production build, copy the relevant distro files from `sdk/redistributable_bin/{YOUR_DISTRO}` into the root of your build. If you are using electron-forge, look for [#75](https://github.com/CynToolkit/steamworks.js/issues/75).

# Steamworks.js

A modern implementation of the Steamworks SDK for HTML/JS and NodeJS based applications.

## API

```js
const steamworks = require('@pipelab/steamworks.js')

// You can pass an appId, or don't pass anything and use a steam_appid.txt file
const client = steamworks.init(480)

// Print Steam username
console.log(client.localplayer.getName())

// Tries to activate an achievement
if (client.achievement.activate('ACHIEVEMENT')) {
    // ...
}
```

You can refer to the [declarations file](https://github.com/CynToolkit/steamworks.js/blob/main/client.d.ts) to check the API support and get more detailed documentation of each function.


## How to build

> You **only** need to build if you are going to change something on steamworks.js code, if you are looking to just consume the library or use it in your game, refer to the [installation section](#installation).

Make sure you have the latest [node.js](https://nodejs.org/en/), [Rust](https://www.rust-lang.org/tools/install) and [Clang](https://rust-lang.github.io/rust-bindgen/requirements.html). We also need [Steam](https://store.steampowered.com/about/) installed and running.

Install dependencies with `npm install` and then run `npm run build:debug` to build the library.

There is no way to build for all targets easily. The good news is that you don't need to. You can develop and test on your current target, and open a PR. When the code is merged to main, a github action will build for all targets and publish a new version.

### Testing Electron

Go to the [test/electron](./test/electron) directory. There, you can run `npm install` and then `npm start` to run the Electron app.

Click "activate overlay" to test the overlay.
