<template>
  <main class="q-pa-md">
    <q-toolbar>
      Projects
      <q-space />
      <q-btn @click="showCreate">Create</q-btn>
    </q-toolbar>
    <div class="project-list">
      <NuxtLink v-for="item of projectList" :to="'/project/' + item.name">
        <q-btn color="primary">
          {{ item.name }}
        </q-btn>
      </NuxtLink>
    </div>
  </main>
</template>

<script setup>
import CreateProject from '~/components/CreateProject.vue';
import { useQuasar } from 'quasar'

const projectStore = useProjectStore();
const { list: projectList } = storeToRefs(projectStore);
const $q = useQuasar()

function showCreate() {
  $q.dialog({
    component: CreateProject,
    componentProps: {},
  }).onOk(() => {
    console.log('OK')
  }).onCancel(() => {
    console.log('Cancel')
  }).onDismiss(() => {
    console.log('Called on OK or Cancel')
  })
}
</script>

<style lang="scss" scoped>
  .project-list {
    display: flex;
    gap: 4px;
  }
</style>
