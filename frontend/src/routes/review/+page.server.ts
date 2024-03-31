import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { dev } from '$app/environment';

export const actions = {
	approve: async ({ request, cookies }) => {
		const data = await request.formData();
		const id = data.get('memory_id');
		if (id === undefined || id === null) {
			return fail(400, { success: false, message: 'No id provided' });
		}
		const token = cookies.get('token');
		if (token === undefined) {
			return fail(401, { success: false, message: 'No token provided' });
		}

		let url = 'https://catiaandjamie.love/api/memory/approve';
		if (dev) {
			url = 'http://localhost:5005/api/memory/approve';
		}

		console.log(id, data);
		try {
			const response = await fetch(url, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					memory_id: Number.parseInt(id.toString())
				})
			});

			if (!response.ok) {
				const data = await response.json();
				return fail(500, { success: false, message: data.message });
			}
		} catch (e) {
			return fail(500, { success: false, message: 'An error occurred' });
		}
	},
	deny: async ({ request, cookies }) => {
		const data = await request.formData();
		const id = data.get('memory_id');
		if (id === undefined || id === null) {
			return fail(400, { success: false, message: 'No id provided' });
		}
		const token = cookies.get('token');
		if (token === undefined) {
			return fail(401, { success: false, message: 'No token provided' });
		}

		let url = 'https://catiaandjamie.love/api/memory/deny';
		if (dev) {
			url = 'http://localhost:5005/api/memory/deny';
		}

		try {
			const response = await fetch(url, {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					memory_id: Number.parseInt(id.toString())
				})
			});

			if (!response.ok) {
				const data = await response.json();
				return fail(500, { success: false, message: data.message });
			}
		} catch (e) {
			return fail(500, { success: false, message: 'An error occurred' });
		}
	}
} satisfies Actions;

export const load: PageServerLoad = async ({ cookies }) => {
	const token = cookies.get('token');
	if (token === undefined) {
		return redirect(302, '/login');
	}

	let url = 'https://catiaandjamie.love/api/memory';
	if (dev) {
		url = 'http://localhost:5005/api/memory';
	}

	try {
		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const data = await response.json();
			return {
				props: {
					memories: data.memories
				}
			};
		} else {
			const data = await response.json();
			console.log(data);
			return {
				props: {
					memories: []
				}
			};
		}
	} catch (e) {
		return {
			props: {
				memories: []
			}
		};
	}
};
