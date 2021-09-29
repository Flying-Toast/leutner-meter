<script lang="ts">
	export let score: number;
	export let outOf: number;
</script>

<div>
	<span id="numerator">{score}</span> <span>/ {outOf}</span>
</div>

<style>
	#numerator {
		font-size: 45px;
	}
</style>
