import { defineStore } from "pinia";

export const useFormStore = defineStore('form', {
    state: () => ({
        formData: {} as Issue
    }),
    actions: {
        setFormData(data: Issue) {
            this.formData = data
        }
    }
})