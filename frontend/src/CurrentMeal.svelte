<script lang="ts">
	export let meal: string;

	function mealToEmoji(m: string): string {
		switch (m) {
			case "breakfast":
				return "🥞";
			case "brunch":
				return "🥯";
			case "lunch":
				return "🍔";
			case "dinner":
				return "🥘";
		}
	}
</script>

<div class="wrapper">
	{#if meal}
		<div class="emoji">{mealToEmoji(meal)}</div>
		It is currently <span class="meal">{meal}</span>.
	{:else}
		<p>😯</p>
		<h3>There isn't a meal going on at Leutner right now. Check back during meal time.</h3>
	{/if}
</div>

<style>
	.wrapper {
		font-size: 17px;
		margin-bottom: 30px;
	}

	.meal {
		font-weight: bold;
	}

	.emoji {
		font-size: 200%;
	}

	p {
		font-size: 400%;
		margin: 4px;
	}
</style>
