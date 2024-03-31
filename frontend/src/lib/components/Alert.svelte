<script lang="ts">
	import { AlertType } from '$lib/interface/alertType';
	import { faX } from '@fortawesome/free-solid-svg-icons';
	import { createEventDispatcher } from 'svelte';
	import Fa from 'svelte-fa';

	const dispatch = createEventDispatcher();

	export let type: AlertType = AlertType.Error;
	export let title: string | null = null;
	export let message: string = 'There was an error!';
	let classes = '';
	export { classes as class };
	classes += ' mb-2 rounded-lg px-4 py-2 text-white relative';
	export let details: string | null = null;
	export let showCloseButton: boolean = true;
</script>

<div
	class={classes}
	class:bg-red-500={type === AlertType.Error}
	class:bg-yellow-600={type === AlertType.Warning}
	class:bg-blue-600={type === AlertType.Info}
	class:bg-green-500={type === AlertType.Success}
>
	<h3 class="font-bold text-3xl">
		{title !== null ? title : type}
	</h3>
	<p class="text-wrap font-bold text-xl">{message}</p>
	{#if details}
		<p>{@html details}</p>
	{/if}
	{#if showCloseButton}
		<button type="button" class="absolute right-4 top-4" on:click={() => dispatch('close')}>
			<Fa icon={faX} size="1x" />
		</button>
	{/if}
</div>
