<script setup lang="ts">
import { ref } from 'vue';

const isRecording = ref(false);

async function startRecording() {
  await window.tauriCommand.invoke('start_recording');
  isRecording.value = true;
}

async function stopRecording() {
  await window.tauriCommand.invoke('stop_recording');
  isRecording.value = false;
}

async function sendRecordingData() {
  await window.tauriCommand.invoke('send_recording_data');
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