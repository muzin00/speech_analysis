<script setup lang="ts">
import { ref } from 'vue';

const isRecording = ref(false);

async function startRecording() {
  await window.tauriCommand.invoke('/recording/start');
  isRecording.value = true;
}

async function stopRecording() {
  await window.tauriCommand.invoke('/recording/stop');
  isRecording.value = false;
}

async function sendRecordingData() {
  await window.tauriCommand.invoke('/recording/send_data');
}
</script>

<template>
  <div v-if="isRecording">
    <button @click="stopRecording">停止</button>
  </div>
  <div v-else>
    <button @click="startRecording">録音</button>
    <button @click="sendRecordingData">録音データ解析</button>
  </div>
</template>