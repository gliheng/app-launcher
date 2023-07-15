// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: [
    'assets/css/global.css',
  ],
  devtools: { enabled: true },
  modules: [
    'nuxt-quasar-ui',
  ],
  quasar: {
    plugins: [
      'Dialog',
    ],
  },
})
