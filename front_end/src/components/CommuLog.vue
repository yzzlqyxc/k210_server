<template>
  <div>
    <div class="uploadfile" v-if="check != ``">
        <input class="form-control" type="file" id="formFile" @change="fileChoose">
        <button type="button" @click="uploadFile" class="btn btn-primary">上传照片</button>
    </div>
  </div>
</template>

<script>
import $ from 'jquery';
import store from '../store';
import { computed } from 'vue';
import { useStore } from 'vuex';

export default {
  setup() {

  },
  data() {
    const stores = useStore();
    return {
        file_input : null,
        check : computed(() => stores.state.k210s.remotePort)
    }
  },
  methods : {
    fileChoose(evnet) {
      this.file_input = evnet.target.files[0]
    },
    async uploadFile() {
      const file = this.file_input
      if (!file) {
        console.log("no file")
        return
      }
      console.log(this.file_input);
      var data = new FormData()
      data.append('pic', file)
      data.append('port', store.state.k210s.remotePort)
      console.log(store.state.k210s.remotePort);
      $.ajax({
        url: "http://47.93.124.97:3000/getPic",
        type : "POST",
        data : data,
        cache: false,
        processData: false,
        contentType: false,
        success(resp) {
          console.log(resp);
        }, 
        error(resp) {
          console.log(resp);
        }

      })
    }
  }
}

</script>

<style>
.uploadfile{
  display:flex;
}
</style>