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
