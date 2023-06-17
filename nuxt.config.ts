// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  css: [
    "assets/theme.css",
    "primevue/resources/primevue.css",
    "/node_modules/primeflex/primeflex.css"
  ],
  build: {
    transpile: ["primevue"],
  },
});
