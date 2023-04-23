import { createApp } from 'vue';
import App from './App.vue';
import './styles';
import initRouterGuard from './router/guard';
import { initGlobalComponent } from './layout';
import { initRouter } from './router';
import { initStore } from './store';

const app = createApp(App);

initStore(app);
initRouter(app);
initRouterGuard();
initGlobalComponent(app);

app.mount('#app');
