<template>
    <div class="w-full min-h-screen bg-gray-900 text-white py-8 px-4 h-full">
        <h1 class="font-bold text-4xl">Software installation</h1>
        <h2 class="mb-8">Install all of your favorite software with the click of a button!</h2>
        <section id="package-selection" class="grid grid-cols-2 gap-x-12 gap-y-4">
            <PackagePreviewBox v-for="pkg in software.packages" :packageName="pkg.name" :description="pkg.description"/>
        </section>
    </div>
</template>

<script setup>
import data from "@/software_list.yml"
import { reactive } from "vue"

const software = reactive(data)
</script>
