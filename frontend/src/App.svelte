<script lang="ts">
	import Gauge from "./Gauge.svelte";
	let foo = 3;
</script>

<input type="number" bind:value={foo}>

<Gauge min={0} max={10} value={foo}/>
