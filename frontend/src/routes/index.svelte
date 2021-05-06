<script lang="ts">
	import { apiData, transactions } from '../store';

	interface Transaction {
		id: string;
		initial_balance: string;
		transport_fare: string;
		card_id: string;
	}

	let poolHandler;

	let transactions_data: Promise<Transaction[]>;

	const setupFetcher = () => {
		if (poolHandler) {
			clearInterval(poolHandler);
		}
		poolHandler = setInterval(() => getTransactions(), 1000);
	};

	const getTransactions = () => {
		fetch('http://localhost:8000/transactions')
			.then((data: any) => {
				apiData.set(data);
			})
			.catch((e: any) => {
				console.log(e);
				return [];
			});
	};

	$: setupFetcher();
</script>

<svelte:head>
	<title>RFID | Dashboard</title>
</svelte:head>

<main class="h-screen text-white flex justify-center items-center flex-col">
	<h1 class="font-black text-3xl mb-6">RFID Transactions</h1>

	<table class="border-collapse border border-gray-300 w-1/2">
		<tr>
			<th class="border border-gray-300">No</th>
			<th class="border border-gray-300">Initial Balance</th>
			<th class="border border-gray-300">Transaction fee</th>
			<th class="border border-gray-300">Customer id</th>
		</tr>

		<!-- {#await transactions_data then transactions_data} -->
		{#each $transactions as tran, i}
			<tr>
				<td class="border border-gray-300">{i + 1}</td>
				<td class="border border-gray-300">{tran.initial_balance}</td>
				<td class="border border-gray-300">{tran.transport_fare}</td>
				<td class="border border-gray-300">{tran.card_id}</td>
			</tr>
		{/each}
		<!-- {/await} -->
	</table>
</main>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Poppins:wght@200&display=swap');
	main {
		background-color: #202124;
		font-family: 'Poppins', sans-serif;
	}

	td,
	th {
		padding: 8px;
	}
</style>
