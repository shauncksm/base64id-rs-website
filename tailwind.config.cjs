const colors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
    content: ["./src-web/**/*.{html,svelte,js,svg}"],
    theme: {
        fontSize: {
            ...defaultTheme.fontSize,
            sm: ["0.875rem", "1.15rem"],
            base: ["1.125rem", "1.4rem"],
            lg: ["1.25rem", "1.65rem"],
            '2xl': ["1.625rem", "2rem"],
        },
        extend: {
            colors: {
                gray: colors.zinc,
            },
            fontFamily: {
                'sans': ["Source Sans Pro", ...defaultTheme.fontFamily.sans],
                'mono': ["Source Code Pro", ...defaultTheme.fontFamily.mono],
            },
        },
    },
    plugins: [],
}