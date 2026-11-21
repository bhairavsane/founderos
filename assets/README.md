# FounderOS — Icon & Brand Asset Kit

Drop-in icon bundle for **Tauri 2** (macOS, Windows, Linux), system tray / menu bar, and web.

## Directory Structure

- `src-tauri/icons/` — Copy directly into your Tauri 2 project's `src-tauri/icons/` folder.
  - `icon.icns` — macOS app bundle icon (HIG-padded squircle, 16px–1024px)
  - `icon.ico` — Windows multi-size icon (16, 24, 32, 48, 64, 128, 256)
  - `icon.png` — 1024×1024 master transparent PNG
  - `32x32.png`, `128x128.png`, `128x128@2x.png` — Linux & Tauri standard PNGs
  - `Square*Logo.png`, `StoreLogo.png` — Windows MSIX / Store tile assets
- `tray/` — Menu bar & system tray icons
  - `tray-template.svg`, `tray-template.png` (16×16), `tray-template@2x.png` (32×32) — macOS template icons (`iconAsTemplate: true`)
  - `tray-light.png`, `tray-light@2x.png` — White icons for dark Windows/Linux taskbars
- `svg/` — Editable vector marks and wordmark lockups for Figma and React UI
- `web/` — Favicons (`favicon.ico`, `apple-touch-icon.png`, Android/PWA PNGs)
- `tokens/design-tokens.json` — Brand colors and typography tokens

## Tauri 2 Configuration (`src-tauri/tauri.conf.json`)

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```
