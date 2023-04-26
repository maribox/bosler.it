<script setup lang="ts">
import { useRouter } from 'vue-router';

import {IconBuild, IconFileArrowUpDown} from '@iconify-prerendered/vue-mdi'
import {IconFileInfo} from "@iconify-prerendered/vue-tabler";
import {IconPerson} from "@iconify-prerendered/vue-ion";



const router = useRouter();

const buttons = [
	{
		url: '/tools',
		icon: IconBuild,
		text: 'Tools',
	},
	{
		url: '/files',
		icon: IconFileArrowUpDown,
		text: 'Dateien',
	},
	{
		url: '/aboutme',
		icon: IconFileInfo,
		text: 'Über mich',
	},
	{
		url: '/me',
		icon: IconPerson,
		text: 'Mein Bereich',
	},
];

function handleRedirect(route: string) {
	console.log("test")
	router.push(route);
}
</script>

<template>
	<div class="ButtonArray d-flex align-center justify-space-around flex-wrap text-white">
		<div
		v-for="(button, index) in buttons" 
		:key="index" 
		@click="handleRedirect(button.url)"
		class="Button mx-auto pa-6 d-flex align-center justify-space-around">
			<div class="buttonIcon">
				<component :is="button.icon" class="text-white buttonIcon"/>
			</div>
			<div>	
			{{ button.text }}
			</div>
		</div>
	</div>
</template>

<style scoped>
.Button {
	background-color: #1e1e1e;
	border: #1e1e1e 1px solid;
	border-radius: 10px;
  padding: 10px 10px 10px 5px;
  margin: 10px;
	transition: all 0.2s;
	box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.5);
	width: 5%;
	min-width: max-content;
	font-size: 1.3rem;
	user-select: none;
}

.Button:hover {
	background-color: #2e2e2e;
	border: #2e2e2e 1px solid;
	box-shadow: 0 0 10px 0 rgba(0, 0, 0, 1);
}

.buttonIcon {
	margin-right: 15px;
	height: 2.4rem;
	width: 2.4rem;
}
</style>