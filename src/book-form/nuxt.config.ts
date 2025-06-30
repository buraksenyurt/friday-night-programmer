// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  imports: {
    dirs: ['composables', 'types', 'services']
  },
  css:['bootstrap/dist/css/bootstrap.min.css']
})
