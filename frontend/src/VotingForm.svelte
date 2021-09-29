<script lang="ts">
	import Score from "./Score.svelte";

	export let maxScore: number;

	let score = maxScore / 2;
	let submitted = false;

	function submitVote(e) {
		e.preventDefault();
		if (submitted) return;

		let prom = fetch(`http://${location.hostname}:8080/vote`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				score: score
			})
		});
		submitted = true;

		prom.then(resp => {
			if (resp.status != 200) {
				resp.text()
					.then(txt => alert(`Error submitting vote: ${txt}`));
			} else {
				window.location = "";
			}
		});
	}
</script>

<form on:submit={submitVote} action="javascript:void(0)">
	<div>Your rating:</div>
	<Score {score} outOf={maxScore}/>
	<input type="range" min="0" max={maxScore} bind:value={score}>
	<br>
	<input type="submit" value="Submit Vote">
</form>
