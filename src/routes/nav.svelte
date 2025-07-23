<script>
    import {onMount, untrack} from 'svelte';

    import Icon from "$lib/Icon.svelte";
    import { fade, fly, slide } from 'svelte/transition';

    let {search = $bindable(), suggestions, langs, lang = $bindable()} = $props();
    let dark_mode = $state(true);
    let searching = $state(false);

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

    onMount(() =>{
        const searchInput = document.getElementById('search');
        if (searchInput) {
            searchInput.focus();
            searchInput.addEventListener('focus', ToggleSearching);
            searchInput.addEventListener('blur', ToggleSearching);
        }

        // Add event listener for keydown events
        document.addEventListener('keydown', handleKeyDown);
        

        // Cleanup event listener on component destroy
        return () => {
            document.removeEventListener('keydown', handleKeyDown);
            searchInput?.removeEventListener('focus', ToggleSearching);
            searchInput?.removeEventListener('blur', ToggleSearching);
        };
    });

    function ToggleSearching(e) {
        searching = e.type === 'focus';
    }

    // handle keydown events for the suggestions list selecting up and down the list with arrow keys
    let suggestion_selection = $state(0);
    function handleKeyDown(event) {
        if (event.key === 'ArrowDown') {
            suggestion_selection = (suggestion_selection + 1) % (suggestions.length + 1); // Include 0th index
        } else if (event.key === 'ArrowUp') {
            suggestion_selection = (suggestion_selection - 1 + (suggestions.length + 1)) % (suggestions.length + 1); // Include 0th index
        } else if (event.key === 'Enter') {
            if (suggestion_selection > 0) {
                const selectedElement = document.querySelector(`.suggestions li:nth-child(${suggestion_selection})`);
                selectedElement?.click();
            }
        } 
    }
    $effect(() => {
        search;
        untrack(() => {
            suggestion_selection = 0; // Reset selection when suggestions change
        });
    });
</script>

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
        bind:value={search}
            />

    <!-- dont show if its just 1 word which is the search word -->
    {#if !(suggestions.length === 1 && suggestions[0] === search) && searching}
        <ul class="suggestions" transition:fly={{duration: 140, y:-20}}>
            {#each suggestions as s, i}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <!-- svelte-ignore event_directive_deprecated -->
                <li on:click={() => search = s} class:selected={i + 1 === suggestion_selection}>{s}</li>
            {/each}
        </ul>
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

<style lang="scss">
    nav {
        display: flex;
        gap: $s-1;
        align-items: center;
        margin-bottom: $s-1;
        position: relative;

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

        .suggestions {
            position: absolute;
            right: $s-0;
            top: 34px + $s-03;
            background: #1d1d1dd1;
            backdrop-filter: blur(8px);
            border: 1px solid $g5;
            border-radius: 4px;
            margin-top: 4px;
            z-index: 1000;
            width: min-content;
            list-style: none;

            li {
                margin: $s-01;
                padding: $s-03;
                cursor: pointer;
                color: $g2;
                white-space: pre;
                word-break: keep-all;
                border-radius: $s-03;

                &:hover, &:focus, &.selected {
                    color: $g1;
                    background: $g5;
                }
            }
        }
    }
</style>