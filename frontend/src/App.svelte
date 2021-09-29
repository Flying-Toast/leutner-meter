<script lang="ts">
	import Gauge from "./Gauge.svelte";
	import Header from "./Header.svelte";
	import Score from "./Score.svelte";

	const maxScore = 10;

	let scoresTotal: number;
	let numVotes: number;
	$: score = numVotes > 0 ? Math.round(scoresTotal / numVotes) : null;

	fetch(`http://${location.hostname}:8080/stats`, { method: "GET" })
		.then(resp => resp.json())
		.then(data => {
			if (data.currentMeal) {
				scoresTotal = data.scoresTotal;
				numVotes = data.numVotes;
			} else {
				// no meal in progress
			}
		});
</script>

<div class="center">
	<Header/>
	<Gauge min={0} max={maxScore} value={score}/>
	<Score {score} outOf={maxScore}/>
</div>

<style>
	.center {
		text-align: center;
	}
</style>
