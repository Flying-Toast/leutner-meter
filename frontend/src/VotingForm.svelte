<script lang="ts">
	import Score from "./Score.svelte";

	export let maxScore: number;

	let score = maxScore / 2;

	function submitVote(e) {
		console.log(e);
	}
</script>

<form on:submit={submitVote} action="javascript:void(0)">
	<div>Your rating:</div>
	<Score {score} outOf={maxScore}/>
	<input type="range" min="0" max={maxScore} bind:value={score}>
	<br>
	<input type="submit" value="Submit Vote">
</form>
