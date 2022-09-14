<template>
    <main class="min-h-screen w-full h-full bg-libadw-bg text-white flex flex-col items-center justify-between px-8">
        <div class="flex flex-col items-center mt-[45px]">
            <section id="welcome-text" class="flex flex-col items-center">
                <img src="@/static/ultramarine-logo.svg" alt="Ultramarine Logo" class="w-36 h-auto my-4" width="144px" height="auto">
                <h1 class="text-4xl font-semibold tracking-tight mb-1">Welcome to Ultramarine Linux</h1>
                <p class="mb-8">Welcome to Ultramarine Linux! We hope you enjoy this new experience.</p>
            </section>
            
            <section id="action-select" class="flex flex-row w-full items-center justify-center gap-8 pb-4">
                <nuxt-link to="/pkg-install" class="action-card">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                    <span>Install Software</span>
                </nuxt-link>
                <a href="#" class="action-card">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                    </svg>
                    <span>Open Tweaks App</span>
                </a>
                <a href="https://wiki.ultramarine-linux.org" target="_blank" class="action-card">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8 14v3m4-3v3m4-3v3M3 21h18M3 10h18M3 7l9-4 9 4M4 10h16v11H4V10z" />
                    </svg>
                    <span>Read Documentation</span>
                </a>
            </section>

            <section id="autostart-settings" class="flex flex-row items-center gap-2">
                <input type="checkbox" v-model="autostartEnabled">
                <span>Show on startup</span>
            </section>
        </div>
        <div>
            <section id="links" class="pb-8 flex flex-row gap-4">
                <a href="https://discord.com/invite/bUuQasHdrF" target="_blank">Discord</a>
                <a href="https://github.com/ultramarine-linux" target="_blank">GitHub</a>
                <a href="https://ultramarine-linux.org" target="_blank">Website</a>
            </section>
        </div>
        
    </main>
</template>

<script setup>
const autostartEnabled = ref(true)

watch(autostartEnabled, (newValue) => {
    console.log(newValue);
    // add rust invoking logic here to add/remove .desktop file from ~/.config/autostart
})
</script>

<style scoped>
.action-card {
    @apply flex flex-col items-center justify-center w-56 p-4 font-bold text-lg bg-libadw-btn hover:bg-libadw-btn-hover rounded-md
}

#links a{
    @apply hover:text-blue-500
}
</style>