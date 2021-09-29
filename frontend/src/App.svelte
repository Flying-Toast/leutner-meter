<script lang="ts">
	import Gauge from "./Gauge.svelte";
	import Header from "./Header.svelte";
	import Score from "./Score.svelte";
	import Loader from "./Loader.svelte";

	const maxScore = 10;

	let scoresTotal: number;
	let numVotes: number;
	let currentMeal: string;
	let finishedFetch: boolean = false;

	//$: score = numVotes > 0 ? Math.round(scoresTotal / numVotes) : null;
	let score = 4;

	Promise.all([
		fetch(`http://${location.hostname}:8080/stats`, { method: "GET" }),
		// delay the loading so that the spinner shows for longer
		new Promise(resolve => setTimeout(resolve, 800)),
	])
		.then(([resp, ..._]) => resp.json())
		.then(data => {
			finishedFetch = true;
			if (data.currentMeal) {
				scoresTotal = data.scoresTotal;
				numVotes = data.numVotes;
				currentMeal = data.currentMeal;
				mealInProgress = true;
			} else {
				mealInProgress = false;
			}
		});
</script>

<div class="center">
	<Header/>

	{#if finishedFetch}
		<Gauge min={0} max={maxScore} value={score}/>
		<Score {score} outOf={maxScore}/>
	{:else}
		<Loader/>
	{/if}
</div>

<style>
	.center {
		text-align: center;
	}
</style>
