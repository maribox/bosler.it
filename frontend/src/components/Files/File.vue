<script lang="ts" setup>
// get props from parent
import type * as env from '@/types'
import {IconTrash} from "@iconify-prerendered/vue-mdi";
import {IconDownload} from "@iconify-prerendered/vue-tabler";
import {IconEdit} from "@iconify-prerendered/vue-material-symbols";
import {ref} from "vue";

const props = defineProps<{ file: env.File }>();
// set fileSize to string that represents the size in KB, MB or GB or TB with if statement
let fileSize: string;
if (props.file.sizeInB < 1000) {
  fileSize = props.file.sizeInB + ' B';
} else if (props.file.sizeInB < 1000000) {
  fileSize = (props.file.sizeInB / 1000).toFixed(2) + ' KB';
} else if (props.file.sizeInB < 1000000000) {
  fileSize = (props.file.sizeInB / 1000000).toFixed(2) + ' MB';
} else if (props.file.sizeInB < 1000000000000) {
  fileSize = (props.file.sizeInB / 1000000000).toFixed(2) + ' GB';
} else {
  fileSize = (props.file.sizeInB / 1000000000000).toFixed(2) + ' TB';
}

const downloadFile = (url: string) => {
  window.open(url, '_blank');
};

async function downloadFileWithAnchor(fileURL: string) {
  const response = await fetch(fileURL);
  const fileBlob = await response.blob();
  const objectURL = URL.createObjectURL(fileBlob);

  const anchor = document.createElement("a");
  anchor.href = objectURL;
  anchor.download = "filename.ext";
  anchor.click();
}

let readMore = ref(false);
let displayReadMore = ref(false);
if (props.file.description.length > 100) {
  displayReadMore.value = true;
}
let editRights = ref(false);
</script>
<template>
  <VCard class="mx-auto"
         max-height="300"
         max-width="500"
         variant="tonal"
  >
    <VCard-title>
      {{ props.file.name }}.{{ props.file.type }}
    </VCard-title>
    <VCard-subtitle class="d-flex justify-space-between flex-wrap">
      <div>{{ fileSize }}
      </div>
      <div>
        uploaded:
        {{ props.file.createdAt.toLocaleString() }}
      </div>
    </VCard-subtitle>
    <VCardText v-if="props.file.description">

      {{ props.file.description.substring(0, 115) }}

      {{
        readMore ?
            props.file.description.substring(115, 1020) + (props.file.description.length > 1020 ? '...' : '')
            : ''
      }}
      <span
          v-if="displayReadMore"
          style="color: rgb(var(--v-theme-primary)); cursor: pointer"
          @click="readMore = !readMore"
      >
        {{ readMore ? 'Read less' : 'Read more' }}
      </span>
    </VCardText>
    <VCard-actions class="d-flex justify-end">
      <vBtn
          :disabled="!editRights"
      >
        <IconTrash class="actionIcon"/>
      </vBtn>
      <VBtn
          :disabled="!editRights"
      >
        <IconEdit class="actionIcon"/>
      </VBtn>
      <VBtn
          :disabled="!props.file.downloadUrl"
          color="primary"
          @click="downloadFile(props.file.downloadUrl)"
      >
        <IconDownload class="actionIcon"/>
      </VBtn>
    </VCard-actions>
  </VCard>
</template>
<style scoped>
.actionIcon {
  color: rgb(var(--v-theme-primary));
  height: 1.5rem;
  width: 1.5rem;
}
</style>