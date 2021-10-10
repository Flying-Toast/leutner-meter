<script lang="ts">
	console.log("%cAre you a %cd%ce%cv%ce%cl%co%cp%ce%cr%c? Contact srs266 if you want to contribute.", "font-size: 25px;", "font-size: 25px; color: red;", "font-size: 25px; color: orange;", "font-size: 25px; color: yellow;", "font-size: 25px; color: green;", "font-size: 25px; color: #3C3CFF;", "font-size: 25px; color: purple;", "font-size: 25px; color: red;", "font-size: 25px; color: orange;", "font-size: 25px; color: yellow;", "font-size: 25px; color: unset;");

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
		fetch(`${location.protocol}//${location.host}/stats`, { method: "GET" }),
		// delay the loading so that the spinner shows for longer
		new Promise(resolve => setTimeout(resolve, 800)),
	])
		.catch(err => {
			alert("Error connecting to server. Either my roommate tripped over the power cord, or Case's shitty wifi is acting up again.");
		})
		.then(([resp, ..._]) => resp.json())
		.then(data => {
			finishedApiFetch = true;
			if (data.current_meal) {
				scoresTotal = data.scores_total;
				numVotes = data.num_votes;
				currentMeal = data.current_meal;
				mealInProgress = true;
			} else {
				mealInProgress = false;
			}
		});
</script>

<div class="center">
	<div class="wrapper">
		{#if mealInProgress}
			<Header/>
		{/if}

		{#if finishedApiFetch}
			{#if mealInProgress}
				{#if hash == "#do-vote"}
					<VotingForm {maxScore}/>
				{:else}
					<Gauge min={0} max={maxScore} value={score}/>
					{#if numVotes == 0}
						<div>Nobody has rated this meal yet.</div>
					{:else}
						<Score {score} outOf={maxScore}/>
						<div class="num-votes">based on {numVotes} vote{numVotes > 1 ? "s" : ""}</div>
					{/if}
					{#if hash != "#did-vote"}
						<a class="votelink" href="#do-vote">Vote now</a>
					{/if}
				{/if}
				<hr>
			{/if}

			<CurrentMeal meal={currentMeal}/>
		{:else}
			<Loader/>
		{/if}
	</div>
</div>

<style>
	.votelink {
		font-size: 175%;
	}

	.wrapper {
		display: inline-block;
		margin: 4px;
	}

	.center {
		text-align: center;
	}

	hr {
		border: 0.5px solid #838383;
		border-radius: 1px;
	}

	.num-votes {
		font-size: 10px;
	}
</style>
