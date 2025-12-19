<script lang="ts">
  import { RadioGroup } from "bits-ui";
  import { createEventDispatcher } from 'svelte';

  export let value = "mica"; 
  const dispatch = createEventDispatcher();

  const options = [
    { value: "mica", label: "Mica", desc: "Standard Windows blur , works fine on most hardware. Reccomended" },
    { value: "acrylic", label: "Acrylic", desc: "Glassy transparency , May Cause Performance issues on weak hardware." },
  ];

  function handleChange(newValue: string) {
    value = newValue;
    dispatch("change", newValue);
  }
</script>

<RadioGroup.Root 
  bind:value 
  onValueChange={handleChange}
  class="selector-grid"
>
  {#each options as option}
    <RadioGroup.Item value={option.value} class="selector-card">
      <div class="check-circle"></div>

      <span class="label">{option.label}</span>
      <span class="desc">{option.desc}</span>
    </RadioGroup.Item>
  {/each}
</RadioGroup.Root>

<style>
  /* 1. The Grid Layout */
  :global(.selector-grid) {
    display: grid;
    grid-template-columns: 1fr 1fr; /* 2 Columns */
    gap: 12px;
    width: 100%;
  }

  /* 2. The Card Style (Windows 11 Look) */
  :global(.selector-card) {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 6px;
    background: rgba(255, 255, 255, 0.03); /* bg-white/5 */
    border: 1px solid rgba(255, 255, 255, 0.08); /* border-white/10 */
    border-radius: 8px;
    padding: 16px;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
    outline: none;
  }

  /* Hover State */
  :global(.selector-card:hover) {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.15);
  }

  /* 3. Selected State (Data Attribute from Bits-UI) */
  :global(.selector-card[data-state="checked"]) {
    background: rgba(96, 205, 255, 0.1); /* Windows Blue Tint */
    border-color: #60cdff;               /* Windows Blue Border */
    box-shadow: 0 0 0 1px #60cdff;       /* Glow effect */
  }

  /* 4. Text Styles */
  .label {
    font-size: 14px;
    font-weight: 600;
    color: white;
  }
  
  .desc {
    font-size: 12px;
    color: #888;
  }

  /* 5. Checkmark Circle (Top Right) */
  .check-circle {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.3);
    transition: all 0.2s;
  }

  /* Fill circle when checked */
  :global(.selector-card[data-state="checked"]) .check-circle {
    background-color: #60cdff;
    border-color: #60cdff;
    box-shadow: inset 0 0 0 3px #202020; /* Creates the "dot" inside */
  }
</style>