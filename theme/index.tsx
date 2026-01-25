import { useLang, usePageData } from 'rspress/runtime';
import Theme from 'rspress/theme';
import Announcement from './components/Announcement';
import { HomeLayout } from './pages/home';
import { useI18nUrl } from './i18n';

const Layout = () => {
    const { page } = usePageData();
    const lang = useLang();
    const tUrl = useI18nUrl();
    const ANNOUNCEMENT_URL= tUrl("/skills-and-components/index");
    return (
      <Theme.Layout
        beforeNav={
          <Announcement
            href={ANNOUNCEMENT_URL}
            message={
              lang === 'en'
                ? 'Developing a Makepad app? Use Agent SKILLS and the component library to speed up your development! ⚡️⚡️⚡️'
                : '在开发 Makepad 应用吗？快使用 Agent SKILLS和组件库帮助你更快的完成开发！⚡️⚡️⚡️'
            }
            localStorageKey="makepad-book-announcement-closed"
            display={page.pageType === 'home'}
          />
        }
      />
    )
};

export * from 'rspress/theme';

export default {
  ...Theme,
  Layout,
  HomeLayout,
};
