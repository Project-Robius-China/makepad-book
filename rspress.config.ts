import * as path from 'path';
import { defineConfig } from '@rspress/core';

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
    link: {
      checkDeadLinks: true,
    },
    shiki: {
      langAlias: {
        rs: 'rust',
      },
    },
  },
  route: {
    cleanUrls: true,
  },
  locales: [
    {
      lang: 'zh',
      label: '简体中文',
    },
    {
      lang: 'en',
      label: 'English',
    }
  ],
  themeConfig: {
    enableContentAnimation: true,
    editLink: {
      docRepoBaseUrl:
        'https://github.com/Project-Robius-China/makepad-book/tree/main/docs',
    },
    socialLinks: [
      { icon: 'github', mode: 'link', content: 'https://github.com/Project-Robius-China/makepad-book' },
    ],
  },
});
