<template>
  <Header></Header>
  <Content></Content>
  <Footer></Footer>
</template>

<script setup lang="ts">
  import Header from './Header.vue';
  import Content from './Content.vue';
  import Footer from './Footer.vue';
  import party from 'party-js';
  document.body.addEventListener('click', function (e) {
    party.sparkles(e, {
      count: party.variation.range(5, 10),
      speed: party.variation.range(150, 200),
      size: party.variation.range(0.5, 0.8),
    });
  });
</script>
