<script setup lang="ts">
import PauseOverlay from "../components/PauseOverlay.vue";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import router from "../router";

interface Sun {
  x_pos: number;
  y_pos: number;
  bx_pos: number;
  by_pos: number;
  value: number;
  label: number;
  is_freeze: boolean;
  is_repeat: boolean;
}

enum ANSWER {
  PASS,
  FAIL,
  LOOKING,
}

const isGameEnd = ref(false);
let isFirstClick = false;
let isFocusPos = false;
let focusedXPos = 0;
let focusedYPos = 0;

let level = ref(0);

const showResume = ref(false);

const sunItem = ref([] as Sun[][]);

const isCorrectAll = ref(ANSWER.LOOKING);

const counterTimer = ref("00:00");
let sec = 0;
const counter = setInterval(() => {
  if (!showResume.value) {
    sec++;
    const s = `${sec % 60}`.padStart(2, "0");
    const m = `${Math.floor(sec / 60)}`.padStart(2, "0");
    counterTimer.value = `${m}:${s}`;
  }
}, 1000);

invoke("get_board_item").then((data) => {
  sunItem.value = data as Sun[][];
});

invoke("get_level").then((data) => {
  level.value = data as number;
});

window.addEventListener("keyup", (ev: KeyboardEvent) => {
  if (isCorrectAll.value == ANSWER.PASS) {
    return;
  }
  switch (ev.code) {
    case "Digit1":
      setValueAtPos(focusedXPos, focusedYPos, 1);
      break;
    case "Digit2":
      setValueAtPos(focusedXPos, focusedYPos, 2);
      break;
    case "Digit3":
      setValueAtPos(focusedXPos, focusedYPos, 3);
      break;
    case "Digit4":
      setValueAtPos(focusedXPos, focusedYPos, 4);
      break;
    case "Digit5":
      setValueAtPos(focusedXPos, focusedYPos, 5);
      break;
    case "Digit6":
      setValueAtPos(focusedXPos, focusedYPos, 6);
      break;
    case "Digit7":
      setValueAtPos(focusedXPos, focusedYPos, 7);
      break;
    case "Digit8":
      setValueAtPos(focusedXPos, focusedYPos, 8);
      break;
    case "Digit9":
      setValueAtPos(focusedXPos, focusedYPos, 9);
      break;
  }
});

const setValueAtPos = async (xPos: number, yPos: number, value: number) => {
  isCorrectAll.value = ANSWER.LOOKING;
  if (isFocusPos) {
    await invoke("set_item_value", { xPos, yPos, value });
    await invoke("get_board_item").then((data) => {
      sunItem.value = data as Sun[][];
    });
  }
};

const boardItems = ref();

let selectedItem: HTMLInputElement;
const selectItem = (event: Event, sun: Sun) => {
  isFirstClick = true;
  if (sunItem.value[focusedXPos][focusedYPos].is_repeat) {
    clearRepeat();
  }
  if (isCorrectAll.value == ANSWER.PASS) {
    return;
  }
  isCorrectAll.value = ANSWER.LOOKING;
  if (sun.is_freeze) {
    isFocusPos = false;
    return;
  }
  const el = event.target as HTMLInputElement;

  if (selectedItem != undefined) {
    selectedItem.classList.remove("border-4");
    selectedItem.classList.remove("border-blue-500");
  }

  selectedItem = el;
  selectedItem.classList.add("border-4");
  selectedItem.classList.add("border-blue-500");

  isFocusPos = true;
  focusedXPos = sun.x_pos;
  focusedYPos = sun.y_pos;
};

const clearRepeat = async () => {
  await invoke("clear_repeat");
  await invoke("get_board_item").then((data) => {
    sunItem.value = data as Sun[][];
  });
};

const checkAnswer = async () => {
  if (!isFirstClick) {
    return;
  }
  isCorrectAll.value = ANSWER.LOOKING;
  await invoke("check_answer_at", { xPos: focusedXPos, yPos: focusedYPos });
  await invoke("get_board_item").then((data) => {
    sunItem.value = data as Sun[][];
  });
};

