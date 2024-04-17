<script lang="ts">
  import { PieChart } from '$lib/charts'
  import { writable, derived } from 'svelte/store'
  export let data: any;
  let current = writable(data);

  const setCurrent = (data) => {
    pushParent($current);
    current.set(data);
  }
  const goBack = () => {
    let parent = popParent();
    if (parent) {
      current.set(parent);
    } else {
    current.set(data);
    }
  }
  let parent = writable([]);
  // pop and push in LILO for parent
  const pushParent = (data) => {
    parent.update((p) => {
      if (p.length > 2) {
        p.shift();
      }
      p.push(data);
      return p;
    });
  }
  const popParent = () => {
    // get the last element
    let p = $parent[$parent.length - 1];
    // remove the last element
    $parent = $parent.slice(0, -1);
    return p;
  }

  const disabled = derived(parent, ($parent) => $parent.length === 0);

  export { Box };
</script>

<div class="block card card-hover p-4">
  <h3 class="h3 text-surface-900 pb-4 underline">{$current.path}</h3>

  <div class="flex">
    <div class="w-[30%] space-y-4">
      <PieChart unsafe={$current.unsafe_lines} total={$current.all_lines} name={$current.path} />
      <button type="button" class="btn variant-filled rounded-lg p-2 space-x-0.5" on:click={goBack} disabled={$disabled}>
        <span>
          <svg class=" p-0 m-0" xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M5 12l14 0" /><path d="M5 12l6 6" /><path d="M5 12l6 -6" /></svg>
        </span>
        <span>back</span>
      </button>
    </div>

    <div class="w-[60%] mx-auto">
      <div class="grid grid-cols-3 gap-3 justify-center">
        {#each $current.children as item}
          <button class="col-span-1 block card-hover h-auto rounded-xl" on:click={() => setCurrent(item)}>
            <h4 class="underline text-surface-900">{item.path.replace($current.path + '/', '')}</h4>
            <PieChart unsafe={item.unsafe_lines} total={item.all_lines} name={item.path} />
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>
