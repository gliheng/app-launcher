<script setup>
const projectStore = useProjectStore();
await projectStore.fetch();
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
