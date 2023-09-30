<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import router from "../router";
import LoadingOverlay from "../components/LoadingOverlay.vue";

const boardSize = ref();
const difficulty = ref();
const isLoading = ref(false);

let selectedlevel = 3;
let selectedDifficulty = 40;

const selectBoardSize = (event: Event, level: number) => {
  const el = event.target as HTMLInputElement;
  const children = boardSize.value.children as Element[];
  for (const item of children) {
    item.classList.add("border-white");
  }
  el.classList.remove("border-white");
  selectedlevel = level;
};

const selectDifficulty = (event: Event, level: number) => {
  const el = event.target as HTMLInputElement;
  const children = difficulty.value.children as Element[];
  for (const item of children) {
    item.classList.add("border-white");
  }
  el.classList.remove("border-white");
  selectedDifficulty = level;
};

const start = () => {
  isLoading.value = true;
  setTimeout(async () => {
    await invoke("generate_board", {
      level: selectedlevel,
      difficultyPercentage: selectedDifficulty,
    });

    setTimeout(() => {
      router.push({ path: "/game" });
    }, 1000);
  }, 100);
};
</script>

<template>
  <div class="container">
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
      <h1
        class="text-red-500 text-5xl text-center uppercase font-bold mb-5 cursor-default"
      >
        Level
      </h1>
      <div class="flex my-10 text-lg">
        <div class="border-r pr-4">
          <div ref="boardSize">
            <button
              @click="selectBoardSize($event, 2)"
              id="level-2"
              class="border-2 border-white rounded-full px-3 mr-2"
            >
              4x4
            </button>
            <button
              @click="selectBoardSize($event, 3)"
              id="level-9"
              class="border-2 rounded-full px-3"
            >
              9x9
            </button>
          </div>
        </div>
        <div class="border-l pl-4">
          <div ref="difficulty">
            <button
              @click="selectDifficulty($event, 20)"
              class="border-2 border-white rounded-full px-3 mr-2"
            >
              Easy
            </button>
            <button
              @click="selectDifficulty($event, 40)"
              class="border-2 rounded-full px-3 mr-2"
            >
              Normal
            </button>
            <button
              @click="selectDifficulty($event, 60)"
              class="border-2 border-white rounded-full px-3"
            >
              Hard
            </button>
          </div>
        </div>
      </div>
      <div class="pt-5 text-center">
        <button
          class="bg-green-500 hover:bg-green-700 text-white py-2 w-40 uppercase rounded-md mb-3"
          @click="start"
        >
          <span>Start</span>
        </button>
      </div>
    </div>
    <LoadingOverlay :show="isLoading" />
  </div>
</template>
