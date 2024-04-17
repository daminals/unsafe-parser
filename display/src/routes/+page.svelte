<script lang="ts">
  import { FileDropzone } from '@skeletonlabs/skeleton';
  import { writable } from 'svelte/store';
  import { getToastStore } from '@skeletonlabs/skeleton';
  import { ProgressRadial } from '@skeletonlabs/skeleton';
  import { Box } from '$lib/box';

  let files: FileList;
  const toastStore = getToastStore();
  const pageState = writable('open');
  const chartData = writable(undefined);

  const onFileDrop = async (event: Event) => {
    console.log('file data:', event);
    console.log(files)
    // check if the file is a JSON file
    if (files[0].type !== 'application/json') {
      const t: ToastSettings = {
        message: 'Only JSON files are allowed',
        type: 'error',
      };
      toastStore.trigger(t);
      // reset the file state
      files = undefined;
      return;
    }

    const t: ToastSettings = {
      message: 'Parsing JSON file...',
    };
    $pageState = 'loading';
    toastStore.trigger(t);

    await parseJson(files[0]);
  };

  const parseJson = async (file: File) => {
    const reader = new FileReader();
    reader.readAsText(file);
    reader.onload = async () => {
      const data = JSON.parse(reader.result as string);
      console.log(data);
      chartData.set(data);
      $pageState = 'chart';
    };
  };


</script>

<div class="container text-center justify-center mx-auto p-8 space-y-8">
	<h1 class="h1">Render Unsafe Tree</h1>
  <div class="flex w-[50%] mx-auto text-center justify-center">
    {#if $pageState === 'open'}
      <FileDropzone 
        name="files" 
        bind:files={files} 
        on:change={onFileDrop} 
      >
        <svelte:fragment slot="meta">Only JSON files are allowed</svelte:fragment>
        </FileDropzone>
    {:else if $pageState === 'loading'}
    <div class="mx-auto">
      <ProgressRadial value={undefined} />
    </div>
    {/if}
  </div>
  {#if $pageState === 'chart' && $chartData !== undefined }
    <Box data={$chartData} />
  {/if}

</div>

