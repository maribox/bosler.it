import Home from "@/views/Home.vue";
import FilePage from "@/views/FilePage.vue";

const routes = [
    {
        path: "/",
        name: "bosler.it",
        component: Home,
    },
    {
        path: "/files",
        name: "Files",
        component: FilePage,
    },
    /*{
      "path": "/aboutme",
      "name": "About Me",
      "componentName": "AboutMe"
    },*/
    /*{
      "path": "/contact",
      "name": "Contact",
      "componentName": "Contact"
    }*/
];


export default routes;