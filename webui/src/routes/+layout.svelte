<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Trigger />
	<main class="@container w-full p-4">
		{@render children?.()}
	</main>
</Sidebar.Provider>
