<template>
  <div>
    <NavBar class="nav-bar">
      <slot name="menu" />
    </NavBar>
    <slot />
  </div>
</template>

<style lang="scss" scoped>
.nav-bar {
  margin: 10px;
}
</style>
