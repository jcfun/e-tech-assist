import { createApp } from 'vue';
import store from './store';
import router from './router';
import App from './App.vue';
import './styles';
import initRouterGuard from './router/guard';
import { initGlobalComponent } from './layout';

const app = createApp(App);

app.use(store);
app.use(router);
initRouterGuard();
initGlobalComponent(app);

app.mount('#app');
