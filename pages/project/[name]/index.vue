<script lang="ts" setup>
import { nanoid } from 'nanoid';
import { matList, matChecklist } from '@quasar/extras/material-icons';

const route = useRoute();
const projectStore = useProjectStore();
const project = projectStore.find(route.params.name);

const { data: deploymentList } = await useFetch('/api/deployment/list', {
  query: {
    name: route.params.name,
  },
});

async function deploy() {
  const data = await $fetch('/api/deployment/new', {
    method: 'POST',
    body: {
      key: nanoid(),
      projectId: project.id,
    },
  });
  deploymentList.value.unshift(data);
}

const columns = [
  {
    name: 'createdAt',
    label: 'Deploy Time',
    field: 'createdAt',
    align: 'left',
  },
  {
    name: 'key',
    label: 'Deploy Key',
    field: 'key'
  },
  {
    name: 'detail',
    label: 'Detail',
  },
];

const pagination = {
  rowsPerPage: 20,
};

</script>

<template>
  <div class="q-pa-md">
    <q-toolbar>
      <div class="info">
        <p>{{ project.name }}</p>
        <p>Git: {{ project.git }}</p>
      </div>
      <q-space />
      <q-btn icon="send" label="Deploy" @click="deploy" />
    </q-toolbar>
    <q-table
      flat bordered
      title="Deployments"
      :rows="deploymentList"
      :columns="columns"
      row-key="name"
      color="amber"
      :pagination="pagination"
    >
      <template v-slot:body-cell-detail="props">
        <q-td key="detail" :props="props">
          <NuxtLink :to="'/project/' + project.name + '/' + props.row.key">
            <q-icon :name="matList" />
          </NuxtLink>
        </q-td>
      </template>
    </q-table>
  </div>
</template>

<style lang="scss" scoped>
.info p {
  margin: 0;
}
</style>
