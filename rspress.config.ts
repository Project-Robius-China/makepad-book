import * as path from 'path';
import { defineConfig } from 'rspress/config';

// plugins
import mermaid from 'rspress-plugin-mermaid';
import alignImage from 'rspress-plugin-align-image';
import ga from 'rspress-plugin-google-analytics';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  title: 'Makepad Book',
  description: 'A book for makepad, want to quickly and easily to help you use makepad.',
  lang: 'en',
  icon: '/favicon.ico',
  globalStyles: path.join(__dirname, 'theme', 'index.css'),
  plugins: [
    mermaid(),
    alignImage(),
    ga({
      id: 'G-F4639SEF31',
    })
  ],
  logo: {
    light: '/logo_makepad.svg',
    dark: '/logo_makepad.svg',
  },
  markdown: {
    showLineNumbers: true,
    checkDeadLinks: true,
    highlightLanguages:[['rs', 'rust']]
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
            'https://github.com/Project-Robius-China/makepad-book/tree/main/docs',
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
            'https://github.com/Project-Robius-China/makepad-book/tree/main/docs',
          text: '📝 Edit this page on GitHub',
        },
      }
    ]
  },
});
