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
		const rawImageInput = data.get('memories');
		if (rawImageInput === null) {
			return fail(400, { message: 'No images provided' });
		}
		const images = rawImageInput.toString().split('_');
		const encodedImages: string[] = [];
		images.forEach((image) => {
			encodedImages.push(image.replace(/\+/g, '-').replace(/\//g, '_').split('base64,')[1]);
		});

		let url = 'https://catiaandjamie.love/api/memory/new';
		if (dev) {
			url = 'http://localhost:5005/api/memory/new';
		}

		console.log(
			'body',
			JSON.stringify({
				table,
				images: encodedImages
			})
		);

		try {
			const response = await fetch(url, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					table,
					images: encodedImages
				})
			});

			if (response.ok) {
				return { success: true };
			} else {
				const data = await response.json();
				return fail(500, { message: data.message });
			}
		} catch (e) {
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

	let apiURL = 'https://catiaandjamie.love/api/memory';
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
