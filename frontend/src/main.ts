import { createApp } from "vue";
// @ts-ignore
import App from "./App.vue";

import "vuetify/styles";
import router from "./routing/router";
import "./assets/main.css";
import vuetify from "./plugins/vuetify";



createApp(App).use(router).use(vuetify).mount("#app");
