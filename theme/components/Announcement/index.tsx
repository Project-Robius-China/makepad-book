import { useEffect, useState } from "react";
import IconCloseCircle from './close';
import styles from './index.module.css';

interface AnnouncementProps {
    href: string;
    message: string;
    localStorageKey: string;
    display: boolean;
}

const Announcement = (props: AnnouncementProps) => {

    const { href, message, localStorageKey, display } = props;

    if (!display) {
        return null;
    }

    const [disable, setDisable] = useState(false);

    useEffect(() => {
        if (typeof window === 'undefined') {
            return;
        }
        setDisable(window.localStorage.getItem(localStorageKey) === 'true');
    }, [localStorageKey]);

    if (disable) {
        return null;
    }

    return (
        <div className={styles.bar}>
            <a href={href} className={styles.link}>
                {message}
            </a>
            <IconCloseCircle
                onClick={() => {
                setDisable(true);
                if (typeof window !== 'undefined') {
                    window.localStorage.setItem(localStorageKey, 'true');
                }
                }}
                className={styles.close}
            />
        </div>
    )
}

export default Announcement;
