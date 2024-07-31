import { defineConfig } from 'vitepress'
import VitePressSidebar from 'vitepress-sidebar';

const vitepressSidebarOptions = {
  documentRootPath: '/docs'
};

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "My Awesome Project",
  description: "A VitePress Site",
  lastUpdated: true,
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    editLink: {
      pattern: 'https://github.com/ryohidaka/zenn2press/edit/main/demo/:path'
    },

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present ryohidaka'
    },

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: VitePressSidebar.generateSidebar(vitepressSidebarOptions),

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
