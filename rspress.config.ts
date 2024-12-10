import * as path from 'path';
import { defineConfig } from 'rspress/config';

// plugins
import mermaid from 'rspress-plugin-mermaid';
import alignImage from 'rspress-plugin-align-image';
import { pluginRss } from '@rspress/plugin-rss';

export default defineConfig({
  base: '/makepad-book/',
  root: path.join(__dirname, 'docs'),
  title: 'Makepad Book',
  description: 'A book for makepad, want to quickly and easily to help you use makepad.',
  lang: 'zh',
  icon: '/favicon.ico',
  globalStyles: path.join(__dirname, 'theme', 'index.css'),
  plugins: [
    mermaid(),
    alignImage(),
    pluginRss({
      siteUrl: 'https://project-robius-china.github.io/makepad-book/',
    }),
  ],
  logo: {
    light: '/logo_makepad.svg',
    dark: '/logo_makepad.svg',
  },
  markdown: {
    showLineNumbers: true,
    checkDeadLinks: true,
  },
  route: {
    cleanUrls: true,
  },
  themeConfig: {
    enableContentAnimation: true,
    socialLinks: [
      { icon: 'github', mode: 'link', content: 'https://github.com/Project-Robius-China/makepad-book' },
    ],
    locales: [
      {
        lang: 'zh',
        label: '简体中文',
        editLink: {
          docRepoBaseUrl:
            'https://github.com/Project-Robius-China/makepad-book/tree/main/docs/zh',
          text: '📝 在 GitHub 上编辑此页',
        },
        searchNoResultsText: '未搜索到相关结果',
        searchPlaceholderText: '搜索文档',
        searchSuggestedQueryText: '可更换不同的关键字后重试',
      },
      {
        lang: 'en',
        label: 'English',
        editLink: {
          docRepoBaseUrl:
            'https://github.com/Project-Robius-China/makepad-book/tree/main/docs/en',
          text: '📝 Edit this page on GitHub',
        },
      }
    ]
  },
});