const submitAnswer = async () => {
  const isCorrect = await invoke("submit_answer");
  isCorrectAll.value = isCorrect ? ANSWER.PASS : ANSWER.FAIL;
  if (isCorrect) {
    isGameEnd.value = true;
    clearInterval(counter);
    if (selectedItem != undefined) {
      selectedItem.classList.remove("border-4");
      selectedItem.classList.remove("border-blue-500");
    }
  }
  if (!isCorrect) {
    setTimeout(() => {
      isCorrectAll.value = ANSWER.LOOKING;
    }, 200);
    setTimeout(() => {
      isCorrectAll.value = ANSWER.FAIL;
    }, 400);
    setTimeout(() => {
      isCorrectAll.value = ANSWER.LOOKING;
    }, 600);
  }
};

const toggleResumePause = () => {
  showResume.value = !showResume.value;
};

const toHome = () => {
  router.push({ path: "/" });
};
const toLevel = () => {
  router.push({ path: "/level-select" });
};
</script>

<template>
  <div class="w-full h-full p-10">
    <div class="flex justify-between">
      <h1 class="text-green-500 text-2xl uppercase font-bold cursor-default">
        Sudoku
      </h1>
      <div class="flex items-center cursor-default">
        <img class="inline-block mb-1 mr-2" src="../assets/timer.svg" />
        <div class="text-xl inline-block w-16">{{ counterTimer }}</div>
      </div>
    </div>
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
      <div class="border border-black w-fit m-auto">
        <div v-for="[rindex, row] in sunItem.entries()">
          <span
            ref="boardItems"
            class="relative w-7 h-7 inline-block text-center align-middle"
            :class="!isGameEnd ? 'cursor-pointer' : 'cursor-default'"
            v-for="[sindex, sun] in row.entries()"
          >
            <div
              v-if="isCorrectAll != ANSWER.FAIL"
              class="absolute w-full h-full border-gray-400 border-r border-b"
            ></div>
            <div
              :class="`pointer-events-none absolute w-full h-full z-40 ${
                sun.y_pos % level == level - 1 && sindex != row.length - 1
                  ? 'border-r-black border-r'
                  : ''
              } ${
                sun.x_pos % level == level - 1 && rindex != sunItem.length - 1
                  ? 'border-b-black border-b'
                  : ''
              }`"
            ></div>
            <div
              class="absolute w-full h-full z-50"
              @click="selectItem($event, sun)"
            ></div>
            <div
              class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-0"
              v-if="sun.value != 0"
              :class="sun.is_freeze ? 'font-bold text-gray-500' : 'font-normal'"
            >
              {{ sun.value }}
            </div>
            <div
              class="absolute w-full h-full -z-10"
              :class="
                sun.is_repeat || isCorrectAll == ANSWER.FAIL ? 'bg-red-500' : ''
              "
            ></div>
            <div
              class="absolute w-full h-full -z-10"
              :class="isCorrectAll == ANSWER.PASS ? 'bg-green-500' : ''"
            ></div>
          </span>
        </div>
      </div>
    </div>
    <div class="absolute bottom-10 right-1/2 translate-x-1/2">
      <div v-if="isCorrectAll != ANSWER.PASS" class="relative">
        <div class="w-full">
          <div>
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 uppercase rounded-md mr-10"
              @click="checkAnswer"
            >
              <span>Check</span>
            </button>
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 uppercase rounded-md"
              @click="submitAnswer"
            >
              Finish
            </button>
          </div>
        </div>
      </div>
      <div v-if="isCorrectAll == ANSWER.PASS" class="relative">
        <div class="w-full text-center">
          <button
            class="bg-green-500 hover:bg-green-700 text-white py-2 px-4 w-28 uppercase rounded-md mb-3"
            @click="toLevel"
          >
            <span>Restart</span>
          </button>
        </div>
        <div class="w-full text-center">
          <button
            class="bg-red-500 hover:bg-red-700 text-white py-2 px-4 w-28 uppercase rounded-md"
            @click="toHome"
          >
            <span>Menu</span>
          </button>
        </div>
      </div>
    </div>
    <div v-if="!isGameEnd" class="absolute bottom-10 right-10">
      <button @click="toggleResumePause">
        <img src="../assets/pause_circle.svg" />
      </button>
    </div>
    <PauseOverlay
      @click-resume="toggleResumePause"
      @click-home="toHome"
      :show="showResume"
    />
  </div>
</template>
