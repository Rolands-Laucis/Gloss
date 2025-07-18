<script>
    import Icon from "$lib/Icon.svelte";

    let {search = $bindable(), suggestions, langs, lang = $bindable()} = $props();
    let dark_mode = $state(true);

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

<style lang="scss">
    nav {
        display: flex;
        gap: $s-1;
        align-items: center;
        margin-bottom: $s-1;

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
    }
</style>