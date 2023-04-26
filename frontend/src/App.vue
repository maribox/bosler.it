<script setup lang="ts">
import HiHeader from './components/Home/HiHeader.vue'
import { mdiAccount } from '@mdi/js';
import ButtonArray from './components/Home/ButtonArray.vue';

</script>

<template>
  <VApp>
    <main class="d-flex flex-wrap align-center justify-center">
      <router-view></router-view>
    </main>
  </VApp>
</template>

<style>
body {
  background: linear-gradient(120deg, #000000 0%, #77230a 100%);
  overflow: hidden;
}
main {
	position:absolute;
	top:0;
	left:0;
	min-width: 100vw;
	min-height: 100vh;
}
</style>