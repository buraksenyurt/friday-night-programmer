<script setup lang="ts">
const { message, messageType, showAlert, clearAlert, autoDismiss } = useAlert()
</script>
<template>
  <div>
    <NuxtLayout>
      <NuxtPage />

      <AlertBox v-if="message" :type="messageType" :message="message" :autoDismiss="autoDismiss || undefined"
        @close="clearAlert" />

    </NuxtLayout>
  </div>
</template>