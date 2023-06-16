// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  css: [
    "primevue/resources/themes/lara-light-blue/theme.css",
    "primevue/resources/primevue.css",
  ],
  build: {
    transpile: ["primevue"],
  },
});
