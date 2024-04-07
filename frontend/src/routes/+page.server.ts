import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { dev } from '$app/environment';

export const actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const table = cookies.get('table');
		if (table === undefined) {
			return fail(400, { message: 'No table selected' });
		}
		data.append('table', table);

		let url = 'http://backend:5005/api/memory/new';
		if (dev) {
			url = 'http://localhost:5005/api/memory/new';
		}

		try {
			const response = await fetch(url, {
				method: 'POST',
				body: data
			});

			if (response.ok) {
				return { success: true };
			} else {
				console.log(await response.text());
				const data = await response.json();
				return fail(500, { message: data.message });
			}
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'An error occurred' });
		}
	}
} satisfies Actions;

export const load: PageServerLoad = async ({ url, cookies }) => {
	const table = url.searchParams.get('table');
	if (table) {
		cookies.set('table', table, {
			path: '/',
			maxAge: 60 * 60 * 24 * 30
		});
		redirect(302, url.pathname);
	}

	if (!table && cookies.get('table') === undefined) {
		cookies.set('table', 'Unknown', {
			path: '/',
			maxAge: 60 * 60 * 24 * 30
		});
	}

	let apiURL = 'http://backend:5005/api/memory';
	if (dev) {
		apiURL = 'http://localhost:5005/api/memory';
	}

	try {
		const response = await fetch(apiURL);
		if (response.ok) {
			const data = await response.json();
			return {
				props: {
					memories: data.memories
				}
			};
		} else {
			return {
				props: {
					memories: []
				}
			};
		}
	} catch (e) {
		return { props: { memories: [] } };
	}
};
