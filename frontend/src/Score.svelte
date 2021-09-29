<script lang="ts">
	export let score: number;
	export let outOf: number;
</script>

<div>
	<span id="numerator">{score ? score : ".."}</span> <span id="slash">/</span> <span id="denominator">{outOf}</span>
</div>

<style>
	* {
		font-weight: bold;
	}

	#numerator {
		font-size: 45px;
		color: #35BFE6;
	}

	#slash, #denominator {
		color: #312FDA;
		font-size: 24px;
	}

	#slash {
		font-size: 30px;
	}
</style>
