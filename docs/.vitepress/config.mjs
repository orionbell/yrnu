import { defineConfig } from "vitepress";

export default defineConfig({
    title: "yrnu docs",
    base: "/yrnu/",
    themeConfig: {
        nav: [
            { text: "Home", link: "/" },
            { text: "Examples", link: "/examples" },
        ],
        sidebar: [
            {
                text: "Pages",
                items: [
                    { text: "Introduction", link: "/introduction" },
                    { text: "Installation", link: "/installation" },
                    {
                        text: "Lua",
                        link: "/lua",
                        items: [
                            { text: "Core Utils", link: "/lua_core" },
                            { text: "Yrnu global", link: "/yrnu_global" },
                            { text: "Creating plugins", link: "/lua_plugin" },
                            { text: "Using a plugin", link: "/plugin_usage" },
                            { text: "libraries", link: "/lua_lib" },
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
        ],
    },
});
