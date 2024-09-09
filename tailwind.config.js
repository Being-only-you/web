const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ['Kumbh Sans', ...defaultTheme.fontFamily.sans],
      },
      colors: {
        white: '#FFFFFF',
        purple: {
          DEFAULT: '#26217F',
          light: '#6E6BAA',
          dark: '#191655',
          darker: '#0D0B2A',
        },
        red: {
          DEFAULT: '#DB4E16',
          dark: '#B23E11',
        },
        orange: '#F7A029',
        blue: '#325BE0',
        green: '#34D399',
        teal: '#14B8A6',
        gray: {
          DEFAULT: '#808080',
          light: '#D4D4D4',
          dark: '#4D4D4D',
        },
      },
      animation: {
        float: 'float 6s ease-in-out infinite',
        'float-delayed': 'float 6s ease-in-out 2s infinite',
        fadeIn: 'fadeIn 1s ease-in',
        slideUp: 'slideUp 0.5s ease-out',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-20px)' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(100px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}