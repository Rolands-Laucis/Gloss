<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import Icon from "./Icon.svelte";

    let search = $state(""),
        suggestions = $state([]); //Lexicon
    let dark_mode = $state(true);
    let stars = $state([]);
    let recents = $state([]);

    onMount(() => {
        stars = JSON.parse(localStorage.getItem("stars") || "[]");
        recents = JSON.parse(localStorage.getItem("recents") || "[]");

        window.addEventListener('keydown', KeyDown)
    });

    function KeyDown(e){
        console.log(e);
        
        // pressing enter on the search bar will confirm it as the search term, so it can be stored in recents and 
        const search_box = document.getElementById('search');
        if(e.key === 'Enter' && e.target === search_box){
            search_box?.blur();
            recents = Array.from(new Set([e.target.value, ...recents]));
            if(recents.length > 25) recents.length = 200;
            localStorage.setItem("recents", JSON.stringify(recents));
            // console.log({recents});
        }
        else if(e.key === 'Tab' && e.target !== search_box){
            e.preventDefault();
            search_box?.focus();
        }
    }

    async function Query(input = "") {
        const res = await invoke("search_words", {
            query: input,
            limit: 15,
        });

        const entries = [];
        for (const e of res) {
            if (e.score < 60) continue;

            e.word = e.word.replaceAll("_", " ");
            e.synonyms = e.synonyms
                .filter((s) => (s.match(/_/g) || []).length < 2)
                .map((s) => s.replace("_", " "));

            const existing = entries.find((entry) => e.word === entry.word);
            if (existing) {
                existing.senses.push(e);
            } else {
                entries.push({
                    word: e.word,
                    senses: [e],
                });
            }
        }

        entries.sort((a, b) => {
            //exact match first
            if (a.word === search) return -1;
            else if (b.word === search) return 1;

            // Sort by space count (ascending)
            const spaceCountA = (a.word.match(/ /g) || []).length;
            const spaceCountB = (b.word.match(/ /g) || []).length;
            console.log(a.word, spaceCountA, b.word, spaceCountB);
            if (spaceCountA !== spaceCountB)
                return spaceCountA > spaceCountB ? 1 : -1;

            // Sort by score (descending)
            const maxScoreA = Math.max(...a.senses.map((s) => s.score));
            const maxScoreB = Math.max(...b.senses.map((s) => s.score));
            if (maxScoreA !== maxScoreB) return maxScoreB > maxScoreA ? 1 : -1;

            // otherwise alphabetically
            return b.word.localeCompare(a.word);
        });

        suggestions = Array.from(new Set(entries.map(e => e.word))); //.filter(w => w !== search)

        return entries;
    }

    function ChangeTheme() {
        dark_mode = !dark_mode;
        document.documentElement.setAttribute(
            "theme",
            dark_mode === true ? "dark" : "light",
        );
    }

    // toggles starred words
    function Star(word = "") {
        // if already starred, remove it
        if (stars.includes(word)) stars.splice(stars.indexOf(word), 1);
        // otherwise add it
        else stars.push(word);
        // persist to localStorage
        localStorage.setItem("stars", JSON.stringify(stars));
        // console.log({stars});
    }
</script>

<main>
    <nav>
        <input
            autofocus
            autocapitalize="off"
            autocorrect="on"
            autocomplete="off"
            type="text"
            name="search"
            id="search"
            placeholder="search..."
            list="suggestions"
            bind:value={search}
        />

        <!-- dont show if its just 1 word which is the search word -->
        {#if !(suggestions.length == 1 && suggestions[0] === search)}
            <datalist id="suggestions" hidden>
                {#each suggestions as s}
                    <option value={s}></option>
                {/each}
            </datalist>
        {/if}

        <button on:click={ChangeTheme}>
            <Icon fill={1} inert size={20} style="color: var(--d);">
                contrast
                <!-- {!dark_mode ? 'light_mode' : 'dark_mode'} -->
            </Icon>
        </button>
    </nav>

    <aside>
        {#if search}
            {#await Query(search)}
                <small>searching...</small>
            {:then entries}
                {#each entries as e}
                    {@const starred = stars.includes(e.word)}
                    <article>
                        <div class="title">
                            <h1 class="word">{e.word}</h1>

                            <Icon
                                fill={starred ? 1 : 0}
                                wgth={starred ? 400 : 200}
                                size={28}
                                on:click={() => {
                                    Star(e.word);
                                }}
                                style="color: var({starred ? '--g1' : '--g4'})"
                            >
                                star
                            </Icon>
                        </div>

                        {#each e.senses as sense}
                            <section>
                                <small>{sense.pos} ~{sense.score}</small>
                                <p>{sense.definition}</p>
                                {#if sense.example}
                                    <p class="example">“{sense.example}”</p>
                                {/if}

                                {#if sense.synonyms.length}
                                    <!-- <h2>synonyms</h2> -->
                                    <div class="word_list">
                                        {#each sense.synonyms as syn}
                                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                                            <!-- svelte-ignore a11y_missing_attribute -->
                                            <!-- svelte-ignore event_directive_deprecated -->
                                            <a
                                                on:click|preventDefault={() =>
                                                    (search = syn)}>{syn}</a
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
        {:else}
            <div class="sides">
                <article class="half">
                    <h1 style="margin-bottom: var(--s-03);">Favorites</h1>
                    {#each stars.toSorted() as star}
                        <div class="horiz">
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <!-- svelte-ignore a11y_missing_attribute -->
                            <!-- svelte-ignore event_directive_deprecated -->
                            <a on:click|preventDefault={() => (search = star)}>{star}</a>
                            <Icon
                                fill={1}
                                wgth={400}
                                size={14}
                                style="color: var(--g3)"
                                on:click={() => {Star(star)}}
                                >star
                            </Icon>
                        </div>
                    {/each}
                </article>

                <article class="half">
                    <h1 style="margin-bottom: var(--s-03);">Recents</h1>
                    {#each recents as recent}
                        {@const starred = stars.includes(recent)}
                        <div class="horiz">
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <!-- svelte-ignore a11y_missing_attribute -->
                            <!-- svelte-ignore event_directive_deprecated -->
                            <a on:click|preventDefault={() => (search = recent)}>{recent}</a>
                            <Icon
                                fill={starred ? 1 : 0}
                                wgth={400}
                                size={14}
                                style="color: var(--g3)"
                                on:click={() => {Star(recent)}}
                                >star
                            </Icon>
                        </div>
                    {/each}
                </article>
            </div>
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
        -webkit-mask-image: linear-gradient(
            to bottom,
            black 90%,
            transparent 100%
        );
    }
    article {
        width: 100%;
        max-width: 100%;
        display: flex;
        flex-direction: column;
    }
    /* fake a flex gap, so that it can be overwritten */
    article > *:not(:last-child) {
        margin-bottom: var(--s-2);
    }
    article > .title {
        margin-bottom: var(--s-01) !important;
    }
    .title {
        display: flex;
        gap: var(--s-02);
        align-items: center;
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
    section > p,
    .word_list {
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
    .line {
        height: 1px;
        width: 100%;
        background: var(--g5);
    }

    .sides{
        display: flex;
        gap: var(--s-0);
    }
    .half{
        flex-grow: 1;
    }
    .horiz{
        margin-bottom: var(--s-03) !important;
        display: flex;
        align-items: center;
        gap: var(--s-03);
    }
</style>
