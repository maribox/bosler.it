import {createVuetify} from "vuetify";
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import type {ThemeDefinition} from "vuetify/vuetify";


const darkTheme: ThemeDefinition  = {
    dark: true,
    colors: {
        background: "linear-gradient(120deg, #000000 0%, #77230a 100%);",
        surface: '#1e1e1e',
        primary: '#77230a',
        'primary-darken-1': '#1a284c',
        secondary: '#03DAC6',
        'secondary-darken-1': '#018786',
        error: '#B00020',
        info: '#2196F3',
        success: '#4CAF50',
        warning: '#FB8C00',
        'on-background': '#c2c2c2',
    }
}


export default createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'darkTheme',
        themes: {
            darkTheme
        }
    }
})