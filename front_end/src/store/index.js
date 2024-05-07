import { createStore } from "vuex";
import { ModuleK210 } from "./k210s"

export default createStore({
  namespace : true,
  state() {
    return {
      a : 1
    } 
  },
  getters: {
  },
  mutations: {
    update(state, a) {
      state.a += a
      console.log(state.a);
    }
  },
  actions: {
    change(store) {
      store.commit("update", 1);
    }
  },
  modules: {
    k210s : ModuleK210
  }
})