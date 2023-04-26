<script setup lang="ts">
import { ref } from 'vue'
// text should be a reactive variable
const textInLanguages = [
	"Hi, ich bin Marius!",
	"Hi, I'm Marius!",
	"Salut, je suis Marius!",
	"¡Hola, soy Marius!",
	"Привет, я Мариус!",
	"你好，我是马里乌斯！",
	"こんにちは、私はマリウスです！",
	"안녕하세요, 나는 마리우스입니다!",
	"नमस्ते, मैं मारियस हूं!",
]
const index = ref(0)
setInterval(() => {
	index.value = (index.value + 1) % textInLanguages.length
}, 3000)

</script>

<template>
		<h1>{{ textInLanguages[index] }}</h1>
</template>

<style scoped>
h1 {
	min-width: 30rem;
	height: fit-content;
	font-weight: 300;
  font-family: papyrus,fantasy, sans-serif;
	color: #ffffff;
	text-transform: uppercase;
	text-rendering: optimizeLegibility;
	text-align: center;
}
</style>