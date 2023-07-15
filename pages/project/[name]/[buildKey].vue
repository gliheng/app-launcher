<script lang="ts" setup>
const route = useRoute();
</script>

<template>
  <div class="q-pa-md">
    <h5>Deployment {{ route.params.buildKey }}</h5>
    <div class="q-pa-md">
      <q-list bordered>
        <q-expansion-item
          group="somegroup"
          icon="explore"
          label="Deployment"
          header-class="text-primary"
        >
          <q-card>
            <q-card-section>
              Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quidem, eius reprehenderit eos corrupti
              commodi magni quaerat ex numquam, dolorum officiis modi facere maiores architecto suscipit iste
              eveniet doloribus ullam aliquid.
            </q-card-section>
          </q-card>
        </q-expansion-item>
      </q-list>
    </div>
  </div>
</template>
