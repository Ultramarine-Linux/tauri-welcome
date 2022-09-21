<template>
    <div class="p-4 bg-libadw-btn rounded-md">
        <h1 class="text-lg font-bold tracking-tighter">{{ packageName }}</h1>
        <p>{{ description }}</p>
    </div>
</template>

<script>
export default {
    props: {
        packageName: {
            type: Text,
            required: true
        },
        description: {
            type: Text,
            required: true
        }
    }
}
</script>
