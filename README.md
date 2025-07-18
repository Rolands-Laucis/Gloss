# Fast, minimal, clean, multi-lingual, cross-platform dictionary app

<img src="screenshot.png" height=300>

Included languages: English (default), Latvian. 
More can be added easily by downloading the wordnet XML file in LMF format of a language, if available online, and parsing it with the included parser.

### Tech stack
* Tauri - backend
* Svelte 5 (with Vite 6 and SCSS) - frontend
* Rust ^1.88.x - used by Tauri
* Node.js ^22.x (or Deno or Bun) - used by frontend
* Python ^3.12.x - for the wordnet parser script

# Install/Run on Windows/Linux/Mac

```bash
npm run tauri build
```
This will produce an executable along with needed runtime files in .\src-tauri\target\release\
You can then move that folder wherever you like, and it will work. It just needs the resources folder next to the exe at runtime.
There might be differences for Linux and Mac, idk, i've never built for them.

I've set it up so that it makes as small of a build as possible, thus the bundle and installers are not created, but you can change that in tauri configs. Read the docs.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
