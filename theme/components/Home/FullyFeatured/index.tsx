import { type CSSProperties, memo } from "react";
import { useI18n, useI18nUrl } from "../../../i18n";
import styles from "./index.module.css";

const FullyFeatured = memo(() => {
    const t = useI18n();
    const tUrl = useI18nUrl();

    const bullets = [
        t("fullyFeaturedBullet1"),
        t("fullyFeaturedBullet2"),
        t("fullyFeaturedBullet3"),
        t("fullyFeaturedBullet4"),
    ];

    const highlights = [
        {
            title: t("fullyFeaturedHighlight1Title"),
            desc: t("fullyFeaturedHighlight1Desc"),
        },
        {
            title: t("fullyFeaturedHighlight2Title"),
            desc: t("fullyFeaturedHighlight2Desc"),
        },
        {
            title: t("fullyFeaturedHighlight3Title"),
            desc: t("fullyFeaturedHighlight3Desc"),
        },
        {
            title: t("fullyFeaturedHighlight4Title"),
            desc: t("fullyFeaturedHighlight4Desc"),
        },
    ];

    const cards = [
        {
            tag: t("fullyFeaturedCardSkillsTag"),
            title: t("fullyFeaturedCardSkillsTitle"),
            desc: t("fullyFeaturedCardSkillsDesc"),
            href: tUrl("/skills-and-components/skills/makepad-skills"),
            cta: t("fullyFeaturedCardSkillsCta"),
        },
        {
            tag: t("fullyFeaturedCardComponentTag"),
            title: t("fullyFeaturedCardComponentTitle"),
            desc: t("fullyFeaturedCardComponentDesc"),
            href: tUrl("/skills-and-components/components/makepad-component"),
            cta: t("fullyFeaturedCardComponentCta"),
        },
        {
            tag: t("fullyFeaturedCardFlowTag"),
            title: t("fullyFeaturedCardFlowTitle"),
            desc: t("fullyFeaturedCardFlowDesc"),
            href: tUrl("/skills-and-components/components/makepad-flow"),
            cta: t("fullyFeaturedCardFlowCta"),
        },
        {
            tag: t("fullyFeaturedCardChartTag"),
            title: t("fullyFeaturedCardChartTitle"),
            desc: t("fullyFeaturedCardChartDesc"),
            href: tUrl("/skills-and-components/components/makepad-chart"),
            cta: t("fullyFeaturedCardChartCta"),
        },
        {
            tag: t("fullyFeaturedCardD3Tag"),
            title: t("fullyFeaturedCardD3Title"),
            desc: t("fullyFeaturedCardD3Desc"),
            href: tUrl("/skills-and-components/components/makepad-d3"),
            cta: t("fullyFeaturedCardD3Cta"),
        },
        {
            tag: t("fullyFeaturedCardMolyTag"),
            title: t("fullyFeaturedCardMolyTitle"),
            desc: t("fullyFeaturedCardMolyDesc"),
            href: tUrl("/skills-and-components/components/moly-kit"),
            cta: t("fullyFeaturedCardMolyCta"),
        },
    ];

    return (
        <section className={styles.fullyFeatured}>
            <div className={styles.inner}>
                <header className={styles.header}>
                    <span className={styles.kicker}>{t("fullyFeaturedKicker")}</span>
                    <h2 className={styles.title}>{t("fullyFeaturedTitle")}</h2>
                    <p className={styles.lead}>{t("fullyFeaturedLead")}</p>
                </header>

                <div className={styles.overview}>
                    <div className={styles.summary}>
                        <h3 className={styles.summaryTitle}>{t("fullyFeaturedSummaryTitle")}</h3>
                        <p className={styles.summaryBody}>{t("fullyFeaturedSummaryBody")}</p>
                        <ul className={styles.bullets}>
                            {bullets.map((item) => (
                                <li key={item}>{item}</li>
                            ))}
                        </ul>
                    </div>

                    <div className={styles.facts}>
                        {highlights.map((item, index) => (
                            <div
                                className={styles.factCard}
                                key={item.title}
                                style={{ "--delay": `${0.1 + index * 0.08}s` } as CSSProperties}
                            >
                                <span className={styles.factTitle}>{item.title}</span>
                                <p className={styles.factDesc}>{item.desc}</p>
                            </div>
                        ))}
                    </div>
                </div>

                <div className={styles.cardGrid}>
                    {cards.map((card, index) => (
                        <article
                            className={styles.featureCard}
                            key={card.title}
                            style={{ "--delay": `${0.05 + index * 0.06}s` } as CSSProperties}
                        >
                            <span className={styles.cardTag}>{card.tag}</span>
                            <h3 className={styles.cardTitle}>{card.title}</h3>
                            <p className={styles.cardDesc}>{card.desc}</p>
                            <a className={styles.cardLink} href={card.href}>
                                {card.cta}
                                <span className={styles.linkArrow} aria-hidden="true">
                                    &gt;
                                </span>
                            </a>
                        </article>
                    ))}
                </div>
            </div>
        </section>
    )
})

export default FullyFeatured;
