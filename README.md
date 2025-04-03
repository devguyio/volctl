# volctl

A fast and minimal system volume controller written in **Rust** for Linux environments. Designed to be used with **i3** or other tiling window managers.

🚧 **Status:** Work in Progress — feel free to contribute or suggest ideas!

Created with ❤️ using **Vibe Coding**.

---

## ✨ Features
- Increase or decrease volume by percentage (supports floats)
- Toggle mute
- Sends OSD notifications using `notify-rust`
- Smart notification replacement (avoids stacking)
- Progress bar support using `value` hint
- Dynamic icons based on volume level
- Works with **PipeWire** via `wpctl`

---

## 🚀 Usage

```sh
volctl up 5        # Increase volume by 5%
volctl down 2.5    # Decrease volume by 2.5%
volctl mute        # Toggle mute
volctl up 10 --verbose  # Verbose output
```

---

## 🔧 Setup in i3

Add these lines to your `~/.config/i3/config`:

```i3
bindsym XF86AudioRaiseVolume exec --no-startup-id volctl up 5
bindsym XF86AudioLowerVolume exec --no-startup-id volctl down 5
bindsym XF86AudioMute exec --no-startup-id volctl mute
```

Then reload your i3 config with:
```sh
i3-msg reload && i3-msg restart
```

Make sure `volctl` is in your PATH. You can place the binary in `~/.local/bin/`:
```sh
cp target/release/volctl ~/.local/bin/
```

---

## 🎥 Demo

![volctl-demo](docs/assets/demo.gif)

---

## 📦 Dependencies
- [notify-rust](https://crates.io/crates/notify-rust)
- PipeWire (via `wpctl`)

Install requirements on Fedora:
```sh
sudo dnf install wireplumber libnotify
```

---

## 🧪 Development
```sh
cargo build --release
./target/release/volctl up 5
```

---

## 🤘 Made with Vibe Coding
Built for clean workflows, fast feedback, and a touch of flair ✨

---

## 📜 License
Apache-2.0 License

