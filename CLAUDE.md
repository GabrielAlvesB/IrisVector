# IrisVector — instructions for agents

## LEIA PRIMEIRO: o IrisVector é uma cópia modificada do VectorCraft

**Esta seção vale mais que todo o resto do arquivo (e que o AGENTS.md).** O resto veio do VectorCraft; onde ele contradizer esta seção, esta seção vence.

O IrisVector é uma cópia (fork) do [VectorCraft](https://github.com/storytold/vectorcraft), da equipe ArtCraft, adaptada para o ecossistema Iris por **Gabriel Alves Tech**. O repositório é `GabrielAlvesB/IrisVector` (remote `origin`); o VectorCraft original é o remote `upstream`, usado **só para trazer atualizações**: nunca envie nada para ele. O IrisWord (`../IrisWord`), o IrisGrid (`../IrisGrid`) e o IrisDeck (`../IrisDeck`) são as cópias irmãs do WordCraft, do GridCraft e do DeckCraft e seguem as mesmas regras.

Todos os ajustes listados abaixo foram feitos de propósito. **Nenhuma atualização vinda do VectorCraft pode desfazê-los.** Ao trazer uma versão nova, o trabalho é incorporar as melhorias do VectorCraft **mantendo o IrisVector como ele é**. Em qualquer conflito entre o código do VectorCraft e um ajuste do IrisVector, fica o do IrisVector. Na dúvida, pergunte ao dono antes de mudar.

### O que é do IrisVector e não pode ser revertido

1. **Nome visível "IrisVector"** em tudo que o usuário vê: título da janela, Sobre, menus, mensagens, traduções, arquivos gerados (`generator` do `.vectorcraft`, `Producer` de PDF, TIFF), README, instaladores, pacotes. "VectorCraft" só aparece nos créditos e avisos de copyright (`LICENSE-MIT`, `NOTICE`, a linha de copyright e "Based on VectorCraft" em `crates/ui-egui/src/dialogs/about.rs`, o comando `help.upstream` em `crates/engine/src/cmd/help.rs`, a seção de licença do README, `LegalCopyright` em `apps/vectorcraft/build.rs` e `packaging/macos/Info.plist.in`, as frases "VectorCraft contributors" / "VectorCraft authors" e as linhas de autoria do `ASSETS.md`); esses avisos são exigidos pela licença e também não podem ser removidos.
2. **Marcadores de formato mantidos com o nome antigo** (compatibilidade de arquivo, não é marca): `%VectorCraft_BeginData` / `%VectorCraft_EndData` (`crates/eps/src/lib.rs`), o procset `VectorCraft_Legacy` (`crates/eps/src/legacy_ai/prolog.rs`) e o formato `"format": "vectorcraft"` / extensão `.vectorcraft`. Trocar quebraria a abertura de arquivos salvos pelo VectorCraft e pelo próprio IrisVector. Os documentos de `examples/` também ficam como vieram.
3. **ID do app `com.iris.irisvector`** (nunca `ai.storyteller.vectorcraft`): `packaging/linux/com.iris.irisvector.*`, `packaging/linux/flatpak/com.iris.irisvector*.yml`, `packaging/macos/Info.plist.in`, os ícones em `assets/app-icon/hicolor/*/apps/`, `xtask/src/bundle.rs`.
4. **Ícone do IrisVector** (curva de Bézier com os pontos de âncora e as alças, como a caneta desenha, gradiente `#fab900` → `#ff7e00`, o mesmo desenho do Workspace do Iris, matiz 88): a fonte é `assets/app-icon/vectorcraft.svg` (cópia em `vectorcraft-small.svg`); todo PNG, `.ico` e `.icns` de `assets/app-icon/` é gerado dela (veja `assets/app-icon/README.md`). O logotipo da barra do app, do Sobre e do aviso de atualização é o PNG de 128 px (`crates/ui-egui/src/brand.rs`). Nunca aceite ícones do VectorCraft (o dragão) por cima destes.
5. **Sem marca ArtCraft**: sem `docs/brand/`, sem logos ArtCraft, sem links para `getartcraft.com` ou `discord.gg/artcraft`, sem botão ou item de Discord (removidos da barra do app, do Sobre, da tela inicial, do menu Ajuda, do menu do app no macOS e das traduções), sem a lista "Crafting Apps" no README. `crates/engine/src/cmd/help.rs` só tem `help.github` (IrisVector) e `help.upstream` (crédito ao original). Se uma atualização trouxer menções novas à ArtCraft ou à suíte Crafting Apps na interface, retire-as. A licença de marca da ArtCraft exige isso de forks. (A busca do PhotoCraft instalado em `ArtCraft\PhotoCraft`, usada por "Editar original", é um caminho de instalação e fica.)
6. **Atualizador** (`UPDATE_APP` em `crates/ui-egui/src/lib.rs`): `name: "IrisVector"`, `repo: "GabrielAlvesB/IrisVector"` (exatamente assim: o atualizador compara os links diferenciando maiúsculas), `developer: "Gabriel Alves Tech"`, `accent` `#e07000`. Nunca volte para `craft_update::suite::VECTORCRAFT` puro: o app passaria a baixar o VectorCraft por cima do IrisVector.
7. **Instalador do Windows** (`packaging/windows/`): `UpgradeCode` `6E81D984-F442-493F-A31A-0FF14C2D6953` (em `vectorcraft.wxs` e `install-policy.json`), GUID do componente `VectorcraftApp` `901E7A39-20D3-4CE0-B34D-0ED380BF9422` e do atalho da área de trabalho `867FB674-5D1A-4313-8B5F-69042A9FBF2B` (nunca os do VectorCraft, senão um instalador substitui o outro), fabricante `Gabriel Alves Tech`, pasta `Programs\Iris\IrisVector`, chaves de registro `Software\IrisVector`, links de ajuda para `GabrielAlvesB/IrisVector`. Em `apps/vectorcraft/build.rs`, `CompanyName` é `Gabriel Alves Tech`.
8. **Pastas de dados próprias**: no Windows `%APPDATA%\IrisVector`, no macOS `Application Support/IrisVector` e a pasta portátil `IrisVectorData` (preferências, recuperação, fontes, bibliotecas), para não misturar com um VectorCraft instalado. No Linux a pasta continua `~/.config/vectorcraft` (nome interno).
9. **CI próprio**: `.github/workflows/ci.yml` é autocontido (sem segredos, token só de leitura, actions fixadas por SHA). **Nunca restaure** os workflows do VectorCraft (`storytold/craft-common-actions@...`, SonarCloud, release, FreeBSD, nix, packaging-lint, print, affinity-corpus, windows-arm64), nem `sonar-project.properties` ou `.coderabbit.yaml`: eles rodam código de outra organização e repassam segredos. O Dependabot roda semanalmente (`America/Sao_Paulo`). As instruções de CI mais abaixo neste arquivo são do VectorCraft e **não valem** aqui.
10. **Segurança**: `tokio` 1.54.1 ou mais nova (`Cargo.lock`; a 1.54.0 tem a RUSTSEC-2026-0384 e foi retirada do crates.io) e `zip` 9 ou mais nova. Não volte para versões antigas. `cargo audit` precisa terminar sem vulnerabilidades (os avisos de crates "unmaintained" são aceitos).
11. **Nomes internos mantidos de propósito**: crates `vectorcraft-*`, binários `vectorcraft.exe` e `vectorcraft-cli`, pastas `apps/vectorcraft*`, arquivos `assets/app-icon/vectorcraft.*`, `packaging/windows/vectorcraft.wxs`, a extensão `.vectorcraft`, variáveis `VECTORCRAFT_*`, identificadores Rust (`VectorcraftApp`) e os nomes dos arquivos de release (`vectorcraft-<versão>-windows-<arq>.msi`, `...-portable.zip`). O atualizador e o Workspace do Iris (`../Iris/src/shared/types/workspace.types.ts`) procuram exatamente esses nomes. Não renomeie.

### Pendências conhecidas

- O fundo do DMG do macOS (`packaging/macos/dmg/background.tiff`) ainda tem o ícone antigo embutido; regenere com `packaging/macos/dmg/generate.py` num Mac antes de publicar uma versão para macOS.

### Como trazer uma atualização do VectorCraft

1. `git fetch upstream --tags` e escolha a tag nova (por exemplo `v0.22.0`). Faça o merge numa branch: `git switch -c update-vectorcraft-0.22.0 && git merge v0.22.0`.
2. Resolva os conflitos mantendo **tudo** da lista acima. Arquivos removidos no IrisVector (`docs/brand/`, workflows antigos, `sonar-project.properties`, `.coderabbit.yaml`) continuam removidos.
3. Procure o que entrou de novo com a marca antiga e adapte: `git grep -n -e VectorCraft -e ArtCraft -e getartcraft -e discord -e ai.storyteller -e "storytold/vectorcraft" -e "Learning Machines"`. Texto visível novo vira IrisVector (nas chaves **e** nas traduções dos `.tsv` em `crates/ui-egui/src/i18n/`, senão a tradução deixa de casar); avisos de copyright, créditos e os marcadores de formato do item 2 ficam como estão.
4. Arquivos novos em `.github/workflows/` vindos do VectorCraft: apague. Ícones ou imagens novas de marca: não adote.
5. Se a revisão de `craft-libs` mudou, confira se `craft_update::app::App` ainda aceita os campos do `UPDATE_APP`.
6. Rode `cargo audit`, `cargo xtask assets`, `cargo xtask brands` e os testes (`cargo test --workspace --no-fail-fast`). Nesta máquina, use `CARGO_BUILD_JOBS=4` (com o paralelismo padrão o Windows fica sem memória virtual, os error 1455).
7. **Versão**: a versão do IrisVector (`[workspace.package]` no `Cargo.toml`) precisa ser **maior que a da última release do IrisVector**, senão o atualizador não oferece a nova. Rode `cargo update -w` depois de mudar.
8. Mostre ao dono o que mudou antes de fazer merge na `main` e de publicar release.

### Como publicar uma release

1. Versão nova no `Cargo.toml`, `cargo update -w`, commit e push na `main`.
2. Rode `packaging/windows/package.ps1 -Arch x64` num processo `powershell.exe` separado, com a saída indo para arquivo (no PowerShell 5.1, redirecionar dentro do mesmo processo transforma avisos do cargo em erro fatal). Precisa do WiX 5 (`dotnet tool install --global wix --version 5.0.2`). Saída em `dist/release/`.
3. Gere `dist/release/SHA256SUMS.txt` no formato `hash  nome` (dois espaços, sem `*`). O Workspace do Iris não instala sem ele.
4. `gh release create v<versão> -R GabrielAlvesB/IrisVector --title "IrisVector v<versão>" --notes-file <notas.md> --latest` com o `.msi`, o `-portable.zip` e o `SHA256SUMS.txt`. Notas em português.
5. Os builds não têm assinatura Authenticode, então o próprio IrisVector só avisa da versão nova e abre a página da release; a instalação automática acontece pelo Workspace do Iris.

IrisVector is a clean-room, open-source, Rust-native vector illustration app targeting Adobe Illustrator parity (and beyond). It runs natively on macOS, Windows and Linux, and on the web via WASM. Siblings: `../photocraft` (Photoshop-class) and `../pdfcraft` (Acrobat-class); same conventions.

Formerly **DrawCraft** (renamed 2026-10-01): old `.drawcraft` files and `"format": "drawcraft"` headers still open (`vectorcraft_format::LEGACY_EXTENSION`), and preferences migrate from the old config folder. Keep those paths working; use the new name everywhere else.

## Start every session here
1. Read [`ROADMAP.md`](ROADMAP.md) (stage, numbers by dimension, what's next) and the ranked work list in [`docs/gaps.md`](docs/gaps.md). Unless the user gives you a task, pick work from that list.
2. Read `plan/STATUS.md` (local session notes, may lag the ROADMAP), then the task in `plan/execution-plan.md` §3 and the relevant `plan/architecture.md` section. Behaviour reference: public documentation only (see Clean-room below); `plan/illustrator/*.md` holds notes from it.
3. Follow the autonomous operation protocol (`plan/execution-plan.md` §7): orient → plan → implement + test → verify → record → commit. Don't stop to ask unless §7 lists the decision as the user's.

`plan/` is gitignored (local only).

## Non-negotiables
- **Clean-room: never use Adobe software (absolute rule, from the project owner).** Adobe's Terms of Use forbid reverse engineering, including observing a program's inputs and outputs to recreate it. So:
  - Never launch, run, script, automate (COM, AppleScript, ExtendScript, UI automation), screenshot or measure Illustrator or any other Adobe application, and never compare our output against one. Don't make test files with them, not even "synthetic" ones.
  - Never read, disassemble or copy anything inside an Adobe install (binaries, plug-ins, presets, procsets, resources, help files, fonts, ICC profiles).
  - Never copy Adobe-authored code or text into this repo, tests included: no PostScript procsets or prologs (AGM, CoolType, the Illustrator prologs), no sample files. Write test inputs from scratch from the public specifications (PostScript Language Reference, PDF / ISO 32000, TIFF, EPSF/DCS).
  - Behaviour comes only from public documentation (helpx, published specs, public forum posts), cited by URL, and from files users send us that they own (used locally, never committed). Never copy Adobe icons, artwork, presets or wording beyond feature names.
  - If a task seems to need any of this, stop and say so instead. `cargo xtask cleanroom` (run by `cargo xtask ci`) fails on Adobe copyright notices and on fixtures made with Adobe applications.
  - Never copy GPL/AGPL code (Inkscape, lib2geom…).
- **Assets: no Adobe iconography or images — ever (absolute rule, from the project owner).**
  - Never add, copy, trace, redraw-from, embed or ship any icon, image, artwork, cursor, preset, swatch/brush/symbol/pattern/style library, ICC profile or screenshot from Adobe products or from any other source whose licence doesn't allow it.
  - Every image, icon, font or other asset must be one of:
    - original work created for IrisVector by a contributor (who licenses it MIT OR Apache-2.0);
    - OSI open source;
    - public domain / CC0;
    - Creative Commons with redistribution allowed.
  - **Every asset file must have a row in [`ASSETS.md`](ASSETS.md)** (path, author, source URL, licence, notes). Put licence texts next to the assets (e.g. `assets/fonts/OFL-*.txt`) and summarize them in `NOTICE`.
  - `cargo xtask assets` (run by `cargo xtask ci`) fails on any unattributed asset.
  - **Fonts live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts), never in this repo**. Never commit a font file here; add new fonts to craft-fonts. The app uses it only through the optional `CRAFT_FONTS_DIR` build option (`CRAFT_FONTS_DIR="$PWD/../craft-fonts" cargo xtask ci`; use an absolute path), read by `crates/text/build.rs` into `vectorcraft_text::CRAFT_FONTS`; never add it to a `Cargo.toml`. Everything must build, test and run with `CRAFT_FONTS` empty, and tests of its glyphs skip when it is. Details: `docs/development.md` › Fonts.
  - Code and original assets are MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`). The Iris name and icon (`assets/app-icon/`) belong to Gabriel Alves Tech and are not open source. No ArtCraft logos or brand files: this fork removed them, never add them back.
  - Prefer art generated in code for defaults (swatches, brushes, symbols, patterns, cursors). If the provenance of an asset is unclear, don't add it.
  - Reference screenshots of Illustrator stay local under the gitignored `plan/` and are never committed, published or used as assets.
- **Never crash: no panics in shipped code.** A crash loses the user's unsaved work. Files, pasted data, MCP and control messages, and command params are all untrusted input.
  - Fallible work returns `Result` (or `Option`); callers handle it or propagate it with `?`. Failures reach the user as an error message (`EngineError` for commands, the crate's own error type or `String` below the engine), never as an abort.
  - Workspace clippy lints ban these outside tests, so `cargo xtask ci` fails on them: `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`. Use instead: `?`, `.ok_or(…)?`, `let … else { return … }`, `if let`, `unwrap_or` / `unwrap_or_default` / `unwrap_or_else`, `.first()` / `.last()` / `.split_last()`, and a real error for a match arm you believe is impossible.
  - New workspace crates must include `[lints] workspace = true` in `Cargo.toml` to inherit `[workspace.lints]`. Production crate roots must declare `#![forbid(unsafe_code)]` before modules or other crate items; crate-level `//!` documentation may precede it.
  - Don't panic implicitly either: use `.get(i)` instead of `v[i]` or `&s[a..b]` when the index comes from data. Don't divide integers by something that can be zero, cap sizes and counts read from input (allocations, images, loops), and check that floats are finite before casting them or looping on them.
  - Don't silence errors just to satisfy the lint: use `let _ =` or `.ok()` only where ignoring the failure is the right behaviour, with a comment saying why.
  - Tests may unwrap and panic (`#[test]`, `#[cfg(test)]`, `tests/`, `testkit`). Integration-test files start with an `#![allow(…)]` for that.
  - `vectorcraft_engine::guard` is the last line of defence. Every entry point (commands, tool events, MCP requests, the UI frame, control requests) catches a panic, rolls the document back and reports an internal error. It exists for bugs and doesn't replace `Result`: wasm aborts on panic, so the web app has no safety net.
  - Untrusted input is fuzzed (`engine/tests/import_fuzz.rs`, `engine/tests/command_sweep.rs`, `format/tests/prop_format.rs`, `mcp/tests/protocol_props.rs`). Extend these when you add an importer, a parser or a protocol method.
- **Everything is a command.** User-visible behaviour = a command in `crates/engine/src/cmd/*` (id, label, menu path, shortcut, params doc, `enabled`, `run`) + tests. Tools emit commands (Begin/Preview/Commit). UI-only commands live in `crates/ui-egui/src/menus.rs` (`UI_COMMANDS`). The control channel and MCP reach all of them.
- **Adding a cursor.** The app decides which cursors exist; whoever draws them works from the list. For a new tool state with a cursor of its own: add the `Cursor` variant (`crates/tools/src/lib.rs`, with a doc comment), return it from the tool's `cursor()` with a tool test, cover it in `cursor_icon` (`crates/ui-egui/src/canvas.rs`), draw it in `crates/ui-egui/src/cursors.rs` or give it a stand-in in `Cursor::stand_in()` until it's drawn, and add its `[[cursor]]` entry to `crates/tools/src/cursors.toml` (when it shows, in plain behaviour terms, and its hotspot; never another app's look). A missing entry only warns (`cargo xtask cursors`): the cursor still works.
- **Layering** is enforced by `cargo xtask layers`. Nothing below L6 depends on egui/eframe/winit/rfd.
- **The UI is thin**: panels read engine state and act through `app.run(id, params)`. Colours come from `theme::Tokens`.
- **Rust only** (no handwritten JS/TS). **Never break wasm** (`cargo xtask wasm`).
- **Time goes through `chrono`** (workspace dependency). Never hand-roll UTC/calendar conversion (days-to-date algorithms, leap-year math); a crate owns only its output format string. The one exception is range extension: past the years chrono can represent, shift the date by whole 400-year Gregorian cycles (146,097 days, after which the calendar repeats), let chrono compute the month and day, and add the cycles back to the year (splitting seconds into hours, minutes and seconds is plain arithmetic).
- **Shared test corpora.** Real-file test oracles (Photoshop-authored PSDs, etc.) live in [`storytold/photocraft-corpus`](https://github.com/storytold/photocraft-corpus). Never commit large binary fixtures to this repo; fetch them pinned by commit and sha256-verified, as PhotoCraft does with `cargo xtask corpus`.
- **Quality gates** before every commit: `cargo xtask ci` (fmt, clippy -D warnings including the no-panic lints, tests, layers, wasm). One task id per commit (`M2.1: pen tool`).

## Running and looking at the app
- `cargo run --release -p vectorcraft -- --control 7979 [file.svg|file.vectorcraft]` (sibling apps' agents use the same default port: if the log says it failed to bind, pick another port — otherwise your requests reach a different app).
- Drive it: JSON lines on `127.0.0.1:7979`, e.g. `{"id":1,"method":"engine.execute","params":{"command":"shape.rectangle","params":{"x":10,"y":10,"width":100,"height":50}}}` then `{"id":2,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}`. Methods: `crates/ui-egui/src/control.rs`.
- **For UI work, look at the result** (take `ui.screenshot`, read the PNG) and compare with the public documentation (`plan/illustrator/02-ui-ux.md` notes). `ui.screenshot` needs a presented frame: if it errors (screen locked), check the art with `ui.render` / `vectorcraft-cli run … --export x.png`, and cover panels with a headless egui frame test (see `panels/transparency.rs` tests).
- **Performance:** `vectorcraft-cli perf` checks the budgets (render/pan/zoom, hit test, save/open, SVG, Pathfinder) on a synthetic 50k-path document; `vectorcraft-cli bench FILE` times one file. Run them before and after renderer, format or geometry changes, on an idle machine (the report warns when the load average makes timings noise).
- MCP: `vectorcraft-cli mcp` (see `docs/mcp.md`).
- Shell gotcha: `mv`/`cp` are aliased interactive here — use `/bin/mv -f` / `/bin/cp -f`.
- Parallel agents: separate `CARGO_TARGET_DIR` per agent; edit only the crates you own; write manifests atomically. Each target dir grows to ~30 GB: delete yours when you finish (a full disk fails links with `errno=28`).

## Roadmap and progress docs
The progress docs:
- [`ROADMAP.md`](ROADMAP.md): stage, headline numbers, dimensions, features, languages, upcoming, progress log.
- [`docs/target-app-parity.md`](docs/target-app-parity.md): the authoritative parity assessment (area table, methodology, calibration, evidence of what ships).
- [`docs/gaps.md`](docs/gaps.md): every known shortfall, ranked; the work list. [`docs/roadmap.md`](docs/roadmap.md): milestones and current focus.
- Checklists: [`ui-parity.md`](docs/ui-parity.md), [`file-format-parity.md`](docs/file-format-parity.md), [`hardware-parity.md`](docs/hardware-parity.md), [`localization-parity.md`](docs/localization-parity.md), [`effects-parity.md`](docs/effects-parity.md), [`type-parity.md`](docs/type-parity.md); [`architecture.md`](docs/architecture.md).

When a task lands, update in the same PR: the area row in `docs/target-app-parity.md` (score, missing items, hours) and its evidence list, the gap entry in `docs/gaps.md` if it closed or shrank, the checklist it belongs to, the milestone row in `docs/roadmap.md`, and the ROADMAP's tables and progress log when a headline number moves. Update each touched doc's status line and revision history.
- Grade by behaviour against the public documentation (`plan/illustrator/` notes), not by whether a menu item exists. Scores are self-assessed, so err low; mark each number measured or estimated.
- Keep the README's Status section in step with the ROADMAP headline.

## CI workflows
- Workflows in `.github/workflows/` call the shared workflows in [`storytold/craft-common-actions`](https://github.com/storytold/craft-common-actions) at `@main`, with their inputs and secrets. No other jobs: if no shared workflow does what's needed, it doesn't go here.
- Don't add or change CI in a feature or fix PR. If the existing workflows can't build or test a change, link an existing craft-common-actions issue or PR, or open a new issue, or a PR only for a small fix that is safe for every app and does not increase CI time.
- Some workflow files here still run their own jobs. Don't add to them. Moving a job into craft-common-actions is its own task, never part of another PR, and the job stays here until a shared workflow runs it.
