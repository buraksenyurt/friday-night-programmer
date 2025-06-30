import { createPinia } from "pinia";

// Server Side Rendering destekli state yöneticisi olarak pinia kullanılmaktadır.
// plugin olarak middleware'e aşağıdaki gibi eklenebilir.
export default defineNuxtPlugin(app => {
    const pinia = createPinia()
    app.vueApp.use(pinia)
})