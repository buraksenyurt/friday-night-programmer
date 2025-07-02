<script setup lang="ts">
const { message, messageType, showAlert, clearAlert, autoDismiss } = useAlert()
</script>
<template>
  <NuxtLayout name="master">
    <NuxtPage />
    <AlertBox v-if="message" :type="messageType" :message="message" :autoDismiss="autoDismiss || undefined"
      @close="clearAlert" />
  </NuxtLayout>
</template>
