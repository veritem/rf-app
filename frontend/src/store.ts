import { derived, writable } from 'svelte/store';

export const apiData = writable([]);

export const transactions = derived(apiData, ($apiData) => {
	return $apiData;
});
