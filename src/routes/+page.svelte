<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let search = $state('');

  async function Query(input='') {
    const res = await invoke("search_words", {
      query: input,
      limit: 5,
    });

    console.log(res);
    return res;
  }
  
  onMount(async () => {
    // This will run once the component is mounted
    console.log('Component mounted');
    
    console.log();
  });
</script>

<input type="text" name="" id="" bind:value={search} placeholder="Search..." />

{#if search}
  <ul>
    {#await Query(search)}
      <h1>searching...</h1>
    {:then entries}
      {#each entries as e}
        <li>
          {e.score} | <strong>{e.word}</strong>: {e.definition}
          <!-- <ul>
            {#each entry?.synonyms as synonym}
              <li>{synonym}</li>
            {/each}
          </ul> -->
        </li>
      {/each}
    {:catch error}
      <li>Error: {error}</li>
    {/await}    
  </ul>
{/if}