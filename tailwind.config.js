// /** @type {import('tailwindcss').Config} */
// module.exports = {
//     content: ["*.html", "./src/**/*.rs", "./preline/*.js", "./posts/**/*.md"],
//     theme: {
//         screens: {
//             sm: '480px',
//             md: '768px',
//             lg: '1020px',
//             xl: '1440px',
//         },
//         fontFamily: {
//             sans: ['Inter', 'sans-serif'],

//         },
//         extend: {
//             //https://play.tailwindcss.com/VCZwwz1e3R
//             animation: {
//                 text: 'text 5s ease infinite',
//             },
//             keyframes: {
//                 text: {
//                     '0%, 100%': {
//                         'background-size': '200% 200%',
//                         'background-position': 'left center',
//                     },
//                     '50%': {
//                         'background-size': '200% 200%',
//                         'background-position': 'right center',
//                     },
//                 },
//             },



//             typography: ({ theme }) => ({
//                 blog: {
//                     css: {
//                         '--tw-prose-body': "#ddd",
//                         '--tw-prose-headings': "#F8F9FA",
//                         '--tw-prose-lead': "#F8F9FA",
//                         '--tw-prose-links': "#F8F9FA",
//                         '--tw-prose-bold': "#F8F9FA",
//                         '--tw-prose-counters': "#F8F9FA",
//                         '--tw-prose-bullets': "#F8F9FA",
//                         '--tw-prose-hr': "#ddd",
//                         '--tw-prose-quotes': "#F8F9FA",
//                         '--tw-prose-quote-borders': "#ddd",
//                         '--tw-prose-captions': "#ddd",
//                         '--tw-prose-code': "#F8F9FA",
//                         '--tw-prose-pre-bg': "#14213D",
//                         '--tw-prose-th-borders': "#ddd",
//                         '--tw-prose-td-borders': "#F8F9FA",
//                         // '--tw-prose-invert-body': theme('colors.pink[200]'),
//                         // '--tw-prose-invert-headings': theme('colors.white'),
//                         // '--tw-prose-invert-lead': "#ddd",
//                         // '--tw-prose-invert-links': theme('colors.white'),
//                         // '--tw-prose-invert-bold': theme('colors.white'),
//                         // '--tw-prose-invert-counters': theme('colors.pink[400]'),
//                         // '--tw-prose-invert-bullets': theme('colors.pink[600]'),
//                         // '--tw-prose-invert-hr': "#ddd",
//                         // '--tw-prose-invert-quotes': theme('colors.pink[100]'),
//                         // '--tw-prose-invert-quote-borders': "#ddd",
//                         // '--tw-prose-invert-captions': theme('colors.pink[400]'),
//                         // '--tw-prose-invert-code': theme('colors.white'),
//                         // '--tw-prose-invert-pre-code': "#ddd",
//                         // '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
//                         // '--tw-prose-invert-th-borders': theme('colors.pink[600]'),
//                         // '--tw-prose-invert-td-borders': "#ddd",
//                     },
//                 },
//             }),
//         },
//     },
//     plugins: [require("preline/plugin"), require("@tailwindcss/typography")],
// }

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["./web/*.html", "./web/src/**/*.rs", "./web/public/posts/*.md"],
    },
    theme: {
        extend: {
            keyframes: {
                fadein: {
                    '0%': { opacity: 0 },
                    '100%': { opacity: 1 },
                },
            },
            animation: {
                'fadein-1': 'fadein 1s ease-in-out 0.25s forwards',
                'fadein-2': 'fadein 1s ease-in-out 1.5s forwards',
                'fadein-3': 'fadein 1s ease-in-out 2.75s forwards',
            },
            backgroundImage: {
                'lapce-gradient': 'linear-gradient(to right top, #4264af, #4f70ba, #5b7dc4, #688acf, #7597d9, #6ca0e0, #63a9e6, #5ab2eb, #2eb9e7, #00bfdd, #00c3cd, #10c6ba)',
                'fingermap-gradient': 'linear-gradient(to right top, #b4014b, #d53e4f, #f46d43, #fdae61, #fee08b, #e6f598, #abdda4, #66c2a5, #3288bd, #6b5ab8)',
                'heatmap-gradient': 'linear-gradient(to right top, #ff2020, #427880)',
                'search-icon': 'url(/public/images/search.svg)',
            },
            backgroundPosition: {
                'right-margin': 'calc(100% - 0.5rem) 0.25rem',
            },
            backgroundSize: {
                '6': '1.5rem',
            },
            colors: {
                'header': '#242424',
                'darker': '#1a1a1a',
                'hovered': '#ffffff10',
                'txt': "#eee",
            },
            gridTemplateColumns: {
                'homepage': '1fr 2fr',
                'metadata': '1fr 4fr',
                'analyzer': '4fr 1fr',
            },
            typography: ({ theme }) => ({
                posts: {
                    css: {
                        '--tw-prose-body': "#eee",
                        '--tw-prose-headings': "#eee",
                        '--tw-prose-lead': "#eee",
                        '--tw-prose-links': "#eee",
                        '--tw-prose-bold': "#eee",
                        '--tw-prose-counters': "#eee",
                        '--tw-prose-bullets': "#eee",
                        '--tw-prose-hr': "#ddd",
                        '--tw-prose-quotes': "#eee",
                        '--tw-prose-quote-borders': "#ddd",
                        '--tw-prose-captions': "#ddd",
                        '--tw-prose-code': "#eee",
                        '--tw-prose-pre-bg': "#1a1a1a",
                        '--tw-prose-th-borders': "#ddd",
                        '--tw-prose-td-borders': "#eee",
                        // '--tw-prose-invert-body': theme('colors.pink[200]'),
                        // '--tw-prose-invert-headings': theme('colors.white'),
                        // '--tw-prose-invert-lead': "#ddd",
                        // '--tw-prose-invert-links': theme('colors.white'),
                        // '--tw-prose-invert-bold': theme('colors.white'),
                        // '--tw-prose-invert-counters': theme('colors.pink[400]'),
                        // '--tw-prose-invert-bullets': theme('colors.pink[600]'),
                        // '--tw-prose-invert-hr': "#ddd",
                        // '--tw-prose-invert-quotes': theme('colors.pink[100]'),
                        // '--tw-prose-invert-quote-borders': "#ddd",
                        // '--tw-prose-invert-captions': theme('colors.pink[400]'),
                        // '--tw-prose-invert-code': theme('colors.white'),
                        // '--tw-prose-invert-pre-code': "#ddd",
                        // '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
                        // '--tw-prose-invert-th-borders': theme('colors.pink[600]'),
                        // '--tw-prose-invert-td-borders': "#ddd",
                    },
                },
                DEFAULT: {
                    css: {
                        maxWidth: 'min(70ch, 95%)',
                        minWidth: '50%',
                    }
                }
            }),
        },
    },
    plugins: [
        require("@tailwindcss/typography"),
        function ({ addUtilities }) {
            const newUtilities = {
                '.container-inline-size': {
                    'container-type': 'inline-size',
                },
                '.br': {
                    'border': '1px solid red'
                },
            };
            addUtilities(newUtilities);
        },
    ],
}
