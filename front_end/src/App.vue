<template>
<Router class="router"></Router>
<div class="container">
  <Content class="left"></Content>
  <CataLog class="right"></CataLog>
</div>
</template>

<script>
import 'bootstrap/dist/css/bootstrap.min.css'
import CataLog from './components/CommuLog.vue'
import Content from './components/K210Catalog.vue'
import Router from './components/RouterCont.vue'
export default {
  name: 'App',
  components: {
    CataLog,
    Content,
    Router
  }
}
</script>

<style>
.container {
  display: flex;
}
.router{
  position: flex;
  top:0;
  width: 100%;
}
.left {
  flex: 2;
  top : 10;
  background-color: rgb(105, 94, 94);
  height: 100vh;
}
.right {
  flex: 5;
  top : 10;
  background-color: grey;
  height: 100vh;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
body {
  background-color: aliceblue;
}
</style>
