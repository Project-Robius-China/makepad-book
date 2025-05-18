import { useLang, usePageData } from 'rspress/runtime';
import Theme from 'rspress/theme';
import Announcement from './components/Announcement';
import { HomeLayout } from './pages/home';
import { useI18nUrl } from './i18n';

const Layout = () => {
    const { page } = usePageData();
    const lang = useLang();
    const tUrl = useI18nUrl();
    const ANNOUNCEMENT_URL= tUrl("https://makepad.nl/");
    return (
      <Theme.Layout
        beforeNav={
          <Announcement
            href={ANNOUNCEMENT_URL}
            message={
              lang === 'en'
                ? 'We are proud to release Makepad 1.0!🦀️, click to know more details.'
                : '我们隆重推出 Makepad 1.0！🦀️, 点击查看更多细节'
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
