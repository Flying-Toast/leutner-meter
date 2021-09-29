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
		it is currently <span class="meal">{meal}</span>.
	{:else}
		There isn't a meal going on at Leutner right now. Check back during meal time.
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
</style>
