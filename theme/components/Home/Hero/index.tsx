import { memo, useCallback } from 'react';
import { useNavigate } from 'rspress/runtime';
import styles from './index.module.scss';

export interface HeroProps {
    logo: string;
    title: string;
    subTitle: string;
    description: string;
    getStartedButtonText?: string;
    goToWebsiteButtonText?: string;
    onClickGetStarted?: () => void;
    onClickGoToWebsite?: () => void;
}

const Hero = memo(({
    logo,
    title,
    subTitle,
    description,
    getStartedButtonText,
    goToWebsiteButtonText,
    onClickGetStarted,
    onClickGoToWebsite,
}: HeroProps) => {
  return (
    <section className={styles.hero}>
        <div className={styles['hero-inner']}>
            <div className={styles.logo}>
                <img className={styles['logo-img']} src={logo} alt='logo' />
            </div>
            <h1 className={styles.title}>{title}</h1>
            <p className={styles.subtitle}>{subTitle}</p>
            <p className={styles.description}>{description}</p>
            <div className={styles['button-group']}>
            <button className={styles['go-to-website']} onClick={onClickGoToWebsite}>{goToWebsiteButtonText}</button>
                <button className={styles['get-started']} onClick={onClickGetStarted}>{getStartedButtonText}</button>
            </div>
        </div>
    </section>
  );
});

export default Hero;
