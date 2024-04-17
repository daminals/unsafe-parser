<script lang="ts">
import "../app.pcss";
import { Toast, getToastStore } from '@skeletonlabs/skeleton';
import type { ToastSettings, ToastStore } from '@skeletonlabs/skeleton';
import { initializeStores } from '@skeletonlabs/skeleton';

initializeStores();
</script>

<Toast />

<slot></slot>

