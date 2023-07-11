<script lang="ts" setup>
const name = ref();
const git = ref();

const templateTypes = ['Vite', 'Next', 'Nuxt'];
const templateType = ref(templateTypes[0]);
const templateSubTypes = computed(() => templateType.value == 'Vite' ? ['vue', 'react', 'lit'] : []);
const templateSubType = ref();

const type = ref();
const importTypes = [{
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

<template>
  <div class="p-4 gap-1 flex flex-col">
    <UInput placeholder="Project name" v-model="name" />
    <h1>Create from template</h1>
    <USelectMenu placeholder="Template type" v-model="templateType" :options="templateTypes" />
    <USelectMenu placeholder="Template sub type" v-if="templateSubTypes.length" v-model="templateSubType" :options="templateSubTypes" />
    
    <h1>Import git repository</h1>
    <UInput placeholder="Git url" v-model="git" />
    <h2>Project type</h2>
    <URadio v-for="t of importTypes" :key="t.name" :model-value="type" @update:model-value="type = t.value" v-bind="t" />
    <UButton @click="create">Create project</UButton>
  </div>
</template>
