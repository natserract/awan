import '../styles/globals.css'
import type { AppProps } from 'next/app'
import Head from 'next/head'
import React from 'react'
import Layout from '../containers/layout';

const HeadDocument = () => (
  <Head>
    <title>ShakeTree - S3 Manager</title>
  </Head>
)

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <React.Fragment>
      <HeadDocument />
      <div suppressHydrationWarning>
        <Layout>
          {typeof window && <Component {...pageProps} />}
        </Layout>
      </div>
    </React.Fragment>
  )

}
export default MyApp
