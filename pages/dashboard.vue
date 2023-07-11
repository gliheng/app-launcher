<template>
  <main class="p-4">
    <div class="mb-2">Projects</div>
    <div class="flex gap-1">
      <NuxtLink v-for="item of data" :to="'/project/' + item.name">
        <UButton type="primary">
          {{ item.name }}
        </UButton>
      </NuxtLink>
    </div>
  </main>
  <CreateProject />
</template>

<script setup>
const { data } = await useFetch('/api/project/list')

</script>
