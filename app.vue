<template>
    <div>
        <NuxtLayout>
            <NuxtPage/>
        </NuxtLayout>
    </div>
</template>

<script setup>
onMounted (() => {
    document.body.classList.add('bg-libadw-bg')
})
</script>


<style>
    .page-enter-active,
    .page-leave-active {
        transition: opacity 0.3s ease;
    }

    .page-enter-from,
    .page-leave-to {
        opacity: 0;
    }
</style>
