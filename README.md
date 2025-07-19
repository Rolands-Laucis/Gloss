# Fast, minimal, clean, multi-lingual, cross-platform dictionary app

<img src="screenshot.png" height=300>

### Features
* Included languages: English (default), Latvian. 
More can be added easily by downloading the wordnet XML file in LMF format of a language, if available online, and parsing it with the included parser.
* Word definition, part-of-speech (POS), examples, synonyms, antonyms (very few available).
* Real-time reactive typing fuzzy search with match score.
* Dark and light mode
* Stared words list
* Recently searched words list
* Acrylic background and unintrusive design, so it blends and doesnt overpower your current actual task (like writing something).

### Tech stack and required programming languages on your machine
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

### Metrics
* App runtime takes about 200MB of RAM. 
* Built app folder takes up about 100MB of disk space.
* Starts up in about 1-1.5 seconds.
* Search bar is reactive and fuzzy fetches all results under a second, so real-time typing is not perfectly smooth, but satisfactory.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
