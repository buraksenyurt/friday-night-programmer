<script setup>
const { message, messageType, clearAlert,showAlert } = useAlert()
</script>

<template>
  <NuxtLayout>
    <NuxtPage />

    <AlertBox
      v-if="message"
      :type="messageType"
      :message="message"
      @close="clearAlert"
    />
  </NuxtLayout>
</template>
