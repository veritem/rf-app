import { readable } from 'svelte/store';

interface Transaction {
	id: string;
	initial_balance: string;
	transport_fare: string;
}

async function getTransactions(): Promise<Transaction[]> {
	const res = await fetch('http://localhost:8000/transactions');
	const data = await res.json();

	return data;
}

let transactions = readable(getTransactions(), function start(set) {
	const interval = setInterval(async () => {
		set(getTransactions());
	}, 500);

	return () => clearInterval(interval);
});

export { transactions };
