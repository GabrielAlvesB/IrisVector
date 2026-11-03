# IrisVector app icon

**Drawing:** a Bézier curve with its two anchor points (squares), their direction handles (lines
ending in round handles) and a selected point in the middle, as the Pen tool draws it. It is the
same drawing the Iris Workspace shows for IrisVector (`../Iris/src/shared/types/workspace.types.ts`,
entry `irisvector`).

**Palette:**

| Colour | Hex | Used for |
|---|---|---|
| Gold | `#fab900` | gradient start (top left), `oklch(0.82 0.2 88)` |
| Orange | `#ff7e00` | gradient end (bottom right), `oklch(0.76 0.21 50)` |
| Ink | `#2b2200` | curve, handles and outlines (dark: the gold is too light for white line work) |
| White | `#ffffff` | anchor points and handle ends |

**Tile:** `viewBox="0 0 512 512"`; the Workspace's 64-grid drawing scaled so its rounded tile
(`rx` 15 on 56) fills the canvas edge to edge. The macOS files add Apple's transparent margin
(tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Licence:** part of the Iris brand, see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `vectorcraft.svg` | master vector | source for everything below |
| `vectorcraft-small.svg` | copy of the master (kept so `packaging/icons.sh` and the DMG background keep working) | `hicolor/scalable` |
| `vectorcraft-1024.png` | full tile, 1024 px | docs, store listings |
| `vectorcraft-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/vectorcraft/src/main.rs`) |
| `vectorcraft.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `vectorcraft.ico` | Windows icon, 16 to 256 px | `apps/vectorcraft/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/com.iris.irisvector.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) and the update prompt | `packaging/linux/com.iris.irisvector.desktop` |
| `hicolor/scalable/apps/com.iris.irisvector.svg` | Linux scalable icon | as above |

## Regenerate

Edit `vectorcraft.svg`, copy it over `vectorcraft-small.svg`, then run `packaging/icons.sh` (needs
`resvg`; `iconutil` on macOS for the `.icns`). Without `resvg`, any SVG rasteriser works as long
as it writes the same files: on Windows the IrisVector icons were rendered with the Electron in
`../Iris/node_modules` (unset `ELECTRON_RUN_AS_NODE`), the `.ico` built with `cargo xtask ico` and
the `.icns` written from PNGs.
