<script setup lang="ts">
const { message, messageType, showAlert, clearAlert } = useAlert()
</script>
<template>
  <div>
    <NuxtLayout>
      <NuxtPage />

      <AlertBox v-if="message" :type="messageType" :message="message" @close="clearAlert" />

    </NuxtLayout>
  </div>
</template>
