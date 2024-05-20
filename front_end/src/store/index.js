import { createStore } from "vuex";
import { ModuleK210 } from "./k210s"

export default createStore({
  namespace : true,
  state() {
  },
  getters: {
  },
  mutations: {
  },
  actions: {
  },
  modules: {
    k210s : ModuleK210
  }
})