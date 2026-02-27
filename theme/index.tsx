import { Layout as BasicLayout } from '@rspress/core/theme-original';
import { HomeLayout } from './pages/home';

const Layout = () => {
    return (
      <BasicLayout />
    )
};

export { Layout, HomeLayout };
export * from '@rspress/core/theme-original';
