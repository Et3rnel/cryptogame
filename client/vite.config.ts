import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	host: true, // Bind to all network interfaces
	port: 5173,
	plugins: [sveltekit()]
});
