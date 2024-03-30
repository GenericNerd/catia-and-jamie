<script lang="ts">
	import { dev } from '$app/environment';
	import { faX } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { writable } from 'svelte/store';

	export let openMemory = writable<null | string>(null);
</script>

<button
	type="button"
	on:click={() => openMemory.set(null)}
	class="{$openMemory !== null
		? 'bg-black/50'
		: ''} h-screen w-screen fixed top-0 left-0 transition-colors flex place-content-center z-10"
	class:cursor-default={$openMemory !== null}
	class:pointer-events-none={$openMemory === null}
>
	<img
		class="transition-opacity h-full object-contain"
		class:opacity-0={$openMemory === null}
		class:opacity-100={$openMemory !== null}
		src="{dev ? 'http://localhost:5005/api' : 'https://catiaandjamie.love/api'}{$openMemory}"
		alt="A memory of the wedding"
	/>
	<button
		type="button"
		on:click={() => openMemory.set(null)}
		class="transition-opacity absolute top-2 right-2 lg:top-6 lg:right-6"
		class:opacity-0={$openMemory === null}
		class:opacity-100={$openMemory !== null}
	>
		<Fa icon={faX} size="3x" color="white" />
	</button>
</button>
