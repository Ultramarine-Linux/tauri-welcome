<template>
    <div>
        <NuxtLayout>
            <NuxtPage/>
        </NuxtLayout>
    </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window'

const closeApp = (() => {
    appWindow.close()
})
</script>


<style>
    html, body {
        overflow: hidden;
    }
</style>
