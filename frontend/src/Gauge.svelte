<script lang="ts">
	export let min: number;
	export let max: number;
	export let value: number;

	let slider: HTMLElement;
	let colorSwatch: HTMLElement;

	$: percent = Math.round((value / (max - min)) * 100);

	$: if (colorSwatch) {
		colorSwatch.style.animationDelay = `-${percent}s`;
		if (slider) {
			const swatchColor = getComputedStyle(colorSwatch).backgroundColor;
			slider.style.backgroundColor = swatchColor;
			slider.style.width = `${percent}%`;
		}
	}
</script>

<div id="color-swatch" bind:this={colorSwatch}/>

<div id="track">
	<div id="slider" bind:this={slider}/>
</div>

<div>
	<div id="ticks">
		<span class="lower-tick">0</span>
		<span class="upper-tick">10</span>
	</div>
</div>

<style>
	#color-swatch {
		animation-name: color-fade;
		animation-duration: 101s;
		animation-play-state: paused;
		width: 0;
		height: 0;
	}

	#ticks, #track {
		width: 450px;
		max-width: 80vw;
		display: inline-block;
	}

	.lower-tick {
		float: left;
		color: #AD0F0F;
	}

	.upper-tick {
		float: right;
		color: #14AD0F;
	}

	#track {
		height: 40px;
		background-color: #939393;
		border-radius: 8px;
		padding: 6px;
	}

	#slider {
		background-color: red;
		height: 100%;
		border-radius: 4px;
		width: 0;
		transition-property: width, background-color;
		transition-duration: 2s;
	}

	@keyframes color-fade {
		0% {
			background-color: #E02222;
		}

		50% {
			background-color: #E0D822;
		}

		100% {
			background-color: #31E022;
		}
	}
</style>
