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
        gray: {
          DEFAULT: '#808080',
          light: '#D4D4D4',
          dark: '#4D4D4D',
        },
      },
      animation: {
        fadeIn: 'fadeIn 1s ease-in',
        slideUp: 'slideUp 0.5s ease-out',
        bounce: 'bounce 1s infinite',
        pulse: 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        ping: 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
        spin: 'spin 1s linear infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(100px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        ping: {
          '75%, 100%': { transform: 'scale(2)', opacity: '0' },
        },
        spin: {
          'to': { transform: 'rotate(360deg)' },
        },
      },
    },
  },
  plugins: [],
}