import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "yrnu docs",
    description: "Yrnu Documentation",
    base:"/yrnu/",
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            { text: "Home", link: "/" },
            { text: "Examples", link: "/examples" },
        ],

        sidebar: [
            {
                text: "Pages",
                items: [
                    { text: "Installation", link: "/installation" },
                    {
                        text: "Lua",
                        link: "/lua",
                        items: [
                            { text: "Core Utils", link: "/lua_core" },
                            { text: "Yrnu global", link: "/yrnu_global" },
                            { text: "Creating a Plugin", link: "/lua_plugin" },
                            { text: "Creating a library", link: "/lua_lib" },
                        ],
                    },
                    { text: "Cli Usage", link: "/cli_usage" },
                    { text: "Avilable Plugins", link: "/plugins" },
                    { text: "Avilable Libraries", link: "/libs" },
                ],
            },
        ],

        socialLinks: [
            { icon: "github", link: "https://github.com/orionbell/yrnu" },
            { icon: "linkedin", link: "https://linkedin.com/in/yishai-shubeli-977071367" },
        ],
    },
});
