<script setup lang="ts">

import type * as env from '@/types'
import {Visibility} from "@/types";

// make request to bosler.it/api/files and set visibility to public and the number of files to 20
//const files = await fetch('https://bosler.it/api/files?visibility=public&limit=20').then(res => res.json());

const file: env.File = {
  name: 'Hallo_lol',
  type: 'pdf',
  sizeInB: 4984654986511685,
  createdAt: new Date(4000000000000),
  downloadUrl: 'https://bosler.it/api/files/1234',
  description: 'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry\'s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.',
  visibility: Visibility.Private,
  uploadedAt: new Date(5000000000000),
}
</script>
<template>
  <main class="d-flex flex-wrap align-center justify-center">
    <File :file = "file"/>
  </main>
</template>
<style scoped>
</style>
