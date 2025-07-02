import { ref } from 'vue'

const message = ref('')
const messageType = ref<'success' | 'error' | 'warning' | 'info'>('info')
const autoDismiss = ref<number | null>(null)

export function useAlert() {
    const showAlert = (
        type: typeof messageType.value,
        text: string,
        duration?: number
    ) => {
        messageType.value = type
        message.value = text
        autoDismiss.value = duration ?? null
    }

    const clearAlert = () => {
        message.value = ''
        autoDismiss.value = null
    }

    return {
        message,
        messageType,
        autoDismiss,
        showAlert,
        clearAlert
    }
}