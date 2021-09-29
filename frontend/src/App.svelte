<script lang="ts">
	import Gauge from "./Gauge.svelte";
	import Header from "./Header.svelte";
</script>

<div class="center">
	<Header/>
	<Gauge min={0} max={10} value={8}/>
</div>

<style>
	.center {
		text-align: center;
	}
</style>
