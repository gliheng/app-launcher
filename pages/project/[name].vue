<template>
  <h1>{{ data.name }}</h1>
  <h2>{{ data.git }}</h2>
</template>

<script setup>
const route = useRoute();

const { data } = await useFetch('/api/project/get', {
  query: {
    name: route.params.name,
  },
});
</script>
