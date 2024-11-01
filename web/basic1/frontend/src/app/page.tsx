'use client'

import Image from "next/image";
import styles from "./page.module.css";
import Square from "@/components/Square";
import HogeFetcher from "@/components/HogeFetcher";
import { Provider, Client, cacheExchange, fetchExchange } from 'urql';

const client = new Client({
  url: 'http://localhost:8000/graphql',
  exchanges: [cacheExchange, fetchExchange],
});

export default function Home() {
  return (
  <>
    <Provider value={client}>
      <div className={styles.page}>
        <main className={styles.main}>
          <Image
            className={styles.logo}
            src="/next.svg"
            alt="Next.js logo"
            width={180}
            height={38}
            priority
          />
          <Square/>
          <HogeFetcher/>
        </main>
        <footer className={styles.footer}>
          <a
            href="https://nextjs.org?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Image
              aria-hidden
              src="/globe.svg"
              alt="Globe icon"
              width={16}
              height={16}
            />
            Go to nextjs.org →
          </a>
        </footer>
    </div>
    </Provider>
    </>
  );
}
