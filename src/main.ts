import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.ts";
import mavonEditor from 'mavon-editor';
import 'mavon-editor/dist/css/index.css';

const app = createApp(App);
app.use(router);
app.use(mavonEditor);
app.mount("#app");
