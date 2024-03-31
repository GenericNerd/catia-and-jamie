import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { dev } from '$app/environment';

export const actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const username = data.get('username');
		if (username === null) {
			return fail(400, { success: false, message: 'No username provided' });
		}
		const password = data.get('password');
		if (password === null) {
			return fail(400, { success: false, message: 'No password provided' });
		}

		let url = 'https://catiaandjamie.love/api/auth/login';
		if (dev) {
			url = 'http://localhost:5005/api/auth/login';
		}

		try {
			const response = await fetch(url, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username,
					password
				})
			});

			if (response.ok) {
				const data = await response.json();
				cookies.set('token', data.token, { path: '/', maxAge: 60 * 60 * 24 * 7 });
			} else {
				const data = await response.json();
				return fail(500, { success: false, message: data.message });
			}
		} catch (e) {
			return fail(500, { success: false, message: 'An error occurred' });
		}
		return redirect(302, '/review');
	}
} satisfies Actions;

export const load: PageServerLoad = async ({ cookies }) => {
	if (cookies.get('token') !== undefined) {
		return redirect(302, '/review');
	}
};
