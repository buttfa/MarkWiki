import { createRouter, createWebHashHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import WorkspaceView from '../views/WorkspaceView.vue';
import EditorView from '../views/EditorView.vue';
import NotFoundView from '../views/NotFoundView.vue';
import ConflictResolverView from '../views/ConflictResolverView.vue';

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
  {
    path: '/workspace/:wikiName',
    name: 'workspace',
    component: WorkspaceView,
    props: true
  },
  {
    path: '/workspace/:wikiName/edit/:filePath(.*)',
    name: 'editor',
    component: EditorView,
    props: true
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: NotFoundView
  }
];

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    ...routes,
    {
      path: '/conflict-resolver',
      name: 'conflict-resolver',
      component: ConflictResolverView
    }
  ]
});

export default router;