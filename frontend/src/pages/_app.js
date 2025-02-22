import React from "react";
import { QueryClient, QueryClientProvider } from "react-query";
import Navbar from "./Navbar";
import "./styles.css";

const queryClient = new QueryClient();

function MyApp({ Component, pageProps }) {
  return (
    <QueryClientProvider client={queryClient}>
      <Navbar />
      <Component {...pageProps} />
    </QueryClientProvider>
  );
}

export default MyApp;