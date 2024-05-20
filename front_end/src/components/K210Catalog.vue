<template>
<div>
<ul class="list-group">

  <li class="list-group-item list-group-item-dark">当前正在连接中的开发板</li>
  <div v-if="flag == 0">
    <li class="list-group-item"> 当前无开发板连接 </li>
  </div>
  <div v-for="(a, idx) in loadedK210s" :key="idx" >
    <li @click="changePort(a)" class="list-group-item list-group-item-action"> {{ a }}</li>
  </div> 
</ul>

</div>
</template>

<style scoped>
body {
    background-color: red;
}
</style>

<script>
import { computed} from 'vue';
import { useStore } from 'vuex';
import store from '../store';

export default {
  setup() {
    const store = useStore();
    setInterval(() => {
      store.dispatch("uploadK210s", {
        success(resp) {
          console.log(resp)
        }, 
        error(resp) {
          console.log(resp)
        }
      });
    }, 2000);
    setInterval(() => {
      store.dispatch("uploadHistories");
    }, 500);

    return {
      loadedK210s : computed(() => store.state.k210s.loadedK210s),
      flag : computed(() => store.state.k210s.loadedK210s.length),
    }
  }, 
  methods: {
    changePort(a) {
      store.commit("updatePort", a);
    }
  }
}

</script>