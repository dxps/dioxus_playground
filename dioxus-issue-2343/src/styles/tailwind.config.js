/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        extend: {
            colors: {
                'lilac': '#8a5bd6'
            }
        },
    },
    plugins: [],
};
