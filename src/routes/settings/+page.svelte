<div class="container">
  <div class="p-8">
  <h2 class="text-xl font-bold mb-4 text-white">Theme:</h2>
  
  <ThemeSelector 
  />
</div>
</div>

<style>
  /* Simple styling just to prove it works */
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: transparent;
    color: white;
    font-family: sans-serif;
  }
</style>
<script>
  import { onMount } from 'svelte';
  import ThemeSelector from '$lib/components/ThemeSelector.svelte';
  // Note: In Tauri v2, 'invoke' is in @tauri-apps/api/core
  import { invoke } from '@tauri-apps/api/core'; 

  onMount(() => {
    // Tell Rust to apply Mica to THIS window
    invoke('set_mica_effect');
  });
</script>