<script setup lang="ts">
defineProps({
  show: {
    type: Boolean,
    required: true,
  },
});

const emit = defineEmits(["clickResume", "clickHome"]);
const clickResume = () => {
  emit("clickResume");
};
const clickHome = () => {
  emit("clickHome");
};
</script>

<template>
  <div v-if="show">
    <div class="fixed top-0 left-0 w-full h-full">
      <div class="bg-gray-500 opacity-60 w-full h-full fixed"></div>
      <div
        class="absolute bg-white rounded-md top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-5 flex flex-col"
      >
        <div class="flex-1">
          <button
            class="bg-green-500 hover:bg-green-700 text-white py-2 w-40 uppercase rounded-md mb-3"
            @click="clickResume"
          >
            <span>Resume</span>
          </button>
        </div>
        <div class="flex-1">
          <button
            class="bg-red-500 hover:bg-red-700 text-white py-2 w-40 uppercase rounded-md"
            @click="clickHome"
          >
            <span>Home</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
