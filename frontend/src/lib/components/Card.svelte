<script lang="ts">
	import { dev } from '$app/environment';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let image: string = '';
	export let table: string | null = null;
</script>

<div class="basis-full md:basis-1/3 lg:basis-1/4">
	<button
		type="button"
		class="hover:bg-black/10 transition-colors p-4 rounded-lg relative flex place-content-center max-h-full mx-auto"
		on:click={() => dispatch('open')}
	>
		<img
			class="rounded-t-md max-w-full h-full object-contain"
			src="{dev ? 'http://localhost:5005/api' : 'https://catiaandjamie.love/api'}{image}"
			alt={table ? 'A memory from the wedding taken by ' + table : 'A memory from the wedding'}
		/>
		{#if table}
			<p class="bg-white rounded-full text-center text-xl font-bold absolute bottom-0 px-2 right-0">
				{table}
			</p>
		{/if}
	</button>
	<slot />
</div>
