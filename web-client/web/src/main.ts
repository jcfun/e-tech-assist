import { createApp } from 'vue';
import { initStyle } from './styles';

import App from './App.vue';

import { initRouter } from './router';

const app = createApp(App);
initStyle(app);
initRouter(app);

app.mount('#app');
