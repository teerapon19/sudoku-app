import { createWebHistory, createRouter } from "vue-router";
import Home from "../views/Home.vue";
import LevelSelect from "../views/LevelSelect.vue";
import GameBoard from "../views/GameBoard.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/level-select",
    name: "LevelSelect",
    component: LevelSelect,
  },
  {
    path: "/game",
    name: "GameBoard",
    component: GameBoard,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;