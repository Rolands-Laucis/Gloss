<script>
  import { invoke } from "@tauri-apps/api/core";

  let search = $state("copper"), suggestions = $state([]);//Lexicon
  let dark_mode = true; 

  async function Query(input = "") {
    const res = await invoke("search_words", {
      query: input,
      limit: 15,
    });
    suggestions = Array.from(new Set(res.map(e => e.word)));

    const entries = [];
    for (const e of res) {
      if (e.score < 60) continue;

      e.word = e.word.replaceAll("_", " ");
      e.synonyms = e.synonyms.filter(s => (s.match(/_/g) || []).length < 2).map(s => s.replace("_", " "));
      
      const existing = entries.find((entry) => e.word === entry.word);
      if (existing) {
        existing.senses.push(e);
      } else {
        entries.push({
          word: e.word,
          senses: [e]
        });
      }
    }

    entries.sort((a, b) => {
      //exact match first
      if(a.word === search) return -1; 
      else if(b.word === search) return 1;

      // Sort by space count (ascending)
      const spaceCountA = (a.word.match(/ /g) || []).length;
      const spaceCountB = (b.word.match(/ /g) || []).length;
      console.log(a.word, spaceCountA, b.word, spaceCountB);
      if (spaceCountA !== spaceCountB) 
        return spaceCountA > spaceCountB ? 1 : -1; 

      // Sort by score (descending)
      const maxScoreA = Math.max(...a.senses.map(s => s.score));
      const maxScoreB = Math.max(...b.senses.map(s => s.score));
      if(maxScoreA !== maxScoreB)
        return maxScoreB > maxScoreA ? 1 : -1; 

      // otherwise alphabetically
      return b.word.localeCompare(a.word);
    });

    return entries;
  }

  function ChangeTheme() {
    dark_mode = !dark_mode;
    document.documentElement.setAttribute(
      "theme",
      dark_mode === true ? "dark" : "light",
    );
  }
</script>

<main>
  <nav>
    <input
      autofocus
      autocapitalize="off"
      autocorrect="on"
      autocomplete="on"
      type="text"
      name="search"
      placeholder="search..."
      list="suggestions"
      bind:value={search}
    />

    <datalist id="suggestions" hidden>
      {#each suggestions as s}
        {#if s !== search}
          <option value={s}></option>
        {/if}
      {/each}
    </datalist>

    <button on:click={ChangeTheme}></button>
  </nav>

  <aside>
    {#if search}
      {#await Query(search)}
        <small>searching...</small>
      {:then entries}
        {#each entries as e}
          <article>
            <h1 class="word">{e.word}</h1>

            {#each e.senses as sense}
              <section>
                <small>{sense.pos} ~{sense.score}</small>
                <p>{sense.definition}</p>
                {#if sense.example}
                  <p class="example">“{sense.example}”</p>
                {/if}
              </section>

              <section>
                {#if sense.synonyms.length}
                  <!-- <h2>synonyms</h2> -->
                  <div class="word_list">
                    {#each sense.synonyms as syn}
                      <!-- svelte-ignore a11y_click_events_have_key_events -->
                      <!-- svelte-ignore a11y_no_static_element_interactions -->
                      <!-- svelte-ignore a11y_missing_attribute -->
                      <!-- svelte-ignore event_directive_deprecated -->
                      <a on:click|preventDefault={() => (search = syn)}
                        >{syn}</a
                      >
                    {/each}
                  </div>
                {/if}
              </section>
            {/each}

            <div class="line"></div>
          </article>

        {/each}
      {:catch error}
        <li>Error: {error}</li>
      {/await}
    {/if}
  </aside>
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    max-height: 100%;
    position: relative;

    display: flex;
    flex-direction: column;
  }
  nav {
    display: flex;
    gap: var(--s-1);
    align-items: center;
    margin-bottom: var(--s-1);
  }
  input {
    flex-grow: 1;
    padding: var(--s-01);

    background: transparent;
    border-radius: 4px;
    border: 2px solid var(--g4);
  }
  nav > button {
    height: var(--s-3);
    aspect-ratio: 1 / 1;
    background: var(--g1);
    border-radius: 2px;
  }
  nav > button:hover {
    background-color: var(--l);
  }
  aside {
    width: 100%;
    max-width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: var(--s-3);
    padding-bottom: var(--s-4);

    overflow-y: auto;

    mask-image: linear-gradient(to bottom, black 90%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, black 90%, transparent 100%);
  }
  article {
    width: 100%;
    max-width: 100%;
    display: flex;
    flex-direction: column;
  }
  /* fake a flex gap, so that it can be overwritten */
  article > *:not(:last-child) {
    margin-bottom: var(--s-1);
  }
  article > h1 {
    margin-bottom: var(--s-0) !important;
  }
  .word {
    margin-bottom: var(--s-03);
  }
  section {
    width: 100%;
    max-width: 100%;

    display: flex;
    flex-direction: column;
    gap: var(--s-03);
  }
  section > p, .word_list {
    margin-left: var(--s-2);
  }
  section > p::after {
    content: ".";
  }
  .example {
    font-style: italic;
    color: var(--g1);
  }
  .word_list {
    max-width: 100%;
    display: flex;
    gap: var(--s-01);
    text-transform: lowercase;
    flex-wrap: wrap;
  }
  .word_list a:not(:last-child)::after {
    content: ",";
  }
  .line{
    height: 1px;
    width: 100%;
    background: var(--g5);
  }
</style>
