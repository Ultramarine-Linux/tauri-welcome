<<<<<<< HEAD
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
=======
import ViteYaml from '@modyfi/vite-plugin-yaml';

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
    modules: ['@nuxtjs/tailwindcss'],
    ssr: false,
    vite: {
        plugins: [
            ViteYaml()
        ]
    }
})
>>>>>>> ec347dad4f24ccdd7d235c2e12d6c7a0e8f6584d
