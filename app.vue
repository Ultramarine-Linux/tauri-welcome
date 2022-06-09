<template>
    <div>  
        <div data-tauri-drag-region class="titlebar">
            <a @click="$router.go(-1)" class="titlebar-button titlebar-button-back flex justify-center items-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                </svg>
            </a>
            <h1 data-tauri-drag-region class='my-1.5 font-semibold'>Ultramarine Welcome</h1>
            <button @click="closeApp">
                <svg class="text-white titlebar-button titlebar-button-close" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24" style="-ms-transform: rotate(360deg); -webkit-transform: rotate(360deg); transform: rotate(360deg);"><path fill="currentColor" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z"/></svg>
            </button>
        </div>
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
        border-radius: 10px;
        overflow: hidden;
    }

    .titlebar {
        min-height: 30px;
        background: #303030;
        color: white;
        user-select: none;
        display: flex;
        justify-content: space-between;
        position: fixed;
        width: 100%;
        border-top-left-radius: 10px;
        border-top-right-radius: 10px;
        border: 2px solid #3c3c3c;
        padding: 0 10px;
    }
    .titlebar-button {
        background-color: #444444;
        flex-shrink: none;
    }

    .titlebar-button-close {
        border-radius: 100%;
        width: 20px;
        height: auto;
        padding: 3px;
    }

    .titlebar-button-back {
        margin: 2px 0;
        padding: 0 8px;
        border-radius: 6px;
    }

    .titlebar-button:hover {
        background: #5f5f5f;
    }
</style>
