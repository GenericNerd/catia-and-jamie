<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/Card.svelte';
	import Overlay from '$lib/components/Overlay.svelte';
	import { writable } from 'svelte/store';
	import type { PageData } from './$types';

	const openMemory = writable<null | string>(null);
	export let data: PageData;

	function onKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			openMemory.set(null);
		}
	}
</script>

<Overlay {openMemory} />
<div class="flex flex-col place-content-center text-white mb-8">
	<h1 class="mt-16 text-center text-9xl font-bold">Catia and Jamie</h1>
	<h2 class="text-center text-7xl">2024</h2>
	<form class="place-self-center" method="POST" enctype="multipart/form-data" use:enhance>
		<label class="block text-3xl font-medium text-center" for="memories">Upload memories</label>
		<div class="flex">
			<input
				class="block text-gray-900 text-sm border border-blue-400 rounded-l-lg cursor-pointer bg-blue-200 hover:bg-blue-300 transition-colors focus:outline-none"
				type="file"
				id="memory"
				name="memory"
				accept="image/*"
				multiple
			/>
			<button
				type="submit"
				class="block text-white px-2 bg-green-500 hover:bg-green-600 transition-colors rounded-r-lg cursor-pointer"
			>
				Submit
			</button>
		</div>
	</form>
</div>
<div class="flex flex-wrap gap-4 place-content-center px-2">
	{#each data.props.memories as memory}
		<Card image={memory.url} table={memory.table_name} on:open={() => openMemory.set(memory.url)} />
	{/each}
</div>
<svelte:window on:keydown={onKeyDown} />
