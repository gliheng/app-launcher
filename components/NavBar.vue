<script setup>
import { fasPaw } from '@quasar/extras/fontawesome-v6';
</script>

<template>
  <nav>
    <NuxtLink to="/">
      <q-btn flat :icon="fasPaw" color="primary" label="Paw"/>
    </NuxtLink>
    <slot />
  </nav>
</template>

<style lang="scss" scoped>
nav {
  display: flex;
  align-items: center;
  gap: 4px;
}
</style>
