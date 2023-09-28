<script setup lang="ts">
document.addEventListener("contextmenu", (event) => event.preventDefault());
</script>

<template>
  <div class="fixed w-full h-full">
    <router-view />
  </div>
</template>
