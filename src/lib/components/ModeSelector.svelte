<script lang="ts">
  import { RadioGroup } from "bits-ui";
  import { createEventDispatcher } from 'svelte';
  import { Sun, Moon, Monitor } from 'lucide-svelte'; // Standard icons

  export let value = "system"; 
  const dispatch = createEventDispatcher();

  const options = [
    { value: "light", label: "Light", icon: Sun },
    { value: "dark", label: "Dark", icon: Moon },
    { value: "system", label: "System", icon: Monitor }
  ];

  function handleChange(newValue: string) {
    value = newValue;
    dispatch("change", newValue);
  }
</script>

<RadioGroup.Root 
  bind:value 
  onValueChange={handleChange}
  class="mode-grid"
>
  {#each options as option}
    <RadioGroup.Item value={option.value} class="mode-card">
      <svelte:component this={option.icon} size={24} class="mb-2 opacity-80" />
      
      <span class="label">{option.label}</span>

      <div class="check-circle"></div>
    </RadioGroup.Item>
  {/each}
</RadioGroup.Root>

<style>
  /* 1. Grid: 3 columns for side-by-side layout */
  :global(.mode-grid) {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    width: 100%;
  }

  /* 2. Card Style */
  :global(.mode-card) {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 20px 10px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  :global(.mode-card:hover) {
    background: rgba(255, 255, 255, 0.08);
  }

  /* Selected State */
  :global(.mode-card[data-state="checked"]) {
    background: rgba(96, 205, 255, 0.1);
    border-color: AccentColor; /* Uses Windows Accent Color */
    color: white;
  }

  /* Icon Color Change on Active */
  :global(.mode-card[data-state="checked"]) :global(svg) {
    color: AccentColor;
  }

  .label { font-size: 14px; font-weight: 500; margin-top: 4px; }

  /* Hidden check circle logic (optional, mainly for accessibility visuals) */
  .check-circle {
    position: absolute; top: 8px; right: 8px;
    width: 8px; height: 8px; border-radius: 50%;
    background: transparent;
  }
  
  :global(.mode-card[data-state="checked"]) .check-circle {
    background: AccentColor;
    box-shadow: 0 0 4px AccentColor;
  }
</style>