<script lang="ts">
	import { enhance } from '$app/forms';
	import Alert from '$lib/components/Alert.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<div
	class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex place-content-center flex-col"
>
	<form method="POST" class="flex flex-row gap-x-2 mb-4" use:enhance>
		<input type="text" name="username" placeholder="Username" class="rounded-lg" required />
		<input type="password" name="password" placeholder="Password" class="rounded-lg" required />
		<button type="submit" class="bg-green-400 hover:bg-green-500 transition-colors px-4 rounded-lg"
			>Login</button
		>
	</form>
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
</div>
