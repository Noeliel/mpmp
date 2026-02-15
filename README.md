# mpmp – Multi‑Player Media‑Player

Synchronize your binge‑watching, jam‑session, or epic movie‑night across any number of devices—without the awkward "who's doing the countdown?" debate.

> "We wanted to watch the same video at the same time, but our friends kept hitting pause on their own Netflix accounts. So we built mpmp. Now the only thing out of sync is our popcorn‑eating speed."

mpmp (pronounced "em‑pee‑em‑pee") is a client‑server playback‑synchronisation framework written in Rust.

* Server – runs anywhere (your laptop, a cloud VM, a Raspberry Pi) and keeps the master timeline.
* Client – talks to the server and drives the media player via a plugin, ensuring every participant sees the exact same frame at the exact same time.

Think of it as a digital "watch‑together" couch, but the couch can be on three continents at once.

## ✨ Features

- **Playback synchronization** – Automatic propagation of seek, play/pause, and speed changes between all clients.
- **MPV client implementation** – A ready‑to‑use client implementation ("mpmpv") that interacts with MPV via its C API.
- **Lightweight protocol** – Client and server only communicate what is necessary to keep the playback state in sync. No exchange on the media file itself and no periodic heartbeats. 
- **Dynamic ephemeral lobby** – The first client initializes the lobby in a flying-start manner. No explicit lobby creation or extra steps required, just straight to action. Five minutes late to movie night? No problem! Clients can join and leave the lobby at any time and are automatically synchronized to the lobby-wide playback state.
- **Hand-written code** – Zero AI generated code (with the sole exception of this very readme!)

## 🛠️ Building mpmp

### 1. Clone the repo

```sh
git clone https://github.com/yourusername/mpmp.git
cd mpmp
```

### 2. Build everything (server + client + mpv plugin)

Make sure you have `llvm` installed (required by bindgen for generating Rust bindings for the MPV C plugin headers.)  
Then, run:

```sh
cargo build --release
```

That's it—no exotic toolchains, no hidden Makefiles, just plain‑vanilla Cargo.
The resulting binaries will appear under `target/release/`.

### 3. Install the MPV plugin

After building, copy the plugin into mpv's script directory:

Linux:
```sh
mkdir -p ~/.config/mpv/scripts
cp target/release/libmpmpv.so ~/.config/mpv/scripts/
```

### Notes on Windows

See win/ directory for additional (possibly) required files and instructions.  
The code will build for Windows but is untested.  
Crucial parts of the server are not implemented with Windows support, so only the client *might* work.  
To install, drop the .dll into `%APPDATA%\mpv\scripts`.  

### Notes on macOS

macOS is not currently supported.

## 🏎️💨 Running

Run the server:
```sh
MPMP_HOST=<hostname:port> server
```

Run a client:
```sh
MPMP_HOST=<hostname:port> mpv <path/to/media/file>
```

Note: If you do not provide a server host when launching mpv with mpmpv installed, it will effectively be disabled!

## 🤝 Contributing

We love contributors! Whether you're fixing a typo, improving documentation, or tweaking something else, feel free to:

1. Fork the repository.  
2. Create a feature branch (`git checkout -b awesome-improvement`).  
3. Open a Pull Request with a clear description of what you've done.

## 📜 License

mpmp is released under the GPL‑2.0‑only license.  
SPDX identifier: [GPL-2.0-only](https://spdx.org/licenses/GPL-2.0-only.html)  
Copyright © 2026 Noeliel / The mpmp Contributors

## ❤️ Acknowledgements

Thanks to the MPV developers for their excellent media player and the Rust community for their language, tooling and libraries.

## 🎉 Happy Sync‑ing!

May your frames stay locked, your buffers stay full, and your friends never argue about who hit "play" first.
