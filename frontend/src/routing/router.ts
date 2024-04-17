import { createRouter, createWebHistory } from "vue-router";
import routes  from "@/routing/routes";

/*
routesImport.map(
    (route: { path: String; name: String; componentName: String }) => {
      const component = function () {
        switch (route.componentName) {
          case "ButtonArray":
            return ButtonArrayVue;
          case "App":
            return App;
        }
      };
      return {
        path: route.path,
        name: route.name,
        component: component,
      };
    });
*/

const history = createWebHistory();
export default createRouter({
  history,
  routes,
});
