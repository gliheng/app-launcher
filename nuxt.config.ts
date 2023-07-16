// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: [
    'assets/css/global.css',
  ],
  devtools: { enabled: true },
  modules: [
    'nuxt-quasar-ui',
    '@pinia/nuxt',
  ],
  quasar: {
    plugins: [
      'Dialog',
    ],
  },
  pinia: {
    autoImports: [
      'defineStore',
      'storeToRefs',
    ],
  },
})
