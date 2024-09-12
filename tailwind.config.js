const defaultTheme = require('tailwindcss/defaultTheme')
const plugin = require('tailwindcss/plugin')


/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      current: 'currentColor',
      transparent: 'transparent',

      black: '#000',
      white: '#fff',

      professional: {
        100: '#C4C4C4',
        300: '#888888',
        500: '#4D4D4D',
        700: '#333333',
        900: '#1A1A1A',
      },
      social: {
        100: '#BBC8F5',
        300: '#7692EA',
        500: '#325BE0',
        700: '#213D95',
        900: '#111E4B',
      },
      power: {
        100: '#B7B5D4',
        300: '#6E6BAA',
        500: '#26217F',
        700: '#191655',
        900: '#0D0B2A',
      },
      extroverted: {
        100: '#FCDFB8',
        300: '#FAC070',
        500: '#F7A029',
        700: '#A56B1B',
        900: '#52350E',
      },
      expression: {
        100: '#F3C4B1',
        300: '#E78964',
        500: '#DB4E16',
        700: '#92340F',
        900: '#491A07',
      },
      honest: {
        100: '#FFFFFF',
        200: '#F8FAFC',
        300: '#BFBFBF',
        500: '#808080',
        700: '#404040',
        900: '#000000',
      },
    },
    backgroundPosition: {
      bottom: 'bottom',
      center: 'center',
      left: 'left',
      'left-bottom': 'left bottom',
      'left-top': 'left top',
      right: 'right',
      'right-bottom': 'right bottom',
      'right-top': 'right top',
      top: 'top',
    },
    borderRadius: {
      none: '0',
      sm: '0.125rem',
      DEFAULT: '0.375rem',
      lg: '0.5rem',
      xl: '0.75rem',
      '2xl': '1rem',
      '3xl': '1.5rem',
      '4xl': '2rem',
      '6xl': '3rem',
      '12xl': '6rem',
      full: '9999px',
    },
    boxShadow: {
      sm: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
      DEFAULT: '0px 1px 2px rgba(148, 163, 184, 0.35), 0px 6px 6px rgba(203, 213, 225, 0.15)',
      md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
      lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
      xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
      '2xl': '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
      inner: 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)',
      none: 'none',
    },
    fontSize: {
      xs: '0.75rem',
      sm: '0.875rem',
      base: '1rem',
      paragraph: '1rem',
      headings: '1.667rem',
      subtitle: '2.037rem',
      title: '4.813rem'
    },
    fontWeight: {
      light: '300',
      normal: '400',
      bold: '700'
    },
    body: 'KumbhSans, "DM Sans", ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"',
    heading: 'KumbhSans, "DM Sans", ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"',
    sans: 'KumbhSans, "DM Sans", ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"',
    extend: {
      animation: {
        float: 'float 6s ease-in-out',
        'float-delayed': 'float 6s ease-in-out 2s',
        fadeIn: 'fadeIn 1s ease-in',
        slideUp: 'slideUp 0.5s ease-out',
        weave: 'weave 5s ease-in-out infinite',
        'weave-reverse': 'weave-reverse 5s ease-in-out infinite',
        expand: 'expand 0.3s ease-out forwards',
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
        weave: {
          '0%, 100%': { transform: 'translateX(0)' },
          '50%': { transform: 'translateX(20px)' },
        },
        'weave-reverse': {
          '0%, 100%': { transform: 'translateX(0)' },
          '50%': { transform: 'translateX(-20px)' },
        },
        expand: {
          '0%': { transform: 'scale(1)' },
          '100%': { transform: 'scale(1.05)' },
        },
      },
      height: {
        minusHeader: 'calc(100dvh - 170px)'
      }
    },
  },
  plugins: [
    plugin(function({ addUtilities, theme, addBase }) {
      addBase({
        ':root': {
          '--font-size-min': '17',
          '--font-size-max': '20',
          '--font-ratio-min': '1.2',
          '--font-ratio-max': '1.33',
          '--font-width-min': '320',
          '--font-width-max': '1500',
        },
        'body': {
          fontFamily: theme('fontFamily.sans'),
        },
      });
      addUtilities({
        '.fluid': {
          '--fluid-min': 'calc(var(--font-size-min) * pow(var(--font-ratio-min), var(--font-level, 0)))',
          '--fluid-max': 'calc(var(--font-size-max) * pow(var(--font-ratio-max), var(--font-level, 0)))',
          '--fluid-preferred': 'calc((var(--fluid-max) - var(--fluid-min)) / (var(--font-width-max) - var(--font-width-min)))',
          '--fluid-type': 'clamp((var(--fluid-min) / 16) * 1rem, ((var(--fluid-min) / 16) * 1rem) - (((var(--fluid-preferred) * var(--font-width-min)) / 16) * 1rem) + (var(--fluid-preferred) * var(--variable-unit, 100vi)), (var(--fluid-max) / 16) * 1rem)',
          'font-size': 'var(--fluid-type)',
        },
        '.fluid--1': { '--font-level': '-1' },
        '.fluid-0': { '--font-level': '0' },
        '.fluid-1': { '--font-level': '1' },
        '.fluid-2': { '--font-level': '2' },
        '.fluid-3': { '--font-level': '3' },
        '.fluid-4': { '--font-level': '4' },
        '.fluid-5': { '--font-level': '5' },
        '.fluid-body': {
          '--font-level': '0',
          'font-size': 'var(--fluid-type)',
        },
      });
    }),
  ],
}