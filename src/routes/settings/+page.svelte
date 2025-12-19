<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  
  // 1. All imports must be at the top
  import ThemeSelector from '$lib/components/ThemeSelector.svelte';
  import ModeSelector from '$lib/components/ModeSelector.svelte';

  // 2. Variables must be at the top level so HTML can see them
  let currentMode = "system";

  // 3. Single onMount for initialization
  onMount(() => {
    // Apply Mica effect
    invoke('set_mica_effect'); 

    // Load saved preference
    const saved = localStorage.getItem('theme-preference');
    if (saved) {
      currentMode = saved;
    }
    applyTheme(currentMode);
  });

  // 4. Functions must be at the top level
  function handleModeChange(e: CustomEvent) {
    const mode = e.detail;
    currentMode = mode;
    localStorage.setItem('theme-preference', mode);
    applyTheme(mode);
  }

  function applyTheme(mode: string) {
    const root = document.documentElement;

    if (mode === 'dark') {
      root.classList.add('dark');
      root.style.colorScheme = 'dark';
    } else if (mode === 'light') {
      root.classList.remove('dark');
      root.style.colorScheme = 'light';
    } else {
      // SYSTEM MODE
      const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (systemDark) {
        root.classList.add('dark');
        root.style.colorScheme = 'dark';
      } else {
        root.classList.remove('dark');
        root.style.colorScheme = 'light';
      }
    }
  }
</script>

<div class="container">
  
  <div class="setting-section">
    <h2 class="title">Background Material</h2>
    <div class="control-wrapper">
      <ThemeSelector />
    </div>
  </div>

  <div class="setting-section">
    <h2 class="title">App Theme</h2>
    <div class="control-wrapper">
      <ModeSelector 
        bind:value={currentMode} 
        on:change={handleModeChange} 
      />
    </div>
  </div>

</div>

<style>
  :global(body) {
    /* Critical for Mica to be visible */
    background: transparent !important; 
    margin: 0;
  }

  .container {
    display: flex;
    flex-direction: column;
    padding: 32px;
    gap: 32px; /* Space between the two big sections */
    color: white;
    font-family: "Segoe UI Variable", sans-serif;
    max-width: 800px;
    margin: 0 auto;
  }

  .setting-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .title {
    font-size: 18px;
    font-weight: 600;
    margin: 0;
    opacity: 0.9;
  }

  .control-wrapper {
    /* Optional: adds a subtle background behind the controls group */
    background: rgba(255, 255, 255, 0.02); 
    padding: 20px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }
</style>