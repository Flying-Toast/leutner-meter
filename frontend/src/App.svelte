<script lang="ts">
	import Gauge from "./Gauge.svelte";
	import Header from "./Header.svelte";
	import Score from "./Score.svelte";
	import Loader from "./Loader.svelte";
	import CurrentMeal from "./CurrentMeal.svelte";
	import VotingForm from "./VotingForm.svelte";

	const maxScore = 10;

	let scoresTotal: number;
	let numVotes: number;
	let mealInProgress: boolean;
	let currentMeal: string;
	let finishedApiFetch = false;
	let hash: string = location.hash;

	$: score = numVotes > 0 ? Math.round(scoresTotal / numVotes) : null;

	addEventListener("hashchange", () => {
		hash = location.hash;
	});

	Promise.all([
		fetch(`http://${location.hostname}:8080/stats`, { method: "GET" }),
		// delay the loading so that the spinner shows for longer
		new Promise(resolve => setTimeout(resolve, 800)),
	])
		.catch(err => {
			alert("Error connecting to server. Either my roommate tripped over the power cord, or Case's shitty wifi is acting up again.");
		})
		.then(([resp, ..._]) => resp.json())
		.then(data => {
			finishedApiFetch = true;
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

	{#if finishedApiFetch}
		{#if mealInProgress}
			{#if hash == "#vote"}
				<VotingForm {maxScore}/>
			{:else}
				<Gauge min={0} max={maxScore} value={score}/>
				<Score {score} outOf={maxScore}/>
				<div class="num-votes">Based on {numVotes} votes</div>
			{/if}
		{/if}

		<div class="current-meal-wrapper">
			<hr>
			<CurrentMeal meal={currentMeal}/>
		</div>
	{:else}
		<Loader/>
	{/if}
</div>

<style>
	.center {
		text-align: center;
	}

	.current-meal-wrapper {
		display: inline-block;
	}

	hr {
		border: 0.5px solid #838383;
		border-radius: 1px;
	}

	.num-votes {
		font-size: 10px;
	}
</style>
