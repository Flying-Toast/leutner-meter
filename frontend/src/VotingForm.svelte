<script lang="ts">
	import Score from "./Score.svelte";

	export let maxScore: number;

	let score = Math.round(maxScore / 2);
	let submitted = false;

	fetch(`${location.protocol}//${location.host}/check-ticket`, { method: "GET" })
		.catch(err => {
			alert("Error connecting to server. Either my roommate tripped over the power cord, or Case's shitty wifi is acting up again.");
		})
		.then(resp => resp.json())
		.then(data => {
			if (!data.is_valid) {
				location = `https://login.case.edu/cas/login?service=${location.protocol}//${location.host}/sso-auth`;
			}
		});

	function submitVote(e) {
		e.preventDefault();
		if (submitted) return;

		let prom = fetch(`${location.protocol}//${location.host}/vote`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				score: score
			})
		});
		submitted = true;

		prom
			.catch(err => {
				alert("Error connecting to server. Either my roommate tripped over the power cord, or Case's shitty wifi is acting up again.");
			})
			.then(resp => {
				if (resp.status != 200) {
					resp.text()
						.then(txt => alert(`Error submitting vote: ${txt}`));
				} else {
					location = "/";
				}
			});
	}
</script>

<form on:submit={submitVote} action="javascript:void(0)">
	<div>Your rating:</div>
	<Score {score} outOf={maxScore}/>
	<input type="range" min="0" max={maxScore} bind:value={score}>
	<br>
	<input type="submit" value="Submit Vote" disabled={submitted}>
</form>
