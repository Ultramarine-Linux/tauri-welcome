<template>
    <div class="p-4 bg-gray-800 rounded-md">
        <h1 class="text-lg font-bold tracking-tighter">{{ packageName }}</h1>
        <p>{{ description }}</p>
    </div>
</template>

<script>
export default {
    props: {
        packageName: {
            type: String,
            required: true
        },
        description: {
            type: String,
            required: true
        }
    }
}
</script>
