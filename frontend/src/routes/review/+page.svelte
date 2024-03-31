<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/Card.svelte';
	import { writable } from 'svelte/store';
	import type { ActionData, PageServerData } from './$types';
	import Overlay from '$lib/components/Overlay.svelte';
	import Alert from '$lib/components/Alert.svelte';

	const openMemory = writable<null | string>(null);
	const changingMemory = writable<null | string>(null);

	export let form: ActionData;
	export let data: PageServerData;

	$: () => {
		if (form?.success) {
			data.props.memories = data.props.memories.filter(
				(memory: any) => memory.id !== $changingMemory
			);
		}
		changingMemory.set(null);
	};
</script>

<Overlay {openMemory} />
<div
	class="w-full overflow-hidden transition-all"
	style:max-height={form !== null && form.success == false ? '200px' : '0px'}
>
	<Alert
		message={form?.message}
		on:close={() => {
			form = null;
		}}
	></Alert>
</div>
<div class="flex flex-wrap gap-4 place-content-center px-2">
	{#each data.props.memories as memory}
		<Card image={memory.url} table={memory.table_name} on:open={() => openMemory.set(memory.url)}>
			<div class="flex">
				<div class="mt-2 flex w-full">
					<form method="POST" action="?/approve" class="basis-1/2" use:enhance>
						<input type="hidden" id="memory_id" name="memory_id" value={memory.memory_id} />
						<button
							type="submit"
							class="block w-full text-white px-2 bg-green-500 hover:bg-green-600 transition-colors rounded-l-lg cursor-pointer"
							on:click={() => changingMemory.set(memory.memory_id)}
						>
							Approve
						</button>
					</form>
					<form method="POST" action="?/deny" class="basis-1/2" use:enhance>
						<input type="hidden" id="memory_id" name="memory_id" value={memory.memory_id} />
						<button
							type="submit"
							class="block w-full text-white px-2 bg-red-500 hover:bg-red-600 transition-colors rounded-r-lg cursor-pointer"
							on:click={() => changingMemory.set(memory.memory_id)}
						>
							Deny
						</button>
					</form>
				</div>
			</div>
		</Card>
	{/each}
</div>
