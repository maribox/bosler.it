<script lang="ts">
import * as THREE from 'three';
import { onMounted, ref, type Ref } from 'vue';

	const container = ref<HTMLCanvasElement|null>(null);
	onMounted(() => {
	console.assert(container.value != null, container.value)
	console.assert(container.value  instanceof HTMLCanvasElement, container.value);
	const renderer = new THREE.WebGLRenderer({
		canvas: container.value  as HTMLCanvasElement,
		antialias: true,
		alpha: true
	});

});
</script>

<template>
	<canvas is="vue:container" id="earthCanvas"></canvas>
</template>

<style scoped>
body {
	background-color: white;
}
</style>