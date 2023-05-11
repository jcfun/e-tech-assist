import { createApp } from 'vue';
import App from './App.vue';
import { initStyle } from './styles';
import { initRouter } from './router';
import { initStores } from './stores';
import '@/styles';

const app = createApp(App);
initStyle(app);
initRouter(app);
initStores(app);

app.mount('#app');
