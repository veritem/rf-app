<script lang="ts">
	import { transactions } from '../store';
</script>

<svelte:head>
	<title>RFID | Dashboard</title>
</svelte:head>

<main class="h-screen text-white flex justify-center items-center flex-col">
	<h1 class="font-black text-3xl mb-6">RFID Transactions</h1>

	{#await $transactions}
		<p />
	{:then transaction}
		<table class="border-collapse border border-gray-300 w-1/2">
			<tr>
				<th class="border border-gray-300">UUID</th>
				<th class="border border-gray-300">Initial Balance</th>
				<th class="border border-gray-300">Transaction fee</th>
			</tr>

			{#each transaction as tran, i}
				<tr>
					<td class="border border-gray-300">{i + 1}</td>
					<td class="border border-gray-300">{tran.initial_balance}</td>
					<td class="border border-gray-300">{tran.transport_fare}</td>
				</tr>
			{/each}
		</table>
	{:catch error}
		<p class="text-red-400">{error.message}</p>
	{/await}
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
