<script>
    import {fly, slide, scale} from 'svelte/transition';
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import {log} from '$lib/shared.js'
    import { readTextFile, writeTextFile, exists, create } from '@tauri-apps/plugin-fs';

    import Nav from "./nav.svelte";
    import Entry from "./entry.svelte";
    import Icon from "$lib/Icon.svelte";

    let search = $state(""),
        suggestions = $state([]); //Lexicon
    let stars = $state([]);
    let recents = $state([]);
    let lang = $state(0);

    const langs = ['en', 'lv']

    onMount(async () => {
        if(!await exists(import.meta.env.VITE_stars)) {
            await create(import.meta.env.VITE_stars);
            // await writeTextFile(import.meta.env.VITE_stars, '[]');
        }
        if(!await exists(import.meta.env.VITE_recents)) {
            await create(import.meta.env.VITE_recents);
            // await writeTextFile(import.meta.env.VITE_recents, '[]');
        }

        stars = JSON.parse(await readTextFile(import.meta.env.VITE_stars) || "[]");
        recents = JSON.parse(await readTextFile(import.meta.env.VITE_recents) || "[]");

        window.addEventListener('keydown', KeyDown)
    });

    function KeyDown(e){
        // console.log(e);
        
        // pressing enter on the search bar will confirm it as the search term, so it can be stored in recents and 
        const search_box = document.getElementById('search');
        if(e.key === 'Enter' && e.target === search_box){
            search_box?.blur();
            recents = Array.from(new Set([e.target.value, ...recents]));
            if(recents.length > 200) recents.length = 200;
            writeTextFile(import.meta.env.VITE_recents, JSON.stringify(recents));
            // console.log({recents});
        }
        // else if(e.key === 'Tab' && e.target !== search_box){
        else if(e.key === 'f' && e.ctrlKey === true && e.target !== search_box){
            e.preventDefault();
            search_box?.focus();
        }else if(e.key === 'w' && e.ctrlKey === true){
            getCurrentWindow().close();
        }
        else if(e.key === 'Escape'){
            search = "";
            suggestions = [];
            search_box?.focus();
        }
    }

    async function Query(input = "") {
        const res = await invoke("search_wordnet", {
            query: input,
            languageCode: langs[lang],
            maxResults: 15
        });
        // log(res);
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

        suggestions = [...new Set(entries.map(e => e.word))]; //.filter(w => w !== search)

        log(entries);
        return entries;
    }

    // toggles starred words
    function Star(word = "") {
        // if already starred, remove it
        if (stars.includes(word)) stars.splice(stars.indexOf(word), 1);
        // otherwise add it
        else stars.push(word);
        // persist to file
        writeTextFile(import.meta.env.VITE_stars, JSON.stringify(stars));
        // console.log({stars});
    }
</script>

<main>
    <Nav bind:search={search} {suggestions} bind:lang={lang} {langs}></Nav>

    <aside>
        {#if search}
            {#await Query(search)}
                <!-- <small>searching...</small> -->
            {:then entries}
                {#each entries as e, i}
                    {@const starred = stars.includes(e.word)}
                    <Entry {e} {starred} {langs} {lang} bind:search={search} {Star} delay={i}></Entry>
                {/each}
            {:catch error}
                <p>Error:</p>
                <p>{error}</p>
            {/await}
        {:else}
            <div class="sides">
                <article class="half">
                    <h1 style="margin-bottom: var(--s-03);">Favorites</h1>
                    {#each stars as star}
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
                                tabindex={0}
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
                                tabindex={0}
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
    aside {
        width: 100%;
        max-width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: $s-3;
        padding-bottom: $s-4;

        overflow-y: auto;
        overflow-x: hidden;

        mask-image: linear-gradient(to bottom, black 90%, transparent 100%);
        -webkit-mask-image: linear-gradient(
            to bottom,
            black 90%,
            transparent 100%
        );
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
