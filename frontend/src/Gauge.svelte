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

<style>
	#color-swatch {
		animation-name: color-fade;
		animation-duration: 101s;
		animation-play-state: paused;
		width: 0;
		height: 0;
	}

	#track {
		width: 450px;
		height: 40px;
		background-color: #939393;
		border-radius: 8px;
		margin: 4px;
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
			background-color: #EB4747;
		}

		50% {
			background-color: #EBDD47;
		}

		100% {
			background-color: #5AEB47;
		}
	}
</style>
