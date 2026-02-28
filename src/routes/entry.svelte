<script>
    import {fly, fade, slide, blur} from 'svelte/transition';
    import {POS_tags} from './pos_tags.js';
    
    import Icon from "$lib/Icon.svelte";
    
    let {e, search = $bindable(), starred, langs, lang, Star, delay, addSenseToEntry} = $props();

    // Add sense form state
    let showAddSenseForm = $state(false);
    let newSenseDefinition = $state("");
    let newSensePos = $state("n");

    function copyToClipboard(asMarkdown = false) {
        let text = '';
        if (asMarkdown) {
            text = `# ${e.word}\n`;
            e.senses.forEach((sense, i) => {
                const posInfo = POS_tags[langs[lang]][sense.pos];
                text += `**${posInfo.long}**\n`;
                if (sense.definitions?.length) {
                    sense.definitions.forEach((d, j) => {
                        text += `${j + 1}. ${d}\n`;
                    });
                    text += '\n';
                }
            });
        } else {
            text = `${e.word}\n`;
            let defNum = 1;
            e.senses.forEach((sense) => {
                if (sense.definitions?.length) {
                    sense.definitions.forEach((d) => {
                        text += `${defNum}. ${d}\n`;
                        defNum++;
                    });
                }
            });
        }
        navigator.clipboard.writeText(text.trim());
    }

    async function handleAddSense() {
        if (!newSenseDefinition.trim()) return;
        
        await addSenseToEntry(e.word, newSensePos, newSenseDefinition.trim());
        
        // Reset form
        newSenseDefinition = "";
        newSensePos = "n";
        showAddSenseForm = false;
    }
</script>

<!-- in:fly|global={{duration:300, y:-100, delay:delay*150}} out:fade|global={{duration:300}} -->
 <!-- transition:fly|global={{duration:200, y:-50, delay:delay*150}} -->
  <!-- transition:blur|global={{duration:200}} -->
<article in:fly|global={{duration:200, y:-20, delay:delay*70}}>
    <div class="title">
        <h1 class="word">{e.word}</h1>

        <Icon
            fill={starred ? 1 : 0}
            hover_fill
            to_fill={starred ? 0 : 1}
            wgth={300}
            size={32}
            tabindex={0}
            title="Favorite this word"
            on:click={() => {
                Star(e.word);
            }}
            style="align-self: flex-start;color: var(--{starred ? 'g1' : 'g2'});"
        >
            star
        </Icon>

        <!-- svelte-ignore event_directive_deprecated -->
        <Icon
            fill={0}
            hover_fill
            to_fill={1}
            wgth={300}
            size={28}
            tabindex={0}
            title="Left click: copy as plain text | Right click: copy as markdown"
            on:click={() => copyToClipboard(false)}
            on:contextmenu={(ev) => { ev.preventDefault(); ev.stopPropagation(); copyToClipboard(true); }}
            style="color: var(--g2);"
        >
            content_copy
        </Icon>

        <!-- svelte-ignore event_directive_deprecated -->
        <Icon
            fill={0}
            hover_fill
            to_fill={1}
            wgth={300}
            size={28}
            tabindex={0}
            title="Add a new sense to this word"
            on:click={() => showAddSenseForm = !showAddSenseForm}
            style="color: var(--{showAddSenseForm ? 'g1' : 'g2'});"
        >
            add
        </Icon>
    </div>

    {#if showAddSenseForm}
        <section class="add-sense-form" transition:slide={{duration: 150}}>
            <div class="sense-header">
                
                <!-- svelte-ignore event_directive_deprecated -->
                <Icon
                    fill={0}
                    hover_fill
                    to_fill={1}
                    wgth={300}
                    size={22}
                    tabindex={0}
                    title="Add sense"
                    on:click={handleAddSense}
                    style="color: #4C4C4C;"
                >
                    check
                </Icon>
                |
                <!-- svelte-ignore event_directive_deprecated -->
                <Icon
                    fill={0}
                    hover_fill
                    to_fill={1}
                    wgth={300}
                    size={22}
                    tabindex={0}
                    title="Cancel"
                    on:click={() => showAddSenseForm = false}
                    style="color: #4C4C4C;"
                >
                    close
                </Icon>
                |
                <select bind:value={newSensePos} class="pos-select">
                    {#each Object.entries(POS_tags[langs[lang]]) as [key, value]}
                        <option value={key}>{value.long}</option>
                    {/each}
                </select>
            </div>
            <textarea 
                placeholder="New definition..." 
                bind:value={newSenseDefinition}
                class="def-input"
                rows="2"
            ></textarea>
        </section>
    {/if}

    {#each e.senses as sense}
        <section>
            <small 
                title={POS_tags[langs[lang]][sense.pos].desc}>
                    {POS_tags[langs[lang]][sense.pos].long} ~{sense.match_score}
                </small>

            {#if sense?.definitions.length}
                {#each sense.definitions as d}
                    <p class="def">{d}</p>
                {/each}
            {/if}

            {#if sense.examples.length}
                {#each sense.examples as e}
                    <p class="example">“{e}”</p>
                {/each}
            {/if}

            {#if sense.synonyms.length}
                <!-- <h2>synonyms</h2> -->
                <div class="block">
                    <small>synonyms:</small>
                    <div class="word_list">
                        <br/>
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
                </div>
            {/if}

            {#if sense.antonyms.length}
                <div class="block">
                    <small>antonyms:</small>
                    <!-- <p>antonyms:</p> -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_missing_attribute -->
                    <!-- svelte-ignore event_directive_deprecated -->
                    <div class="word_list">
                        <!-- <p>ant:</p> -->
                        {#each sense.antonyms as ant}
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <!-- svelte-ignore a11y_missing_attribute -->
                            <!-- svelte-ignore event_directive_deprecated -->
                            <a on:click|preventDefault={() => (search = ant)}
                                >{ant}</a
                            >
                        {/each}
                    </div>
                </div>
            {/if}
        </section>
    {/each}

    <div class="line"></div>
</article>

<style lang="scss">
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

    .add-sense-form {
        width: 100%;
        max-width: 100%;
        display: flex;
        flex-direction: column;
        gap: $s-03;

        .sense-header {
            display: flex;
            align-items: center;
            gap: $s-02;
        }

        .pos-select {
            width: fit-content;
            padding: $s-03 $s-02;
            background: transparent;
            border: none;
            border-bottom: 1px solid $g4;
            border-radius: 0;
            color: $g1;
            font-size: 0.85rem;
            cursor: pointer;

            &:focus {
                border-color: $g2;
                outline: none;
            }
        }

        .def-input {
            background: transparent;
            border: none;
            color: $l;
            font-family: inherit;
            resize: vertical;
            font-size: var(--s-1, 14px);

            &::placeholder {
                color: $g3;
            }
        }
    }

    section {
        width: 100%;
        max-width: 100%;

        display: flex;
        flex-direction: column;
        gap: $s-03;

        // counter-increment: definition-counter;

        & > .def::after {
            content: ".";
        }
        // & > p::before {
        //     content: counter(definition-counter) ". ";
        // }
        & > p{
            margin-left: $s-2;
            padding-right: $s-03;
        }

        .block{
            margin-left: $s-2;
            margin-top: $s-03;

            small{
                // font-weight: 300;
                color: $g1;
            }
        }
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
</style>