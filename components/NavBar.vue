<script setup>
import { matRocketLaunch } from '@quasar/extras/material-icons';
const projectStore = useProjectStore();
const { list: projectList } = storeToRefs(projectStore);
const route = useRoute();
const project = computed(() => projectStore.find(route.params.name));
</script>

<template>
  <nav>
    <NuxtLink to="/">
      <q-btn flat :icon="matRocketLaunch" color="primary" label="App launcher"/>
    </NuxtLink>
    <q-btn-dropdown v-if="project" :label="project.name">
      <q-list>
        <NuxtLink v-for="item of projectList" :to="'/project/' + item.name">
          <q-item clickable v-close-popup>
            <q-item-section>
              <q-item-label>{{ item.name }}</q-item-label>
            </q-item-section>
          </q-item>
        </NuxtLink>
      </q-list>
    </q-btn-dropdown>
  </nav>
</template>

<style lang="scss" scoped>
nav {
  display: flex;
  align-items: center;
  gap: 4px;
}
</style>
