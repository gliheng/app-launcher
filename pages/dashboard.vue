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
  <div class="p-4 gap-1 flex flex-col">
    <UInput placeholder="Project name" v-model="name" />
    <h1>Create from template</h1>

    
    <h1>Import git repository</h1>
    <UInput placeholder="Git url" v-model="git" />
    <h2>Project type</h2>
    <URadio v-for="t of allTypes" :key="t.name" :model-value="type" @update:model-value="type = t.value" v-bind="t" />
    <UButton @click="create">Create project</UButton>
  </div>
</template>

<script setup>
const { data } = await useFetch('/api/project/list')

const name = ref();
const git = ref();
const type = ref();
const allTypes = [{
  name: 'static',
  value: 'static',
  label: 'Static'
}, {
  name: 'node',
  value: 'node',
  label: 'Node'
}];

async function create() {
  
  const { data } = await useFetch('/api/project/new', {
    method: 'POST',
    body: {
      name: name.value,
      git: git.value,
    },
  })
  console.log('create:', data);
}
</script>
