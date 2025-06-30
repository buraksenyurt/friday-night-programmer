import { ref } from 'vue'

const message = ref('')
const messageType = ref<'success' | 'error' | 'warning' | 'info'>('info')

export function useAlert() {

    const showAlert = (type: typeof messageType.value, text: string) => {
        messageType.value = type,
            message.value = text
    }
    const clearAlert = () => {
        message.value = ''
    }

    return {
        message,
        messageType,
        showAlert,
        clearAlert
    }
}