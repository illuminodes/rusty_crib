/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors')
module.exports = {
    content: ['./src/**/*.{html,rs}', 'index.html'],

    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
    ],
    theme: {
        fontFamily: {
            'robomono': ['RobotoMono', 'sans-serif'],
        },
        colors: {
            transparent: 'transparent',
            current: 'currentColor',
            nostr: {
                dark: '#800080',
                DEFAULT: '#be29ec',
                light: '#d896ff',
            },
            black: colors.black,
            white: colors.white,
            green: '#16a34a',
            red: '#dc2626',
        },
    },
};
