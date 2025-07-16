<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import {POS_tags} from './pos_tags.js';

    import Icon from "./Icon.svelte";

    let search = $state("good"),
        suggestions = $state([]); //Lexicon
    let dark_mode = $state(true), lang = $state(0);
    let stars = $state([]);
    let recents = $state([]);

    const langs = ['en', 'lv']

    onMount(() => {
        stars = JSON.parse(localStorage.getItem("stars") || "[]");
        recents = JSON.parse(localStorage.getItem("recents") || "[]");

        window.addEventListener('keydown', KeyDown)
    });

    function KeyDown(e){
        // console.log(e);
        
        // pressing enter on the search bar will confirm it as the search term, so it can be stored in recents and 
        const search_box = document.getElementById('search');
        if(e.key === 'Enter' && e.target === search_box){
            search_box?.blur();
            recents = Array.from(new Set([e.target.value, ...recents]));
            if(recents.length > 25) recents.length = 200;
            localStorage.setItem("recents", JSON.stringify(recents));
            // console.log({recents});
        }
        // else if(e.key === 'Tab' && e.target !== search_box){
        else if(e.key === 'f' && e.ctrlKey === true && e.target !== search_box){
            e.preventDefault();
            search_box?.focus();
        }else if(e.key === 'w' && e.ctrlKey === true){
            getCurrentWindow().close();
        }
    }

    async function Query(input = "") {
        const res = await invoke("search_wordnet", {
            query: input,
            languageCode: langs[lang],
            maxResults: 15
        });
        console.log(res);
        // return [];

        const entries = [];
        for (const e of res) {
            if (e.match_score < 30) continue;

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
            // console.log(a.word, spaceCountA, b.word, spaceCountB);
            if (spaceCountA !== spaceCountB)
                return spaceCountA > spaceCountB ? 1 : -1;

            // Sort by score (descending)
            const maxScoreA = Math.max(...a.senses.map((s) => s.match_score));
            const maxScoreB = Math.max(...b.senses.map((s) => s.match_score));
            if (maxScoreA !== maxScoreB) return maxScoreB > maxScoreA ? 1 : -1;

            // otherwise alphabetically
            return b.word.localeCompare(a.word);
        });

        suggestions = [...new Set(entries.map(e => e.word))]; //.filter(w => w !== search)

        return entries;
    }

    function ChangeTheme() {
        dark_mode = !dark_mode;
        document.documentElement.setAttribute(
            "theme",
            dark_mode === true ? "dark" : "light",
        );
    }

    function ChangeLang(){
        lang += 1;
        if(lang > langs.length - 1) lang = 0;
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

        <button on:click={ChangeLang} class="outline">
            {langs[lang]}
        </button>

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
                                tabindex=0
                                title="Favorite this word"
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
                                <small title={POS_tags[langs[lang]][sense.pos].desc}>{POS_tags[langs[lang]][sense.pos].long} ~{sense.match_score}</small>
                                {#each sense.definitions as d}
                                    <p>{d}</p>
                                {/each}
                                {#if sense.examples.length}
                                    {#each sense.examples as e}
                                        <p class="example">“{e}”</p>
                                    {/each}
                                {/if}

                                {#if sense.synonyms.length}
                                    <!-- <h2>synonyms</h2> -->
                                    <div class="word_list">
                                        <!-- <p>syn:</p> -->
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

                                {#if sense.antonyms.length}
                                    <!-- <h2>synonyms</h2> -->
                                    <div class="word_list">
                                        <!-- <p>ant:</p> -->
                                        {#each sense.antonyms as ant}
                                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                                            <!-- svelte-ignore a11y_missing_attribute -->
                                            <!-- svelte-ignore event_directive_deprecated -->
                                            <a
                                                on:click|preventDefault={() =>
                                                    (search = ant)}>{ant}</a
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
                                title="Favorite this word"
                                tabindex=0
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
                                title="Favorite this word"
                                tabindex=0
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

<style lang="scss">
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
        gap: $s-1;
        align-items: center;
        margin-bottom: $s-1;

        & > button {
            height: $s-3;
            aspect-ratio: 1 / 1;
            background: $g1;
            border-radius: 2px;

            &:hover {
                background-color: $l;
            }
            &:focus{
                outline: 2px solid $l;
                border-radius: $s-03;
            }

            &.outline{
                background: transparent;
                border: 2px solid $g3;
                border-radius: $s-03;
                color: $g3;

                &:hover {
                    border: 2px solid $g1;
                    color: $g1;
                }
            }
        }
    }
    input {
        flex-grow: 1;
        padding: $s-01;

        background: transparent;
        border-radius: 4px;
        border: 2px solid $g4;

        &:hover, &:focus{
            border: 2px solid $g3;
        }
    }
    aside {
        width: 100%;
        max-width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: $s-3;
        padding-bottom: $s-4;

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
        
        /* fake a flex gap, so that it can be overwritten */
        & > *:not(:last-child) {
            margin-bottom: $s-2;
        }
        & > .title {
            margin-bottom: $s-01 !important;

            display: flex;
            gap: $s-02;
            align-items: center;
        }
    }
    .word {
        margin-bottom: $s-03;
    }
    section {
        width: 100%;
        max-width: 100%;

        display: flex;
        flex-direction: column;
        gap: $s-03;

        & > p::after {
            content: ".";
        }
        & p{
            padding-right: $s-03;
        }
    }
    section > p,
    .word_list {
        margin-left: $s-2;
    }
    .example {
        font-style: italic;
        color: $g1;
    }
    .word_list {
        max-width: 100%;
        display: flex;
        text-transform: lowercase;
        flex-wrap: wrap;

        & a:not(:last-child){
            margin-right: $s-0;
            &::after {
                content: ",";
            }
        }
        
    }
    .line {
        height: 1px;
        width: 100%;
        background: $g5;
    }

    .sides{
        display: flex;
        gap: $s-0;
    }
    .half{
        flex-grow: 1;
    }
    .horiz{
        margin-bottom: $s-03 !important;
        display: flex;
        align-items: center;
        gap: $s-03;
    }
</style>
