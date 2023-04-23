import { createRouter, createWebHashHistory } from 'vue-router';
import { commonRoutes, perLoadPathRoute } from './common';
import type { App } from 'vue';

const router = createRouter({
  history: createWebHashHistory(),
  routes: [...commonRoutes, perLoadPathRoute],
});

export default router;

export function initRouter(app: App) {
  app.use(router);
}
